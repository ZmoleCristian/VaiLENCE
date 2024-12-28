use crate::processing::parse_response;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApiResult {
    pub results: Vec<ModerationResult>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModerationResult {
    pub categories: Categories,
    pub category_scores: CategoryScores,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Categories {
    pub violence: bool,
    pub violence_graphic: bool,
    pub harassment_threatening: bool,
    pub hate_threatening: bool,
    pub illicit_violent: bool,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CategoryScores {
    pub violence: f64,
    pub violence_graphic: f64,
    pub harassment_threatening: f64,
    pub hate_threatening: f64,
    pub illicit_violent: f64,
}

#[derive()]
pub struct Config {
    pub severity_min: f64,
    pub file_path: Option<String>,
    pub output_file: Option<String>,
    pub chunk_size: usize,
    pub error_retry: usize,
    pub verbose: bool,
    pub loop_mode: bool,
    pub api_key: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            severity_min: 0.01,
            file_path: None,
            output_file: None,
            chunk_size: 100,
            error_retry: 3,
            verbose: false,
            loop_mode: false,
            api_key: String::new(),
        }
    }
}

impl ApiResult {
    pub fn from_json_slice(slice: &[u8]) -> Option<Self> {
        parse_response(&String::from_utf8_lossy(slice))
    }
}
