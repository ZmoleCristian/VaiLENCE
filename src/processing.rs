use std::fs::OpenOptions;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use crate::request::{self};
use crate::types::{ApiResult, Categories, CategoryScores, Config, ModerationResult};

pub fn process_chunk(config: &Config, lines: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let input_chunk = lines.iter().map(|s| format!(r#""{}""#, sanitize_input(s))).collect::<Vec<String>>().join(",");
    let mut response;

    let mut retry_count = config.error_retry;
    loop {
        let request = request::build_request(&config.api_key, &input_chunk);
        response = request::send_request(request);

        if let Some((message, error_type)) = parse_error(&String::from_utf8_lossy(&response)) {
            eprintln!("Error: {} \nError type: {}\nTrying again for {} more times", message, error_type, retry_count);
            if error_type == "server_error" && retry_count > 0 {
                retry_count -= 1;
                sleep(Duration::from_secs(1));
                continue;
            }
        }
        break;
    }

    handle_response(&config, &response, lines)?;

    Ok(())
}

fn handle_response(config: &Config, response: &[u8], lines: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(body_start) = response.windows(4).position(|window| window == b"\r\n\r\n") {
        let body = &response[body_start + 4..];

        if let Some(parsed_response) = ApiResult::from_json_slice(body) {
            for (i, result) in parsed_response.results.iter().enumerate() {
                let (violence_score, threat_score) = gather_scores(&result.categories, &result.category_scores);
                if should_output(violence_score, threat_score, config.severity_min) {
                    output_result(&config, &lines[i], violence_score, threat_score)?;
                }
            }
        } else {
            println!("Failed to parse JSON body.");
        }
    } else {
        println!("Invalid response format.");
    }
    Ok(())
}

fn gather_scores(categories: &Categories, scores: &CategoryScores) -> (f64, f64) {
    let violence_scores: Vec<f64> = vec![
        if categories.violence { Some(scores.violence) } else { None },
        if categories.violence_graphic { Some(scores.violence_graphic) } else { None },
        if categories.illicit_violent { Some(scores.illicit_violent) } else { None },
    ]
    .into_iter()
    .flatten()
    .collect();

    let threat_scores: Vec<f64> = vec![
        if categories.harassment_threatening {
            Some(scores.harassment_threatening)
        } else {
            None
        },
        if categories.hate_threatening { Some(scores.hate_threatening) } else { None },
    ]
    .into_iter()
    .flatten()
    .collect();

    let avg_violence = if !violence_scores.is_empty() {
        violence_scores.iter().sum::<f64>() / violence_scores.len() as f64
    } else {
        0.0
    };

    let avg_threat = if !threat_scores.is_empty() {
        threat_scores.iter().sum::<f64>() / threat_scores.len() as f64
    } else {
        0.0
    };

    (avg_violence, avg_threat)
}

fn should_output(violence_score: f64, threat_score: f64, severity_min: f64) -> bool {
    violence_score >= severity_min || threat_score >= severity_min
}

fn output_result(config: &Config, line: &str, violence_score: f64, threat_score: f64) -> Result<(), Box<dyn std::error::Error>> {
    let output = line.to_string();
    let scores = format!("Violence:{:.2}|Threat:{:.2}", violence_score, threat_score);

    if let Some(file_path) = &config.output_file {
        let mut file = OpenOptions::new().append(true).create(true).open(file_path)?;
        let escaped_text = escape_json_string(&output);
        let json_output = format!(
            r#"{{"text": "{}", "scores": {{"violence": {:.2}, "threat": {:.2}}}}}"#,
            escaped_text, violence_score, threat_score
        );
        writeln!(file, "{}", json_output)?;
    }

    if config.verbose || config.output_file.is_none() {
        let highest_score = violence_score.max(threat_score);
        let background_color_code = if highest_score <= 0.05 {
            "\x1b[48;2;0;255;0m".to_string()
        } else {
            let red_intensity = 255;
            let green_intensity = (255.0 * (1.0 - (highest_score - 0.05) / 0.95)).max(0.0) as u8;
            format!("\x1b[48;2;{};{};0m", red_intensity, green_intensity)
        };

        let text_color_code = if highest_score <= 0.05 {
            "\x1b[38;2;0;255;0m".to_string()
        } else {
            let red_intensity = 255;
            let green_intensity = (255.0 * (1.0 - (highest_score - 0.05) / 0.95)).max(0.0) as u8;
            format!("\x1b[38;2;{};{};0m", red_intensity, green_intensity)
        };

        println!(
            "\x1b[1m{}{}{}\x1b[0m\x1b[1m{} -> {}\x1b[0m",
            background_color_code, scores, "\x1b[0m", text_color_code, output
        );
    }

    Ok(())
}

fn escape_json_string(input: &str) -> String {
    input.replace("\\", "\\\\").replace("\"", "\\\"").replace("\n", "\\n").replace("\t", "\\t")
}

fn sanitize_input(input: &str) -> String {
    input.replace("\\", "\\\\").replace("\"", "\\\"")
}

pub fn parse_error(body: &str) -> Option<(String, String)> {
    let error_start = body.find("\"error\": {")?;
    let error_body = &body[error_start..];

    let message_start = error_body.find("\"message\": \"")? + "\"message\": \"".len();
    let message_end = error_body[message_start..].find('\"')? + message_start;
    let message = &error_body[message_start..message_end];

    let type_start = error_body.find("\"type\": \"")? + "\"type\": \"".len();
    let type_end = error_body[type_start..].find('\"')? + type_start;
    let error_type = &error_body[type_start..type_end];

    Some((message.to_string(), error_type.to_string()))
}

pub fn parse_response(body: &str) -> Option<ApiResult> {
    let results_start = body.find("\"results\": [")?;
    let results_body = &body[results_start..];

    let mut results = Vec::new();

    for result_str in results_body.split("\"categories\": {").skip(1) {
        let categories_end = result_str.find("},")?;
        let categories_str = &result_str[..categories_end];

        let scores_start = result_str.find("\"category_scores\": {")? + "\"category_scores\": {".len();
        let scores_end = result_str[scores_start..].find("}")? + scores_start;
        let scores_str = &result_str[scores_start..scores_end];

        let categories = Categories {
            violence: categories_str.contains("\"violence\": true"),
            violence_graphic: categories_str.contains("\"violence/graphic\": true"),
            harassment_threatening: categories_str.contains("\"harassment/threatening\": true"),
            hate_threatening: categories_str.contains("\"hate/threatening\": true"),
            illicit_violent: categories_str.contains("\"illicit/violent\": true"),
        };

        let category_scores = CategoryScores {
            violence: extract_score(scores_str, "\"violence\":"),
            violence_graphic: extract_score(scores_str, "\"violence/graphic\":"),
            harassment_threatening: extract_score(scores_str, "\"harassment/threatening\":"),
            hate_threatening: extract_score(scores_str, "\"hate/threatening\":"),
            illicit_violent: extract_score(scores_str, "\"illicit/violent\":"),
        };

        results.push(ModerationResult { categories, category_scores });
    }

    Some(ApiResult { results })
}

fn extract_score(scores_str: &str, key: &str) -> f64 {
    scores_str
        .split(key)
        .nth(1)
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.trim().parse::<f64>().ok())
        .unwrap_or_default()
}
