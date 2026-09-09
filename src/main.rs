use nysiis::{nysiis_opts, Options};

fn main() {
    let mut opts = Options::default();
    let mut words = Vec::new();

    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--keep-numbers" => opts.keep_numbers = true,
            _ => words.push(arg),
        }
    }

    for word in words {
        println!("{} -> {}", word, nysiis_opts(&word, opts));
    }
}
