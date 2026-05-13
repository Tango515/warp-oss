#!/usr/bin/env powershell
#
# Build a portable Windows ZIP package for Warp.

Param (
    [Switch]$DEBUG_BUILD = $False,

    [ValidateSet('local', 'dev', 'preview', 'stable', 'oss')]
    [String]$CHANNEL = 'oss',

    [ValidateSet('x64', 'arm64')]
    [String]$ARCH = 'x64',

    [String]$RELEASE_TAG = '',

    [Switch]$SKIP_BUILD_BINARY = $False
)

$ErrorActionPreference = 'Stop'

if ($RELEASE_TAG) {
    $env:GIT_RELEASE_TAG = $RELEASE_TAG
}

$WorkspaceRoot = (Get-Location).Path
$WindowsDir = Join-Path $WorkspaceRoot 'script\windows'
$TargetTriple = if ($ARCH -eq 'arm64') { 'aarch64-pc-windows-msvc' } else { 'x86_64-pc-windows-msvc' }

if ($DEBUG_BUILD) {
    $CargoProfile = 'dev'
    $TargetProfileDir = Join-Path (Join-Path (Join-Path $WorkspaceRoot 'target') $TargetTriple) 'debug'
} elseif (($CHANNEL -eq 'local') -or ($CHANNEL -eq 'dev')) {
    $CargoProfile = 'release'
    $TargetProfileDir = Join-Path (Join-Path (Join-Path $WorkspaceRoot 'target') $TargetTriple) $CargoProfile
} else {
    $CargoProfile = 'release'
    $TargetProfileDir = Join-Path (Join-Path (Join-Path $WorkspaceRoot 'target') $TargetTriple) $CargoProfile
}

switch ($CHANNEL) {
    'local' {
        $BinaryName = 'warp.exe'
        $AppName = 'WarpLocal'
        $CliName = 'warp-local.cmd'
    }
    'dev' {
        $BinaryName = 'dev.exe'
        $AppName = 'WarpDev'
        $CliName = 'oz-dev.cmd'
    }
    'preview' {
        $BinaryName = 'preview.exe'
        $AppName = 'WarpPreview'
        $CliName = 'oz-preview.cmd'
    }
    'stable' {
        $BinaryName = 'warp.exe'
        $AppName = 'Warp'
        $CliName = 'oz.cmd'
    }
    'oss' {
        $BinaryName = 'warp-oss.exe'
        $AppName = 'WarpOss'
        $CliName = 'warp-oss.cmd'
    }
}

if (-not $SKIP_BUILD_BINARY) {
    $PreviousNoLto = $env:WARP_WINDOWS_NO_LTO
    if (-not $DEBUG_BUILD) {
        $env:WARP_WINDOWS_NO_LTO = '1'
    }

    $BundleArgs = @{
        CHANNEL = $CHANNEL
        SKIP_BUILD_INSTALLER = $True
        ARCH = $ARCH
    }
    if ($DEBUG_BUILD) {
        $BundleArgs.DEBUG_BUILD = $True
    }
    if ($RELEASE_TAG) {
        $BundleArgs.RELEASE_TAG = $RELEASE_TAG
    }

    try {
        & "$WindowsDir\bundle.ps1" @BundleArgs
    } finally {
        if ($null -eq $PreviousNoLto) {
            Remove-Item Env:\WARP_WINDOWS_NO_LTO -ErrorAction SilentlyContinue
        } else {
            $env:WARP_WINDOWS_NO_LTO = $PreviousNoLto
        }
    }
}

$ResourcesDir = Join-Path $TargetProfileDir 'resources'
& "$WindowsDir\prepare_bundled_resources.ps1" -DestinationDir $ResourcesDir -Channel $CHANNEL -CargoProfile $CargoProfile

$DistRoot = Join-Path $WorkspaceRoot 'dist'
$PortableDir = Join-Path $DistRoot "$AppName-Portable-Windows-$ARCH"
$ZipPath = "$PortableDir.zip"

if (Test-Path $PortableDir) {
    Remove-Item -Path $PortableDir -Recurse -Force
}
if (Test-Path $ZipPath) {
    Remove-Item -Path $ZipPath -Force
}

New-Item -ItemType Directory -Path $PortableDir -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $PortableDir $ARCH) -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $PortableDir 'bin') -Force | Out-Null

$AssetsDir = Join-Path (Join-Path (Join-Path $WorkspaceRoot 'app') 'assets\windows') $ARCH
$IconPath = Join-Path (Join-Path (Join-Path (Join-Path $WorkspaceRoot 'app') 'channels') $CHANNEL) 'icon\no-padding\icon.ico'
$BootstrapPath = Join-Path $WorkspaceRoot 'app\assets\bundled\bootstrap\pwsh.ps1'

Copy-Item -Path (Join-Path $TargetProfileDir $BinaryName) -Destination $PortableDir -Force
Copy-Item -Path (Join-Path $AssetsDir 'conpty.dll') -Destination $PortableDir -Force
Copy-Item -Path (Join-Path $AssetsDir 'vcruntime140.dll') -Destination $PortableDir -Force
Copy-Item -Path (Join-Path $AssetsDir 'vcruntime140_1.dll') -Destination $PortableDir -Force
Copy-Item -Path (Join-Path $AssetsDir 'msvcp140.dll') -Destination $PortableDir -Force
Copy-Item -Path (Join-Path $AssetsDir 'dxcompiler.dll') -Destination $PortableDir -Force
Copy-Item -Path (Join-Path $AssetsDir 'dxil.dll') -Destination $PortableDir -Force
Copy-Item -Path (Join-Path $AssetsDir 'OpenConsole.exe') -Destination (Join-Path $PortableDir $ARCH) -Force
Copy-Item -Path $IconPath -Destination (Join-Path $PortableDir 'icon.ico') -Force
Copy-Item -Path $BootstrapPath -Destination $PortableDir -Force
Copy-Item -Path $ResourcesDir -Destination (Join-Path $PortableDir 'resources') -Recurse -Force

$CmdPath = Join-Path (Join-Path $PortableDir 'bin') $CliName
$CmdContent = "@echo off`r`nset `"WARP_CLI_MODE=1`"`r`n`"%~dp0..\$BinaryName`" %*`r`n"
Set-Content -Path $CmdPath -Value $CmdContent -Encoding ASCII

$ReadmePath = Join-Path $PortableDir 'README-portable.txt'
$Readme = @"
$AppName portable build

Run:
  $BinaryName

CLI helper:
  bin\$CliName

This package is unsigned and intended for local testing.
"@
Set-Content -Path $ReadmePath -Value $Readme -Encoding UTF8

Compress-Archive -Path (Join-Path $PortableDir '*') -DestinationPath $ZipPath -Force

Write-Output "Portable package: $ZipPath"
