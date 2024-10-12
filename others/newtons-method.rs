/**
 * Newton's method a root-finding algorithm implementation.
 * 
 * https://en.wikipedia.org/wiki/Newton%27s_method
 */

fn find_root(num: i32, tolerance: f32) -> f32 {
    // check for valid input
    if num < 0 && tolerance >= 1.0 {
        panic!("Invalid inputs!")
    }

    // use this method
    let mut x: f32 = num as f32;

    loop {
        let root = 0.5 * (x + ((num as f32) / x));
        if (root-x).abs() < tolerance {
            break;
        }
        x = root;
    }

    // return approximate value
    x
}

fn main() {
    let result = find_root(21, 0.00000001);
    println!("The square root of {} is approximately {}", 21, result);
}