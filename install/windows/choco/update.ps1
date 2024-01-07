#!/usr/bin/env pwsh

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 3.0

if ($null -eq $ENV:ORBITER_VERSION) {
    Write-Host "Version is required"
    exit 1
}

$version = "$ENV:ORBITER_VERSION"
$versionNumber = $version.TrimStart("v")

[xml]$nuspec_file = Get-Content -Path ./install/windows/choco/orbiter.nuspec
$nuspec_file.package.metadata.version = $versionNumber

$changelog = (Get-Content -Path ./CHANGELOG.md | Out-String)
$nuspec_file.package.metadata.releaseNotes = $changelog

# Create variant nuspec files
$nuspec_file.package.metadata.id = "orbiter.portable"
$nuspec_file.Save("./orbiter.portable.nuspec")

$nuspec_file.package.metadata.id = "orbiter.install"
$nuspec_file.Save("./orbiter.install.nuspec")

# Have metapackage depend on orbiter.install
$nuspec_file.package.metadata.id = "orbiter"

$deps = $nuspec_file.createelement("dependencies")
$dep = $nuspec_file.createelement("dependency")
$dep.SetAttribute("id", "orbiter.install")
$dep.SetAttribute("version", "[$versionNumber]")
$deps.AppendChild($dep)
$nuspec_file.package.metadata.AppendChild($deps)
$nuspec_file.Save("./orbiter.nuspec")

$url_x86_64_zip = "https://github.com/orbiter-rs/orbiter/releases/download/$version/orbiter-x86_64-pc-windows-msvc.zip"
$url_i686_zip = "https://github.com/orbiter-rs/orbiter/releases/download/$version/orbiter-i686-pc-windows-msvc.zip"
$url_x86_64_msi = "https://github.com/orbiter-rs/orbiter/releases/download/$version/orbiter-x86_64-pc-windows-msvc.msi"
$url_i686_msi = "https://github.com/orbiter-rs/orbiter/releases/download/$version/orbiter-i686-pc-windows-msvc.msi"

$checksum_x86_64_zip = Get-FileHash -Algorithm SHA256 -Path "./orbiter-x86_64-pc-windows-msvc.zip/orbiter-x86_64-pc-windows-msvc.zip" | Select-Object -ExpandProperty Hash
$checksum_i686_zip = Get-FileHash -Algorithm SHA256 -Path "./orbiter-i686-pc-windows-msvc.zip/orbiter-i686-pc-windows-msvc.zip" | Select-Object -ExpandProperty Hash
$checksum_x86_64_msi = Get-FileHash -Algorithm SHA256 -Path "./orbiter-x86_64-pc-windows-msvc.msi/orbiter-x86_64-pc-windows-msvc.msi" | Select-Object -ExpandProperty Hash
$checksum_i686_msi = Get-FileHash -Algorithm SHA256 -Path "./orbiter-i686-pc-windows-msvc.msi/orbiter-i686-pc-windows-msvc.msi" | Select-Object -ExpandProperty Hash

if (Test-Path "./tools") {
    Remove-Item -Path "./tools" -Recurse -Force
}
New-Item -ItemType Directory -Path "./tools"

# Pack the metapackage as-is without install script
choco pack ./orbiter.nuspec

foreach ($install_type in @('portable', 'install')) {
    Get-Content ./install/windows/choco/chocolateyInstall.$install_type.ps1 | ForEach-Object {
        if ($_ -match '^\$url_x86_64_zip = (.*)') {
            "`$url_x86_64_zip = '$url_x86_64_zip'"
        }
        elseif ($_ -match '^\$url_i686_zip = (.*)') {
            "`$url_i686_zip = '$url_i686_zip'"
        }
        elseif ($_ -match '^\$url_x86_64_msi = (.*)') {
            "`$url_x86_64_msi = '$url_x86_64_msi'"
        }
        elseif ($_ -match '^\$url_i686_msi = (.*)') {
            "`$url_i686_msi = '$url_i686_msi'"
        }
        elseif ($_ -match '^\$checksum_x86_64_zip = (.*)') {
            "`$checksum_x86_64_zip = '$checksum_x86_64_zip'"
        }
        elseif ($_ -match '^\$checksum_i686_zip = (.*)') {
            "`$checksum_i686_zip = '$checksum_i686_zip'"
        }
        elseif ($_ -match '^\$checksum_x86_64_msi = (.*)') {
            "`$checksum_x86_64_msi = '$checksum_x86_64_msi'"
        }
        elseif ($_ -match '^\$checksum_i686_msi = (.*)') {
            "`$checksum_i686_msi = '$checksum_i686_msi'"
        }
        else {
            $_
        }
    } | Set-Content ./tools/chocolateyInstall.ps1
    
    choco pack ./orbiter.$install_type.nuspec
}

if ($null -ne $ENV:PUSH_TOKEN) {
    choco push orbiter.portable.$versionNumber.nupkg --key $ENV:PUSH_TOKEN --source="'https://push.chocolatey.org/'"
    choco push orbiter.install.$versionNumber.nupkg --key $ENV:PUSH_TOKEN --source="'https://push.chocolatey.org/'"
    choco push orbiter.$versionNumber.nupkg --key $ENV:PUSH_TOKEN --source="'https://push.chocolatey.org/'"
}
else {
    Write-Host "No API key provided, skipping push"
}
