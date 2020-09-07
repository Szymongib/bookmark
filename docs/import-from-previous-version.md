# Import from previous version

The structure of bookmarks data can change between certain versions. 
For example between versions `v0.0.x` and `v0.1.x`.

To make it easy to update between versions, Bookmark introduces `import` command that should correctly move old bookmarks to be compatible with the new version.

If you are using default file paths to store bookmarks, simply run:
```bash
bookmark import
```

In a case that you used custom file, you can specify it with the `--old-file` flag:
```bash
bookmark import --old-file /home/my-old-bookmark-file.json 
```
