use std::cmp;
use std::collections::HashMap;
use std::io;

#[derive(Clone, Debug)]
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
    #[allow(dead_code)]
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

    // Reduce resource_maps down to a single map of seed-to-location.
    {
        let seed_string = String::from("seed");
        while resource_maps.len() > 1 {
            let mut first_map = resource_maps.remove(&seed_string).expect("first_map");
            let mut second_map = resource_maps
                .remove(&first_map.dest_resource)
                .unwrap_or_else(|| panic!("second_map {}", first_map.dest_resource));
            let combined_map = ResourceMap {
                dest_resource: second_map.dest_resource.clone(),
                ranges: common_ranges(&mut first_map.ranges, &mut second_map.ranges, true),
            };
            resource_maps.insert(seed_string.clone(), combined_map);
        }
    }

    let mut seed_to_location_map = resource_maps.remove("seed").expect("seed_to_location_map");
    // This is a bit like a resource map of seeds-to-seeds.
    let mut seed_ranges = seeds
        .chunks_exact(2)
        .map(|s_pair| Range {
            dest_start: s_pair[0],
            src_start: s_pair[0],
            length: s_pair[1],
        })
        .collect::<Vec<Range>>();
    let seeds_to_location_ranges =
        common_ranges(&mut seed_ranges, &mut seed_to_location_map.ranges, false);
    println!(
        "{}",
        seeds_to_location_ranges
            .iter()
            .map(|r| r.dest_start)
            .min()
            .unwrap()
    );
}

fn common_ranges(
    first: &mut Vec<Range>,
    second: &mut Vec<Range>,
    include_second: bool,
) -> Vec<Range> {
    let mut common = Vec::new();

    while !first.is_empty() {
        let f = first.pop().expect("first non-empty");
        if let Some(s_overlap_pos) = second.iter().position(|s| {
            f.dest_start < s.src_start + s.length && f.dest_start + f.length > s.src_start
        }) {
            let s = second.remove(s_overlap_pos);

            let max_start = cmp::max(f.dest_start, s.src_start);
            let min_end = cmp::min(f.dest_start + f.length, s.src_start + s.length);
            let common_length = min_end - max_start;
            assert!(common_length > 0);

            // dest_start should be translated into second's dest number space.
            // src_start should be translated into first's src number space.
            common.push(Range {
                dest_start: max_start - s.src_start + s.dest_start,
                src_start: max_start - f.dest_start + f.src_start,
                length: common_length,
            });

            // Push unshared prefix, if any, back onto the respective list.
            if f.dest_start < max_start {
                first.push(Range {
                    dest_start: f.dest_start,
                    src_start: f.src_start,
                    length: max_start - f.dest_start,
                });
            } else if s.src_start < max_start {
                second.push(Range {
                    dest_start: s.dest_start,
                    src_start: s.src_start,
                    length: max_start - s.src_start,
                });
            }

            // Push unshared suffix, if any, back onto the respective list.
            if f.dest_start + f.length > min_end {
                first.push(Range {
                    dest_start: min_end,
                    src_start: min_end - f.dest_start + f.src_start,
                    length: f.dest_start + f.length - min_end,
                });
            } else if s.src_start + s.length > min_end {
                second.push(Range {
                    dest_start: min_end - s.src_start + s.dest_start,
                    src_start: min_end,
                    length: s.src_start + s.length - min_end,
                });
            }
        } else {
            common.push(f);
        }
    }

    if include_second {
        common.extend_from_slice(&second[..]);
    }

    common
}
