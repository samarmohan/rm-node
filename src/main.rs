use clap::{Arg, Command};
use delete::{rapid_delete_dir_all, delete_file_async};
use std::path::Path;
use owo_colors::OwoColorize;
use std::thread;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};

#[tokio::main]
async fn main() {
    let app = Command::new("rm-node")
        .version("0.1.0")
        .about("Delete node_modules, package-lock.json and/or yarn.lock quickly and asynchronously on all platforms")
        .author("Samar Mohan");

    let lockfiles_option = Arg::new("lockfiles")
        .long("lockfiles")
        .takes_value(false)
        .help("Should delete lockfiles?");

    let app = app.arg(lockfiles_option);
    let matches = app.get_matches();

    let should = matches.is_present("lockfiles");
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(120);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&[
                "⠋",
                "⠙",
                "⠹",
                "⠸",
                "⠼",
                "⠴",
                "⠦",
                "⠧",
                "⠇",
                "⠏"
            ])
            .template("{spinner:.blue} {msg}"),
    );
    pb.set_message("Deleting...");
    thread::sleep(Duration::from_secs(3));
    pb.finish_with_message("Done!");

    if Path::new("node_modules").exists() {
        rapid_delete_dir_all("node_modules", None, None).await.unwrap();
        println!("{}", "Successfully deleted node_modules".on_green());
    } else {
        println!("{}", "Folder node_modules not found".on_red());
    }
    if should {
        delete_stuff("package-lock.json").await;
        delete_stuff("yarn.lock").await;
    }
}

async fn delete_stuff(path: &str) {
    if Path::new(path).exists() {
        delete_file_async(path).await.unwrap();
        println!("{}", format!("Successfully deleted {}", path).on_green());
    } else {
        println!("{}", format!("File {} not found", path).on_red());
    }
}