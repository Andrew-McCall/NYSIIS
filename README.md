# nysiis

A fast NYSIIS (New York State Identification and Intelligence System) phonetic encoding library and CLI.

_Dependency Free_

## Example

```
Franklin -> FRANCLAN
Wheeler  -> WALAR
Mitchell -> MATCAL
```

## Usage

Encode one or more words:

```sh
nysiis Franklin Wheeler Mitchell
```

## How It Works

[NYSIIS](https://en.wikipedia.org/wiki/New_York_State_Identification_and_Intelligence_System) is a phonetic algorithm that reduces names to a canonical key, so that names that sound alike map to the same code. It handles common English spelling variations: prefix normalisation (`MAC` → `MCC`, `KN` → `NN`, `PH`/`PF` → `FF`, `SCH` → `SSS`), suffix trimming (`EE`/`IE` → `Y`, `DT`/`RT`/`RD`/`NT`/`ND` → `D`), vowel collapsing, and character substitutions (`Q`→`G`, `Z`→`S`, `M`→`N`, etc.), followed by deduplication of adjacent identical characters.

```rust
use nysiis::nysiis;

fn main() {
    println!("{}", nysiis("Franklin")); // FRANCLAN
}
```

## License

MIT License

Copyright (c) 2026

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
