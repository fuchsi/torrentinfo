torrentinfo
===========

A torrent file parser

## Usage

```
USAGE:
    torrentinfo [OPTIONS] <filename>

OPTIONS:
    -d, --details       Show detailed information about the torrent
    -e, --everything    Print everything about the torrent
    -f, --files         Show files within the torrent
    -h, --help          Prints help information
    -n, --nocolour      No Colours
    -V, --version       Prints version information

ARGS:
    <filename>
```

## Installation

```bash
cargo install torrentinfo
```

Or from source

```bash
git clone https://github.com/fuchsi/torrentinfo.git
cd torrentinfo
cargo install
```