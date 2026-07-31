$ErrorActionPreference = "Stop"

$Repo = "zalaya/Portman"
$BinName = "portman"
$Target = "x86_64-pc-windows-msvc"
$InstallDir = if ($env:PORTMAN_INSTALL_DIR) { $env:PORTMAN_INSTALL_DIR } else { "$env:LOCALAPPDATA\Portman" }
$Version = if ($env:PORTMAN_VERSION) { $env:PORTMAN_VERSION } else { "latest" }

$Url = if ($Version -eq "latest") {
    "https://github.com/$Repo/releases/latest/download/$BinName-$Target.zip"
} else {
    "https://github.com/$Repo/releases/download/$Version/$BinName-$Target.zip"
}

$TempDir = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid())
New-Item -ItemType Directory -Path $TempDir | Out-Null
$ZipPath = Join-Path $TempDir "$BinName.zip"

try {
    Write-Host "Downloading $BinName ($Target, $Version)..."

    try {
        Invoke-WebRequest -Uri $Url -OutFile $ZipPath -UseBasicParsing
    } catch {
        throw "download failed — is there a release for $Target yet?"
    }

    Expand-Archive -Path $ZipPath -DestinationPath $TempDir -Force

    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    Copy-Item -Path (Join-Path $TempDir "$BinName.exe") -Destination (Join-Path $InstallDir "$BinName.exe") -Force

    Write-Host "Installed to $InstallDir\$BinName.exe"

    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")

    if ($UserPath -notlike "*$InstallDir*") {
        [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
        Write-Host "Added $InstallDir to your PATH — restart your terminal for it to take effect."
    }
} finally {
    Remove-Item -Recurse -Force $TempDir -ErrorAction SilentlyContinue
}
