use std::{collections::BTreeMap, num::ParseIntError};

pub fn solve_part1(input: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let (_, mut peers, mut map) = parse_and_prepare(input)?;

    merge_until(&mut peers, &mut map, 1000, |_, _, _group_count| None);

    let mut sorted = peers.clone();
    sorted.sort_by(|a, b| b.len().cmp(&a.len()));
    Ok(sorted.iter().take(3).map(|g| g.len()).product::<usize>() as i64)
}

pub fn solve_part2(input: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let (lines, mut peers, mut map) = parse_and_prepare(input)?;

    let mut result = 0;

    merge_until(&mut peers, &mut map, usize::MAX, |a, b, group_count| {
        if group_count == 1 {
            result = lines[a][0] * lines[b][0];
            Some(result)
        } else {
            None
        }
    });

    Ok(result)
}

fn merge_until<F>(
    peers: &mut Vec<Vec<usize>>,
    map: &mut BTreeMap<i64, (usize, usize)>,
    max_steps: usize,
    mut callback: F,
) where
    F: FnMut(usize, usize, usize) -> Option<i64>,
{
    for _ in 0..max_steps {
        if let Some((_, (a, b))) = map.pop_first() {
            let ga = match find_group_index(peers, a) {
                Some(idx) => idx,
                None => continue,
            };
            let gb = match find_group_index(peers, b) {
                Some(idx) => idx,
                None => continue,
            };

            if ga != gb {
                let (low, high) = if ga < gb { (ga, gb) } else { (gb, ga) };

                let (left, right) = peers.split_at_mut(high);
                left[low].append(&mut right[0]);
                peers.remove(high);

                let group_count = peers.len();

                if let Some(_) = callback(a, b, group_count) {
                    return;
                }
            }
        }
    }
}

fn parse_and_prepare(
    input: &str,
) -> Result<
    (
        Vec<Vec<i64>>,
        Vec<Vec<usize>>,
        BTreeMap<i64, (usize, usize)>,
    ),
    Box<dyn std::error::Error>,
> {
    let mut lines = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let numbers: Result<Vec<i64>, ParseIntError> =
            line.split(',').map(|n| n.trim().parse::<i64>()).collect();

        let numbers = numbers.map_err(|e| format!("Failed to parse line {}: {}", i + 1, e))?;
        lines.push(numbers);
    }

    let peers = (0..lines.len()).map(|i| vec![i]).collect();

    let mut map = BTreeMap::new();
    get_peer_map(&lines, &mut map);

    Ok((lines, peers, map))
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
