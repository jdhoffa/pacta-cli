use clap::Parser;
use reqwest::IntoUrl; // Import IntoUrl trait
use reqwest::Url;
use std::error::Error;
use std::process::Command;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/// Command-line arguments struct
#[derive(Parser, Debug)]
struct Cli {
    portfolio_name: String,
    data_path: std::path::PathBuf,
    working_dir: std::path::PathBuf,
}

/// Download an image tarball and save it to a file
async fn download_image(image_url: impl IntoUrl) -> Result<Vec<u8>, Box<dyn Error>> {
    let response = reqwest::get(image_url).await?;
    let image_data = response.bytes().await?;
    Ok(image_data.to_vec())
}

/// Load a Docker image from a tarball
fn load_docker_image(image_path: &str) -> Result<(), Box<dyn Error>> {
    Command::new("docker")
        .args(&["load", "-i", image_path])
        .status()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the command line arguments
    let args = Cli::parse();
    // Define the image URL
    let image_url = Url::parse("https://ghcr.io/rmi-pacta/workflow.transition.monitor:latest")?;

    let image_data = download_image(image_url).await?;

    // Create or open the file
    let mut image_file = File::create("image.tar").await?;
    image_file.write_all(&image_data).await?;

    load_docker_image("image.tar")?;

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
            args.portfolio_name.as_str(),
        ])
        .status()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download_image() {
        let image_url =
            Url::parse("https://github.com/github/hello-docker/pkgs/container/hello-docker")
                .unwrap();
        let image_data = download_image(image_url).await.unwrap();
        assert!(!image_data.is_empty());
    }

    #[test]
    fn test_load_docker_image() {
        // TODO: Add your test for load_docker_image here
    }
}
