```bash
sudo apt install libgl1-mesa-dev xorg-dev
sudo apt install libglib2.0-dev
sudo apt install libatk-bridge2.0
sudo apt install libatk-3-dev
sudo apt install libpango1.0-dev
sudo apt install libgtk2.0-dev
sudo apt install libjavascriptcoregtk-4.0-dev
sudo apt install libgtk-3-dev
sudo apt install libsoup2.4
sudo apt install libsoup2.4-dev
sudo apt install libwebkit2gtk-4.0-dev

export DISPLAY=$(ipconfig.exe | grep "IPv4" | head -1 | awk '{print $NF}' | awk 'sub(/\r$/,"")'):0
```

https://astherier.com/blog/2020/08/run-gui-apps-on-wsl2/