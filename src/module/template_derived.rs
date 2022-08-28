#[derive(Deserialize, Debug, Clone)]
struct StructTemplateDeserializer {
    constants: Option<Table>,    // changes only between different logs
    groups: Option<Table>,       // changes periodically within a log. Has finite set of values.
    obs: Option<Table>,          // observations are the only thing that must exist for a valid log
    nullable: Option<Table>, // nullables never need to have values, they are null unless a <value> is entered
    timestamp: Option<bool>, // if not present then false
    date: Option<bool>,      // if not present then false
    date_format: Option<String>, // if not present, but date is present then '%F'
    arguments: Option<Table>, // currently just holds delimiter
    includes: Option<Table>,
}

#[derive(Deserialize, Debug)]
struct StructTemplate {
    constant_values: Option<Vec<String>>,
    group_keys: Option<Vec<String>>,
    obs_keys: Option<Vec<String>>,
    obs_full_replace_keys: Option<Vec<String>>,
    nullable: Option<Vec<String>>,
    // number_of_cols: u8,
    // ordered_vector_of_col_names: Vec<String>,
    //time_stamp: bool,
    //date: bool,
}

impl StructTemplate {
    fn number_of_cols(self: StructTemplate) -> u8 {
        let constant_count: u8 = self.constant_values.unwrap().len().try_into().unwrap();
        let group_count: u8 = self.group_keys.unwrap().len().try_into().unwrap();
        let obs_count: u8 = self.obs_keys.unwrap().len().try_into().unwrap();
        let nullable_count: u8 = self.nullable.unwrap().len().try_into().unwrap();
        //  let time_count: u8 = if self.time_stamp { 1 } else { 0 };
        //  let date_count: u8 = if self.date { 1 } else { 0 };
        // return constant_count + group_count + obs_count + nullable_count + time_count + date_count;
        return constant_count + group_count + obs_count + nullable_count;
    }
    fn ordered_vector_of_col_names(self: StructTemplate) -> Vec<String> {
        let vec_const = self.constant_values.unwrap();
        let vec_group = self.group_keys.unwrap();
        let obs_full_replace_keys = self.obs_full_replace_keys.unwrap();
        let obs_keys = self.obs_keys.unwrap();
        let nullable = self.nullable.unwrap();
        // let date: Vec<String> = if self.date {
        //     vec!["date".to_owned()]
        // } else {
        //     vec!["".to_owned()]
        // };
        // let time: Vec<String> = if self.date {
        //     vec!["time".to_owned()]
        // } else {
        //     vec!["".to_owned()]
        // };

        let ordered_vec: Vec<String> = Vec::new();
        vec_const
    }
}

// impl StructTemplate {
//     fn which_line_type(&self) -> LineType {
//     //         let string = self.string;}

impl StructTemplateDeserializer {
    fn template_to_group_keys(self: StructTemplateDeserializer) -> Option<Vec<String>> {
        let k: Option<Map<String, Value>> = self.groups;

        match k {
            // match can return things on its own
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
                None => panic!("Must define values with the 'field' key and an array of values."),
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
                        .unwrap() // tell user this must be an array. wrap in []
                        .into_iter()
                        .map(|x| x.as_str().unwrap().to_owned())
                        .collect::<Vec<String>>(),
                ),
                None => panic!("Must define values with the 'field' key and an array of values."),
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
                None => panic!("Must define values with the 'field' key and an array of values."),
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
                None => panic!("Must define values with the 'field' key and an array of values."),
            }
        } else {
            None
        }
    }
}
