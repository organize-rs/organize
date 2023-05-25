use serde::Deserialize;

#[cfg(test)]
mod tests {
    use crate::config::PyOrganizeConfig;

    type TestResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

    #[test]
    pub fn test_single_rule_deserialisation_passes() -> TestResult<()> {
        let rules = r#"
rules:
  - name: "Sort my invoices and receipts"
    locations: 
      - ~/Downloads
    subfolders: true
    filters:
      - extension: 
        - pdf
      - name:
          contains:
            - Invoice
            - Order
            - Purchase
          case_sensitive: false
    actions:
      - move: ~/Documents/Shopping/
"#;
        serde_yaml::from_str::<PyOrganizeConfig>(rules).unwrap();

        Ok(())
    }
}
