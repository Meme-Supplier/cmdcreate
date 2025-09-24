# cmdcreate v0.6.1
Cmdcreate allows you to create custom commands for your Linux Terminal without needing to enter the same "complex" commands over and over (unless if your are lazy like me).

Cmdcreate will usually get smaller, more frequent updates than larger less frequent updates. You should update Cmdcreate often to stay up to date.
  
## Usage:

```
Commands:
  create <name> <contents>     Create a custom command
  remove <name>                Remove a custom command
  edit <name>                  Modify a custom command
  list                         Display custom commands
  search <command>             Searches for matched command
  reset                        Removes all installed commands
  display <command>            Display contents of a command
  rename <command> <new name>  Renames a command

 Update:
    check                      Checks for updates
    update                     Updates your system

Flags:
  --version                    Displays version
  --supported_editors          Displays supported text editors
  --changelog                  Displays changelog
  --license                    Displays license
  --debugging                  Displays flags used for debugging

  Offline:
    --get_offline_files        Downloads files for offline use
    --remove_offline_files     Removes files for offline use
```

# Installation

## Arch Linux
Install through the AUR

`$ yay -S cmdcreate` (Or another AUR manager)

## Debian/Ubuntu
You can get the `.deb` file from the latest [release](https://github.com/Meme-Supplier/cmdcreate/releases)

## Other
```
$ mv cmdcreate-<ver>-linux-bin cmdcreate
$ chmod +x cmdcreate
$ sudo cp cmdcreate /usr/bin/
```

## You can also run it portably

# Example usage

### Creation
```
$ cmdcreate create say-hi "echo hi"

Success! Created command: say-hi

$ say-hi
hi
```

### Deletion
```
$ cmdcreate remove say-hi
Are you sure you want to delete command "say-hi"? (y/N)
y

Removed command "say-hi"
```

### List commands
```
$ cmdcreate list
Installed commands: (1 installed)
--------
say-hi
```
