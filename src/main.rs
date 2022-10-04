#![allow(dead_code)]
#![allow(unused_variables)]
// auto rerun
// cargo install cargo-watch
// cargo watch -x 'run -- test/data/ham_log/data.txt test/data/ham_log/template.toml'
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::path::PathBuf;

use csv;
use regex::Regex;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io;

use structopt::StructOpt;
#[derive(StructOpt)]
#[structopt(
    name = "fwdt",
    about = "📝🔥 Few Word Do Trick (fwdt) is a fast logger for manual data entry that supports templates. 📝🔥\n
    Example Usage:
    fwdt data.csv
"
)]
struct Cli {
    #[structopt(name = "FILE", parse(from_os_str), help = "Data file to process")]
    file: Option<PathBuf>,
}

//

fn main() {
    // when the first observation appears make the mother_row.
    //      - Look at previous btree and current line and fill in a dictionary with all needed keys.
    // use template meta data to get the number of columns needed
    // use the meta data to get the ordering of the columns
    let debug: bool = true;
    let opt = Cli::from_args();
    // Regex helpers
    let regex_line_comment = Regex::new("^[[:blank:]]*#").unwrap();
    let regex_trailing_white_space = Regex::new(r#"[ \t]+$"#).unwrap();
    let regex_blank_line = Regex::new(r#"^\s*$"#).unwrap();
    let regex_null_line = Regex::new(r#"(<.*?>)"#).unwrap();
    // read data file
    let fp: File = File::open(Path::new(&opt.file.unwrap().as_path())).unwrap();
    let file: BufReader<&File> = BufReader::new(&fp);
    let mut hashmap_db: HashMap<String, String> = HashMap::new();
    let lines = file
        .lines()
        .map(|x| x.expect("csv line expected"))
        .collect::<Vec<String>>();
    let delim = ",";
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

    //dbg!(list_dicts);

    let vec_vec: Vec<Vec<String>> = list_dicts
        .clone()
        .into_iter()
        .map(|x| x.values().cloned().collect::<Vec<String>>())
        .collect();

    let mut wtr = csv::Writer::from_writer(io::stdout());
    vec_vec.clone().iter().map(|x| wtr.write_record(x));

    wtr.write_record(names);
    for i in 0..vec_vec.len() {
        wtr.write_record(vec_vec[i].clone());
    }

    //dbg!(vec_vec);
}
