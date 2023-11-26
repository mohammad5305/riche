# Riche: A Rust-Based Alternative to Fiche

Riche is a minimalist pastebin tool that draws inspiration from [fiche](https://github.com/solusipse/fiche). Built using Rust, Riche is designed for simplicity, efficiency, and ease of use.

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
