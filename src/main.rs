use std::time::{Duration, Instant};
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use lingua::Language::{English, Dutch};

fn main() {
    let languages = vec![English,Dutch];
    let detector: LanguageDetector = LanguageDetectorBuilder::from_languages(&languages).build();

    loop {
        
        let mut input_line = String::new();
        println!("Enter text: ");
        std::io::stdin().read_line(&mut input_line).unwrap();
        if input_line.trim() == "q" {
            println!("Quitting");
            break;
        }
        let start = Instant::now();
        let detected_language: Option<Language> = detector.detect_language_of(input_line);
        let duration: Duration = start.elapsed();
        println!("Time elapsed: {:?}", duration);
        if detected_language == Some(English) {
            println!("Detected English");
        } else if detected_language == Some(Dutch) {
            println!("Detected Dutch");
        } else {
            println!("Detection failed")
        }
    }
}
