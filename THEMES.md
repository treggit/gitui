# Themes

default on light terminal:
![](assets/light-theme.png)

to change the colors of the default theme you have to modify `theme.ron` file
[Ron format](https://github.com/ron-rs/ron) located at config path. The path differs depending on the operating system:

* `$HOME/Library/Application Support/gitui/theme.ron` (mac)
* `$XDG_CONFIG_HOME/gitui/theme.ron` (linux using XDG)
* `$HOME/.config/gitui/theme.ron` (linux)
* `%APPDATA%/gitui/theme.ron` (Windows)

Alternatively you may make a theme in the same directory mentioned above with and select with the `-t` flag followed by the name of the file in the directory. E.g. If you are on linux calling `gitui -t arc.ron` wil use `$XDG_CONFIG_HOME/gitui/arc.ron` or `$HOME/.config/gitui/arc.ron`

Valid colors can be found in tui-rs' [Color](https://docs.rs/tui/0.12.0/tui/style/enum.Color.html) struct. note that rgb colors might not be supported in every terminal.
