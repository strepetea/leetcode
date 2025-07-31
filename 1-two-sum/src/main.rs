pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for num in &nums {
        let x = target - num;
        if nums.contains(&x) {
            let first_numpos = nums.iter().position(|i| i == num).unwrap() as i32;
            let second_offset = (first_numpos + 1) as usize;
            if let Some(pos) = nums
                .get(second_offset..)
                .and_then(|slice| slice.iter().position(|i| i == &x))
            {
                let second_numpos = (pos + second_offset) as i32;
                if first_numpos != second_numpos {
                    result.push(first_numpos);
                    result.push(second_numpos);
                    break;
                }
            }
        }
    }
    result
}
fn main() {
    let nums = vec![2, 2, 1, 5];
    let result = two_sum(nums, 4);
    for num in result {
        println!("{num}");
    }
}
