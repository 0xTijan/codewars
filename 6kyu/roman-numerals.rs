fn num_as_roman(num: i32) -> String {
    let roman_values = [
        (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
        (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
        (10, "X"), (9, "IX"), (5, "V"), (4, "IV"),
        (1, "I")
    ];

    let mut roman_number = String::new();
    let mut n = num;
    
    for &(value, symbol) in &roman_values {
        while n >= value {
            roman_number.push_str(symbol);
            n -= value;
        }
    }

    roman_number
}