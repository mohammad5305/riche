# Riche: A Rust-Based Alternative to Fiche

Riche is a minimalist pastebin tool that draws inspiration from [fiche](https://github.com/solusipse/fiche). Built using Rust, Riche is designed for simplicity, efficiency, and ease of use.

## Installation
download from `release` section or build from source
```bash
git clone https://github.com/mohammad5305/riche.git && cd riche
cargo build --release
cp ./target/release/riche /usr/local/bin
```

## Usage
To run the Riche server, use the following command:
```bash
riche -d "test.com" -p 9999 -o ./codes
```

This will start the server at port 9999 and will use `./codes` as the directory for storing the pasted files.

To send a code snippet from the client using raw TCP, simply use a command like:
```bash
echo "hello, world!" | nc test.com 9999
```
The server will return a URL for accessing the pasted code.

### Blacklist 
you can pass a list of ip with or without CIDR for blocking, example:
```bash
$ cat blacklist.txt
192.168.8.100
127.0.0.0/8
$ riche -p 9999 -o ./codes --blacklist ./blacklist.txt
```
This will block any connection from `192.168.8.100` and any ip within `127.0.0.0`
