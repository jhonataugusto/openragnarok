$ErrorActionPreference = "SilentlyContinue"

$signature = @"
using System;
using System.Runtime.InteropServices;

public static class ConsoleMode
{
    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern IntPtr GetStdHandle(int nStdHandle);

    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern bool GetConsoleMode(IntPtr hConsoleHandle, out int lpMode);

    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern bool SetConsoleMode(IntPtr hConsoleHandle, int dwMode);
}
"@

Add-Type -TypeDefinition $signature

$stdInputHandle = -10
$enableQuickEditMode = 0x0040
$enableExtendedFlags = 0x0080

$handle = [ConsoleMode]::GetStdHandle($stdInputHandle)
$mode = 0

if ([ConsoleMode]::GetConsoleMode($handle, [ref]$mode)) {
    $newMode = ($mode -bor $enableExtendedFlags) -band (-bnot $enableQuickEditMode)
    [void][ConsoleMode]::SetConsoleMode($handle, $newMode)
}
