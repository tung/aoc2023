fn reduce_nums(nums: &mut [i64]) -> i64 {
    let mut length = nums.len();
    let mut sum: i64 = 0;
    while length >= 2 {
        sum += nums[length - 1];
        if nums.iter().take(length).all(|&n| n == 0) {
            break;
        }
        for i in 0..(length - 1) {
            nums[i] = nums[i + 1] - nums[i];
        }
        length -= 1;
    }
    sum
}

fn main() {
    let sum = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|w| w.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            nums.reverse();
            nums
        })
        .map(|mut nums| reduce_nums(&mut nums[..]))
        .sum::<i64>();
    println!("{sum:?}");
}
