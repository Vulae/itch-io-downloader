
> **WARNING: This only works on Windows OS with Chrome browser**  

# [Itch.io downloader](https://github.com/Vulae/itch-io-downloader)

Adds a button on itch.io games to download & play.

# [Installation](https://github.com/Vulae/itch-io-downloader#installation)

1. Download [latest release](https://github.com/Vulae/itch-io-downloader/releases/latest) & extract to where you want your games to be.
2. Execute `install.bat`
3. Add chrome addon
    1. Go to [chrome://extensions/](chrome://extensions/)
    2. Enable developer mode
    3. Load unpacked `itch-io-downloader/addon/`
4. Create `itch-io-downloader/api_key.txt` with [itch.io api key](https://itch.io/user/settings/api-keys).

# [Develop](https://github.com/Vulae/itch-io-downloader#develop)

1. Clone repo
2. Do installation process above but run `install_develop.bat` instead of `install.bat`
3. run `cargo run -- "path/to/itch-io-downloader" "itch-io-downloader://play/GAME_ID"`
