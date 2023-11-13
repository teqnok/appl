use checksums::hash_file;
use colored::ColoredString;
use colored::Colorize;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::header::CONTENT_LENGTH;
use std::error::Error;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
#[tokio::main]
// Downloading function. (sends a HTTP GET request to a URL and saves it to the $path var)
pub async fn download_file(
    url: &str,
    path: &str,
    name: ColoredString,
) -> Result<(), Box<dyn std::error::Error>> {
    // Remove quotes from the string ("string" -> string)
    let url = url.trim_matches('"');
    // Create a reqwest client
    let client = reqwest::Client::new();
    let new_path = PathBuf::from(path);
    // Send a GET request to the file URL
    let response = client.get(url).send().await?;
    // Get the total size of the file from the Content-Length header
    let total_size = response
        .headers()
        .get(CONTENT_LENGTH)
        .and_then(|ct_len| ct_len.to_str().ok())
        .and_then(|ct_len| ct_len.parse().ok())
        .unwrap_or(0);
    // If there is no Content-Length (GitHub doesn't provide one sometimes), indicatif will report a size of 0.
    // TODO see if fixable

    // Create a progress bar with the total size of the file
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template(
            "{msg} \t \t [{elapsed_precise}] [{bar:25.cyan/blue}] {bytes}/{total_bytes}",
        )
        .unwrap()
        .progress_chars("#>-"),
    );
    pb.set_message(format!("{}{} {}", "=".blue(), ">".green(), name));
    // Open the file in write-only mode
    std::fs::create_dir_all(new_path.parent().unwrap())?;
    let mut file = std::fs::File::create(path)?;

    // Read the response body in chunks
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        // Write the chunk to the file
        file.write_all(&chunk)?;
        // Update the progress bar with the length of the chunk
        pb.inc(chunk.len() as u64);
    }

    // Finish the progress bar
    pb.finish();

    Ok(())
}

// Function to read a TOML file and parse it to a std::String
pub fn read_repos() -> Result<Vec<String>, Box<dyn Error>> {
    let mut return_vec: Vec<String> = vec![];
    let uname = whoami::username();
    for item in std::fs::read_dir(format!("/home/{}/.config/appl/", uname))? {
        let item = item?;
        let path = item.path();
        if path.is_dir() {
            if let Some(file_name) = path.file_name() {
                if let Some(path_str) = file_name.to_str() {
                    return_vec.push(path_str.to_string());
                }
            }
        }
    }

    Ok(return_vec)
}
pub async fn verify_checksums(path: &Path, expected: String) -> bool {
    let hash = hash_file(path, checksums::Algorithm::SHA2256);
    if hash == expected {
        true
    } else {
        false
    }
    
}