# discac
discac - small program to change your discord bot's avatar.  

[![GitHub](https://img.shields.io/github/license/TanteiBot/discac?style=flat-square)](https://github.com/TanteiBot/discac/blob/master/LICENSE)  

## Reason
I need to periodically change avatars for my discord bots.  
I think small program that will run periodically triggered by native system timer is better than integrating this functionality in every bot.  

## Building
Requires: `git`, `cargo`
1. `git clone https://github.com/TanteiBot/discac.git`
2. `cd discac/`
3. `sh ./scripts/publish-x86-64-Linux.sh`
4. Executable and sample config will be located in `output` directory

## Installation and running, currently supported only on x86-64 Linux with GNU libc
Requires `wget`  
1. `wget https://github.com/N0D4N/discac/releases/latest/download/x86_64-linux.zip`
2. `unzip x86_64-linux.zip`
3. Fill your bot token and paths to directories where bot's avatars are located to `temp.config.json`
4. Rename `temp.config.json` to `config.json`
5. You are ready to go  
### Follow next steps for using with systemd timer
1. `sudo mkdir -p /usr/local/bin/discac`
2. Choose a name for a profile. `discac` supports updating profiles of multiple bots with one executable. For differentiating between them profiles are used.
3. Copy next command and replace first `profile_name` at its start with name of profile you have chosen earlier.
4. `DISCAC_PROFILE_NAME=profile_name sudo mkdir -p /usr/local/bin/discac/profiles/"$DISCAC_PROFILE_NAME" && sudo cp config.json /usr/local/bin/discac/profiles/"$DISCAC_PROFILE_NAME" && sudo cp discac /usr/local/bin/discac/ && sudo cp -r systemd/* /etc/systemd/system/`
5. `sudo systemctl daemon-reload`
6. Next command will enable timer to run `discac` every 12h, you can change it by editing `discac@.timer` file. Replace `profile_name` in it with profile name you have chosen earlier.  
   `sudo systemctl enable --now discac@profile_name.timer`

## Contributing/Contacting
[Open an issue](https://github.com/TanteiBot/discac/issues/new)

## License
Copyright 2021-2022 N0D4N  
discac is licensed under the GPLv3: <https://www.gnu.org/licenses/gpl-3.0>  
See <https://github.com/TanteiBot/discac/blob/master/LICENSE> for more details.  
[![License](https://www.gnu.org/graphics/gplv3-127x51.png)](https://www.gnu.org/licenses/gpl-3.0.html)
