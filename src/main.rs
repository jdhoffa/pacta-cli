#![allow(unused)]

use clap::Parser;
use reqwest::Url;
use std::process::Command;
use tokio::fs::File;
use tokio::io::AsyncWriteExt; // Import AsyncWriteExt trait

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The portfolio to run
    portfolio_name: String,
    /// The path to the file to read
    data_path: std::path::PathBuf,
    // The path to the working directory
    working_dir: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the command line arguments
    let args = Cli::parse();
    // Define the image URL
    let image_url = Url::parse("https://ghcr.io/rmi-pacta/workflow.transition.monitor:latest")?;

    // Download the image tarball
    let response = reqwest::get(image_url).await?;
    let image_data = response.bytes().await?;

    // Create or open the file
    let mut image_file = File::create("image.tar").await?;

    image_file.write_all(&image_data).await?;

    // Pull the Docker image from the downloaded tarball
    Command::new("docker")
        .args(&["load", "-i", "image.tar"])
        .status()?;

    // Run a Docker container to display ASCII art
    Command::new("docker")
        .args(&[
            "run",
            "-it",
            "--rm",
            "--mount",
            format!(
                "type=bind,source={},target=/bound/working_dir",
                args.working_dir.display()
            )
            .as_str(),
            "--mount",
            format!(
                "type=bind,source={},target=/pacta-data",
                args.data_path.display()
            )
            .as_str(),
            "ghcr.io/rmi-pacta/workflow.transition.monitor:latest",
            "/bound/bin/run-r-scripts",
            //format!("/bound/bin/run-r-scripts {}", args.portfolio_name).as_str(),
        ])
        .status()?;

    Ok(())
}
