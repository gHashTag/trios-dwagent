use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use reqwest::Client;
use std::fs::{self, File};
use std::io::{copy, Write};
use std::path::Path;
use std::process::Command;
use zip::ZipArchive;

const RUSTDESK_SERVER_VERSION: &str = "1.1.15";
const RUSTDESK_SERVER_REPO: &str = "rustdesk/rustdesk-server";

fn rustdesk_dir() -> String {
    // Use /app on Railway, otherwise ~/.local/share/trios-dwagent
    if std::path::Path::new("/app").exists() {
        "/app/rustdesk-server".to_string()
    } else {
        format!(
            "{}/.local/share/trios-dwagent",
            std::env::var("HOME").unwrap_or_else(|_| ".".to_string())
        )
    }
}

fn download_dir() -> String {
    format!("{}/download", rustdesk_dir())
}

const HBBS_BINARY: &str = "hbbs";
const HBBR_BINARY: &str = "hbbr";

#[derive(Debug, Parser)]
#[command(name = "trios-dwagent")]
#[command(about = "RustDesk Server installer for Railway deployment", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Download RustDesk Server binaries
    Download {
        /// Force re-download even if files exist
        #[arg(long)]
        force: bool,
    },
    /// Start RustDesk Server (hbbs and hbbr)
    Start {
        /// Restart if already running
        #[arg(long)]
        restart: bool,
    },
    /// Stop RustDesk Server
    Stop,
    /// Check status of RustDesk Server
    Status,
    /// Clean up downloaded files and data
    Cleanup,
    /// Full setup (download + start)
    Setup {
        /// Force re-download even if files exist
        #[arg(long)]
        force: bool,
    },
}

/// Constructs the download URL for RustDesk Server Linux binaries
fn get_download_url(architecture: &str) -> String {
    format!(
        "https://github.com/{}/releases/download/{}/rustdesk-server-linux-{}.zip",
        RUSTDESK_SERVER_REPO,
        RUSTDESK_SERVER_VERSION,
        architecture
    )
}

/// Detects the system architecture for binary download
fn detect_architecture() -> Result<String> {
    let output = Command::new("uname")
        .args(["-m"])
        .output()
        .context("Failed to detect architecture")?;

    let arch = String::from_utf8(output.stdout)
        .context("Architecture output is not valid UTF-8")?
        .trim()
        .to_lowercase();

    match arch.as_str() {
        "x86_64" => Ok("amd64".to_string()),
        "aarch64" | "arm64" => Ok("arm64v8".to_string()),
        "armv7l" => Ok("armv7".to_string()),
        _ => anyhow::bail!("Unsupported architecture: {}", arch),
    }
}

/// Downloads and extracts RustDesk Server binaries
async fn download_binaries(force: bool) -> Result<()> {
    println!("📦 Setting up RustDesk Server v{}...", RUSTDESK_SERVER_VERSION);

    let rustdesk_dir = rustdesk_dir();
    let download_dir = download_dir();

    // Check if binaries already exist
    let hbbs_path = Path::new(&rustdesk_dir).join(HBBS_BINARY);
    let hbbr_path = Path::new(&rustdesk_dir).join(HBBR_BINARY);

    if !force && hbbs_path.exists() && hbbr_path.exists() {
        println!("✅ Binaries already exist. Use --force to re-download.");
        return Ok(());
    }

    // Create directories
    fs::create_dir_all(&rustdesk_dir)
        .context("Failed to create RustDesk Server directory")?;
    fs::create_dir_all(&download_dir)
        .context("Failed to create download directory")?;

    let architecture = detect_architecture()?;
    let url = get_download_url(&architecture);

    println!("📥 Downloading from: {}", url);

    let client = Client::builder()
        .user_agent("trios-dwagent/1.0")
        .build()?;

    let response = client
        .get(&url)
        .send()
        .await
        .context("Failed to fetch RustDesk Server")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Failed to download RustDesk Server: HTTP {}",
            response.status()
        );
    }

    let archive_path = Path::new(&download_dir).join("rustdesk-server.zip");
    let mut file = File::create(&archive_path)
        .context("Failed to create archive file")?;

    let content = response.bytes().await?;
    file.write_all(&content)
        .context("Failed to write archive file")?;

    println!("📦 Extracting binaries...");

    // Extract the zip archive
    let archive_file = File::open(&archive_path)
        .context("Failed to open archive file")?;
    let mut archive = ZipArchive::new(archive_file)
        .context("Failed to read zip archive")?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .context("Failed to get file from archive")?;

        let enclosed = file.enclosed_name();
        let filename = match enclosed {
            Some(path) => match path.file_name() {
                Some(name) => name.to_string_lossy().to_string(),
                None => continue,
            },
            None => continue,
        };

        // Only copy hbbs and hbbr binaries
        if filename == HBBS_BINARY || filename == HBBR_BINARY {
            let dest_path = Path::new(&rustdesk_dir).join(&filename);
            let mut dest_file = File::create(&dest_path)
                .context("Failed to create binary file")?;
            copy(&mut file, &mut dest_file)
                .context("Failed to copy binary")?;

            // Set executable permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&dest_path)?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&dest_path, perms)?;
            }
        }
    }

    println!("✅ Binaries installed to {}", rustdesk_dir);

    // Clean up archive
    fs::remove_file(&archive_path)
        .context("Failed to remove archive file")?;

    Ok(())
}

/// Checks if a server is running
fn is_server_running(name: &str) -> bool {
    Command::new("pgrep")
        .arg(name)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Starts the RustDesk Server (hbbs and hbbr)
fn start_server(restart: bool) -> Result<()> {
    let rustdesk_dir = rustdesk_dir();
    let hbbs_path = Path::new(&rustdesk_dir).join(HBBS_BINARY);
    let hbbr_path = Path::new(&rustdesk_dir).join(HBBR_BINARY);

    if !hbbs_path.exists() || !hbbr_path.exists() {
        anyhow::bail!("RustDesk Server binaries not found. Run 'setup' first.");
    }

    // Stop if restart requested
    if restart {
        stop_server()?;
    }

    // Start hbbs if not running
    if !is_server_running(HBBS_BINARY) || restart {
        println!("🚀 Starting hbbs (Rendezvous Server)...");

        Command::new(&hbbs_path)
            .arg("-k")
            .arg("_")
            .spawn()
            .context("Failed to start hbbs")?;

        println!("✅ hbbs started");
    } else {
        println!("ℹ️  hbbs already running");
    }

    // Start hbbr if not running
    if !is_server_running(HBBR_BINARY) || restart {
        println!("🚀 Starting hbbr (Relay Server)...");

        Command::new(&hbbr_path)
            .arg("-k")
            .arg("_")
            .spawn()
            .context("Failed to start hbbr")?;

        println!("✅ hbbr started");
    } else {
        println!("ℹ️  hbbr already running");
    }

    print_connection_info();
    Ok(())
}

/// Stops the RustDesk Server
fn stop_server() -> Result<()> {
    println!("🛑 Stopping RustDesk Server...");

    let mut stopped = false;

    if is_server_running(HBBS_BINARY) {
        Command::new("pkill")
            .arg(HBBS_BINARY)
            .output()?;
        println!("✅ hbbs stopped");
        stopped = true;
    }

    if is_server_running(HBBR_BINARY) {
        Command::new("pkill")
            .arg(HBBR_BINARY)
            .output()?;
        println!("✅ hbbr stopped");
        stopped = true;
    }

    if !stopped {
        println!("ℹ️  No servers were running");
    }

    Ok(())
}

/// Checks and prints server status
fn check_status() -> Result<()> {
    println!("📊 RustDesk Server Status:");
    println!();

    let hbbs_running = is_server_running(HBBS_BINARY);
    let hbbr_running = is_server_running(HBBR_BINARY);

    println!("  hbbs (Rendezvous): {}", if hbbs_running { "🟢 Running" } else { "🔴 Stopped" });
    println!("  hbbr (Relay):      {}", if hbbr_running { "🟢 Running" } else { "🔴 Stopped" });
    println!();

    if hbbs_running || hbbr_running {
        print_connection_info();
    }

    // Check for key files
    let rustdesk_dir = rustdesk_dir();
    let key_dir = Path::new(&rustdesk_dir);
    let id_key = key_dir.join("id_ed25519");
    let id_pub = key_dir.join("id_ed25519.pub");

    if id_key.exists() && id_pub.exists() {
        println!("🔑 Server keys generated at {}", rustdesk_dir);
    }

    Ok(())
}

/// Prints connection information
fn print_connection_info() {
    println!();
    println!("🔗 Connection Information:");
    println!("   HBBS Port: 21115 (ID server)");
    println!("   HBBR Port: 21116 (Relay server)");
    println!("   Web Port:   21114 (Web client)");
    println!();
    println!("💡 Configure RustDesk client:");
    println!("   ID Server:  <your-railway-host>:21115");
    println!("   Relay:      <your-railway-host>:21116");
    println!();
}

/// Cleans up downloaded files
fn cleanup() -> Result<()> {
    let rustdesk_dir = rustdesk_dir();
    let download_dir = download_dir();
    let mut cleaned = false;

    if Path::new(&download_dir).exists() {
        fs::remove_dir_all(&download_dir)
            .context("Failed to remove download directory")?;
        println!("🧹 Removed download directory");
        cleaned = true;
    }

    // Optional: Remove RustDesk Server directory
    if Path::new(&rustdesk_dir).exists() {
        println!("ℹ️  RustDesk Server directory exists: {}", rustdesk_dir);
        println!("   To remove manually: rm -rf {}", rustdesk_dir);
    }

    if !cleaned {
        println!("ℹ️  Nothing to clean up");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Download { force } => {
            download_binaries(force).await?;
        }
        Commands::Start { restart } => {
            start_server(restart)?;
        }
        Commands::Stop => {
            stop_server()?;
        }
        Commands::Status => {
            check_status()?;
        }
        Commands::Cleanup => {
            cleanup()?;
        }
        Commands::Setup { force } => {
            download_binaries(force).await?;
            start_server(true)?;
            println!("\n🎉 RustDesk Server is ready!");
        }
    }

    Ok(())
}
