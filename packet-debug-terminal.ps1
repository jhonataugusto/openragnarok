param(
    [Parameter(Mandatory = $true)]
    [string]$TracePath
)

$ErrorActionPreference = "SilentlyContinue"
[Console]::OutputEncoding = [System.Text.UTF8Encoding]::new()
$host.UI.RawUI.WindowTitle = "Korangar packet debug"

Write-Host "[packet-debug] acompanhando: $TracePath"
Write-Host "[packet-debug] formato: DIRECAO STATUS HEADER LEN TIPO BYTES"
Write-Host "[packet-debug] aguardando pacotes..."

$offset = 0L
$creationTimeUtc = [datetime]::MinValue

while ($true) {
    if (Test-Path -LiteralPath $TracePath) {
        $item = Get-Item -LiteralPath $TracePath

        if ($item.CreationTimeUtc -ne $creationTimeUtc -or $item.Length -lt $offset) {
            $creationTimeUtc = $item.CreationTimeUtc
            $offset = 0L
            Write-Host ""
            Write-Host "[packet-debug] trace iniciado/reiniciado"
        }

        $stream = [System.IO.File]::Open(
            $TracePath,
            [System.IO.FileMode]::Open,
            [System.IO.FileAccess]::Read,
            [System.IO.FileShare]::ReadWrite
        )

        try {
            if ($stream.Length -lt $offset) {
                $offset = 0L
            }

            [void]$stream.Seek($offset, [System.IO.SeekOrigin]::Begin)
            $reader = [System.IO.StreamReader]::new($stream, [System.Text.Encoding]::UTF8, $true, 4096, $true)
            $text = $reader.ReadToEnd()
            $offset = $stream.Position
            $reader.Dispose()

            if ($text.Length -gt 0) {
                Write-Host -NoNewline $text
            }
        }
        finally {
            $stream.Dispose()
        }
    }

    Start-Sleep -Milliseconds 200
}
