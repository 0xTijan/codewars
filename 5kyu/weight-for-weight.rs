fn order_weight(s: &str) -> String {
    let mut numbers: Vec<&str> = s
        .split_whitespace()
        .collect();
    
    numbers.sort_by(|a, b| {
        let sum_a = digit_sum(a);
        let sum_b = digit_sum(b);
        
        sum_a.cmp(&sum_b).then_with(|| a.cmp(b))
    });
    
    numbers.join(" ")
}

fn digit_sum(number: &str) -> u32 {
    number.chars()
        .filter_map(|c| c.to_digit(10))
        .sum()
}