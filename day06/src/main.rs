fn main() {
    let mut lines = std::io::stdin().lines().map(Result::unwrap);

    let time_line = lines.next().expect("first line");
    let times = time_line
        .strip_prefix("Time:")
        .expect("'Time:' prefix")
        .split_whitespace()
        .map(|t| t.parse::<f64>().expect("f64 time value"));

    let distance_line = lines.next().expect("second line");
    let distances = distance_line
        .strip_prefix("Distance:")
        .expect("'Distance:' prefix")
        .split_whitespace()
        .map(|d| d.parse::<f64>().expect("f64 distance value"));

    let product: f64 = times
        .zip(distances)
        .map(|(t, d)| {
            let min_hold = f64::floor(0.5 * (t - f64::sqrt(t * t - 4.0 * d)));
            t - 1.0 - min_hold * 2.0
        })
        .product();
    println!("{product}");
}
