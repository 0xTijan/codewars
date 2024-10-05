mod preloaded;
use preloaded::MORSE_CODE;
// MORSE_CODE is `HashMap<String, String>`. e.g. ".-" -> "A".

pub fn decode_bits(encoded: &str) -> String {
    let rate = calculate_rate(encoded);
    let zeros_1 = "0".repeat(rate * 7);
    let words = encoded.trim_matches('0').split(&zeros_1);
    let mut morse_words = Vec::new();
    
    for word in words {
        let zeros_2 = "0".repeat(rate * 3);
        let letters = word.split(&zeros_2);
        let mut morse_letters: Vec<&str> = Vec::new();
        
        for letter in letters {
            let zeros_3 = "0".repeat(rate);
            let symbols = letter.split(&zeros_3);
            
            for x in symbols {
                if x.len() == rate * 3 {
                    morse_letters.push("-");
                } else {
                    morse_letters.push(".");
                }
            }
            
            morse_letters.push(" ");
        }
       
        morse_letters.pop();
        morse_words.push(morse_letters.join(""));
    }

    morse_words.join("   ")
}

pub fn calculate_rate(encoded: &str) -> usize {
    let split: Vec<&str> = encoded.trim_matches('0').split('0').collect();
    if split.len() <= 1 {
        return split[0].len();
    }
    
    let rate_zeros = encoded.split('0')
        .filter(|group| !group.is_empty())
        .map(|group| group.len())
        .min()
        .unwrap_or(1);
    
    let rate_ones = encoded.split('1')
        .filter(|group| !group.is_empty())
        .map(|group| group.len())
        .min()
        .unwrap_or(1);
    
    if rate_zeros < rate_ones {
        return rate_zeros;
    } else {
        return rate_ones;
    }
}

pub fn decode_morse(encoded: &str) -> String {
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