param([switch]$SkipBuild, [string]$OutputDir, [string]$SignParams, [string]$AzureTrustedSignFile)
$ErrorActionPreference = 'Stop'
if ($SignParams -and $AzureTrustedSignFile) { throw 'Choisir une seule méthode de signature.' }
if ($AzureTrustedSignFile -and -not (Test-Path -LiteralPath $AzureTrustedSignFile -PathType Leaf)) { throw 'Fichier de configuration Azure Artifact Signing introuvable.' }
$azureSigningPath = if ($AzureTrustedSignFile) { (Resolve-Path -LiteralPath $AzureTrustedSignFile).Path } else { $null }
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
$output = if ($OutputDir) { $OutputDir } else { Join-Path $projectRoot "artifacts\releases" }
New-Item -ItemType Directory -Path $stage,$output -Force | Out-Null
$targetRoot = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $projectRoot 'target' }
Copy-Item -LiteralPath (Join-Path $targetRoot 'release\boxmaker.exe') -Destination (Join-Path $stage 'Boxmaker.exe') -Force
$packArgs = @('pack', '--packId', 'Swiss3Design.Boxmaker', '--packVersion', $version, '--packDir', $stage, '--mainExe', 'Boxmaker.exe', '--packTitle', 'Boxmaker', '--packAuthors', "Thomas Prud'homme", '--outputDir', $output, '--framework', 'webview2', '--icon', (Join-Path $projectRoot 'src-tauri\icons\icon.ico'), '--releaseNotes', (Join-Path $projectRoot "docs\releases\v$version.md"))
if ($SignParams) { $packArgs += @('--signParams', $SignParams) }
if ($azureSigningPath) { $packArgs += @('--azureTrustedSignFile', $azureSigningPath) }
& dotnet tool run vpk -- @packArgs
if ($LASTEXITCODE -ne 0) { throw 'Échec du packaging Velopack.' }
if ($SignParams -or $AzureTrustedSignFile) {
    $setup = Join-Path $output 'Swiss3Design.Boxmaker-win-Setup.exe'
    $signature = Get-AuthenticodeSignature -LiteralPath $setup
    if ($signature.Status -ne 'Valid') { throw "Signature de l'installateur non valide : $($signature.Status)." }
    Write-Output "Éditeur Windows vérifié : $($signature.SignerCertificate.Subject)"
}
& (Join-Path $PSScriptRoot 'checksums.ps1') -Directory $output
Write-Output "Installateur Velopack : $output"
