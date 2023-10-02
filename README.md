# tantivy-recover

Recover original data from tantivy index.

I wrote this tool to recover data from the index of [book-searcher](https://github.com/book-searcher-org/book-searcher), since they haven't provided raw data and they no longer provide the index since version 0.8.0.

## Usage

Download the binary (statically linked) from [release page](https://github.com/cmj2002/tantivy-recover/releases), make sure a valid `index/` directory exists in the same directory as the binary, then run the binary in this directory.

## Limitation

Since I am not familiar with tantivy or rust, and I am a bit lazy, this tool is not very robust. I only tested it on the index of [book-searcher](https://github.com/book-searcher-org/book-searcher), and it may not work on other tantivy index. If you encounter any problem, please open an issue, or even better, send a pull request.

Known limitation:
1. Only support `text` or `U64` field type.
