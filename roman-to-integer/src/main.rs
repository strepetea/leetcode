use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let mut result: i32 = 0;
    
    let roman_subtractions = HashMap::from([
        ("CM", 900),
        ("CD", 400),
        ("XC", 90),
        ("XL", 40),
        ("IX", 9),
        ("IV", 4),
    ]);
    let roman_nums = HashMap::from([
        ("M", 1000),
        ("D", 500),
        ("C", 100),
        ("L", 50),
        ("X", 10),
        ("V", 5),
        ("I", 1),
        ("N", 0),
    ]);

    let mut subtractions_handled: bool = false;
    let mut s_clone = s.clone();
    
    'outer: while !subtractions_handled {
        for (r, i) in &roman_subtractions {
            if s_clone.contains(r) {
                s_clone = s_clone.replace(r, "N");
                result += i;
                continue 'outer;
            }
        }
        subtractions_handled = true;
    }

    for c in s_clone.chars() {
        for (r, i) in &roman_nums {
            let c = c.to_string();
            if c == *r {
                result += i;
            }
        }
    }
    
    result
}

fn main() {
    let toprint: i32 = roman_to_int(String::from("LVIII"));
    println!("{toprint}");
}
