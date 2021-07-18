Encryption-rs
====

This encrypt and decrypt file.(using Rust programming language) 

> **WARNING:**  
> This is not made with security in mind.
> Therefore, if you actually use it, please do so at your own risk.

## Description

This can be used to encrypt files.
You can also use the ID generated during encryption to decrypt the file.

## Usage

encrypting is `encrypt` command.
```
USAGE:
    encryption.exe encrypt [OPTIONS] <file-path>

ARGS:
    <file-path>    File to be encrypted.

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --id <id>            ID to be used for encryption
    -l, --length <length>    length of ID
    -o, --output <output>    Output destination of encrypting file
```

decryption is `decrypt` command.
```
USAGE:
    encryption.exe decrypt [OPTIONS] <id> <file-path>

ARGS:
    <id>           The ID generated during encryption that is required for decryption.
    <file-path>    File to be decrypted.

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <output>    Output destination of decrypting file.
```

Sentences as above is printed by `help` command or `-h`,`--help` option. 

### Options

`-h` `--help`
Prints help information.

`-V` `--version`
Prints version information.

`-i <id>` `--id <id>`
Manually set the ID to be used for encryption.

`-l <length>` `--length <length>`
set length of the ID to be used for encryption. Default is 20.

`-o <output>` `--output <output>`
set output destination of encrypting file. Default is the same as encrypting file.


## Licence

This project is licensed under the terms of the [MIT license](https://github.com/waryu-YND/encryption-rs/blob/main/LICENSE).