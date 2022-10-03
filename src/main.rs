#![allow(dead_code)]
#![allow(unused_variables)]
// auto rerun
// cargo install cargo-watch
// cargo watch -x 'run -- test/data/ham_log/data.txt test/data/ham_log/template.toml'
use std::fs::read_to_string;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::path::PathBuf;
use std::ptr::hash;
use std::thread::panicking;

use csv;
use regex::Regex;
use serde::Deserialize;
use serde_derive::Deserialize;
use std::collections::hash_set::HashSet;
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

    // btree_numbered file is a simple dictionary of line numbers and the string captured on this line (comments and spaces removed)
    //let mut btree_numbered_file: BTreeMap<usize, StructNumberedFile> = BTreeMap::new();
    let mut hashmap_db: HashMap<String, String> = HashMap::new();

    let lines = file
        .lines()
        .map(|x| x.expect("csv line expected"))
        .collect::<Vec<String>>();
    let delim = ",";
    //names = [item.strip() for item in lines[0].split(delim)]
    let names = lines[0]
        .split(delim)
        .map(|x| x.trim().to_owned())
        .collect::<Vec<String>>();
    // rev_names = names[::-1]
    let rev_names = names.clone().into_iter().rev().collect::<Vec<String>>();
    let first_line = lines[1]
        .split(delim)
        .map(|x| x.trim().to_owned())
        .collect::<Vec<String>>();
    // Dictionary comprehension: mother_line = {names[i]: first_line[i] for i in range(len(names))}
    let mother_line: HashMap<String, String> = (0..names.len())
        .map(|i| (names[i].clone(), first_line[i].clone()))
        .collect::<HashMap<_, _>>();
    let mut list_dicts: Vec<HashMap<String, String>> = Vec::new();
    list_dicts.push(mother_line);
    // slicing: remainder_lines = lines[2::]. Do not need header and first line for iterations.
    let remainder_line = lines.as_slice()[2..].to_vec();
    dbg!(list_dicts);

    // let mut i = 0;
    // for line in file.lines() {
    //     //  Clean file: remove comments and empty lines
    //     let line_string = regex_trailing_white_space
    //         .replace_all(line.as_ref().unwrap().as_str(), "")
    //         .into_owned();

    //     if regex_line_comment.is_match(line_string.as_str()) {
    //         continue;
    //     }

    //     if regex_blank_line.is_match(line_string.as_str()) {
    //         continue;
    //     }

    //     btree_numbered_file.insert(i, struct_numbered_file.clone());

    //     // use btree to grab previous entries!
    //     let line_data = StructLineData {
    //         line_number: struct_numbered_file.clone().line_number,
    //         string: struct_numbered_file.clone().string,
    //         num_entries: line_string
    //             .as_str()
    //             .to_owned()
    //             .split(&templ.delim)
    //             .into_iter()
    //             .map(|x| x.to_string())
    //             .collect::<Vec<String>>()
    //             .len(), // replace with delim logic
    //         line_type: struct_numbered_file.which_line_type(templ.clone(), template.clone()),
    //         vec_entries: line_string
    //             .as_str()
    //             .to_owned()
    //             .split(&templ.delim)
    //             .into_iter()
    //             .map(|x| x.to_string())
    //             .collect::<Vec<String>>(),
    //         is_full: false,                 // are observations are complete
    //         contains_nullable_entry: false, // ...
    //         dict_line_data: btree_numbered_file.get(&i).map(|x| x.to_owned()), // insert each key that exists in each line
    //         dict_line_data_previous: {
    //             if i > 0 {
    //                 btree_numbered_file
    //                     .clone()
    //                     .get(&(i - 1))
    //                     .map(|x| x.to_owned())
    //             } else {
    //                 None
    //             }
    //         },
    //     };

    //     btree_line_data.insert(i, line_data.clone());

    //     // make a match statement that takes in the line type and appends to the appropriate vector
    //     match line_data.line_type {
    //         // match on a LineType. Get the last value in the vector of that type. Append to that vector.
    //         //
    //         LineType::Date => {
    //             let last_entry = hashmap_db.get("date").unwrap().to_owned();
    //             let vec_date = vec![line_data.vec_entries[1].clone()];
    //             hashmap_db.insert("date".to_string(), [last_entry, vec_date].concat());
    //             let non_entry: Vec<String> = table_meta
    //                 .ordered_vector_of_col_names
    //                 .clone()
    //                 .into_iter()
    //                 .filter(|x| x.to_owned() != "date".to_string())
    //                 .collect::<Vec<String>>();

    //             for i in 0..non_entry.len() {
    //                 let last_entry = hashmap_db.get(&non_entry[i]).unwrap().to_owned();
    //                 hashmap_db.insert(non_entry[i].clone(), last_entry.clone());
    //             }
    //         }
    //         LineType::Header => {
    //             let last_entry = hashmap_db
    //                 .get(&line_data.vec_entries[0])
    //                 .unwrap()
    //                 .to_owned();
    //             let vec_header_key = line_data.vec_entries[0].clone();
    //             let vec_header_value = vec![line_data.vec_entries[1].clone()];
    //             hashmap_db.insert(
    //                 vec_header_key.clone(),
    //                 [last_entry, vec_header_value].concat(),
    //             );
    //             let non_entry: Vec<String> = table_meta
    //                 .ordered_vector_of_col_names
    //                 .clone()
    //                 .into_iter()
    //                 .filter(|x| x.to_owned() != vec_header_key)
    //                 .collect::<Vec<String>>();

    //             for i in 0..non_entry.len() {
    //                 let last_entry = hashmap_db.get(&non_entry[i]).unwrap().to_owned();
    //                 hashmap_db.insert(non_entry[i].clone(), last_entry.clone());
    //             }
    //         }
    //         LineType::Group => {
    //             // get vec entries
    //             // match the value to the key
    //             // put the value in the key
    //             //
    //             // for vec_enties get key and insert key value pair
    //             let vec_ent = line_data.vec_entries;
    //             let vec_keys = templ.clone().group_keys.unwrap();
    //             dbg!(vec_ent.clone());

    //             for i in 0..vec_keys.len() {
    //                 let vec_string_hash_values: Vec<String> = templ
    //                     .group_hashmap
    //                     .clone()
    //                     .unwrap()
    //                     .get(&vec_keys[i])
    //                     .unwrap()
    //                     .to_owned();

    //                 // let last_entry = hashmap_db
    //                 //     .get(&vec_keys[i])
    //                 //     .unwrap()
    //                 //     .to_owned()
    //                 //     .last()
    //                 //     .unwrap() // unwrap failes if no previous entry
    //                 //     .to_owned();

    //                 // hashsets needed for `is_disjoint()` method
    //                 let vec_string_hash_values_hashset: HashSet<String> =
    //                     vec_string_hash_values.into_iter().collect();
    //                 let vec_ent_hashset: HashSet<String> = vec_ent.clone().into_iter().collect();

    //                 // if the hashsets are disjoint (they do not contain any of the same values) then drag down the most recent value
    //                 // if is_disjoint is false (they do contains same values) then update the value in this group key
    //                 if vec_string_hash_values_hashset.is_disjoint(&vec_ent_hashset) {
    //                     // no match case
    //                     // hashmap_db.insert(vec_keys[i].clone(), vec![last_entry]);
    //                     hashmap_db.insert(vec_keys[i].clone(), vec!["None".to_string()]);
    //                 } else {
    //                     // match case
    //                     hashmap_db.insert(vec_keys[i].clone(), vec![vec_ent[i].clone()]);
    //                 }
    //             }
    //             // let matched_key: Option<String> = templ
    //             //     .group_hashmap
    //             //     .clone()
    //             //     .unwrap()
    //             //     .into_iter()
    //             //     .find_map(|(key, val)| {
    //             //         if val.iter().any(|val| vec_ent.contains(val)) {
    //             //             Some(key)
    //             //         } else {
    //             //             None
    //             //         }
    //             //     });

    //             // let matches = vec_string // cartisian product
    //             // .clone()
    //             // .into_iter()
    //             // .flat_map(|x| std::iter::repeat(x).zip(group_values.clone()))
    //             // .filter(|(a, b)| a == b)
    //             // .collect::<Vec<_>>();

    //             // dbg!(vec_ent);
    //         }
    //         LineType::Observation => {}
    //     }

    //     // If the line type is header then grab the column name and the entry as key and value
    //     // If the line type is group then grab the column name from template and the value from the data
    //     // If the line type is date then use the 'date'
    //     // if the line type is an observation then use the observation set of columns

    //     i += 1
    // }
    // println!("hashmap_db {:?}", hashmap_db);

    // let cols = table_meta.clone().ordered_vector_of_col_names;
    //     let line_data = StructLineData {
    //         line_number: i,
    //         string: line_string.as_str().to_owned(),
    //         num_entries: data_and_config.count_entries(),
    //         line_type: data_and_config.which_line_type(),
    //         vec_entries: data_and_config.make_vec_entries(),
    //         is_maximized: data_and_config.is_maximized(),
    //     };
    //     if debug {
    //     }
    //     numbered_file.insert(i, line_data.clone());

    //     match line_data.line_type {
    //         LineType::Header => {
    //             btree_data_row.insert(
    //                 line_data.clone().vec_entries[0].clone(),
    //                 line_data.clone().vec_entries[1].clone(),
    //             );
    //         }
    //         LineType::Date => {
    //             btree_data_row.insert("date".to_owned(), line_data.clone().vec_entries[1].clone());
    //         }
    //         LineType::Group => {
    //             for i in 0..group_keys.len() {
    //                 btree_data_row.insert(
    //                     group_keys[i].to_owned(),
    //                     line_data.clone().vec_entries[i].clone(),
    //                 );
    //             }
    //         }
    //         LineType::Observation => {
    //             for i in 0..obs_keys.len() {
    //                 btree_data_row.insert(
    //                     obs_keys[i].to_owned(),
    //                     line_data.clone().vec_entries[i].clone(),
    //                 );
    //             }
    //         }
    //         LineType::ObservationPartial => {
    //             let entries = line_data.clone().vec_entries.clone();
    //             for i in 0..entries.len() {
    //                 btree_data_row.insert(
    //                     obs_keys[i].to_owned(),
    //                     line_data.clone().vec_entries[i].clone(),
    //                 );
    //             }
    //         }
    //     };

    //     btree_df.insert(i, btree_data_row.clone());
    //     if debug {
    //     }

    //     i += 1;
    // }

    // // the node entry is the most complete entry and should be the first
    // // of the observations
    // let mut complete_observation_entries: BTreeMap<usize, StructLineData> = numbered_file.clone();
    // complete_observation_entries.retain(|_, v| v.is_maximized == true);
    // let obs_node_key: &usize = complete_observation_entries.iter().next().unwrap().0;

    // // make a BtreeMap that holds the data. The keys are the column names. The values are Some(entries). Matching can be done to see if empty.
    // // group is

    // //filter rows where btree_df is greater than obs_node_key

    // btree_df.retain(|k, _| k >= obs_node_key);

    // let mut btree_df_values: BTreeMap<usize, Vec<&String>> = BTreeMap::new();

    // for i in *obs_node_key..btree_df.len() {
    //     let vec_string = btree_df.get(&i).unwrap().values().collect::<Vec<&String>>();
    //     btree_df_values.insert(i - *obs_node_key, vec_string);
    // }

    // let mut wtr = csv::Writer::from_writer(io::stdout());

    // // I have to be more careful here with collecting in the correct order
    // fn get_all_column_names_ordered(
    //     constant_keys: Vec<String>,
    //     group_keys: Vec<String>,
    //     obs_keys: Vec<String>,
    //     time: bool,
    // ) -> Vec<String> {
    //     //<date><time>  |  low change --> high change   | high change ---------------> low change
    //     //<date><time>  |  <constants>     <groups>     |  <obs_full_replace> <obs_right_replace>

    //     let vec_time_stamp: Vec<String> = match time {
    //         true => vec!["date".to_owned(), "time".to_owned()],
    //         false => vec!["date".to_owned()],
    //     };

    //     let mut constant_keys_mut = constant_keys;
    //     constant_keys_mut.retain(|x| *x != "date".to_owned());

    //     let mut obs_keys_mut = obs_keys;
    //     obs_keys_mut.retain(|x| *x != "time".to_owned());

    //     [vec_time_stamp, constant_keys_mut, group_keys, obs_keys_mut].concat()
    // }

    // let constant_keys_owned: Vec<String> =
    //     constant_keys.into_iter().map(|x| x.to_owned()).collect();
    // let group_keys_owned: Vec<String> = group_keys.into_iter().map(|x| x.to_owned()).collect();
    // let obs_keys: Vec<String> = obs_keys.into_iter().map(|x| x.to_owned()).collect();
    // let all_column_names_ordered =
    //     get_all_column_names_ordered(constant_keys_owned, group_keys_owned, obs_keys, true);

    // let col_names: Vec<&String> = btree_data_row.keys().collect();

    // wtr.write_record(all_column_names_ordered.clone());

    // //mapping btree: map btree index to  all_column_names_ordered index
    // let mut mapping_btree: BTreeMap<String, usize> = BTreeMap::new();
    // for i in 0..col_names.len() {
    //     let index = col_names.iter().position(|&r| r == col_names[i]).unwrap();
    //     let key = all_column_names_ordered[index].clone();
    //     mapping_btree.insert(key, i);
    // }

    // let mut mapping_btree_values: Vec<&usize> = mapping_btree.values().into_iter().collect();
    // let range = seq(mapping_btree_values.len());
    // let mut index_btree: BTreeMap<&usize, usize> = BTreeMap::new();
    // for i in 0..mapping_btree_values.len() {
    //     index_btree.insert(mapping_btree_values[i], range[i]);
    // }

    // for i in 0..btree_df_values.len() {
    //     //wtr.write_record(btree_df_values.get(&i).unwrap());
    //     let rowwise_values_unordered = btree_df_values[&i]
    //         .clone()
    //         .into_iter()
    //         .collect::<Vec<&String>>();

    //     let mut rowwise_values_ordered: Vec<&String> = Vec::new();
    //     for j in 0..rowwise_values_unordered.len() {
    //         let order_idx = index_btree.clone().get(&j).unwrap().to_owned();
    //         rowwise_values_ordered.push(rowwise_values_unordered[order_idx])
    //     }
    //     wtr.write_record(rowwise_values_ordered);
    // }
}
