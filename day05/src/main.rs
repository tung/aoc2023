use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Range {
    dest_start: i64,
    src_start: i64,
    length: i64,
}

#[derive(Debug)]
struct ResourceMap {
    dest_resource: String,
    ranges: Vec<Range>,
}

impl ResourceMap {
    fn map(&self, input: i64) -> i64 {
        for r in &self.ranges {
            if input >= r.src_start && input < r.src_start + r.length {
                return input - r.src_start + r.dest_start;
            }
        }
        return input;
    }
}

pub fn main() {
    let mut lines = io::stdin().lines();

    let seeds = {
        let line = lines.next().unwrap().unwrap();
        let line = line.strip_prefix("seeds: ").expect("'seeds: ' prefix");
        line.split_whitespace()
            .map(|s| s.parse::<i64>().expect("seed i64"))
            .collect::<Vec<i64>>()
    };

    // Skip blank line after seeds line.
    lines.next();

    let mut resource_maps: HashMap<String, ResourceMap> = HashMap::new();

    while let Some(line) = lines.next() {
        // Read map line and parse resources.
        let line = line.expect("map line");
        let line = line.strip_suffix(" map:").expect("' map:' suffix");
        let (src_res, dest_res) = line.split_once("-to-").expect("");
        let mut res_map = ResourceMap {
            dest_resource: dest_res.to_string(),
            ranges: Vec::new(),
        };

        // Process range lines.
        while let Some(line) = lines.next() {
            let line = line.expect("range line");
            if line.is_empty() {
                break;
            }
            let num_vec = line
                .split_whitespace()
                .map(|s| s.parse::<i64>().expect("mapping i64"))
                .collect::<Vec<i64>>();
            assert_eq!(3, num_vec.len());
            res_map.ranges.push(Range {
                dest_start: num_vec[0],
                src_start: num_vec[1],
                length: num_vec[2],
            });
        }

        resource_maps.insert(src_res.to_string(), res_map);
    }

    // Trace seeds through all resources until there's no resource left to trace.
    let locations = seeds
        .iter()
        .map(|seed| {
            let mut res = String::from("seed");
            let mut value = *seed;
            loop {
                if let Some(res_map) = resource_maps.get(&res) {
                    value = res_map.map(value);
                    res = res_map.dest_resource.clone();
                } else {
                    return value;
                }
            }
        })
        .collect::<Vec<i64>>();
    println!("{}", locations.iter().min().expect("min location"));
}
