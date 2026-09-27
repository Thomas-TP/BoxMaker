$ErrorActionPreference = 'Stop'
Set-Location -LiteralPath (Split-Path -Parent $PSScriptRoot)
. (Join-Path $PSScriptRoot 'rust-env.ps1')
# Resource compiler discovery can fail outside a Visual Studio developer shell.
if (-not (Get-Command rc.exe -ErrorAction SilentlyContinue)) {
    $sdkBin = Join-Path ${env:ProgramFiles(x86)} 'Windows Kits\10\bin'
    $sdkArchitecture = if ($env:PROCESSOR_ARCHITECTURE -eq 'ARM64') { 'arm64' } else { 'x64' }
    $sdkArchitectures = @($sdkArchitecture, 'x64', 'arm64') | Select-Object -Unique
    $resourceCompiler = Get-ChildItem -LiteralPath $sdkBin -Directory -ErrorAction SilentlyContinue |
        Sort-Object Name -Descending |
        ForEach-Object { $sdkVersion = $_.FullName; $sdkArchitectures | ForEach-Object { Join-Path $sdkVersion "$_\rc.exe" } } |
        Where-Object { Test-Path -LiteralPath $_ } |
        Select-Object -First 1
    if (-not $resourceCompiler) { throw 'Windows SDK introuvable : installez les outils C++ et le SDK Windows de Visual Studio.' }
    $env:RC = $resourceCompiler
}
& bun run tauri build --no-bundle
if ($LASTEXITCODE -ne 0) { throw 'Échec de compilation Windows.' }
