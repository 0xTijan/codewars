mod preloaded;
use preloaded::MORSE_CODE;
// MORSE_CODE is `HashMap<String, String>`. e.g. ".-" -> "A".

fn decode_morse(encoded: &str) -> String {
    let words = encoded.split("   ");
    let mut real_letters: Vec<&str> = Vec::new();
    
    for word in words {
        real_letters.push(" ");

        let letters = word.split(" ");
        for letter in letters {
            let real_letter = MORSE_CODE.get(letter);
            if let Some(l) = real_letter {   
                real_letters.push(l);
            }
        }
    }
    
    real_letters.join("").trim().to_string()
}