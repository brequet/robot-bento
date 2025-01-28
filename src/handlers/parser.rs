use quick_xml::{
    de::{from_str, Deserializer},
    DeError,
};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct TestRun {
    #[serde(rename = "@generator")]
    generator: String,
    #[serde(rename = "@generated")]
    generated_date: String,
    #[serde(rename = "suite", default)]
    suites: Vec<Suite>,
    #[serde(rename = "statistics")]
    statistics: Statistics,
    #[serde(rename = "errors")]
    errors: Errors,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Suite {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@source")]
    source_file: String,
    #[serde(rename = "status")]
    status: Status,
    #[serde(rename = "$value", default)]
    suite_children: Vec<SuiteChildren>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
enum SuiteChildren {
    #[serde(rename = "suite")]
    Suite(Suite),
    #[serde(rename = "kw")]
    Keyword(Keyword),
    #[serde(rename = "test")]
    Test(Test),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Test {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@line")]
    line: String,
    #[serde(rename = "doc")]
    doc: Option<String>,
    #[serde(rename = "tag", default)]
    tags: Vec<String>,
    #[serde(rename = "kw", default)]
    keywords: Vec<Keyword>,
    #[serde(rename = "status")]
    status: Status,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Keyword {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@library")]
    library: Option<String>,
    #[serde(rename = "@type")]
    type_: Option<String>,
    #[serde(rename = "msg", default)]
    msg: Vec<Message>,
    #[serde(rename = "arg", default)]
    args: Vec<String>,
    #[serde(rename = "var", default)]
    var: Vec<String>,
    #[serde(rename = "tag", default)]
    tags: Vec<String>,
    #[serde(rename = "doc")]
    doc: Option<String>,
    #[serde(rename = "status")]
    status: Option<Status>,
    #[serde(rename = "return")]
    return_: Option<Return>,
    #[serde(rename = "$value", default)]
    keyword_children: Vec<KeywordChildren>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct If {
    #[serde(rename = "branch", default)]
    branch: Vec<Branch>,
    #[serde(rename = "status")]
    status: Status,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
enum KeywordChildren {
    #[serde(rename = "kw")]
    Keyword(Keyword),
    #[serde(rename = "if")]
    If(If),
    #[serde(rename = "for")]
    For(For),
    #[serde(rename = "test")]
    Test(Test),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Branch {
    #[serde(rename = "@type")]
    type_: String,
    #[serde(rename = "@condition")]
    condition: Option<String>,
    #[serde(rename = "kw", default)]
    keyword: Vec<Keyword>,
    #[serde(rename = "status")]
    status: Status,
    #[serde(rename = "return")]
    return_: Option<Return>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct For {
    #[serde(rename = "@flavor")]
    flavor: String,
    #[serde(rename = "iter", default)]
    iters: Vec<Iter>,
    #[serde(rename = "value", default)]
    values: Vec<String>,
    #[serde(rename = "var", default)]
    vars: Vec<String>,
    #[serde(rename = "status")]
    status: Status,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Iter {
    #[serde(rename = "status")]
    status: Status,
    #[serde(rename = "var", default)]
    vars: Vec<IterVar>,
    #[serde(rename = "$value", default)]
    keyword_children: Vec<KeywordChildren>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct IterVar {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Return {
    #[serde(rename = "status")]
    status: Status,
    #[serde(rename = "value")]
    value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Status {
    #[serde(rename = "@status")]
    status: String,
    #[serde(rename = "@starttime")]
    start_time: String,
    #[serde(rename = "@endtime")]
    end_time: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Message {
    #[serde(rename = "@timestamp")]
    timestamp: String,
    #[serde(rename = "@level")]
    level: String,
    #[serde(rename = "$text")]
    value: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Statistics {
    #[serde(rename = "total")]
    total: StatisticsTotal,
    #[serde(rename = "tag")]
    tags: StatisticsTags,
    #[serde(rename = "suite")]
    suites: StatisticsSuites,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct StatisticsTotal {
    #[serde(rename = "stat")]
    stats: StatisticsTag,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct StatisticsTags {
    #[serde(rename = "stat", default)]
    stats: Vec<StatisticsTag>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct StatisticsSuites {
    #[serde(rename = "stat", default)]
    stats: Vec<StatisticsSuiteTag>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Errors {
    #[serde(rename = "msg", default)]
    messages: Vec<Message>,
}

pub fn from_str_custom<'de, T>(s: &'de str) -> Result<T, DeError>
where
    T: Deserialize<'de>,
{
    let mut de = Deserializer::from_str(s);
    match T::deserialize(&mut de) {
        Ok(value) => Ok(value),
        Err(err) => {
            println!("something happened: {:?}", err);
            de.is_empty();
            Err(err)
        }
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use quick_xml::errors;

    use super::*;

    #[test]
    fn test_parse_xml() {
        // TODO: test and kw in same enum
        // let file_path = "robot-data-sample/8-tests-1-ko/output.xml";
        let file_path = "src/handlers/resources/output_simplified.xml";
        let xml_content = fs::read_to_string(file_path).expect("Unable to read file");
        let result: Result<TestRun, _> = from_str_custom(&xml_content);

        match result {
            Ok(ref _res) => println!("result parsed"),
            Err(err) => panic!("Failed to parse XML: {:?}", err),
        }

        assert!(result.is_ok());

        let test_run = result.unwrap();

        // let json = serde_json::to_string_pretty(&test_run).unwrap();
        // println!("{}", json);

        assert_eq!(test_run.generator, "Robot 7.1 (Python 3.10.4 on win32)");
        assert_eq!(test_run.generated_date, "20241217 11:27:23.676");
        assert_eq!(test_run.suites.len(), 1);

        let suite = &test_run.suites[0];
        assert_eq!(suite.id, "s1");
        assert_eq!(suite.name, "Acceptance");
        assert_eq!(suite.source_file, r"D:\robot-run\tests\acceptance");
        assert_eq!(suite.status.status, "FAIL");
        assert_eq!(suite.status.start_time, "20241217 11:27:23.679");

        if let SuiteChildren::Suite(suite_s1_s1) = &suite.suite_children[1] {
            assert_eq!(suite_s1_s1.id, "s1-s1");

            if let SuiteChildren::Suite(suite_s1_s1_s1) = &suite_s1_s1.suite_children[0] {
                assert_eq!(suite_s1_s1_s1.id, "s1-s1-s1");
                {
                    if let SuiteChildren::Test(test_s1_s1_s1_t1) = &suite_s1_s1_s1.suite_children[1]
                    {
                        assert_eq!(test_s1_s1_s1_t1.id, "s1-s1-s1-t1");
                        assert_eq!(test_s1_s1_s1_t1.name, "History Test");
                        assert_eq!(
                            test_s1_s1_s1_t1.doc.as_ref().unwrap(),
                            "The aim of this test is to do something"
                        );
                        assert_eq!(test_s1_s1_s1_t1.tags.len(), 2);
                        assert_eq!(test_s1_s1_s1_t1.status.status, "PASS");
                    } else {
                        panic!("Expected a Keyword, but got something else");
                    }
                }
            } else {
                panic!("Expected a Suite, but got something else");
            }
        } else {
            panic!("Expected a Suite, but got something else");
        }
        if let SuiteChildren::Keyword(first_kw) = &suite.suite_children[0] {
            assert_eq!(first_kw.name, "Acceptance Setup");
            assert_eq!(first_kw.library.as_ref().unwrap(), "init-keywords");
            assert_eq!(first_kw.type_.as_ref().unwrap(), "SETUP");

            let first_kw_base_kw = &first_kw.keyword_children;
            assert_eq!(first_kw_base_kw.len(), 4);

            if let KeywordChildren::Keyword(kw_with_for) = &first_kw_base_kw[0] {
                if let KeywordChildren::For(for_kw) = &kw_with_for.keyword_children[0] {
                    assert_eq!(for_kw.flavor, "IN");
                    assert_eq!(for_kw.iters.len(), 2);
                    assert_eq!(for_kw.vars.len(), 2);
                    assert_eq!(for_kw.vars[0], "${aVar}");
                    assert_eq!(for_kw.values[0], "@{aList}");
                    assert_eq!(for_kw.values[1], "0");

                    let first_iter = &for_kw.iters[0];
                    assert_eq!(first_iter.vars.len(), 2);
                    assert_eq!(first_iter.vars[0].name, "${aVar}");
                    assert!(first_iter.vars[0].value.is_none());

                    let first_iter_kws = &first_iter.keyword_children;
                    assert_eq!(first_iter_kws.len(), 1);
                } else {
                    panic!("Expected a For, but got something else");
                }
            } else {
                panic!("Expected a Keyword, but got something else");
            }

            if let KeywordChildren::If(if_) = &first_kw_base_kw[1] {
                assert_eq!(if_.branch.len(), 2);

                let first_branch = &if_.branch[0];
                assert_eq!(first_branch.type_, "IF");
                assert_eq!(
                    first_branch.condition.as_ref().unwrap(),
                    "not ${SCREENSHOT NEEDED}"
                );

                let kw = &first_branch.keyword[0];
                assert_eq!(kw.name, "Register Keyword To Run On Failure");
                assert_eq!(kw.msg[0].timestamp, "20241217 11:27:26.201");
                assert_eq!(kw.msg[0].value, "Keyword will not be run on failure.");
                assert_eq!(kw.var[0], "${dummyVar}");
                assert_eq!(kw.args[0], "${None}");
                assert_eq!(kw.tags[0], "Config");
                assert_eq!(
                    kw.doc.as_ref().unwrap(),
                    "Sets the keyword to execute, when a Browser keyword fails."
                );

                let if_status = &if_.status;
                assert_eq!(if_status.status, "PASS");
            } else {
                panic!("Expected an If, but got something else");
            }

            assert_eq!(first_kw.status.as_ref().unwrap().status, "PASS");

            let first_kw_return = &first_kw.return_.as_ref().unwrap();
            assert_eq!(first_kw_return.value.as_ref().unwrap(), "${response}");
            assert_eq!(first_kw_return.status.status, "PASS");
        } else {
            panic!("Expected an Keyword, but got something else");
        }

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
