// src/main.rs
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use futures_util::StreamExt;
use reqwest::multipart::{Form, Part};
use serde::Serialize;
use std::path::{Path, PathBuf};
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use tracing::{debug, error, info, level_filters::LevelFilter};
use tracing_subscriber::fmt::format::FmtSpan;

#[derive(Parser)]
#[command(name = "robot-cli")]
#[command(about = "CLI tool for robot data operations", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch robot data from CI server
    Fetch {
        /// The main job name (e.g., Horus-Robot-Full)
        #[arg(long)]
        job_name: String,

        /// The sub job name (e.g., v5.2)
        #[arg(long)]
        sub_job: String,

        /// The build number (e.g., 89)
        #[arg(long)]
        build_number: u32,

        /// Destination folder to save the output.xml file
        #[arg(long)]
        destination: PathBuf,
    },
    /// Upload robot data to the API
    Upload {
        #[arg(long)]
        folder_path: PathBuf,

        #[arg(long)]
        app_name: String,

        #[arg(long)]
        app_version: String,

        #[arg(long, default_value = "http://localhost:5325/api/robot/upload")]
        api_url: String,
    },
}

#[derive(Serialize)]
struct Metadata {
    #[serde(rename = "appName")]
    app_name: String,
    #[serde(rename = "appVersion")]
    app_version: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Fetch {
            job_name,
            sub_job,
            build_number,
            destination,
        } => {
            fetch_robot_data(&job_name, &sub_job, build_number, &destination).await?;
        }
        Commands::Upload {
            folder_path,
            app_name,
            app_version,
            api_url,
        } => {
            upload_robot_data(&folder_path, &app_name, &app_version, &api_url).await?;
        }
    }

    Ok(())
}

fn setup_logging() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .with_span_events(FmtSpan::CLOSE)
        .init();
    Ok(())
}

async fn fetch_robot_data(
    job_name: &str,
    sub_job: &str,
    build_number: u32,
    destination: &Path,
) -> Result<()> {
    let url = format!(
        "http://10.82.136.131/ci/job/{}/job/{}/{}/robot/report/output.xml",
        job_name, sub_job, build_number
    );

    // Create full destination path including the job-specific folder
    let job_folder = format!("{}-{}-{}", job_name, sub_job, build_number);
    let dest_folder = destination.join(job_folder);
    let file_path = dest_folder.join("output.xml");

    // Create the destination directory if it doesn't exist
    fs::create_dir_all(&dest_folder)
        .await
        .context("Failed to create destination directory")?;

    info!("Downloading file from: {}", url);
    info!("Saving to: {}", file_path.display());

    // Create a client with timeout configurations
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(600)) // 10-minute timeout for large files
        .build()
        .context("Failed to create HTTP client")?;

    // Start the request to fetch the file
    let res = client
        .get(&url)
        .send()
        .await
        .context("Failed to send request")?;

    // Check if the request was successful
    if !res.status().is_success() {
        error!(
            "Failed to download file. Status: {}, URL: {}",
            res.status(),
            url
        );
        anyhow::bail!("Failed to download file: HTTP {}", res.status());
    }

    // Get the content length if available
    let total_size = res.content_length();
    let total_size_str = total_size
        .map(|l| format!("{:.2} MB", l as f64 / 1_048_576.0))
        .unwrap_or_else(|| "unknown size".to_string());

    info!("File size: {}", total_size_str);

    // Create the output file
    let mut file = File::create(&file_path)
        .await
        .context("Failed to create output file")?;

    // Stream the response body to the file with progress updates
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();
    let mut last_progress_time = std::time::Instant::now();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.context("Failed to download chunk")?;
        file.write_all(&chunk)
            .await
            .context("Failed to write chunk to file")?;

        downloaded += chunk.len() as u64;

        // Log progress every 10MB or at least every 5 seconds
        let now = std::time::Instant::now();
        if downloaded % 10_485_760 < chunk.len() as u64
            || now.duration_since(last_progress_time).as_secs() >= 5
        {
            let percentage = total_size
                .map(|total| (downloaded as f64 / total as f64) * 100.0)
                .unwrap_or(0.0);

            if let Some(total) = total_size {
                info!(
                    "Downloaded: {:.2} MB / {:.2} MB ({:.1}%)",
                    downloaded as f64 / 1_048_576.0,
                    total as f64 / 1_048_576.0,
                    percentage
                );
            } else {
                info!("Downloaded: {:.2} MB", downloaded as f64 / 1_048_576.0);
            }
            last_progress_time = now;
        }
    }

    info!("Download complete: {}", file_path.display());
    Ok(())
}

async fn upload_robot_data(
    folder_path: &Path,
    app_name: &str,
    app_version: &str,
    api_url: &str,
) -> Result<()> {
    let xml_path = folder_path.join("output.xml");

    if !xml_path.exists() {
        anyhow::bail!("output.xml not found in {}", folder_path.display());
    }

    info!("Uploading file: {}", xml_path.display());
    info!("API URL: {}", api_url);
    info!("App Name: {}, App Version: {}", app_name, app_version);

    let metadata = Metadata {
        app_name: app_name.to_string(),
        app_version: app_version.to_string(),
    };
    let metadata_json = serde_json::to_string(&metadata).context("Failed to serialize metadata")?;

    let file_contents = fs::read(&xml_path)
        .await
        .context("Failed to read output.xml")?;

    let file_part = Part::bytes(file_contents)
        .file_name(xml_path.file_name().unwrap().to_string_lossy().to_string());
    let metadata_part = Part::text(metadata_json).mime_str("application/json")?;

    let form = Form::new()
        .part("file", file_part)
        .part("metadata", metadata_part);

    let client = reqwest::Client::new();
    let response = client
        .post(api_url)
        .multipart(form)
        .send()
        .await
        .context("Failed to send upload request")?;

    if response.status().is_success() {
        info!("Upload successful: {}", response.status());
        if let Ok(text) = response.text().await {
            debug!("Response: {}", text);
        }
        Ok(())
    } else {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        error!("Upload failed: {} - {}", status, body);
        anyhow::bail!("Upload failed: HTTP {}", status)
    }
}
