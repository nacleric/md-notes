# About
Command line utility to quickly view markdown files in the browser

## Subcommands
mdview supports 2 main commands

### Serve
Runs a small webserver to display markdown in the browser. Port will be set to `5000` by default.
```mdview serve <filepath> [port] ```

### Debug
Runs aspell in the background and print out a list of mispelled words in the terminal.
```mdview debug <filepath>```

