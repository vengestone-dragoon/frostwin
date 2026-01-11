# Frostwin #
Frostwin is a small Windows 10/11 Shell replacement that I began.
### Installation? ###
The intended purpos is to be installed and set as the user shell at startup.
As such, you have to manually change the related regestry keys to point to Frostwin as the user shell.
### Why? ###
Because the default windows shell has horendous performance on older computers, computers that would run fin and can still run many games, if windows performance wasnt so bogged down by the modern windows shell.
By launching a custom shell, we prevent most of the heavy system services from starting, and can achieve much better performance on older hardware.

### Plans ###
My current next plans for this project would be:
- adding system tray icons
- adding display brightness control
- adding wifi network connection management
- adding customization settings
- adding desktop icons
- adding desktop customization
- fix desktop in front of taskbar issue
- add settings menu
- add context menus / context menu window

### Contributing ###
I am open to further development, and i know that Frostwin is still missing features that i want it to have.
I wrote most of this code durring Christmas break 2025, and will be going back to college soon, so i probably wont be continuing to work on this for a while.
Feel free to make changes and submit pull requests. I may get a chance to look over the changes...

### x-win ###
For my task view to work, I used the x-win crate to get the active applications icons, but due to what i guess is a bug in the code of x-win, my usage wasn't working until i changed one line in x-win's code. now it works flawlessly in Frostwin! Because I am uncertain if this was a bug or just a specific uscase issue, I have included my modified code and not submitted a bug report.
