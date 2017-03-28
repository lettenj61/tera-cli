## Installation

`cargo install --git https://github.com/lettenj61/tera-cli`

## Usage

```
tera-cli 0.1.0
Command line interface for Tera template engine

USAGE:
    tera-cli.exe [FLAGS] [OPTIONS] [NAME]

FLAGS:
    -d, --dump-stdout    Print rendered HTML into STDOUT
    -h, --help           Prints help information
    -V, --version        Prints version information

OPTIONS:
    -c, --context <FILE>     Valid JSON file contains context data for rendering
    -o, --output <DEST>      Set output file to store rendered template
    -t, --template <GLOB>    Set globs to locate template files, defaults to
                             templates/**/*

ARGS:
    <NAME>    Name of the template to be rendered, defaults to
              templates/layout.html
```

## License
The software is licensed under MIT license.
