use std::collections::{HashMap, VecDeque};

fn find_path(graph: &Vec<Vec<u16>>, start: u16, goal: u16) -> Option<Vec<u16>> {
    let mut came_from: Vec<Option<u16>> = vec![None; graph.len()];
    let mut queue: VecDeque<u16> = VecDeque::new();
    queue.push_back(start);
    while let Some(current) = queue.pop_front() {
        for adjacent in graph[current as usize].iter().copied() {
            if came_from[adjacent as usize].is_none() {
                came_from[adjacent as usize] = Some(current);
                if adjacent == goal {
                    let mut path: Vec<u16> = vec![goal];
                    let mut prev = came_from[goal as usize];
                    while let Some(prev_inner) = prev {
                        path.push(prev_inner);
                        if prev_inner == start {
                            break;
                        }
                        prev = came_from[prev_inner as usize];
                    }
                    return Some(path);
                }
                queue.push_back(adjacent);
            }
        }
    }
    return None;
}

fn remove_path(graph: &mut Vec<Vec<u16>>, path: &[u16]) {
    for edge in path.windows(2) {
        let a = edge[0];
        let b = edge[1];
        graph[a as usize].retain(|&x| x != b);
        graph[b as usize].retain(|&x| x != a);
    }
}

fn count_subgraph(graph: &Vec<Vec<u16>>, start: u16, avoid: u16) -> Option<usize> {
    let mut visited: Vec<bool> = vec![false; graph.len()];
    let mut queue: VecDeque<u16> = VecDeque::new();
    visited[start as usize] = true;
    queue.push_back(start);
    while let Some(current) = queue.pop_front() {
        for adjacent in graph[current as usize].iter().copied() {
            if !visited[adjacent as usize] {
                visited[adjacent as usize] = true;
                if adjacent == avoid {
                    return None;
                }
                queue.push_back(adjacent);
            }
        }
    }
    return Some(visited.iter().filter(|&v| *v).count());
}

fn main() {
    let mut labels: HashMap<String, u16> = HashMap::new();
    let mut graph: Vec<Vec<u16>> = Vec::new();

    for line in std::io::stdin().lines().map(Result::unwrap) {
        let mut tokens = line.split_whitespace();

        let label1 = tokens.next().expect("label1").trim_end_matches(':');
        let labels_len = labels.len();
        let vertex1 = *labels
            .entry(label1.to_string())
            .or_insert(labels_len.try_into().expect("new u16 vertex"));
        if vertex1 as usize >= graph.len() {
            graph.resize(vertex1 as usize + 1, Vec::new());
        }

        for label2 in tokens {
            let labels_len = labels.len();
            let vertex2 = *labels
                .entry(label2.to_string())
                .or_insert(labels_len.try_into().expect("new u16 vertex"));
            if vertex2 as usize >= graph.len() {
                graph.resize(vertex2 as usize + 1, Vec::new());
            }
            graph[vertex1 as usize].push(vertex2);
            graph[vertex2 as usize].push(vertex1);
        }
    }

    for a in 0..graph.len() - 1 {
        for b in a + 1..graph.len() {
            let a = u16::try_from(a).expect("u16 a");
            let b = u16::try_from(b).expect("u16 b");

            let mut temp_graph = graph.clone();
            let path1 = find_path(&temp_graph, a, b).expect("path1");
            remove_path(&mut temp_graph, &path1[..]);
            let path2 = find_path(&temp_graph, a, b).expect("path2");
            remove_path(&mut temp_graph, &path2[..]);
            let path3 = find_path(&temp_graph, a, b).expect("path3");
            remove_path(&mut temp_graph, &path3[..]);

            if let Some(size) = count_subgraph(&temp_graph, a, b) {
                println!(
                    "{} * {} = {}",
                    size,
                    graph.len() - size,
                    size * (graph.len() - size),
                );
                return;
            }
        }
    }
}
