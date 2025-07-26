# cmdcreate v0.2
Allows you to create custom commands for your custom scripts. Works on any Linux system.

## Usage:

```
create <name> <file> <contents>
remove <name> <file>
list
```

## Examples

### Creation
```
$ ./cmdcreate create /usr/bin/say-hi say-hi.sh "echo hi"
$ say-hi
hi
```

### Deletion
```
$ ./cmdcreate remove /usr/bin/say-hi say-hi.sh
Removed ~/.local/share/cmdcreate/files/say-hi.sh and symlink /usr/bin/say-hi
```
