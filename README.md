# catsay 0.1.0
## Usage
```
Show a cat banner saying a message (done in Rust)

USAGE:
    catsay.exe [FLAGS] [OPTIONS] [message]

FLAGS:
    -c, --color      Use color
    -d, --dead       Make that cat appear dead
    -h, --help       Prints help information
    -i, --stdin      Read the message from STDIN instead of the argument
    -V, --version    Prints version information

OPTIONS:
    -f, --file <catfile>    Load the cat picture from the specified file (ASCII)

ARGS:
    <message>    What does the cat say? [default: Meow!]
```
## Example
```
Hello World!
    \
     \
      \  /\__/\
        /`    '\
      === o  o ===
        \  --  /
       /        \
      /          \
     |            |
      \  ||  ||  /
       \_oo__oo_/#######o
       
I'm dead...
    \
     \
      \  /\__/\
        /`    '\
      === x  x ===
        \  --  /
       /        \
      /          \
     |            |
      \  ||  ||  /
       \_oo__oo_/#######o
```
## License
[Apache License 2.0](https://github.com/riki420/catsay/blob/master/LICENSE) 
