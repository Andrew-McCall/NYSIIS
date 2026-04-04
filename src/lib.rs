pub fn nysiis(input: &str) -> String {
    let s: Vec<u8> = input
        .bytes()
        .filter_map(|b| match b {
            b'a'..=b'z' => Some(b - 32),
            b'A'..=b'Z' => Some(b),
            _ => None,
        })
        .collect();

    if s.is_empty() {
        return String::new();
    }

    fn is_vowel(c: u8) -> bool {
        matches!(c, b'A' | b'E' | b'I' | b'O' | b'U')
    }

    let mut buf = s;

    if buf.starts_with(b"MAC") {
        buf[0..3].copy_from_slice(b"MCC");
    } else if buf.starts_with(b"KN") {
        buf[0..2].copy_from_slice(b"NN");
    } else if buf.starts_with(b"K") {
        buf[0] = b'C';
    } else if buf.starts_with(b"PH") || buf.starts_with(b"PF") {
        buf[0..2].copy_from_slice(b"FF");
    } else if buf.starts_with(b"SCH") {
        buf[0..3].copy_from_slice(b"SSS");
    }

    if buf.len() >= 2 {
        let n = buf.len();
        if [b"EE", b"IE"].iter().any(|&suf| &buf[n - 2..] == suf) {
            buf.truncate(n - 2);
            buf.push(b'Y');
        } else if [b"DT", b"RT", b"RD", b"NT", b"ND"].iter().any(|&suf| &buf[n - 2..] == suf) {
            buf.truncate(n - 2);
            buf.push(b'D');
        }
    }

    let mut out: Vec<u8> = Vec::with_capacity(buf.len());
    out.push(buf[0]);

    let n = buf.len();
    let mut i = 1;
    while i < n {
        let mut cur = buf[i];

        match cur {
            b'E' if i + 1 < n && buf[i + 1] == b'V' => cur = b'A',
            b'A' | b'E' | b'I' | b'O' | b'U' => cur = b'A',
            b'Q' => cur = b'G',
            b'Z' => cur = b'S',
            b'M' => cur = b'N',
            b'K' => cur = if i + 1 < n && buf[i + 1] == b'N' { b'N' } else { b'C' },
            b'S' if i + 2 < n && &buf[i..i + 3] == b"SCH" => {
                cur = b'S';
                i += 2;
            }
            b'P' if i + 1 < n && buf[i + 1] == b'H' => {
                cur = b'F';
                i += 1;
            }
            b'H' => {
                if !is_vowel(buf[i - 1]) || i + 1 >= n || !is_vowel(buf[i + 1]) {
                    cur = buf[i - 1];
                }
            }
            b'W' if is_vowel(buf[i - 1]) => cur = b'A',
            _ => {}
        }

        if out.last().copied() != Some(cur) {
            out.push(cur);
        }
        i += 1;
    }

    if out.last() == Some(&b'S') {
        out.pop();
    }

    if out.len() >= 2 {
        let n = out.len();
        if out[n - 2] == b'A' && out[n - 1] == b'Y' {
            out.truncate(n - 2);
            out.push(b'Y');
        }
    }

    if out.last() == Some(&b'A') {
        out.pop();
    }

    String::from_utf8(out).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::nysiis;

    #[test]
    fn matches_given_examples() {
        let cases = [
            ("Bishop", "BASAP"),
            ("Carlson", "CARLSAN"),
            ("Carr", "CAR"),
            ("Chapman", "CAPNAN"),
            ("Franklin", "FRANCLAN"),
            ("Greene", "GRAN"),
            ("Harper", "HARPAR"),
            ("Jacobs", "JACAB"),
            ("Larson", "LARSAN"),
            ("Lawrence", "LARANC"),
            ("Lawson", "LASAN"),
            ("Louis, XVI", "LASXV"),
            ("Lynch", "LYNC"),
            ("Mackenzie", "MCANSY"),
            ("Matthews", "MAT"),
            ("McCormack", "MCARNAC"),
            ("McDaniel", "MCDANAL"),
            ("McDonald", "MCDANALD"),
            ("Mclaughlin", "MCLAGLAN"),
            ("Morrison", "MARASAN"),
            ("O'Banion", "OBANAN"),
            ("O'Brien", "OBRAN"),
            ("Richards", "RACARD"),
            ("Silva", "SALV"),
            ("Watkins", "WATCAN"),
            ("Wheeler", "WALAR"),
            ("Willis", "WAL"),
            ("brown, sr", "BRANSR"),
            ("browne, III", "BRAN"),
            ("browne, IV", "BRANAV"),
            ("knight", "NAGT"),
            ("mitchell", "MATCAL"),
            ("o'daniel", "ODANAL"),
        ];

        for (input, expected) in cases {
            assert_eq!(nysiis(input), expected, "{input}");
        }
    }
}
