use anyhow::{Context, Result};
use std::collections::HashMap;
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

fn print_line(s: &String) -> Result<()> {
    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut handle = io::BufWriter::new(handle);
    writeln!(&mut handle, "{}", s).with_context(|| format!("Cannot write to STDOUT"))
}

impl<'a> Csv<'a> {
    pub fn from(path: &PathBuf, delimiter: &'a str) -> Result<Self> {
        let mut lines = Self::read_lines(path)?;
        let columns = if let Some(Ok(header)) = lines.next() {
            header.split(delimiter).map(String::from).collect()
        } else {
            vec![]
        };
        Ok(Csv {
            data: Data { columns, lines },
            delimiter,
        })
    }

    pub fn list_header(&self) -> Result<()> {
        for c in &self.data.columns {
            if let Err(e) = print_line(c) {
                let io_err = e.downcast_ref::<io::Error>().unwrap();
                // ignore borken pipe error
                if io_err.kind() == io::ErrorKind::BrokenPipe {
                    return Ok(());
                } else {
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    pub fn list_columns(&mut self, selected: &[String], top_n: Option<usize>) -> Result<()> {
        let indexes = self.get_indexes(selected);
        Self::print_by_indexes(
            &self
                .data
                .columns
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>(),
            &indexes,
        )?;
        let mut i = 0;
        let top_n = top_n.unwrap_or(0);
        for line in &mut self.data.lines {
            i += 1;
            if top_n < i && top_n != 0 {
                break;
            }
            let line = line?;
            let line: Vec<&str> = line.split(self.delimiter).collect();
            if let Err(e) = Self::print_by_indexes(&line, &indexes) {
                let io_err = e.downcast_ref::<io::Error>().unwrap();
                // ignore borken pipe error
                if io_err.kind() == io::ErrorKind::BrokenPipe {
                    return Ok(());
                } else {
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    fn read_lines(path: &PathBuf) -> Result<io::Lines<io::BufReader<File>>> {
        let file = File::open(path.as_path())
            .with_context(|| format!("Cannot open file, `{}`", path.to_str().unwrap_or("")))?;
        Ok(io::BufReader::new(file).lines())
    }

    fn get_indexes(&self, selected: &[String]) -> Vec<usize> {
        let mut indexes = vec![];
        if selected.is_empty() {
            indexes = (0..self.data.columns.len()).collect();
        } else {
            let mut map = HashMap::new();
            for (i, c) in self.data.columns.iter().enumerate() {
                map.insert(c, i);
            }
            for sel in selected {
                if let Some(i) = map.get(sel) {
                    indexes.push(*i);
                }
            }
        }

        indexes
    }

    fn print_by_indexes(line: &[&str], indexes: &Vec<usize>) -> Result<()> {
        let content = indexes.iter().map(|i| line[*i]).collect::<Vec<_>>();
        print_line(&content.join("\t"))
    }
}
