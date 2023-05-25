use organize_rs_core::rules::Rule;

pub fn main() {
    let rule = r#"
- name: "Sort my invoices and receipts"
  locations: ~/Downloads
  subfolders: true
  filters:
    - extension: pdf
    - name:
        contains:
          - Invoice
          - Order
          - Purchase
        case_sensitive: false
  actions:
    - move: ~/Documents/Shopping/
"#;

    println!("{:?}", serde_yaml::from_str::<Rule>(rule).unwrap());
}
