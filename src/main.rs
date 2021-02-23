use ansi_term::Color;
use itertools::Itertools;
use std::collections::BTreeMap;

fn collect_memory_usage() -> BTreeMap<String, i64> {
    let mut process_memory = BTreeMap::new();

    for p in procfs::process::all_processes().unwrap() {
        if p.stat.rss != 0 {
            let cmd_name = p.stat.comm.clone();
            let rss = p.stat.rss_bytes();
            if let Some(rss_acc) = process_memory.get_mut(&cmd_name) {
                *rss_acc += rss;
            } else {
                process_memory.insert(cmd_name, rss);
            }
        }
    }

    for m in process_memory.values_mut() {
        *m = bytes_to_mb(m);
    }

    process_memory
}

fn bytes_to_mb(memory: &i64) -> i64 {
    memory / (1024 * 1024)
}

fn format_mb(m: i64, mean: i64, stddev: i64) -> String {
    if m > mean + stddev {
        Color::Red.paint(format!("{} MB", m)).to_string()
    } else if m > mean {
        Color::Yellow.paint(format!("{} MB", m)).to_string()
    } else {
        format!("{} MB", m)
    }
}

fn mean(values: &Vec<&i64>) -> i64 {
    let sum: i64 = values.into_iter().map(|i| **i).sum();
    let count = values.len() as i64;

    sum / count as i64
}

fn std_deviation(values: &Vec<&i64>) -> i64 {
    let mean = mean(values);
    let count = values.len();
    let variance = values.iter().map(|value| {
        let diff = mean - (**value as i64);

        diff * diff
    }).sum::<i64>() / count as i64;


    (variance as f64).sqrt() as i64
}

fn main() {
    let process_memory = collect_memory_usage();
    let memory_sizes = process_memory.values().collect();
    let mean = mean(&memory_sizes);
    let stddev = std_deviation(&memory_sizes);

    let sorted_processes: Vec<_> = process_memory.iter().sorted_by(|a, b| Ord::cmp(&a.1, &b.1)).collect();
    for (p, m) in sorted_processes {
        println!("{}: {}", p, format_mb(*m, mean, stddev));
    }

    println!("");
    println!("Mean: {} MB std-dev: {}", mean, stddev);
    println!("Total Memory Used: {} MB", memory_sizes.into_iter().map(|i| *i).sum::<i64>());
}
