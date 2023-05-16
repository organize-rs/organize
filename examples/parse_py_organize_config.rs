use organize_rs_core::rules::py_organize::OrganizeRuleConfiguration;
use serde_yaml::from_reader;
use std::fs::File;
use std::io::Read;

pub fn main() {
    let file =
        File::open(r#"C:\Users\dailyuse\AppData\Local\organize\organize_no_anchors.yaml"#).unwrap();

    let parsed: OrganizeRuleConfiguration = from_reader(file).unwrap();

    println!("{parsed:#?}")
}
