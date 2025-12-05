use std::fs;
use std::time::{Duration, Instant};

pub fn parse_file(_file_path:String) -> String {
    fs::read_to_string(_file_path).expect("Unable to read file")
}

pub fn mesurer<F, R>(f: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    (result, start.elapsed())
}

pub fn afficher_resultats(jour: u32, resultat1: impl std::fmt::Display, temps1: Duration, resultat2: impl std::fmt::Display, temps2: Duration) {
    println!("\n🎄 Jour {} 🎄", jour);
    println!("  Énigme 1 ({:>8.2?}) : {}", temps1, resultat1);
    println!("  Énigme 2 ({:>8.2?}) : {}", temps2, resultat2);
}