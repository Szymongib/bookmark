# Usage

Bookmark can be used in one of two modes:
- Interactive mode
- Standard CLI mode

## Interactive mode

Interactive mode opens simple user interface in the terminal, allowing user to open, delete and modify bookmarks.
To open interactive mode, simply run:
```bash
bookmark
```

### Controls

To display list of available controls, press `h` while in the interactive mode.

Other controls include:

| Key | Action |
|:-------:|:------:|
| `ENTER` | Opens bookmarked URL in default browser |
| `/` or `CTRL + f` | Starts bookmark search |
| `h` | Shows/Hides the help panel |
| `d` | Deletes URL (confirmation needed) |
| `i` | Shows/Hides bookmark ids |
| `q` | Exits interactive mode |
| `:` | Enters command input mode |

### Commands

Command input mode can be enabled with the `:` while in the interactive mode.

Commands will be applied to the currently selected bookmark in the table.

The commands names are similar to those in the Standard mode:

| Command | Arguments | Action |
|:-------:|:---------:|:------:|
| `tag` | [TAG_NAME] | Adds tag to the bookmark |
| `untag` | [TAG_NAME] | Removes tag from the bookmark |
| `chg` | [NEW_GROUP] | Changes group of the bookmark |
| `chn` | [NEW_NAME] | Changes name of the bookmark |
| `chu` | [NEW_URL] | Changes URL of the bookmark |
| `q` | - | Exits interactive mode |


## Standard mode

In standard mode use commands directly in the terminal following the pattern:
```bash
bookmark [COMMAND] [OPTS] [ARGS]
```
For example to list all bookmarks in group `dev`, run:
```bash
bookmark list -g dev
```

### Commands

To see available commands together with the description, run:
```bash
bookmark -h
```
To see options and arguments of the specific command, run:
```bash
bookmark [COMMAND] -h
```
