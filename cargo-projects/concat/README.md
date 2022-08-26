# Concat

**Concat** is a super simple **cat** command line util clone. It does not have all the features of **cat**.

## Usage

```
USAGE:
    concat [OPTIONS] [FILE]...

ARGS:
    <FILE>...    [default: -]

OPTIONS:
    -b, --number-nonblank    Number nonblank lines
    -h, --help               Print help information
    -n, --number             Number lines
    -V, --version            Print version information
```

### Example

```bash
$ concat -n file.txt
```