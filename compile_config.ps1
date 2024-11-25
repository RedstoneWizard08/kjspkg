$TempFile = New-TemporaryFile
Invoke-WebRequest -OutFile $TempFile "https://github.com/apple/pkl/releases/download/0.27.0/pkl-windows-amd64.exe"

if (!$IsWindows) {
    chmod a+rx $TempFile
}

exec $TempFile eval -f toml config.pkl -o ModHost.toml
Remove-Item $TempFile
