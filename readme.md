# Workflows


## Functionality left to support
P0:
- Works on init

P1's
- Support other platforms 
    * in order to do this right, I think I need to just build a completely separate utility that works for zsh. I *could* make it work but it won't always be more accurate. 
    It's better for the user to request the specific package for the shell they're running rather than have it automatically figure out and fail. 
- Relative vs absolute workflows

Commands to support
- wf start:
    * make a copy of the current pwsh history
    * set a flag in a shared folder that you're in start mode
- wf fin
    * find the change in the history since you've started
    * save that to a location and make that an accessible binary
- wf list
    * list the open file names
- wf run --name name
    * run a specified binary
- wf alias


# Done:
P0's
- Recording things. 
- List'ing workflows
- Running workflows
- works for windows powershell

P1's
- deleting workflows
