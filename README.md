# caesar-eyes

**_caesar-eyes_** is a command-line application to cipher/decipher text files using the encryption technique of [Caesar Cipher](https://en.wikipedia.org/wiki/Caesar_cipher)

## Usage

### Encrypt word

```bash
$ caesar-eyes -c=13 "Here be dragons"

urer or qentbaf
```

### Decrypt word

```bash
$ caesar-eyes -d=13 "urer or qentbaf"

Here be dragons
```

### Execute a blind brute force attack

```bash
$ caesar-eyes -b "urer or qentbaf"
ksjc ak udtgdad
exmc of vsjkads
here be dragons  <--------
kcna ja klfansa
```

### Help

```bash
$ caesar-eyes --help
caesar-eyes 0.1.0
USAGE:
    caesar-eyes [FLAGS] [OPTIONS] [--] [input]

FLAGS:
    -b, --brutef     Brute force all possible rotations
    -h, --help       Prints help information
        --stdin      Read input from "stdin"
    -V, --version    Prints version information

OPTIONS:
    -c, --cipher <cipher>...        Cipher input with the given rotation(s) - using equals when declaring the flag is
                                    required
    -d, --decipher <decipher>...    Decipher input with the given rotation(s) - using equals when declaring the flag is
                                    required
    -f, --file <file>               Read [input] as a file
    -o, --output <output>           Define an output file

ARGS:
    <input>    Text to (de)cipher
```
