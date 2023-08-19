# Documentation of the code
## Just for myself

the command `window-cycler -d` should open the daemon and initialize the ui but keep it hidden

the command `window-cycler -c <command>` should send the command to the daemon where next and prev will cycle through the windows and close will hide the window again


## In hyprland
```conf
bind = ALT, Tab, exec, window-cycler -c next
bind = SHIFT ALT, Tab, exec, window-cycler -c prev
```