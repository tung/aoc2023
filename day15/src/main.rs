fn hash(s: &str) -> u32 {
    let mut v: u32 = 0;
    for c in s.chars() {
        let a = (c as u32) & 255;
        v += a;
        v *= 17;
        v %= 256;
    }
    v & 255
}

fn main() {
    let sum = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.split(',').map(hash).sum::<u32>())
        .sum::<u32>();
    println!("{sum}");
}

#[test]
#[rustfmt::skip]
fn test_hash() {
    assert_eq!(52, hash("HASH"));
}
