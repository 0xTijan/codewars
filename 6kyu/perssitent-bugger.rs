fn persistence(num: u64) -> u64 {  
    let mut x = num;
    let mut counter = 0;
    
    while x > 9 {
        // find digits
        let mut digits: Vec<u64> = Vec::new();
        let mut temp = x;
        while temp > 0 {
            let digit = temp % 10;
            temp /= 10;
            digits.push(digit);
        }
        
        // multiply digits and save them in x
        let mut sum = 1;
        for y in digits {
            sum *= y;
        }
        x = sum;
        
        // update counter
        counter += 1;
    }
    
    counter
}