# cmdcreate v0.4.2
Cmdcreate allows you to create custom commands for your Linux Terminal without needing to enter the same "complex" commands over and over (unless if your are lazy like me).

Cmdcreate will usually get smaller, more frequent updates than larger less frequent updates. You should update Cmdcreate often to stay up to date.
  
## Usage:

```
create <name> <contents>    Create a custom command
remove <name>               Remove a custom command
edit <name>                 Modify a custom command
list                        Display custom commands

Flags:

--version                   Displays cmdcreate's version
--supported_editors         Displays supported text editors
```

# Installation

## Arch Linux
`$ yay -S cmdcreate`

## Other
```
$ sudo mv cmdcreate-<ver>-bin cmdcreate
$ sudo chmod +x cmdcreate
$ sudo cp cmdcreate /usr/bin
```

# Example usage

### Creation
```
$ ./cmdcreate create say-hi "echo hi"

Success! Created executable:
  Script path: /home/user/.local/share/cmdcreate/files/say-hi
  Symlink path: /usr/bin/say-hi

$ say-hi
hi
```

### Deletion
```
$ ./cmdcreate remove say-hi
Are you sure you want to delete command "say-hi"? (y/N)
y

Removed command "say-hi"
```

### List commands
```
$ ./cmdcreate list
Installed commands:

<command> <symlink>

say-hi -> /usr/bin/say-hi
```
