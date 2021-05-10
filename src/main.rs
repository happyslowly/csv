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

    #[structopt(
        short = "n",
        long = "top",
        default_value = "-1",
        help = "select top N records"
    )]
    top: isize,

    #[structopt(name = "FILE", parse(from_os_str))]
    file_name: PathBuf,

    #[structopt(name = "COLUMNS", required_unless("list"))]
    columns: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();
    let csv = Csv::from(&opt.file_name, &opt.delimiter);

    match csv {
        Ok(mut csv) => {
            if opt.list {
                csv.list_header();
            } else {
                csv.list_columns(&opt.columns, opt.top);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    };
}
