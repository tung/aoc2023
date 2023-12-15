fn hash(s: &str) -> usize {
    let mut v: usize = 0;
    for c in s.chars() {
        let a = (c as usize) & 255;
        v += a;
        v *= 17;
        v %= 256;
    }
    v & 255
}

fn main() {
    let mut boxes: Vec<Vec<(String, u8)>> = vec![Vec::new(); 256];

    for step in std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .next()
        .expect("non-empty first line")
        .split(',')
    {
        if let Some((label, focal_length_str)) = step.split_once('=') {
            let focal_length = focal_length_str.parse::<u8>().expect("u8 focal_length");
            let i = hash(label);
            if let Some(lens) = boxes[i].iter_mut().find(|l| l.0 == label) {
                lens.1 = focal_length;
            } else {
                boxes[i].push((label.to_string(), focal_length));
            }
        } else if let Some(label) = step.strip_suffix('-') {
            let i = hash(label);
            if let Some(pos) = boxes[i].iter().position(|l| l.0 == label) {
                boxes[i].remove(pos);
            }
        } else {
            unreachable!();
        }
    }

    let power = boxes
        .iter()
        .enumerate()
        .map(|(bi, b)| {
            b.iter()
                .enumerate()
                .map(|(li, (_, fl))| (bi + 1) * (li + 1) * (*fl as usize))
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{power}");
}

#[test]
#[rustfmt::skip]
fn test_hash() {
    assert_eq!(52, hash("HASH"));
}
