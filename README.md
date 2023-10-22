# leetfy
Leetfy encodes and transforms your text in leet using low frequency leetcode, or full frequency leetcode.
Low frequency leetcode is good for passwords, as you aren't using a full leet dialect, and it can enhance your passwords quality.

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

* **From file**
```
$ leetfy --filename ~/Documents/Caderno_de_poesias.txt full
C4)32~0 )3 ?035145

C4)32~0 )3 ?035145
é vm 8310 1v642.
T4~+45 [01545 11~)45
9v3 3v 605+4214 )3 f4142.
Ev f410 3m f02m4 )3 v32505
?424 +0)05 ?0)323m 35[v+42.
A6024 v0[ê ]á 5483
?02 9v3 05 ?03+45 ?4554m 05 )145
35[23v3~)0 3m 53v5 [4)32~05 )3 ?035145.
```