use std::fs::File;
use std::io;
use std::io::BufRead;

const DEFAULT_DELIM: &str = "\x07";

struct Csv {
    header: String,
    lines: io::Lines<io::BufReader<File>>,
    delim: String,
}

fn read_lines(path: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

impl Csv {
    fn load(path: &str, delim: Option<String>) -> Option<Csv> {
        if let Ok(mut lines) = read_lines(path) {
            if let Some(Ok(header)) = lines.next() {
                return Some(Csv {
                    header,
                    lines,
                    delim: delim.unwrap_or(DEFAULT_DELIM.to_string()),
                });
            }
        }
        None
    }
}

pub fn list_headers(path: &str, delim: Option<String>) {
    if let Some(csv) = Csv::load(path, delim) {
        let names: Vec<&str> = csv.header.split(&csv.delim).collect();
        for n in names {
            println!("{}", n);
        }
    }
}

pub fn list_columns(path: &str, selected: &[String], delim: Option<String>, top_n: Option<String>) {
    if let Some(csv) = Csv::load(path, delim) {
        let names: Vec<&str> = csv.header.split(&csv.delim).collect();
        let indexes = get_indexes(&names, selected);
        print_by_indexes(&names, &indexes);
        let top_n = top_n
            .unwrap_or("-1".to_string())
            .parse::<i32>()
            .unwrap_or(-1);
        let mut i = 0;
        for data in csv.lines {
            i += 1;
            if top_n < i && top_n != -1 {
                break;
            }
            if let Ok(data) = data {
                let data: Vec<&str> = data.split(&csv.delim).collect();
                print_by_indexes(&data, &indexes);
            }
        }
    }
}

fn get_indexes(columns: &Vec<&str>, selected: &[String]) -> Vec<usize> {
    let mut indexes = vec![];
    if selected.is_empty() {
        indexes = (0..columns.len()).collect();
    } else {
        for sel in selected {
            for (i, col) in columns.iter().enumerate() {
                if sel == col {
                    indexes.push(i);
                }
            }
        }
    }

    indexes
}

fn print_by_indexes(data: &Vec<&str>, indexes: &Vec<usize>) {
    for i in indexes {
        print!("{}\t", data[*i])
    }
    println!();
}
