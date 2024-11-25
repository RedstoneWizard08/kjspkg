$ErrorActionPreference = "Stop"
$tmpdir = $Env:TEMP
$MODHOST_VERSION = $Env:MODHOST_VERSION

if (-not $MODHOST_VERSION) {
    $MODHOST_VERSION = 'latest'
}

$base_url = "https://github.com/RedstoneWizard08/ModHost/releases/$MODHOST_VERSION/download/modhost"
$proc_arch = [Environment]::GetEnvironmentVariable("PROCESSOR_ARCHITECTURE", [EnvironmentVariableTarget]::Machine)

if ($proc_arch -eq "AMD64") {
	$arch = "x86_64"
} elseif ($proc_arch -eq "ARM64") {
	$arch = "aarch64"
} else {
	Write-Host "Unsupported Architecture: $type" -ForegroundColor Red
	exit 1
}

$url = "$base_url$arch-pc-windows-gnu.zip"
$extract_path = "$tmpdir\modhost-cli"
$zip_path = "$tmpdir\modhost-cli.zip"
$local_bins = if ($Env:LOCAL_BINS -ne $null) { $Env:LOCAL_BINS } else { "$HOME\.local\bin" }

if (!(Test-Path -Path "$local_bins" -PathType Container)) {
	New-Item -ItemType Directory -Path "$local_bins" | Out-Null
}

Invoke-WebRequest "$url" -OutFile "$zip_path"
Expand-Archive -Force "$zip_path" "$extract_path"
Copy-Item -Path "$extract_path\modhost.exe" -Destination "$local_bins\modhost.exe" -Force

if ($Env:Path -split ";" -notcontains "$local_bins") {
    if (($Env:CI -ne $null) -and ($Env:GITHUB_PATH -ne $null)) {
        Add-Content -Path "$Env:GITHUB_PATH" -Value "$local_bins"
    } else {
	    Write-Host ""
    	Write-Host "Your path is missing $local_bins, you might want to add it." -ForegroundColor Red
	    Write-Host ""
     }
}
