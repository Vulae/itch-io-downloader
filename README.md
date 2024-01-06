
### **WARNING: This is Windows ONLY, There may be other OS but not for a while.**
### **WARNING: This is Google Chrome ONLY, there may be other web browsers but not for a while.**

# Itch.io downloader

Adds a button on itch.io games to download & play.

# Installation

1. Download latest release & unzip to where you want your games to be.
2. Execute install.bat
3. Add chrome addon
    1. Go to [chrome://extensions/](chrome://extensions/)
    2. Enable developer mode
    3. Load unpacked "itch-io-downloader/addon/"
4. Download & play button will appear on any itch.io game page.

# Develop

1. Clone repo
2. install.bat registry key to point to debug
    1. Replace line 13 with `reg add "HKCR\itch-io-downloader\Shell\Open\Command" /f /ve /t REG_SZ /d "\"%~dp0target/debug/itch-io-downloader.exe\" \"%~dp0games\" \"%%1\""`
    2. Rerun install.bat
