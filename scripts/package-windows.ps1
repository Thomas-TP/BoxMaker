param([switch]$SkipBuild)
$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
Set-Location -LiteralPath $projectRoot
if (-not $SkipBuild) {
    & bun run desktop:build
    if ($LASTEXITCODE -ne 0) { throw 'Échec de compilation de Boxmaker.' }
}
$version = (Get-Content -LiteralPath (Join-Path $projectRoot 'package.json') -Raw | ConvertFrom-Json).version
& bun run release:check
if ($LASTEXITCODE -ne 0) { throw 'Versions ou notes de release incohérentes.' }
$stage = Join-Path $projectRoot "artifacts\stage-$version"
$output = Join-Path $projectRoot "artifacts\releases"
New-Item -ItemType Directory -Path $stage,$output -Force | Out-Null
Copy-Item -LiteralPath (Join-Path $projectRoot 'target\release\boxmaker.exe') -Destination (Join-Path $stage 'Boxmaker.exe') -Force
& dotnet tool run vpk -- pack --packId Swiss3Design.Boxmaker --packVersion $version --packDir $stage --mainExe Boxmaker.exe --packTitle 'Boxmaker' --packAuthors 'Swiss3Design' --outputDir $output --framework webview2 --icon (Join-Path $projectRoot 'src-tauri\icons\icon.ico') --releaseNotes (Join-Path $projectRoot "docs\releases\v$version.md")
if ($LASTEXITCODE -ne 0) { throw 'Échec du packaging Velopack.' }
& (Join-Path $PSScriptRoot 'checksums.ps1') -Directory $output
Write-Output "Installateur Velopack : $output"
