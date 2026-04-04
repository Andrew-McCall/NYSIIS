use nysiis::nysiis;

fn main() {
    for word in std::env::args().skip(1) {
        println!("{} -> {}", word, nysiis(&word));
    }
}
