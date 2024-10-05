fn move_zeros(arr: &[u8]) -> Vec<u8> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }
    
    let mut result: Vec<u8> = Vec::new();
    
    // push all non zero elements to new array
    for element in arr {
        if *element != 0 {   
            result.push(*element);
        }
    }
    
    // check the difference and add zeros
    let difference = arr.len() - result.len();
    for _ in 0..difference {
        result.push(0 as u8);
    }
    
    return result;
}