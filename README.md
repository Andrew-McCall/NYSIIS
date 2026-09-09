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

### Options

`--keep-numbers` keeps digits in the input instead of stripping them:

```sh
nysiis --keep-numbers Bishop2
Bishop2 -> BASAP2
```

Without the flag, digits are dropped (`Bishop2` -> `BASAP`).

## How It Works

[NYSIIS](https://en.wikipedia.org/wiki/New_York_State_Identification_and_Intelligence_System) is a phonetic algorithm that reduces names to a canonical key, so that names that sound alike map to the same code. It handles common English spelling variations: prefix normalisation (`MAC` → `MCC`, `KN` → `NN`, `PH`/`PF` → `FF`, `SCH` → `SSS`), suffix trimming (`EE`/`IE` → `Y`, `DT`/`RT`/`RD`/`NT`/`ND` → `D`), vowel collapsing, and character substitutions (`Q`→`G`, `Z`→`S`, `M`→`N`, etc.), followed by deduplication of adjacent identical characters.

```rust
use nysiis::nysiis;

fn main() {
    println!("{}", nysiis("Franklin")); // FRANCLAN
}
```

Use `nysiis_opts` with an `Options` struct for control over encoding behaviour, such as keeping digits:

```rust
use nysiis::{nysiis_opts, Options};

fn main() {
    let opts = Options { keep_numbers: true };
    println!("{}", nysiis_opts("Bishop2", opts)); // BASAP2
}
```

## License

MIT License

Copyright (c) 2026
