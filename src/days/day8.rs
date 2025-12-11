use std::collections::BTreeMap;

pub fn solve_part1(input: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let lines: Vec<Vec<i64>> = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<i64>().unwrap_or(0))
                .collect()
        })
        .collect();

    let mut peers: Vec<Vec<usize>> = Vec::new();
    let mut map: BTreeMap<i64, (usize, usize)> = BTreeMap::new();

    for (i, _) in lines.iter().enumerate() {
        peers.push(vec![i]);
    }

    get_peer_map(&lines, &mut map);

    for _ in 0..1000 {
        if let Some((_, value)) = map.pop_first() {
            let (a, b) = value;

            let group_a_idx =
                find_group_index(&peers, a).ok_or("Something went wrong during index lookup")?;
            let group_b_idx =
                find_group_index(&peers, b).ok_or("Something went wrong during index lookup")?;

            if group_a_idx != group_b_idx {
                let (low_idx, high_idx) = if group_a_idx < group_b_idx {
                    (group_a_idx, group_b_idx)
                } else {
                    (group_b_idx, group_a_idx)
                };

                let (left, right) = peers.split_at_mut(high_idx);
                left[low_idx].append(&mut right[0]);
                peers.remove(high_idx);
                continue;
            }
        }
    }

    let answer = {
        let mut sorted = peers.clone();
        sorted.sort_by(|a, b| b.len().cmp(&a.len()));
        sorted.iter().take(3).map(|g| g.len()).product::<usize>()
    } as i64;

    Ok(answer)
}

fn find_group_index(peers: &Vec<Vec<usize>>, point: usize) -> Option<usize> {
    peers.iter().position(|group| group.contains(&point))
}

fn get_peer_map(lines: &Vec<Vec<i64>>, map: &mut BTreeMap<i64, (usize, usize)>) {
    for (i, line) in lines.iter().enumerate() {
        for j in i + 1..lines.len() {
            let iteration = &lines[j];

            let distance = (line[0] - iteration[0]).pow(2)
                + (line[1] - iteration[1]).pow(2)
                + (line[2] - iteration[2]).pow(2);

            map.insert(distance, (i, j));
        }
    }
}
