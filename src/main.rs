use clap::Parser;
use reqwest::IntoUrl;
use reqwest::Url;
use std::error::Error;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;
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

    // Create a temporary file to store the image tarball
    let mut image_temp_file = NamedTempFile::new()?;

    // Write the image data to the temporary file
    image_temp_file.write_all(&image_data)?;

    // Load the Docker image from the tarball
    let image_path = image_temp_file.path().to_str().unwrap();
    load_docker_image(image_path)?;

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
