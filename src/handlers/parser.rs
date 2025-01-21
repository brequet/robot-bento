use quick_xml::de::from_str;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, PartialEq)]
struct TestRun {
    #[serde(rename = "@generated")]
    generated_date: String,
    #[serde(rename = "suite")]
    suites: Vec<Suite>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Suite {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@source")]
    source_file: String,
}

fn parse_xml(file_path: &str) -> Result<TestRun, quick_xml::DeError> {
    let xml_content = fs::read_to_string(file_path).expect("Unable to read file");
    let test_run: TestRun = from_str(&xml_content)?;
    Ok(test_run)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_xml() {
        let file_path = "uploads/output.xml";

        let result = parse_xml(file_path);
        assert!(result.is_ok());

        let test_run = result.unwrap();
        assert_eq!(test_run.generated_date, "20250115 10:08:53.099");
        assert_eq!(test_run.suites.len(), 1);

        let suite = &test_run.suites[0];
        assert_eq!(suite.id, "s1");
        assert_eq!(suite.name, "Acceptance");
        assert_eq!(
            suite.source_file,
            r"D:\w\workspace\Horus-Robot-Full_v5.1\tests\acceptance"
        );
    }
}
