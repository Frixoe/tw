# tw
Your todos, as your wallpaper.

## Config
Store your config in `~/.config/tw/config.json` in the following format:
```json
{
    "personName": "a name",
    "taskStatus": "Working on it",
    "apiKey": "superSecretAPIKey",
    "font": {
        "path": "poppins.ttf",
        "size": 30
    },
    "widthOffsetPerc": 50,
    "startHeight": 170,
    "heightIncrement": 50,
    "outputImage": {
        "path": "~/.config/tw/output.png",
        "width": 1920,
        "height": 1080
    },
    "todosOnly": true,
    "todosPath": "~/dev/projects/tw/todods",
    "bgSetCommand": "swaymsg output HDMI-A-1 bg ~/.config/tw/output.png fill"
}
```
> If you set relative paths, they will be inferred at runtime, relative to where the binary is being run from

## Build
To build the project:
```
sudo apt install lld llvm clang gcc g++ # figure out how to install for your linux distro
git clone https://github.com/Frixoe/tw.git
cd tw
cargo build --release 
mv target/release/tw ~/.local/bin/
```
> Make sure `~/.local/bin` is in your $PATH

## Usage
Create a timer to run this as often as you expect to be changing your tasks

## Commands

## Todos
- [ ] Write docs
- [ ] Update Commands section in Readme
- [ ] Improve logging
- [ ] Add support for custom backgrounds
- [ ] Add support just personal TODO support
- [ ] Infer the size, placement and font based on screen size
- [ ] Provide a command to install a systemd service and run it
- [ ] Instead of coordinates, ask the user for top-right/left, bottom-right/left positions for text placement
