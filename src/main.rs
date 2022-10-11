// auto rerun with
// cargo install cargo-watch
// cargo watch -x 'run -- -s, test/data/radio_log_small.csv'
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::path::PathBuf;

use csv;
//use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::str;
use structopt::StructOpt;
#[derive(StructOpt)]
#[structopt(
    name = "fwdt",
    about = "📝🔥 Few Word Do Trick (fwdt) is a fast data logger 📝🔥\n
    Example Usage:
    fwdt -s, data.csv
"
)]
struct Cli {
    #[structopt(name = "FILE", parse(from_os_str), help = "Data file to process")]
    file: Option<PathBuf>,
    #[structopt(
        short = "s",
        long = "delimiter",
        parse(try_from_str = parse_delimiter),
        help = "The delimiter separating the columns. Example #1 `fwdt -s ' '  test/data/power_lift.csv`. Example #2 `fwdt -s, test/data/radio_log_small.csv`"
    )]
    delimiter: Option<u8>,
    #[structopt(
        short = "d",
        long = "debug-mode",
        help = "Print object details to make it easier for the maintainer to find and resolve bugs."
    )]
    debug_mode: bool,
}

//
pub fn parse_delimiter(src: &str) -> Result<u8, String> {
    let bytes = src.as_bytes();
    match *bytes {
        [del] => Ok(del),
        [b'\\', b't'] => Ok(b'\t'),
        _ => Err(format!(
            "expected one byte as delimiter, got {} bytes (\"{}\")",
            bytes.len(),
            src
        )),
    }
}

fn main() {
    // when the first observation appears make the mother_row.
    //      - Look at previous hashmap and current line and fill in a dictionary with all needed keys.
    // use template meta data to get the number of columns needed
    // use the meta data to get the ordering of the columns
    let opt = Cli::from_args();
    let debug_mode = opt.debug_mode;
    // Regex helpers
    // let regex_line_comment = Regex::new("^[[:blank:]]*#").unwrap();
    // let regex_trailing_white_space = Regex::new(r#"[ \t]+$"#).unwrap();
    // let regex_blank_line = Regex::new(r#"^\s*$"#).unwrap();
    // let regex_null_line = Regex::new(r#"(<.*?>)"#).unwrap();
    // read data file
    let fp: File = File::open(Path::new(
        &opt.file
            .expect("A file is required. For example: fwdt -s, data.csv")
            .as_path(),
    ))
    .unwrap();
    let binding = [opt
        .delimiter
        .expect("A separator is required. For example: fwdt -s, data.csv")];
    let delim = match str::from_utf8(&binding) {
        Ok(v) => v.clone(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let file: BufReader<&File> = BufReader::new(&fp);
    let lines = file
        .lines()
        .map(|x| x.expect("csv line expected"))
        .collect::<Vec<String>>();
    //names = [item.strip() for item in lines[0].split(delim)]
    let names: Vec<String> = lines[0]
        .split(delim)
        .map(|x| x.trim().to_owned())
        .collect::<Vec<String>>();
    // rev_names = names[::-1]
    let rev_names: Vec<String> = names.clone().into_iter().rev().collect::<Vec<String>>();
    let first_line: Vec<String> = lines[1]
        .split(delim)
        .map(|x| x.trim().to_owned())
        .collect::<Vec<String>>();

    if debug_mode {
        dbg!(first_line.clone());
    }
    // Dictionary comprehension: mother_line = {names[i]: first_line[i] for i in range(len(names))}
    let mother_line: HashMap<String, String> = (0..names.len())
        .map(|i| (names[i].clone(), first_line[i].clone()))
        .collect::<HashMap<_, _>>();
    let mut list_dicts: Vec<HashMap<String, String>> = Vec::new();
    list_dicts.push(mother_line);
    // slicing: remainder_lines = lines[2::]. Do not need header and first line for remainder iterations.
    let remainder_line: Vec<String> = lines.as_slice()[2..].to_vec();
    for i in 0..remainder_line.len() {
        let mut previous_line: HashMap<String, String> = list_dicts[i].clone();
        let current_line: Vec<String> = remainder_line[i]
            .split(delim)
            .map(|x| x.trim().to_owned())
            .collect::<Vec<String>>();
        let rev_current_lines = current_line
            .clone()
            .into_iter()
            .rev()
            .collect::<Vec<String>>();
        for j in 0..rev_current_lines.len() {
            previous_line.insert(rev_names[j].to_owned(), rev_current_lines[j].to_owned());
        }
        list_dicts.push(previous_line)
    }

    let keys = names.clone();
    // get the keys in the proper order
    let vec_vec: Vec<Vec<String>> = list_dicts
        .clone()
        .into_iter()
        .map(|x| {
            keys.iter()
                .filter_map(|key| x.get(key).cloned())
                .collect::<Vec<String>>()
        })
        .collect();

    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(names)
        .expect("Expected a list of names for column header");
    vec_vec
        .clone()
        .iter()
        .for_each(|x| wtr.write_record(x).expect("Expected valid list of entries"));
    // list dicts is in the wrong order
    if debug_mode {
        dbg!(list_dicts);
    }
    if debug_mode {
        dbg!(vec_vec);
    }
}
