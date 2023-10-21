# leetfy
Leetfy encodes and transforms your text in leet using low frequency leetcode frequency, or full frequency leetcode.
Low frequency leet is good for passwords, as you aren't using a full leet dialect, and it can enhance your passwords quality.

* **Install**

```
cargo install leetfy
```

* **Test**

Download or clone the source code.
```
cargo test
```

* **Usage**
```
$ leetfy -h
Leetfy your texts

Usage: leetfy [OPTIONS] <MODE>

Arguments:
  <MODE>  The dictionary type [possible values: low, full]

Options:
  -f, --filename <filename>  Read from a file
  -s, --stdin                Read from stdin
  -h, --help                 Print help
  -V, --version              Print version
```

* **From stdin**
```
$ echo "the quick brown fox jumps over the lazy dog" | leetfy -s low
th3 qu1ck 8r0wn f0x jump5 0v3r th3 l4zy d0g
```