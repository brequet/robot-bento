use quick_xml::de::from_str;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, PartialEq)]
struct TestRun {
    #[serde(rename = "@generator")]
    generator: String,
    #[serde(rename = "@generated")]
    generated_date: String,
    #[serde(rename = "suite")]
    suites: Vec<Suite>,
    #[serde(rename = "statistics")]
    statistics: Statistics,
    #[serde(rename = "errors")]
    errors: Errors,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Suite {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@source")]
    source_file: String,
    #[serde(rename = "suite")]
    suites: Option<Vec<Suite>>,
    #[serde(rename = "kw")]
    keywords: Option<Vec<Keyword>>,
    #[serde(rename = "test")]
    tests: Option<Vec<Test>>,
    #[serde(rename = "status")]
    status: Status,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Test {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@line")]
    line: String,
    #[serde(rename = "doc")]
    doc: Option<String>,
    #[serde(rename = "tag")]
    tags: Option<Vec<String>>,
    #[serde(rename = "kw")]
    keywords: Vec<Keyword>,
    #[serde(rename = "status")]
    status: Status,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Keyword {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@library")]
    library: String,
    #[serde(rename = "@type")]
    type_: Option<String>,
    #[serde(rename = "kw")]
    keywords: Option<Vec<Keyword>>,
    #[serde(rename = "if")]
    if_: Option<If>,
    #[serde(rename = "msg")]
    msg: Option<Message>,
    #[serde(rename = "arg")]
    args: Option<Vec<String>>,
    #[serde(rename = "var")]
    var: Option<String>,
    #[serde(rename = "tag")]
    tags: Option<Vec<String>>,
    #[serde(rename = "doc")]
    doc: Option<String>,
    #[serde(rename = "status")]
    status: Option<Status>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct If {
    #[serde(rename = "branch")]
    branch: Vec<Branch>,
    #[serde(rename = "status")]
    status: Status,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Branch {
    #[serde(rename = "@type")]
    type_: String,
    #[serde(rename = "@condition")]
    condition: String,
    #[serde(rename = "kw")]
    keyword: Keyword,
    #[serde(rename = "status")]
    status: Status,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Status {
    #[serde(rename = "@status")]
    status: String,
    #[serde(rename = "@starttime")]
    start_time: String,
    #[serde(rename = "@endtime")]
    end_time: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Message {
    #[serde(rename = "@timestamp")]
    timestamp: String,
    #[serde(rename = "@level")]
    level: String,
    #[serde(rename = "$text")]
    value: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Statistics {
    #[serde(rename = "total")]
    total: StatisticsTotal,
    #[serde(rename = "tag")]
    tags: StatisticsTags,
    #[serde(rename = "suite")]
    suites: StatisticsSuites,
}

#[derive(Debug, Deserialize, PartialEq)]
struct StatisticsTotal {
    #[serde(rename = "stat")]
    stats: StatisticsTag,
}

#[derive(Debug, Deserialize, PartialEq)]
struct StatisticsTags {
    #[serde(rename = "stat")]
    stats: Vec<StatisticsTag>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct StatisticsSuites {
    #[serde(rename = "stat")]
    stats: Vec<StatisticsSuiteTag>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct StatisticsTag {
    #[serde(rename = "@pass")]
    pass: u32,
    #[serde(rename = "@fail")]
    fail: u32,
    #[serde(rename = "@skip")]
    skip: u32,
    #[serde(rename = "$text")]
    text: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct StatisticsSuiteTag {
    #[serde(rename = "@pass")]
    pass: u32,
    #[serde(rename = "@fail")]
    fail: u32,
    #[serde(rename = "@skip")]
    skip: u32,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    text: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Errors {
    #[serde(rename = "msg")]
    messages: Vec<Message>,
}

#[cfg(test)]
mod tests {
    use quick_xml::errors;

    use super::*;

    #[test]
    fn test_parse_xml() {
        let file_path = "src/handlers/resources/output_simplified.xml";
        let xml_content = fs::read_to_string(file_path).expect("Unable to read file");
        let result: Result<TestRun, _> = from_str(&xml_content);

        match result {
            Ok(ref parsed) => println!("{:?}", parsed),
            Err(err) => panic!("Failed to parse XML: {}", err),
        }

        assert!(result.is_ok());

        let test_run = result.unwrap();
        assert_eq!(test_run.generator, "Robot 7.1 (Python 3.10.4 on win32)");
        assert_eq!(test_run.generated_date, "20241217 11:27:23.676");
        assert_eq!(test_run.suites.len(), 1);

        let suite = &test_run.suites[0];
        assert_eq!(suite.id, "s1");
        assert_eq!(suite.name, "Acceptance");
        assert_eq!(suite.source_file, r"D:\robot-run\tests\acceptance");
        assert_eq!(suite.status.status, "FAIL");
        assert_eq!(suite.status.start_time, "20241217 11:27:23.679");
        {
            let suite_s1_s1 = &suite.suites.as_ref().unwrap()[0];
            assert_eq!(suite_s1_s1.id, "s1-s1");

            let suite_s1_s1_s1 = &suite_s1_s1.suites.as_ref().unwrap()[0];
            assert_eq!(suite_s1_s1_s1.id, "s1-s1-s1");

            let test_s1_s1_s1_t1 = &suite_s1_s1_s1.tests.as_ref().unwrap()[0];
            assert_eq!(test_s1_s1_s1_t1.id, "s1-s1-s1-t1");
            assert_eq!(test_s1_s1_s1_t1.name, "History Test");
            assert_eq!(
                test_s1_s1_s1_t1.doc.as_ref().unwrap(),
                "The aim of this test is to do something"
            );
            assert_eq!(test_s1_s1_s1_t1.tags.as_ref().unwrap().len(), 2);
            assert_eq!(test_s1_s1_s1_t1.status.status, "PASS");
        }

        let first_kw = &suite.keywords.as_ref().unwrap()[0];
        assert_eq!(first_kw.name, "Acceptance Setup");
        assert_eq!(first_kw.library, "init-keywords");
        assert_eq!(first_kw.type_.as_ref().unwrap(), "SETUP");
        {
            let if_ = &first_kw.if_.as_ref().unwrap();
            assert_eq!(if_.branch.len(), 1);

            let branch = &if_.branch[0];
            assert_eq!(branch.type_, "IF");
            assert_eq!(branch.condition, "not ${SCREENSHOT NEEDED}");

            let kw = &branch.keyword;
            assert_eq!(kw.name, "Register Keyword To Run On Failure");
            assert_eq!(kw.msg.as_ref().unwrap().timestamp, "20241217 11:27:26.201");
            assert_eq!(
                kw.msg.as_ref().unwrap().value,
                "Keyword will not be run on failure."
            );
            assert_eq!(kw.var.as_ref().unwrap(), "${dummyVar}");
            assert_eq!(kw.args.as_ref().unwrap()[0], "${None}");
            assert_eq!(kw.tags.as_ref().unwrap()[0], "Config");
            assert_eq!(
                kw.doc.as_ref().unwrap(),
                "Sets the keyword to execute, when a Browser keyword fails."
            );

            let if_status = &if_.status;
            assert_eq!(if_status.status, "PASS");
        }
        assert_eq!(first_kw.status.as_ref().unwrap().status, "PASS");

        let total_stats = &test_run.statistics.total.stats;
        assert_eq!(total_stats.pass, 7);
        assert_eq!(total_stats.fail, 1);
        assert_eq!(total_stats.skip, 0);
        assert_eq!(total_stats.text, "All Tests");

        let tag_stats = &test_run.statistics.tags.stats;
        assert_eq!(tag_stats.len(), 15);
        assert_eq!(tag_stats[5].pass, 2);
        assert_eq!(tag_stats[5].fail, 1);
        assert_eq!(tag_stats[5].skip, 0);
        assert_eq!(tag_stats[5].text, "something");

        let suite_stats = &test_run.statistics.suites.stats;
        assert_eq!(suite_stats.len(), 8);
        assert_eq!(suite_stats[0].pass, 7);
        assert_eq!(suite_stats[0].fail, 1);
        assert_eq!(suite_stats[0].skip, 0);
        assert_eq!(suite_stats[0].id, "s1");
        assert_eq!(suite_stats[0].name, "Acceptance");
        assert_eq!(suite_stats[0].text, "Acceptance");

        let errors = &test_run.errors;
        assert_eq!(errors.messages.len(), 2);
        assert_eq!(errors.messages[0].timestamp, "20250115 10:52:56.694");
        assert_eq!(errors.messages[0].level, "WARN");
    }
}
