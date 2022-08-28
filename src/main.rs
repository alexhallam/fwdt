#![allow(dead_code)]
#![allow(unused_variables)]
// auto rerun
// cargo install cargo-watch
// cargo watch -c -w src -x run
// kevin
// fwdt
use std::fs::read_to_string;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::path::PathBuf;
use std::thread::panicking;

use csv;
use regex::Regex;
use serde::Deserialize;
use serde_derive::Deserialize;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io;
use toml::map::Map;
use toml::value::Table;
use toml::Value;

use structopt::StructOpt;
#[derive(StructOpt)]
#[structopt(
    name = "fwdt",
    about = "📝🔥 Few Word Do Trick (fwdt) is a fast logger for manual data entry that supports templates. 📝🔥\n
    Example Usage:
    wget data
    wget template
    fwdt data template
"
)]
struct Cli {
    #[structopt(name = "FILE", parse(from_os_str), help = "Data file to process")]
    file: Option<PathBuf>,
    #[structopt(
        name = "TEMPLATE",
        parse(from_os_str),
        help = "Template file to process"
    )]
    template: Option<PathBuf>,
}

#[derive(Deserialize, Debug, Clone)]
struct StructTemplateDeserializer {
    constants: Option<Table>, // changes only between different logs
    groups: Option<Table>,    // changes periodically within a log. Has finite set of values.
    obs: Option<Table>,       // observations are the only thing that must exist for a valid log
    nullable: Option<Table>, // nullables never need to have values, they are null unless a <value> is entered
    //timestamp: Option<bool>, // if not present then false
    //date: Option<bool>,      // if not present then false
    //date_format: Option<String>, // if not present, but date is present then '%F'
    arguments: Option<Table>, // currently just holds delimiter
    includes: Option<Table>,
}

#[derive(Deserialize, Debug)]
struct StructTableMeta {
    number_of_cols: u8,
    ordered_vector_of_col_names: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
struct StructTemplate {
    constant_values: Option<Vec<String>>,
    group_keys: Option<Vec<String>>,
    obs_keys: Option<Vec<String>>,
    obs_full_replace_keys: Option<Vec<String>>,
    nullable: Option<Vec<String>>,
    timestamp: bool,
    date: bool,
    date_format: String,
    delim: String,
}

impl StructTemplate {
    fn number_of_cols(self: StructTemplate) -> u8 {
        let constant_count: u8 = self.constant_values.unwrap().len().try_into().unwrap();
        let group_count: u8 = self.group_keys.unwrap().len().try_into().unwrap();
        let obs_count: u8 = self.obs_keys.unwrap().len().try_into().unwrap();
        let nullable_count: u8 = self.nullable.unwrap().len().try_into().unwrap();
        let time_count: u8 = if self.timestamp { 1 } else { 0 };
        let date_count: u8 = if self.date { 1 } else { 0 };
        return constant_count + group_count + obs_count + nullable_count + time_count + date_count;
    }
    fn ordered_vector_of_col_names(self: StructTemplate) -> Vec<String> {
        let mut vec_const: Vec<String> = self.constant_values.unwrap();
        let mut vec_group: Vec<String> = self.group_keys.unwrap();
        let mut obs_full_replace_keys: Vec<String> = self.obs_full_replace_keys.unwrap();
        let mut obs_keys: Vec<String> = self.obs_keys.unwrap();
        let mut nullable: Vec<String> = self.nullable.unwrap();
        let mut date: Vec<String> = if self.date {
            vec!["date".to_owned()]
        } else {
            vec!["".to_owned()]
        };
        let mut time: Vec<String> = if self.timestamp {
            vec!["time".to_owned()]
        } else {
            vec!["".to_owned()]
        };

        // 1. run a deduplicating algorithm here to make sure the the full replace columns are before the standard observation columns
        // 2. Drop columns with null outputs

        let mut ordered_vec: Vec<String> = Vec::new();
        ordered_vec.append(&mut date);
        ordered_vec.append(&mut time);
        ordered_vec.append(&mut vec_const);
        ordered_vec.append(&mut vec_group);
        //ordered_vec.append(&mut obs_full_replace_keys);
        ordered_vec.append(&mut obs_keys);
        ordered_vec.append(&mut nullable);
        return ordered_vec;
    }
}

impl StructTemplateDeserializer {
    fn template_to_group_keys(self: StructTemplateDeserializer) -> Option<Vec<String>> {
        let k: Option<Map<String, Value>> = self.groups;
        match k {
            Some(k) => Some(
                k.keys()
                    .into_iter()
                    .map(|x| x.to_owned())
                    .collect::<Vec<String>>(),
            ),
            None => None,
        }
    }

    fn template_to_constant_values(self: StructTemplateDeserializer) -> Option<Vec<String>> {
        let constants: Option<Map<String, Value>> = self.constants.clone();
        let fields: Option<Option<&Value>> = match &constants {
            Some(x) => Some(x.get("fields")),
            None => None,
        };

        if fields.unwrap().is_some() {
            match fields {
                Some(fields) => Some(
                    fields
                        .unwrap()
                        .to_owned()
                        .as_array()
                        .unwrap() // tell user this must be an array. wrap in []
                        .into_iter()
                        .map(|x| x.as_str().unwrap().to_owned())
                        .collect::<Vec<String>>(),
                ),
                None => {
                    panic!("Must define values with the 'field' key and an array of values.")
                }
            }
        } else {
            None
        }
    } // fn template_to_constant_values
    fn template_to_obs_values(self: StructTemplateDeserializer) -> Option<Vec<String>> {
        let obs: Option<Map<String, Value>> = self.obs.clone();
        let fields: Option<Option<&Value>> = match &obs {
            Some(x) => Some(x.get("fields")),
            None => None,
        };

        if fields.unwrap().is_some() {
            match fields {
                Some(fields) => Some(
                    fields
                        .unwrap()
                        .to_owned()
                        .as_array()
                        .unwrap()
                        .into_iter()
                        .map(|x| x.as_str().unwrap().to_owned())
                        .collect::<Vec<String>>(),
                ),
                None => {
                    panic!("Must define values with the 'field' key and an array of values.")
                }
            }
        } else {
            None
        }
    }

    fn template_to_obs_full_replace_keys(self: StructTemplateDeserializer) -> Option<Vec<String>> {
        let obs: Option<Map<String, Value>> = self.obs.clone();
        let fields: Option<Option<&Value>> = match &obs {
            Some(x) => Some(x.get("full_replace")),
            None => None,
        };

        // full replace is not necessary to define
        if fields.unwrap().is_some() {
            match fields {
                Some(fields) => Some(
                    fields
                        .unwrap()
                        .to_owned()
                        .as_array()
                        .unwrap() // tell user this must be an array. wrap in []
                        .into_iter()
                        .map(|x| x.as_str().unwrap().to_owned())
                        .collect::<Vec<String>>(),
                ),
                None => {
                    panic!("Must define values with the 'field' key and an array of values.")
                }
            }
        } else {
            None
        }
    }
    fn template_to_nullable(self: StructTemplateDeserializer) -> Option<Vec<String>> {
        let nullable: Option<Map<String, Value>> = self.nullable.clone();
        let fields: Option<Option<&Value>> = match &nullable {
            Some(x) => Some(x.get("fields")),
            None => None,
        };

        if fields.unwrap().is_some() {
            match fields {
                Some(fields) => Some(
                    fields
                        .unwrap()
                        .to_owned()
                        .as_array()
                        .unwrap() // tell user this must be an array. wrap in []
                        .into_iter()
                        .map(|x| x.as_str().unwrap().to_owned())
                        .collect::<Vec<String>>(),
                ),
                None => {
                    panic!("Must define values with the 'field' key and an array of values.")
                }
            }
        } else {
            None
        }
    }

    fn template_to_arguments(self: StructTemplateDeserializer) -> Option<Vec<String>> {
        let k: Option<Map<String, Value>> = self.arguments;
        match k {
            Some(k) => Some(
                k.keys()
                    .into_iter()
                    .map(|x| x.to_owned())
                    .collect::<Vec<String>>(),
            ),
            None => None,
        }
    }
    fn template_to_timestamp(self: StructTemplateDeserializer) -> bool {
        let arguments: Option<Map<String, Value>> = self.arguments.clone();
        let fields: Option<Option<&Value>> = match &arguments {
            Some(x) => Some(x.get("timestamp")),
            None => None,
        };

        if let fields = fields.unwrap() {
            fields.unwrap().as_bool().unwrap()
        } else {
            true
        }
    }
    fn template_to_date(self: StructTemplateDeserializer) -> bool {
        let arguments: Option<Map<String, Value>> = self.arguments.clone();
        let fields: Option<Option<&Value>> = match &arguments {
            Some(x) => Some(x.get("date")),
            None => None,
        };

        if let fields = fields.unwrap() {
            fields.unwrap().as_bool().unwrap()
        } else {
            true
        }
    }

    fn template_to_date_format(self: StructTemplateDeserializer) -> String {
        let arguments: Option<Map<String, Value>> = self.arguments.clone();
        let fields: Option<Option<&Value>> = match &arguments {
            Some(x) => Some(x.get("date_format")),
            None => None,
        };

        if let fields = fields.unwrap() {
            fields.unwrap().as_str().unwrap().to_owned()
        } else {
            "&F".to_owned()
        }
    }
    fn template_to_delim(self: StructTemplateDeserializer) -> String {
        let arguments: Option<Map<String, Value>> = self.arguments.clone();
        let fields: Option<Option<&Value>> = match &arguments {
            Some(x) => Some(x.get("delim")),
            None => None,
        };

        if let fields = fields.unwrap() {
            fields.unwrap().as_str().unwrap().to_owned()
        } else {
            " ".to_owned()
        }
    }
}

// Data File Parsing

struct StructLineData {
    line_number: usize,
    string: String,
    num_entries: usize,
    line_type: LineType,
    vec_entries: Vec<String>,
    // is_maximized: bool,
    is_full: bool, // are observations are complete
    contains_nullable_entry: bool,
    dict_line_data: BTreeMap<String, String>, // insert each key that exists in each line
    dict_line_data_previous: Option<BTreeMap<String, String>>, // in
    obs: BTreeMap<String, String>,            // right replace observations
    nullable: BTreeMap<String, String>,
    row_to_insert: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
enum LineType {
    Header,
    Group,
    Date,
    Observation,
    ObservationPartial,
}

fn main() {
    // make a toml struct
    // eventually allow for serde struct for library integration?
    // better data struct kjl

    let debug: bool = true;

    let opt = Cli::from_args();

    // Regex helpers
    let regex_line_comment = Regex::new("^[[:blank:]]*#").unwrap();
    let regex_trailing_white_space = Regex::new(r#"[ \t]+$"#).unwrap();
    let regex_blank_line = Regex::new(r#"^\s*$"#).unwrap();

    // read toml file
    let toml_file = read_to_string(&opt.template.unwrap().as_path()).unwrap();
    //let toml_parse = toml_file.parse::<Value>().unwrap();

    // read data file
    let fp: File = File::open(Path::new(&opt.file.unwrap().as_path())).unwrap();
    let file: BufReader<&File> = BufReader::new(&fp);
    let template: StructTemplateDeserializer = toml::from_str(toml_file.as_str()).unwrap();

    //println!("{:#?}", template.groups);

    let templ = StructTemplate {
        constant_values: template.clone().template_to_constant_values(),
        group_keys: template.clone().template_to_group_keys(),
        obs_keys: template.clone().template_to_obs_values(),
        obs_full_replace_keys: template.clone().template_to_obs_full_replace_keys(),
        nullable: template.clone().template_to_nullable(),
        timestamp: template.clone().template_to_timestamp(),
        date: template.clone().template_to_date(),
        delim: template.clone().template_to_delim(),
        date_format: template.clone().template_to_date_format(),
    };

    println!("{:#?}", templ);

    let table_meta = StructTableMeta {
        number_of_cols: templ.clone().number_of_cols(),
        ordered_vector_of_col_names: templ.clone().ordered_vector_of_col_names(),
    };

    println!("{:#?}", table_meta);

    // // get the following from the toml file
    // //      constants
    // //      groups
    // //      obs
    // //      obs_full_replace
    // //      See: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=e1236a33f78ac162b5887fc311a38722
    // let mut constant_keys: Vec<&str> = toml_parse["constants"]["fields"]
    //     .as_array()
    //     .unwrap()
    //     .into_iter()
    //     .map(|x| x.as_str().unwrap())
    //     .collect();
    // let group_keys: Vec<_> = toml_parse["groups"].as_table().unwrap().keys().collect();
    // let group_table: &Map<String, Value> = toml_parse["groups"].as_table().unwrap();
    // //println!("{:?}", group_table);

    // let mut toml_obs_keys: Vec<&str> = toml_parse["obs"]["fields"]
    //     .as_array()
    //     .unwrap()
    //     .into_iter()
    //     .map(|x| x.as_str().unwrap())
    //     .collect();
    // let mut obs_keys = vec!["time"];
    // obs_keys.append(&mut toml_obs_keys);

    // let mut date_key = vec!["date"];
    // constant_keys.append(&mut date_key);

    // make empty HashMap
    //  i
    //  string
    //  space_count
    // #[derive(Debug)]
    // struct StructDataAndConfig<'a> {
    //     string: &'a str,
    //     constant_keys: &'a Vec<&'a str>,
    //     group_keys: &'a Vec<&'a String>,
    //     group_table: &'a Map<std::string::String, Value>,
    //     obs_keys: &'a Vec<&'a str>,
    //     delim: &'a str,
    // }

    // impl StructDataAndConfig<'_> {
    //     fn which_line_type(&self) -> LineType {
    //         let string = self.string;
    //         let constant_keys = self.constant_keys;
    //         //let obs_keys = self.obs_keys;
    //         let group_keys = self.group_keys;
    //         let group_table = self.group_table;
    //         let obs_keys = self.obs_keys;
    //         if string.contains("date") {
    //             return LineType::Date;
    //         } else if is_constant_line(string, constant_keys) {
    //             return LineType::Header;
    //         } else if is_group_line(string, group_keys, group_table, self.delim) {
    //             return LineType::Group;
    //         } else if is_observation(string, obs_keys, self.delim) {
    //             return LineType::Observation;
    //         } else {
    //             return LineType::ObservationPartial;
    //         }
    //     }

    //     fn count_entries(&self) -> usize {
    //         let string = self.string;
    //         let vec: Vec<&str> = string.split(self.delim).collect();
    //         return vec.len();
    //     }

    //     fn is_maximized(&self) -> bool {
    //         let string = self.string;
    //         let obs_keys: &Vec<&str> = self.obs_keys;
    //         let vec: Vec<&str> = string.split(self.delim).collect();
    //         return obs_keys.len() == vec.len();
    //     }

    //     fn make_vec_entries(&self) -> Vec<String> {
    //         let string = self.string;
    //         let vec: Vec<String> = string
    //             .split(self.delim)
    //             .into_iter()
    //             .map(|x| x.to_owned())
    //             .collect();
    //         return vec;
    //     }
    // } // DataAndConfig Impl

    // fn is_constant_line(string: &str, constant_keys: &Vec<&str>) -> bool {
    //     let mut n = 0;
    //     for i in 0..constant_keys.len() {
    //         if string.contains(constant_keys[i]) {
    //             n += 1;
    //         }
    //     }
    //     return n == 1;
    // }

    // fn is_group_line(
    //     string: &str,
    //     group_keys: &Vec<&String>,
    //     group_table: &Map<String, Value>,
    //     delim: &str,
    // ) -> bool {
    //     let vec_string: Vec<String> = string.split(delim).map(|x| x.to_owned()).collect();
    //     let mut n = 0;
    //     for i in 0..group_keys.len() {
    //         let key = group_keys.clone().get(i).unwrap().to_owned();
    //         let array = group_table[key].clone().try_into::<Vec<String>>().unwrap();
    //         let matches = vec_string // cartisian product
    //             .clone()
    //             .into_iter()
    //             .flat_map(|x| std::iter::repeat(x).zip(array.clone()))
    //             .filter(|(a, b)| a == b)
    //             .collect::<Vec<_>>();
    //         if matches.len() > 0 {
    //             n += 1
    //         }
    //     }
    //     return n > 0;
    // }

    // fn is_observation(string: &str, obs_keys: &Vec<&str>, delim: &str) -> bool {
    //     let vec_string: Vec<String> = string.split(delim).map(|x| x.to_owned()).collect();
    //     return vec_string.len() == obs_keys.len();
    // }

    // #[derive(Debug, Clone)]
    // struct StructLineData {
    //     num: usize,
    //     string: String,
    //     num_entries: usize,
    //     line_type: LineType,
    //     vec_entries: Vec<String>,
    //     is_maximized: bool,
    // }

    // fn seq(stop: usize) -> Vec<usize> {
    //     let mut vec = Vec::new();
    //     for i in 0..stop {
    //         vec.push(i);
    //     }
    //     return vec;
    // }

    let mut numbered_file: BTreeMap<usize, StructLineData> = BTreeMap::new();
    // let mut btree_data_row: BTreeMap<String, String> = BTreeMap::new();
    // let mut btree_df: BTreeMap<usize, BTreeMap<String, String>> = BTreeMap::new();

    // if debug {
    //     println!("{}", "--------------------debug--------------------");
    //     println!("header keys: {:?}", constant_keys);
    //     println!("group keys: {:?}", group_keys);
    //     println!("group_table: {:?}", group_table);
    //     println!("observation keys: {:?}", obs_keys);
    //     println!("hash_map_data len: {:?}", numbered_file.len());
    //     println!("constant_keys len: {:?}", constant_keys.len());
    //     println!("{}", "--------------------debug--------------------");
    // }

    let mut i = 0;
    for line in file.lines() {
        //  Clean file
        // remove comments and empty lines

        let line_string = regex_trailing_white_space
            .replace_all(line.as_ref().unwrap().as_str(), "")
            .into_owned();

        if regex_line_comment.is_match(line_string.as_str()) {
            continue;
        }

        if regex_blank_line.is_match(line_string.as_str()) {
            continue;
        }

        //    let mut numbered_file: BTreeMap<usize, LineData> = BTreeMap::new();
        let data_and_config = StructTemplate {
            string: line_string.as_str(),
            constant_keys: &constant_keys,
            group_keys: &group_keys,
            group_table: group_table,
            obs_keys: &obs_keys,
            delim: "|",
        };

        // instead of data and config use template. missing line_string.as_str()
        // ----make count_entries an implimentation of StructLineData
        // ----make which line type an implimantation of StructLineData

        //let line_type = which_line_type(line.as_ref().unwrap(), &constant_keys, &group_keys);
        //let num_delim = count_entries(line.as_ref().unwrap().to_owned(), " ".to_owned());

        let line_data = StructLineData {
            line_number: i,
            string: line_string.as_str().to_owned(),
            num_entries: data_and_config.count_entries(),
            line_type: data_and_config.which_line_type(),
            vec_entries: data_and_config.make_vec_entries(),
            is_maximized: data_and_config.is_maximized(),
        };
        if debug {
            // println!("{:?}", line.as_ref().unwrap().to_owned());
            println!("Line Data {:#?}", line_data.clone());
        }
        numbered_file.insert(i, line_data.clone());

        match line_data.line_type {
            LineType::Header => {
                btree_data_row.insert(
                    line_data.clone().vec_entries[0].clone(),
                    line_data.clone().vec_entries[1].clone(),
                );
            }
            LineType::Date => {
                btree_data_row.insert("date".to_owned(), line_data.clone().vec_entries[1].clone());
            }
            LineType::Group => {
                for i in 0..group_keys.len() {
                    btree_data_row.insert(
                        group_keys[i].to_owned(),
                        line_data.clone().vec_entries[i].clone(),
                    );
                }
            }
            LineType::Observation => {
                for i in 0..obs_keys.len() {
                    btree_data_row.insert(
                        obs_keys[i].to_owned(),
                        line_data.clone().vec_entries[i].clone(),
                    );
                }
            }
            LineType::ObservationPartial => {
                let entries = line_data.clone().vec_entries.clone();
                for i in 0..entries.len() {
                    btree_data_row.insert(
                        obs_keys[i].to_owned(),
                        line_data.clone().vec_entries[i].clone(),
                    );
                }
            }
        };

        btree_df.insert(i, btree_data_row.clone());
        if debug {
            println!("btree_data_row: {:?}", btree_data_row.clone());
            //println!("btree_df: {:?}", btree_df.clone());
        }

        i += 1;
    }

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

    // //println!("all_column_names_ordered: {:?}", all_column_names_ordered);
    // let col_names: Vec<&String> = btree_data_row.keys().collect();

    // wtr.write_record(all_column_names_ordered.clone());

    // //mapping btree: map btree index to  all_column_names_ordered index
    // let mut mapping_btree: BTreeMap<String, usize> = BTreeMap::new();
    // for i in 0..col_names.len() {
    //     let index = col_names.iter().position(|&r| r == col_names[i]).unwrap();
    //     let key = all_column_names_ordered[index].clone();
    //     mapping_btree.insert(key, i);
    // }

    // // println!("mapping_btree {:?}", mapping_btree);
    // // println!("mapping_btree values {:?}", mapping_btree.values());
    // // println!("btree_df_values: {:?}", btree_df_values);

    // let mut mapping_btree_values: Vec<&usize> = mapping_btree.values().into_iter().collect();
    // let range = seq(mapping_btree_values.len());
    // let mut index_btree: BTreeMap<&usize, usize> = BTreeMap::new();
    // for i in 0..mapping_btree_values.len() {
    //     index_btree.insert(mapping_btree_values[i], range[i]);
    // }

    // //println!("index_btree: {:?}", index_btree);

    // for i in 0..btree_df_values.len() {
    //     //wtr.write_record(btree_df_values.get(&i).unwrap());
    //     let rowwise_values_unordered = btree_df_values[&i]
    //         .clone()
    //         .into_iter()
    //         .collect::<Vec<&String>>();
    //     //println!("rowwise_values_unordered {:?}", rowwise_values_unordered);

    //     let mut rowwise_values_ordered: Vec<&String> = Vec::new();
    //     for j in 0..rowwise_values_unordered.len() {
    //         let order_idx = index_btree.clone().get(&j).unwrap().to_owned();
    //         rowwise_values_ordered.push(rowwise_values_unordered[order_idx])
    //     }
    //     //println!("rowwise_values_ordered {:?}", rowwise_values_ordered);
    //     wtr.write_record(rowwise_values_ordered);
    // }

    // // if debug {
    // //     println!("{}", "--------------------debug--------------------");
    // //     println!("btree_df_values: {:#?}", btree_df_values.clone());
    // //     println!("btree_data_row: {:#?}", btree_data_row.clone().keys());
    // //     println!("{}", "--------------------debug--------------------");
    // // }
}
