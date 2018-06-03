# vape
ｆｕｌｌ ｗｉｄｔｈ ａｅｓｔｈｅｔｉｃｓ

[![](https://img.shields.io/crates/v/vape.svg)](https://crates.io/crates/vape) [![Build Status](https://travis-ci.org/JoshuaRLi/vape.svg?branch=master)](https://travis-ci.org/JoshuaRLi/vape) [![License](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/JoshuaRLi/vape/blob/master/LICENSE)

`vape` is a command-line tool that converts text into "vaporwave" text - fullwidth UTF-8 characters.

More technically, characters in the range `U+0021` to `U+007E` are translated forward by `0xFEE0`, and the space character `U+0020` is converted to the ideographic (fullwidth) space `U+3000`.


## Usage

```sh
$ printf 'the longer you live, the more ad revenue you generate\n' | vape
ｔｈｅ　ｌｏｎｇｅｒ　ｙｏｕ　ｌｉｖｅ，　ｔｈｅ　ｍｏｒｅ　ａｄ　ｒｅｖｅｎｕｅ　ｙｏｕ　ｇｅｎｅｒａｔｅ
```

You can also append up to 255 random fullwidth katakana:

```sh
$ printf 'born to die\n' | vape -k 5
ｂｏｒｎ　ｔｏ　ｄｉｅ　リマヾソル
```

## Installation

`cargo install vape`

Alternatively, direct binary downloads for a variety of 64-bit platforms can be found on the [releases](https://github.com/JoshuaRLi/vape/releases) page.

`vape` is also available on the AUR as [`vape`](https://aur.archlinux.org/packages/vape)!