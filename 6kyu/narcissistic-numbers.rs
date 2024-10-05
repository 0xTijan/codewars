fn narcissistic(num: u64) -> bool {
    let num_string = num.to_string();
    let split_string = num_string.split("");

    let mut digits: Vec<u64> = Vec::new();
    let mut sum: u64 = 0;

    for x in split_string {
        if x.len() > 0 {
            let digit_res = x.parse::<u64>();
            if let Ok(digit) = digit_res {
                digits.push(digit);
            }            
        }
    }
        
    for digit in &digits {
        let power: u32 = digits.len() as u32;
        sum += u64::pow(*digit, power);
    }
        
    sum == num
}