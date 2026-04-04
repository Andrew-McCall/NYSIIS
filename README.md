# nysiis

A fast NYSIIS (New York State Identification and Intelligence System) phonetic encoding library and CLI.

_Dependency Free_

## Usage

Encode one or more words:

```sh
nysiis Franklin Wheeler Mitchell
Franklin -> FRANCLAN
Wheeler  -> WALAR
Mitchell -> MATCAL
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
