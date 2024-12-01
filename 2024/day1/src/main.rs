use std::collections::HashMap;

type Column = Vec<i32>;
type Input = Vec<Column>;

fn read_input() -> Input {
    let input = std::fs::read_to_string("day1/src/input.txt").unwrap();
    let rows: Input = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    // Convert each row as vector into each column as vector
    let num_cols = rows[0].len();
    let mut columns = vec![Vec::new(); num_cols];

    for row in rows {
        for (col_idx, value) in row.iter().enumerate() {
            columns[col_idx].push(*value);
        }
    }

    columns
}

fn find_distance(columns: &Input) -> i32 {
    let mut columns = columns.clone();
    columns[0].sort();
    columns[1].sort();

    columns[0]
        .iter()
        .zip(columns[1].iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn find_similarity(columns: &Input) -> i32 {
    let mut occurrence_count = HashMap::new();

    for i in 0..columns[1].len() {
        let count = occurrence_count.entry(columns[1][i]).or_insert(0);
        *count += 1;
    }

    columns[0]
        .iter()
        .map(|num| num * occurrence_count.get(num).unwrap_or(&0))
        .sum()
}

fn main() {
    let columns = read_input();

    let distance = find_distance(&columns);
    println!("Total distance: {}", distance);

    let similarity = find_similarity(&columns);
    println!("Total similarity: {}", similarity);
}
