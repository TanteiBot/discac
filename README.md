# discac
discac - small program to change your discord bot's avatar.  

[![GitHub](https://img.shields.io/github/license/N0D4N/discac?style=flat-square)](https://github.com/N0D4N/discac/blob/master/LICENSE)  

## Reason
I need to periodically change avatars for my discord bots.  
I think small program that will run periodically triggered by native system timer is better than integrating this functionality in every bot.  

## Building
Requires: `git`, `cargo`, `upx` (needed for reducing binary size)
1. `git clone https://github.com/N0D4N/discac.git`
2. `cd discac/`
3. `sh ./publish-x86-64-Linux.sh`
4. `cd output/x86_64-unknown-linux-gnu/`
5. `cp temp.config.json config.json`
6. Fill `config.json` with your discord bot token and path to directory where avatar
7. Run `./discac`

## Installation
1. `wget https://github.com/N0D4N/discac/releases/latest/download/x86-64-linux.zip`
2. `unzip x86-64-linux.zip`
3. Follow Building instructions 5 & 6.  
4. You are ready to go  
5. Follow next steps for using with systemd timer  
   `sudo mkdir -p /usr/local/bin/discac`
6. `sudo cp discac /usr/local/bin/discac/ && sudo cp config.json /usr/local/bin/discac/ && sudo cp systemd/discac.service /etc/systemd/system/ && sudo cp systemd/discac.timer /etc/systemd/system/`
7. `sudo systemctl daemon-reload`
8. Next command will enable timer to run `discac` every 12h, you can change it by editing `discac.timer` file   
   `sudo systemctl enable --now discac.timer`
   
## Contributing/Contacting
[Open an issue](https://github.com/N0D4N/discac/issues/new)

## License
Copyright 2021 N0D4N  
discac is licensed under the GPLv3: https://www.gnu.org/licenses/gpl-3.0.html  
See https://github.com/N0D4N/discac/blob/master/LICENSE for more details.  
[![License](https://www.gnu.org/graphics/gplv3-127x51.png)](https://www.gnu.org/licenses/gpl-3.0.html)
