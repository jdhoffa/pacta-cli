use reqwest::Url;
use std::process::Command;
use tokio::fs::File;
use tokio::io::AsyncWriteExt; // Import AsyncWriteExt trait

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the image URL
    let image_url = Url::parse("https://ghcr.io/jonashackt/hello-world:latest")?;

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
        .args(&["run", "--rm", "hello-world"])
        .status()?;

    Ok(())
}
