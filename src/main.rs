use anyhow::Result;
use csv::Csv;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "csv", about = "A tool to read CSV files")]
struct Opt {
    #[structopt(short, long, help = "list header")]
    list: bool,

    #[structopt(short, long, default_value = "\x07", help = "specify the delimiter")]
    delimiter: String,

    #[structopt(short = "n", long = "top", help = "select top N records")]
    top: Option<usize>,

    #[structopt(name = "FILE", parse(from_os_str))]
    file_name: PathBuf,

    #[structopt(name = "COLUMNS", required_unless("list"))]
    columns: Vec<String>,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let mut csv = Csv::from(&opt.file_name, &opt.delimiter)?;
    if opt.list {
        csv.list_header()
    } else {
        csv.list_columns(&opt.columns, opt.top)
    }
}
