$ErrorActionPreference = 'Stop'
# Manifold builds its pinned C++ kernel on the first Cargo build.
if (-not (Get-Command cmake -ErrorAction SilentlyContinue)) {
    $locator = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\vswhere.exe'
    if (Test-Path -LiteralPath $locator) {
        $visualStudio = & $locator -latest -products '*' -property installationPath
        if ($visualStudio) {
            $cmakeBin = Join-Path $visualStudio 'Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin'
            if (Test-Path -LiteralPath (Join-Path $cmakeBin 'cmake.exe')) {
                $env:PATH = "$cmakeBin;$env:PATH"
            }
        }
    }
    if (-not (Get-Command cmake -ErrorAction SilentlyContinue)) {
        throw 'CMake manque : installez les outils CMake C++ de Visual Studio ou ajoutez cmake au PATH.'
    }
}
