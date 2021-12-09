use std::fs;

const FILE: &str = "data/day1data.txt";

pub fn part_one() -> i32 {
    let data = get_data();
    let mut count: i32 = 0;
    for (idx, depth) in data.iter().enumerate() {
        if idx == 0 {
            continue;
        } else if data[idx - 1] < *depth {
            count += 1;
        }
    }

    count
}

pub fn part_two() -> i32 {
    let data = get_data();
    let windows = data.windows(3).collect::<Vec<_>>();
    let sums = windows.into_iter().map(|window| window.iter().sum()).collect::<Vec<i32>>();
    let mut count: i32 = 0;
    for (idx, sum) in sums.iter().enumerate() {
        if idx == 0 {
            continue;
        } else if sums[idx - 1] < *sum {
            count += 1;
        }
    }
    count
}

fn read_file() -> String {
    fs::read_to_string(FILE).expect("Couldnt Read File")
}

fn get_data() -> Vec<i32> {
    read_file()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}