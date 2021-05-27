use std::error::Error;
use std::io;
use std::io::{BufRead, Write};
use std::{fs::File, path::PathBuf};

pub struct Csv<'a> {
    data: Data,
    delimiter: &'a str,
}

struct Data {
    columns: Vec<String>,
    lines: io::Lines<io::BufReader<File>>,
}

macro_rules! print_line {
    ($($arg:tt)*) => {
        let mut stdout = io::stdout();
        if let Err(e) = writeln!(&mut stdout, $($arg)*) {
            if e.kind() != io::ErrorKind::BrokenPipe {
                eprintln!("{}", e);
            }
            return;
        }
    };
}

impl<'a> Csv<'a> {
    pub fn from(path: &PathBuf, delimiter: &'a str) -> Result<Self, Box<dyn Error>> {
        match Self::read_lines(path) {
            Ok(mut lines) => {
                let columns;
                if let Some(Ok(header)) = lines.next() {
                    columns = header.split(delimiter).map(String::from).collect();
                } else {
                    columns = vec![];
                }
                Ok(Csv {
                    data: Data { columns, lines },
                    delimiter,
                })
            }
            Err(e) => Err(e.into()),
        }
    }

    pub fn list_header(&self) {
        for c in &self.data.columns {
            print_line!("{}", c);
        }
    }

    pub fn list_columns(&mut self, selected: &[String], top_n: isize) {
        if self.data.columns.len() > 0 {
            let indexes = self.get_indexes(selected);
            Self::print_by_indexes(
                &self
                    .data
                    .columns
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<&str>>(),
                &indexes,
            );
            let mut i = 0;
            for line in &mut self.data.lines {
                i += 1;
                if top_n < i && top_n != -1 {
                    break;
                }
                if let Ok(line) = line {
                    let line: Vec<&str> = line.split(self.delimiter).collect();
                    if let Some(ek) = Self::print_by_indexes(&line, &indexes) {
                        if ek == io::ErrorKind::BrokenPipe {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn read_lines(path: &PathBuf) -> io::Result<io::Lines<io::BufReader<File>>> {
        let file = File::open(path.as_path())?;
        Ok(io::BufReader::new(file).lines())
    }

    fn get_indexes(&self, selected: &[String]) -> Vec<usize> {
        let mut indexes = vec![];
        if selected.is_empty() {
            indexes = (0..self.data.columns.len()).collect();
        } else {
            for sel in selected {
                for (i, col) in self.data.columns.iter().enumerate() {
                    if sel == col {
                        indexes.push(i);
                    }
                }
            }
        }

        indexes
    }

    fn print_by_indexes(line: &[&str], indexes: &Vec<usize>) -> Option<io::ErrorKind> {
        let content = indexes.iter().map(|i| line[*i]).collect::<Vec<_>>();
        let mut stdout = io::stdout();
        if let Err(e) = writeln!(&mut stdout, "{}", content.join("\t")) {
            Some(e.kind())
        } else {
            None
        }
    }
}
