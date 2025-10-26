use serde::{Deserialize, Serialize};
use sha1::Digest;
use std::{fs, path::Path};
use thiserror::Error;
use tracing::info;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TestRun {
    #[serde(rename = "@generator")]
    pub generator: String,
    #[serde(rename = "@generated")]
    pub generated_date: String,
    #[serde(rename = "@rpa")]
    pub rpa: bool,
    #[serde(rename = "@schemaversion")]
    pub schema_version: String,
    #[serde(rename = "suite", default)]
    pub suites: Vec<Suite>,
    #[serde(rename = "statistics")]
    pub statistics: Statistics,
    #[serde(rename = "errors")]
    pub errors: Errors,
    #[serde(skip)]
    pub sha1: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Suite {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@source")]
    pub source_file: Option<String>,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "doc")]
    pub doc: Option<String>,
    #[serde(rename = "$value", default)]
    pub children: Vec<SuiteChildren>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum SuiteChildren {
    /*
    In suite:
    - setup kw
    - suites
    - tests
    - teardown kw
     */
    #[serde(rename = "suite")]
    Suite(Suite),
    #[serde(rename = "test")]
    Test(Test),
    #[serde(rename = "kw")]
    Keyword(Keyword),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Test {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@line")]
    pub line: String,
    #[serde(rename = "$value", default)]
    pub keywords: Vec<BaseBody>,
    #[serde(rename = "doc")]
    pub doc: Option<String>,
    #[serde(rename = "tag", default)]
    pub tags: Vec<String>,
    #[serde(rename = "timeout")]
    pub timeout: Option<String>,
    #[serde(rename = "status")]
    pub status: Status,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
pub enum BaseBody {
    // 'Keyword', 'For', 'While', 'Group', 'If', 'Try', 'Var', 'Return', 'Continue', 'Break', 'Message', 'Error'
    #[serde(rename = "kw")]
    Keyword(Keyword),
    #[serde(rename = "for")]
    For(For),
    #[serde(rename = "while")]
    While(While),
    #[serde(rename = "group")]
    Group(Group),
    #[serde(rename = "if")]
    If(If),
    #[serde(rename = "try")]
    Try(Try),
    #[serde(rename = "variable")]
    Var(Var),
    #[serde(rename = "return")]
    Return(Return),
    #[serde(rename = "continue")]
    Continue(Continue),
    #[serde(rename = "break")]
    Break(Break),
    #[serde(rename = "message")]
    Message(Message),
    // missing "Error" ?
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
pub struct Keyword {
    #[serde(alias = "@name")]
    pub name: String,
    #[serde(alias = "@owner")] // v5
    pub owner: Option<String>,
    #[serde(alias = "@library")] // v4
    pub library: Option<String>,
    #[serde(alias = "@type")]
    pub type_: Option<String>,
    #[serde(rename = "msg", default)]
    pub msg: Vec<Message>,
    #[serde(alias = "$value", default)]
    pub keywords: Vec<BaseBody>,
    #[serde(rename = "var", default)]
    pub var: Vec<String>,
    #[serde(alias = "arg", default)]
    pub args: Vec<String>,
    #[serde(alias = "tag", default)]
    pub tags: Vec<String>,
    #[serde(rename = "doc")]
    pub doc: Option<String>,
    #[serde(rename = "timeout")]
    pub timeout: Option<String>,
    #[serde(rename = "status")]
    pub status: Option<Status>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct For {
    #[serde(alias = "@flavor")]
    pub flavor: String, // 'IN', 'IN RANGE', 'IN ENUMERATE', 'IN ZIP'
    #[serde(alias = "@start")]
    pub start: Option<String>,
    #[serde(alias = "@mode")]
    pub mode: Option<String>,
    #[serde(alias = "@fill")]
    pub fill: Option<String>,
    #[serde(alias = "iter", default)]
    pub iters: Vec<ForIter>,
    #[serde(alias = "var", default)]
    pub vars: Vec<String>,
    #[serde(alias = "value", default)]
    pub values: Vec<String>,
    #[serde(rename = "status")]
    pub status: Status,
}
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct ForIter {
    #[serde(alias = "$value", default)]
    pub children: Vec<BaseBody>,
    #[serde(alias = "var", default)]
    pub vars: Vec<ForIterVar>,
    #[serde(rename = "status")]
    pub status: Status,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct ForIterVar {
    #[serde(alias = "@name")]
    pub name: String,
    #[serde(alias = "$text")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct While {
    #[serde(alias = "@condition")]
    pub condition: Vec<String>,
    #[serde(alias = "@limit")]
    pub limit: Vec<String>,
    #[serde(alias = "@on_limit")]
    pub on_limit: Vec<String>,
    #[serde(alias = "@on_limit_message")]
    pub on_limit_message: Vec<String>,
    #[serde(alias = "iter", default)]
    pub iters: Vec<WhileIter>,
    #[serde(rename = "status")]
    pub status: Status,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct WhileIter {
    #[serde(alias = "$value", default)]
    pub children: Vec<BaseBody>,
    #[serde(rename = "status")]
    pub status: Status,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Group {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(alias = "$value", default)]
    pub children: Vec<BaseBody>,
    #[serde(rename = "status")]
    pub status: Status,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct If {
    #[serde(alias = "branch", default)]
    pub branches: Vec<IfBranch>,
    #[serde(rename = "status")]
    pub status: Status,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct IfBranch {
    #[serde(alias = "@type")]
    pub type_: String,
    #[serde(alias = "@condition")]
    pub condition: Option<String>,
    #[serde(alias = "$value", default)]
    pub children: Vec<BaseBody>,
    #[serde(rename = "status")]
    pub status: Status,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Try {
    #[serde(alias = "branch", default)]
    pub branches: Vec<TryBranch>,
    #[serde(rename = "status")]
    pub status: Status,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct TryBranch {
    #[serde(alias = "@type")]
    pub type_: String,
    #[serde(alias = "@pattern_type")]
    pub pattern_type: Option<String>,
    #[serde(alias = "@assign")]
    pub assign: Option<String>,
    #[serde(alias = "$value", default)]
    pub children: Vec<BaseBody>,
    #[serde(rename = "status")]
    pub status: Status,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Var {
    #[serde(alias = "@name")]
    pub name: String,
    #[serde(alias = "@scope")]
    pub scope: Option<String>,
    #[serde(alias = "@separator")]
    pub separator: Option<String>,
    #[serde(rename = "msg")]
    pub message: Message,
    #[serde(alias = "var", default)]
    pub vars: Vec<String>,
    #[serde(rename = "status")]
    pub status: Status,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Return {
    #[serde(alias = "value", default)]
    pub value: Vec<String>,
    #[serde(rename = "status")]
    pub status: Status,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Continue {
    #[serde(rename = "status")]
    pub status: Status,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Break {
    #[serde(rename = "status")]
    pub status: Status,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Message {
    #[serde(alias = "@timestamp")]
    pub timestamp: String,
    #[serde(alias = "@level")]
    pub level: String,
    #[serde(alias = "$text")]
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Status {
    #[serde(alias = "@status")]
    pub status: String,
    #[serde(alias = "@starttime")]
    pub start_time: String,
    #[serde(alias = "@endtime")]
    pub end_time: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Statistics {
    #[serde(rename = "total")]
    pub total: StatisticsTotal,
    #[serde(rename = "tag")]
    pub tags: StatisticsTags,
    #[serde(rename = "suite")]
    pub suites: StatisticsSuites,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct StatisticsTotal {
    #[serde(rename = "stat")]
    pub stats: StatisticsTag,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct StatisticsTags {
    #[serde(rename = "stat", default)]
    pub stats: Vec<StatisticsTag>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct StatisticsSuites {
    #[serde(rename = "stat", default)]
    pub stats: Vec<StatisticsTag>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct StatisticsTag {
    #[serde(rename = "@pass")]
    pub pass: u32,
    #[serde(rename = "@fail")]
    pub fail: u32,
    #[serde(rename = "@skip")]
    pub skip: u32,
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "$text")]
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Errors {
    #[serde(rename = "msg", default)]
    pub messages: Vec<Message>,
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("XML deserialization error: {0}")]
    Deserialization(#[from] quick_xml::DeError),
    #[error("Invalid file extension")]
    InvalidFileExtension(String),
}

pub struct RobotOutputParserService;

impl RobotOutputParserService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn from_file<P: AsRef<Path>>(
        &self,
        file_name: String,
        path: P,
    ) -> Result<TestRun, ParserError> {
        info!("Parsing file: {:?}", path.as_ref());

        let extension = file_name.split('.').last().unwrap_or("no extension");
        if extension != "xml" {
            return Err(ParserError::InvalidFileExtension(extension.to_string()));
        }

        let content = fs::read_to_string(path)?;

        let mut test_run = self.from_content(&content)?;

        let mut hasher = sha1::Sha1::new();
        hasher.update(&content);
        let hash = format!("{:x}", hasher.finalize());
        test_run.sha1 = hash;

        Ok(test_run)
    }

    pub fn from_content(&self, content: &str) -> Result<TestRun, ParserError> {
        quick_xml::de::from_str(content).map_err(ParserError::from)
    }
}

// #[cfg(test)]
// mod tests {
//     use core::panic;

//     use super::*;

//     #[test]
//     fn test_parse_xml() {
// let file_path = "robot-data-sample/8-tests-1-ko/output.xml";
// let file_path = "./src/services/resources/output_simplified.xml";
// let result: Result<TestRun, _> = get_test_run_from_xml(&file_path);

// match result {
//     Ok(ref _res) => println!("result parsed"),
//     Err(err) => panic!("Failed to parse XML: {:?}", err),
// }

// assert!(result.is_ok());

// TODO: correct tests
// let test_run = result.unwrap();

// // let json = serde_json::to_string_pretty(&test_run).unwrap();
// // println!("{}", json);

// assert_eq!(test_run.generator, "Robot 7.1 (Python 3.10.4 on win32)");
// assert_eq!(test_run.generated_date, "20241217 11:27:23.676");
// assert_eq!(test_run.rpa, false);
// assert_eq!(test_run.schema_version, "4");
// assert_eq!(test_run.suites.len(), 1);

// let suite = &test_run.suites[0];
// assert_eq!(suite.id, "s1");
// assert_eq!(suite.name, "Acceptance");
// assert_eq!(
//     suite.source_file.as_ref().unwrap(),
//     r"D:\robot-run\tests\acceptance"
// );
// assert_eq!(suite.status.status, "FAIL");
// assert_eq!(suite.status.start_time, "20241217 11:27:23.679");

// if let SuiteChildren::Suite(suite_s1_s1) = &suite.children[1] {
//     assert_eq!(suite_s1_s1.id, "s1-s1");

//     if let SuiteChildren::Suite(suite_s1_s1_s1) = &suite_s1_s1.children[0] {
//         assert_eq!(suite_s1_s1_s1.id, "s1-s1-s1");
//         {
//             if let SuiteChildren::Test(test_s1_s1_s1_t1) = &suite_s1_s1_s1.children[1] {
//                 assert_eq!(test_s1_s1_s1_t1.id, "s1-s1-s1-t1");
//                 assert_eq!(test_s1_s1_s1_t1.name, "History Test");
//                 assert_eq!(
//                     test_s1_s1_s1_t1.doc.as_ref().unwrap(),
//                     "The aim of this test is to do something"
//                 );
//                 assert_eq!(test_s1_s1_s1_t1.tags.len(), 2);
//                 assert_eq!(test_s1_s1_s1_t1.status.status, "PASS");
//             } else {
//                 panic!("Expected a Keyword, but got something else");
//             }
//         }
//     } else {
//         panic!("Expected a Suite, but got something else");
//     }
// } else {
//     panic!("Expected a Suite, but got something else");
// }
// if let SuiteChildren::Keyword(first_kw) = &suite.children[0] {
//     assert_eq!(first_kw.name, "Acceptance Setup");
//     assert_eq!(first_kw.library.as_ref().unwrap(), "init-keywords");
//     assert_eq!(first_kw.type_.as_ref().unwrap(), "SETUP");

//     let first_kw_base_kw = &first_kw.children;
//     assert_eq!(first_kw_base_kw.len(), 4);

//     if let KeywordChildren::Keyword(kw_with_for) = &first_kw_base_kw[0] {
//         if let KeywordChildren::For(for_kw) = &kw_with_for.children[0] {
//             assert_eq!(for_kw.flavor, "IN");
//             assert_eq!(for_kw.iters.len(), 2);
//             assert_eq!(for_kw.vars.len(), 2);
//             assert_eq!(for_kw.vars[0], "${aVar}");
//             assert_eq!(for_kw.values[0], "@{aList}");
//             assert_eq!(for_kw.values[1], "0");

//             let first_iter = &for_kw.iters[0];
//             assert_eq!(first_iter.vars.len(), 2);
//             assert_eq!(first_iter.vars[0].name, "${aVar}");
//             assert!(first_iter.vars[0].value.is_none());

//             let first_iter_kws = &first_iter.children;
//             assert_eq!(first_iter_kws.len(), 1);
//         } else {
//             panic!("Expected a For, but got something else");
//         }
//     } else {
//         panic!("Expected a Keyword, but got something else");
//     }

//     if let KeywordChildren::If(if_) = &first_kw_base_kw[1] {
//         assert_eq!(if_.branches.len(), 2);

//         let first_branch = &if_.branches[0];
//         assert_eq!(first_branch.type_, "IF");
//         assert_eq!(
//             first_branch.condition.as_ref().unwrap(),
//             "not ${SCREENSHOT NEEDED}"
//         );

//         if let KeywordChildren::Keyword(kw) = &first_branch.children[0] {
//             assert_eq!(kw.name, "Register Keyword To Run On Failure");
//             assert_eq!(kw.msg[0].timestamp, "20241217 11:27:26.201");
//             assert_eq!(kw.msg[0].value, "Keyword will not be run on failure.");
//             assert_eq!(kw.var[0], "${dummyVar}");
//             assert_eq!(kw.args[0], "${None}");
//             assert_eq!(kw.tags[0], "Config");
//             assert_eq!(
//                 kw.doc.as_ref().unwrap(),
//                 "Sets the keyword to execute, when a Browser keyword fails."
//             );
//         } else {
//             panic!("Expected a kw, but got something else");
//         }

//         let if_status = &if_.status;
//         assert_eq!(if_status.status, "PASS");
//     } else {
//         panic!("Expected an If, but got something else");
//     }

//     assert_eq!(first_kw.status.as_ref().unwrap().status, "PASS");

//     let first_kw_return = &first_kw.return_.as_ref().unwrap();
//     assert_eq!(first_kw_return.value.as_ref().unwrap(), "${response}");
//     assert_eq!(first_kw_return.status.status, "PASS");
// } else {
//     panic!("Expected an Keyword, but got something else");
// }

// let total_stats = &test_run.statistics.total.stats;
// assert_eq!(total_stats.pass, 7);
// assert_eq!(total_stats.fail, 1);
// assert_eq!(total_stats.skip, 0);
// assert_eq!(total_stats.text, "All Tests");

// let tag_stats = &test_run.statistics.tags.stats;
// assert_eq!(tag_stats.len(), 15);
// assert_eq!(tag_stats[5].pass, 2);
// assert_eq!(tag_stats[5].fail, 1);
// assert_eq!(tag_stats[5].skip, 0);
// assert_eq!(tag_stats[5].text, "something");

// let suite_stats = &test_run.statistics.suites.stats;
// assert_eq!(suite_stats.len(), 8);
// assert_eq!(suite_stats[0].pass, 7);
// assert_eq!(suite_stats[0].fail, 1);
// assert_eq!(suite_stats[0].skip, 0);
// assert_eq!(suite_stats[0].id, "s1");
// assert_eq!(suite_stats[0].name, "Acceptance");
// assert_eq!(suite_stats[0].text, "Acceptance");

// let errors = &test_run.errors;
// assert_eq!(errors.messages.len(), 2);
// assert_eq!(errors.messages[0].timestamp, "20250115 10:52:56.694");
// assert_eq!(errors.messages[0].level, "WARN");
//     }
// }
