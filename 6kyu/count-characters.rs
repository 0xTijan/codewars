use std::collections::HashMap;

fn count(input: &str) -> HashMap<char, i32> {
    let mut char_count: HashMap<char, i32> = HashMap::new();
    
    for x in input.chars() {
        *char_count.entry(x).or_insert(0) += 1;
    }
    
    char_count
}