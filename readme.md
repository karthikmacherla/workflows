# Workflows

This powershell utility allows you to quickly write and save workflows without having to go through the trouble of writing shell scripts yourself. 

Simply hit `workflows.exe start`, type the commands you want, hit `workflows.exe fin` when done to save the sequence of commands you just executed. 


# Setting up

`cargo build --release` to generate the release binary and then move the `.\target\release\workflows.exe` to your desired location with your desired alias. 

By default, this utility assumes the start command will look like `.*workflows.exe start`. If you alias the utility, you have to tell the utility what the new start command looks like with something like:

`workflows.exe alias "wf start"`

** This decision was taken in case you want to even alias out the `start` part of the recording process. You could hypothetically do something like:

```
Set-Alias start ".\workflows.exe start"
```
and then set the start to:

`workflows.exe alias "start"`

