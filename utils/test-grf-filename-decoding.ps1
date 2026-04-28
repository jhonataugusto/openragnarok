param(
    [string]$Root = (Join-Path $PSScriptRoot "korangar\korangar\archive\data"),
    [int]$FromCodePage = 1252,
    [int]$ToCodePage = 949,
    [string]$ReportPath = "",
    [int]$PreviewLimit = 200,
    [int]$MaxItems = 0,
    [int]$ProgressEvery = 1,
    [int]$ScanProgressEvery = 500,
    [switch]$DirectoriesOnly,
    [switch]$IncludeUnchanged,
    [switch]$Apply
)

$ErrorActionPreference = "Stop"
[Console]::OutputEncoding = [System.Text.UTF8Encoding]::new()

try {
    $providerType = [System.Type]::GetType("System.Text.CodePagesEncodingProvider")
    if ($providerType -ne $null) {
        [System.Text.Encoding]::RegisterProvider($providerType::Instance)
    }
}
catch {
    # Windows PowerShell already exposes Windows code pages without a provider.
}

function New-StrictEncoding {
    param([int]$CodePage)

    return [System.Text.Encoding]::GetEncoding(
        $CodePage,
        [System.Text.EncoderExceptionFallback]::new(),
        [System.Text.DecoderExceptionFallback]::new()
    )
}

function Convert-MojibakeName {
    param(
        [string]$Name,
        [System.Text.Encoding]$SourceEncoding,
        [System.Text.Encoding]$TargetEncoding
    )

    try {
        $bytes = $SourceEncoding.GetBytes($Name)
        return $TargetEncoding.GetString($bytes)
    }
    catch {
        return $null
    }
}

function Get-RelativePathCompat {
    param(
        [string]$BasePath,
        [string]$TargetPath
    )

    $baseFullPath = [System.IO.Path]::GetFullPath($BasePath).TrimEnd('\', '/') + [System.IO.Path]::DirectorySeparatorChar
    $targetFullPath = [System.IO.Path]::GetFullPath($TargetPath)

    $baseUri = [System.Uri]::new($baseFullPath)
    $targetUri = [System.Uri]::new($targetFullPath)
    $relativeUri = $baseUri.MakeRelativeUri($targetUri)

    return [System.Uri]::UnescapeDataString($relativeUri.ToString()).Replace('/', [System.IO.Path]::DirectorySeparatorChar)
}

$resolvedRoot = [System.IO.Path]::GetFullPath($Root)

if (-not (Test-Path -LiteralPath $resolvedRoot -PathType Container)) {
    throw "Pasta alvo nao encontrada: $resolvedRoot"
}

$sourceEncoding = New-StrictEncoding -CodePage $FromCodePage
$targetEncoding = New-StrictEncoding -CodePage $ToCodePage
$invalidChars = [System.IO.Path]::GetInvalidFileNameChars()

Write-Host "[config] raiz: $resolvedRoot"
Write-Host "[config] conversao: codepage $FromCodePage -> codepage $ToCodePage"
if ($Apply) {
    Write-Host "[apply] MODO RENOMEACAO ATIVO"
}
else {
    Write-Host "[dry-run] nada sera renomeado por este script"
}
if ($DirectoriesOnly) { Write-Host "[dry-run] modo: somente diretorios" }
if ($MaxItems -gt 0) { Write-Host "[dry-run] limite de analise: $MaxItems itens" }
Write-Host ""

$renameResults = [System.Collections.Generic.List[object]]::new()
$previewResults = [System.Collections.Generic.List[object]]::new()
$reportResults = if ($ReportPath -ne "") { [System.Collections.Generic.List[object]]::new() } else { $null }

$itemsAnalyzed = 0
$notEncodable = 0
$invalidTargetNames = 0
$unchanged = 0
$ScanProgressEvery = [Math]::Max(1, $ScanProgressEvery)

$rootWithSeparator = $resolvedRoot.TrimEnd('\', '/') + [System.IO.Path]::DirectorySeparatorChar

$childItemParams = @{
    LiteralPath = $resolvedRoot
    Recurse = $true
    Force = $true
}

if ($DirectoriesOnly) {
    $childItemParams["Directory"] = $true
}

$totalItemsForScan = 0

if ($Apply -and $MaxItems -eq 0) {
    Write-Host "[apply] fase 1/4: contando arquivos para mostrar porcentagem"

    foreach ($item in Get-ChildItem @childItemParams) {
        $totalItemsForScan += 1

        if ($totalItemsForScan -eq 1 -or ($totalItemsForScan % $ScanProgressEvery) -eq 0) {
            Write-Host ("[count] {0} itens encontrados..." -f $totalItemsForScan)
        }
    }

    Write-Host ("[apply] fase 1/4 concluida: {0} itens encontrados" -f $totalItemsForScan)
    Write-Host "[apply] fase 2/4: varredura e validacao"
}
elseif ($Apply) {
    Write-Host "[apply] fase 1/3: varredura e validacao"
}

foreach ($item in Get-ChildItem @childItemParams) {
    $itemsAnalyzed += 1

    if ($MaxItems -gt 0 -and $itemsAnalyzed -gt $MaxItems) {
        $itemsAnalyzed -= 1
        break
    }

    if ($itemsAnalyzed -eq 1 -or ($itemsAnalyzed % $ScanProgressEvery) -eq 0) {
        if ($totalItemsForScan -gt 0) {
            $scanPercent = [Math]::Floor(($itemsAnalyzed * 100.0) / $totalItemsForScan)
            Write-Host ("[scan {0,3}%] {1}/{2} itens analisados, {3} renomes candidatos..." -f $scanPercent, $itemsAnalyzed, $totalItemsForScan, $renameResults.Count)
        }
        else {
            Write-Host ("[scan] {0} itens analisados, {1} renomes candidatos..." -f $itemsAnalyzed, $renameResults.Count)
        }
    }

    $decodedName = Convert-MojibakeName -Name $item.Name -SourceEncoding $sourceEncoding -TargetEncoding $targetEncoding

    if ($null -eq $decodedName) {
        $notEncodable += 1
        $currentRelative = if ($null -ne $reportResults) { Get-RelativePathCompat -BasePath $resolvedRoot -TargetPath $item.FullName } else { "" }
        $result = [pscustomobject]@{
            Kind = if ($item.PSIsContainer) { "dir" } else { "file" }
            Status = "skip-not-encodable"
            Current = $currentRelative
            Proposed = ""
            FullName = $item.FullName
            TargetFullName = ""
        }
        if ($null -ne $reportResults) { $reportResults.Add($result) }
        continue
    }

    if ($decodedName.IndexOfAny($invalidChars) -ge 0) {
        $invalidTargetNames += 1
        $currentRelative = Get-RelativePathCompat -BasePath $resolvedRoot -TargetPath $item.FullName
        $result = [pscustomobject]@{
            Kind = if ($item.PSIsContainer) { "dir" } else { "file" }
            Status = "invalid-target-name"
            Current = $currentRelative
            Proposed = $decodedName
            FullName = $item.FullName
            TargetFullName = ""
        }
        if ($null -ne $reportResults) { $reportResults.Add($result) }
        continue
    }

    $parentPath = if ($item.PSIsContainer) { $item.Parent.FullName } else { $item.DirectoryName }
    $targetFullName = Join-Path $parentPath $decodedName
    $targetFullPath = [System.IO.Path]::GetFullPath($targetFullName)

    if ($decodedName -eq $item.Name) {
        $unchanged += 1
        if ($IncludeUnchanged) {
            $currentRelative = Get-RelativePathCompat -BasePath $resolvedRoot -TargetPath $item.FullName
            $targetRelative = Get-RelativePathCompat -BasePath $resolvedRoot -TargetPath $targetFullName
            $result = [pscustomobject]@{
                Kind = if ($item.PSIsContainer) { "dir" } else { "file" }
                Status = "unchanged"
                Current = $currentRelative
                Proposed = $targetRelative
                FullName = $item.FullName
                TargetFullName = $targetFullName
            }
            if ($previewResults.Count -lt $PreviewLimit) { $previewResults.Add($result) }
            if ($null -ne $reportResults) { $reportResults.Add($result) }
        }
        continue
    }

    $currentRelative = Get-RelativePathCompat -BasePath $resolvedRoot -TargetPath $item.FullName
    $targetRelative = Get-RelativePathCompat -BasePath $resolvedRoot -TargetPath $targetFullName

    if (-not $targetFullPath.StartsWith($rootWithSeparator, [System.StringComparison]::OrdinalIgnoreCase)) {
        $invalidTargetNames += 1
        $result = [pscustomobject]@{
            Kind = if ($item.PSIsContainer) { "dir" } else { "file" }
            Status = "target-outside-root"
            Current = Get-RelativePathCompat -BasePath $resolvedRoot -TargetPath $item.FullName
            Proposed = $targetRelative
            FullName = $item.FullName
            TargetFullName = $targetFullName
        }
        if ($null -ne $reportResults) { $reportResults.Add($result) }
        continue
    }

    $result = [pscustomobject]@{
        Kind = if ($item.PSIsContainer) { "dir" } else { "file" }
        Status = "would-rename"
        Current = $currentRelative
        Proposed = $targetRelative
        FullName = $item.FullName
        TargetFullName = $targetFullName
    }

    $renameResults.Add($result)
    if ($previewResults.Count -lt $PreviewLimit) { $previewResults.Add($result) }
    if ($null -ne $reportResults) { $reportResults.Add($result) }
}

if ($Apply) {
    if ($totalItemsForScan -gt 0) {
        Write-Host ("[apply] fase 2/4 concluida: {0} itens analisados, {1} renomes candidatos" -f $itemsAnalyzed, $renameResults.Count)
        Write-Host "[apply] fase 3/4: checagem de colisoes"
    }
    else {
        Write-Host ("[apply] fase 1/3 concluida: {0} itens analisados, {1} renomes candidatos" -f $itemsAnalyzed, $renameResults.Count)
        Write-Host "[apply] fase 2/3: checagem de colisoes"
    }
}

$targetCounts = @{}
$collisionProgressIndex = 0
$collisionProgressTotal = $renameResults.Count

foreach ($rename in $renameResults) {
    $collisionProgressIndex += 1

    if ($Apply -and ($collisionProgressIndex -eq 1 -or ($collisionProgressIndex % $ScanProgressEvery) -eq 0 -or $collisionProgressIndex -eq $collisionProgressTotal)) {
        $collisionPercent = if ($collisionProgressTotal -gt 0) { [Math]::Floor(($collisionProgressIndex * 100.0) / $collisionProgressTotal) } else { 100 }
        Write-Host ("[check {0,3}%] {1}/{2} alvos comparados para colisao interna..." -f $collisionPercent, $collisionProgressIndex, $collisionProgressTotal)
    }

    if ($targetCounts.ContainsKey($rename.TargetFullName)) {
        $targetCounts[$rename.TargetFullName] += 1
    }
    else {
        $targetCounts[$rename.TargetFullName] = 1
    }
}

$targetCollisions = @(
    foreach ($entry in $targetCounts.GetEnumerator()) {
        if ($entry.Value -gt 1) {
            [pscustomobject]@{
                Name = $entry.Key
                Count = $entry.Value
                Group = @($renameResults | Where-Object { $_.TargetFullName -eq $entry.Key })
            }
        }
    }
)

$existingTargetCollisions = [System.Collections.Generic.List[object]]::new()
$existingProgressIndex = 0

foreach ($rename in $renameResults) {
    $existingProgressIndex += 1

    if ($Apply -and ($existingProgressIndex -eq 1 -or ($existingProgressIndex % $ScanProgressEvery) -eq 0 -or $existingProgressIndex -eq $collisionProgressTotal)) {
        $existingPercent = if ($collisionProgressTotal -gt 0) { [Math]::Floor(($existingProgressIndex * 100.0) / $collisionProgressTotal) } else { 100 }
        Write-Host ("[check {0,3}%] {1}/{2} alvos verificados no disco..." -f $existingPercent, $existingProgressIndex, $collisionProgressTotal)
    }

    if ((Test-Path -LiteralPath $rename.TargetFullName) -and
        ([System.IO.Path]::GetFullPath($rename.TargetFullName) -ne [System.IO.Path]::GetFullPath($rename.FullName))) {
        $existingTargetCollisions.Add($rename)
    }
}

$existingTargetCollisions = @($existingTargetCollisions)

if ($previewResults.Count -gt 0) {
    $previewResults |
        Select-Object Kind, Status, Current, Proposed |
        Format-Table -AutoSize -Wrap

    if ($renameResults.Count -gt $PreviewLimit) {
        Write-Host ""
        Write-Host "[dry-run] exibindo apenas os primeiros $PreviewLimit renames. Use -ReportPath para salvar tudo em CSV."
    }
}
else {
    Write-Host "Nenhum nome recuperavel encontrado."
}

Write-Host ""
Write-Host "[resumo]"
Write-Host "itens analisados: $itemsAnalyzed"
Write-Host "renames sugeridos: $($renameResults.Count)"
Write-Host "sem mudanca: $unchanged"
Write-Host "nao codificaveis em codepage ${FromCodePage}: $notEncodable"
Write-Host "nomes invalidos apos decode: $invalidTargetNames"
Write-Host "colisoes entre alvos sugeridos: $($targetCollisions.Count)"
Write-Host "alvos que ja existem: $($existingTargetCollisions.Count)"

if ($ReportPath -ne "") {
    $reportFullPath = [System.IO.Path]::GetFullPath($ReportPath)
    $reportResults |
        Select-Object Kind, Status, Current, Proposed |
        Export-Csv -LiteralPath $reportFullPath -NoTypeInformation -Encoding UTF8

    Write-Host "relatorio salvo em: $reportFullPath"
}

if ($targetCollisions.Count -gt 0 -or $existingTargetCollisions.Count -gt 0) {
    Write-Host ""
    Write-Host "[alerta] Existem colisoes. Nao aplique renomeacao antes de resolver isso."

    foreach ($collision in $targetCollisions) {
        Write-Host ""
        Write-Host "colisao: $($collision.Name)"
        $collision.Group | Select-Object Current, Proposed | Format-Table -AutoSize -Wrap
    }

    if ($existingTargetCollisions.Count -gt 0) {
        Write-Host ""
        Write-Host "alvos ja existentes:"
        $existingTargetCollisions | Select-Object Current, Proposed | Format-Table -AutoSize -Wrap
    }

    exit 2
}

if ($Apply) {
    if ($MaxItems -gt 0) {
        Write-Host ""
        Write-Host "[bloqueado] -Apply nao pode ser usado com -MaxItems, para evitar renomeacao parcial."
        exit 3
    }

    if ($renameResults.Count -eq 0) {
        Write-Host ""
        Write-Host "[apply] nada para renomear."
        exit 0
    }

    $orderedRenames = @(
        $renameResults |
            Sort-Object @{ Expression = { $_.FullName.Length }; Descending = $true }
    )

    $total = $orderedRenames.Count
    $index = 0
    $ProgressEvery = [Math]::Max(1, $ProgressEvery)

    Write-Host ""
    if ($totalItemsForScan -gt 0) {
        Write-Host "[apply] fase 4/4: iniciando renomeacao de $total itens"
    }
    else {
        Write-Host "[apply] fase 3/3: iniciando renomeacao de $total itens"
    }

    foreach ($rename in $orderedRenames) {
        $index += 1
        $percent = [Math]::Floor(($index * 100.0) / $total)

        if ($index -eq 1 -or $index -eq $total -or ($index % $ProgressEvery) -eq 0) {
            Write-Host ("[{0,3}%] {1}/{2} {3} -> {4}" -f $percent, $index, $total, $rename.Current, $rename.Proposed)
        }

        if (-not (Test-Path -LiteralPath $rename.FullName)) {
            throw "Origem desapareceu antes da renomeacao: $($rename.FullName)"
        }

        $targetFullPath = [System.IO.Path]::GetFullPath($rename.TargetFullName)
        if (-not $targetFullPath.StartsWith($rootWithSeparator, [System.StringComparison]::OrdinalIgnoreCase)) {
            throw "Alvo fora da raiz bloqueado: $($rename.TargetFullName)"
        }

        Rename-Item -LiteralPath $rename.FullName -NewName ([System.IO.Path]::GetFileName($rename.TargetFullName))
    }

    Write-Host ""
    Write-Host "[apply] renomeacao concluida: $total/$total itens processados (100%)"
}
