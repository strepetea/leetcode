pub fn is_palindrome(x: i32) -> bool {
    let mut digits_count: i32 = 0;
    let mut reversed_int: i32 = 0;
    if x < 0 { return false };
    if x == 0 {
        return true;
    } else {
        let mut x = x;
        while x != 0 {
            x /= 10;
            digits_count += 1;
        }
    }
    for i in 0..digits_count {
        reversed_int += ((x / 10i32.pow((i) as u32)) % 10) * 10i32.pow((digits_count - 1 - i) as u32);
    }
    x == reversed_int
}

fn main() {
    println!("{}", is_palindrome(10));
    println!("{}", is_palindrome(121));
    println!("{}", is_palindrome(-121));
}
