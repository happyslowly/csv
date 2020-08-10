use std::fs::File;
use std::io;
use std::io::BufRead;

struct Csv {
    header: String,
    lines: io::Lines<io::BufReader<File>>,
}

fn read_lines(path: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

impl Csv {
    fn load(path: &str) -> Option<Csv> {
        if let Ok(mut lines) = read_lines(path) {
            if let Some(Ok(header)) = lines.next() {
                return Some(Csv { header, lines });
            }
        }
        None
    }
}

pub fn list_headers(path: &str) {
    if let Some(csv) = Csv::load(path) {
        let names: Vec<&str> = csv.header.split(",").collect();
        for n in names {
            println!("{}", n);
        }
    }
}

pub fn list_columns(path: &str, selected: &[String]) {
    if let Some(csv) = Csv::load(path) {
        let names: Vec<&str> = csv.header.split(",").collect();
        let indexes = get_indexes(&names, selected);
        print_by_indexes(&names, &indexes);
        for data in csv.lines {
            if let Ok(data) = data {
                let data: Vec<&str> = data.split(",").collect();
                print_by_indexes(&data, &indexes);
            }
        }
    }
}

fn get_indexes(columns: &Vec<&str>, selected: &[String]) -> Vec<usize> {
    let mut indexes = vec![];
    for sel in selected {
        for (i, col) in columns.iter().enumerate() {
            if sel == col {
                indexes.push(i);
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
