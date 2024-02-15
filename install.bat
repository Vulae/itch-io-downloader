
:: Elevate privileges if needed.
net file 1>NUL 2>NUL
if not '%errorlevel%' == '0' (
    powershell Start-Process -FilePath "%0" -ArgumentList "%cd%" -verb runas >NUL 2>&1
    exit /b
)
cd /d %1

:: Add registry keys.
reg add "HKCR\itch-io-downloader" /f /ve /t REG_SZ /d "URL:itch-io-downloader protocol"
reg add "HKCR\itch-io-downloader" /f /v "URL Protocol" /t REG_SZ
reg add "HKCR\itch-io-downloader\Shell\Open\Command" /f /ve /t REG_SZ /d "\"%~dp0itch-io-downloader.exe\" uri \"%%1\""

:: Finished.
echo "If you ever move the install location, rerun this script."
pause
