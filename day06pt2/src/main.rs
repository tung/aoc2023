fn main() {
    let mut lines = std::io::stdin().lines().map(Result::unwrap);

    let time_line = lines.next().expect("first line");
    let time = time_line
        .strip_prefix("Time:")
        .expect("'Time:' prefix")
        .split_whitespace()
        .map(|t| (t.len() as i32, t.parse::<f64>().expect("f64 time value")))
        .reduce(|acc, e| (0, acc.1 * 10_f64.powi(e.0) + e.1))
        .map(|(_, t)| t)
        .expect("final time");

    let distance_line = lines.next().expect("second line");
    let distance = distance_line
        .strip_prefix("Distance:")
        .expect("'Distance:' prefix")
        .split_whitespace()
        .map(|d| {
            (
                d.len() as i32,
                d.parse::<f64>().expect("f64 distance value"),
            )
        })
        .reduce(|acc, e| (0, acc.1 * 10_f64.powi(e.0) + e.1))
        .map(|(_, d)| d)
        .expect("final distance");

    let min_hold = f64::floor(0.5 * (time - f64::sqrt(time * time - 4.0 * distance)));
    let answer = time - 1.0 - min_hold * 2.0;
    println!("{answer}");
}
