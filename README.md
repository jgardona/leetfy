# leetfy
Leetfy encodes and transforms your text in leet using low frequency leet alphabet, and full leet alphabet.
Low frequency leet is good for passwords, as you aren't using a full leet dialect, and it can enhance your passwords quality.

* **Install**

```
cargo install --git https://github.com/jcbritobr/leetfy
```

* **Test**

Download or clone the source code.
```
cargo test
```

* **Use**
```
$ leetfy --help

Leetfy your texts

Usage: leetfy --filename <FILENAME> <MODE>

Arguments:
  <MODE>  [possible values: low, full]

Options:
  -f, --filename <FILENAME>  
  -h, --help                 Print help
  -V, --version              Print version
```