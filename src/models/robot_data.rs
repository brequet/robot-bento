use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RobotSuite {
    name: String,
    tests: Vec<RobotTest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RobotTest {
    name: String,
    status: String,
    message: Option<String>,
}
