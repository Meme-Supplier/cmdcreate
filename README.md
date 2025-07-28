# cmdcreate v0.3
Allows you to create custom commands for your custom scripts. Works on any Linux system.

## Usage:

```
create <name> <contents>
remove <name>
list
```

## Installation
```
$ sudo mv cmdcreate-<ver>-bin cmdcreate
$ sudo chmod +x cmdcreate
$ sudo cp cmdcreate /usr/bin
```

## Removal
`$ sudo rm /usr/bin/cmdcreate`

## Examples

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
