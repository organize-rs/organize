//! deserializers

#[cfg(test)]
mod tests {
    use crate::{config::OrganizeConfig, py_config::PyOrganizeConfig};

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
        exts: 
          - pdf
      - name:
        contains:
          - Invoice
          - Order
          - Purchase
        case_sensitive: false
    actions:
      - move: ~/Documents/Shopping/
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
        let value = serde_yaml::from_str::<serde_yaml::Value>(rules)?;
        let data = PyOrganizeConfig::deserialize_from_value(value.into()).unwrap();

        Ok(())
    }
}
