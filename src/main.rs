use clap::{Arg, App};
use delete::{rapid_delete_dir_all, delete_file_async};
use std::path::Path;
use ansi_term::Colour::{Red, Green};
use std::thread;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};

#[tokio::main]
async fn main() {
    let app = App::new("rm-node")
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
        rapid_delete_dir_all("node_modules", None, None).await
            .unwrap_or_else(|e| {
                eprintln!("{}", Red.paint("Error: "));
                eprintln!("{}", e);
            });
        println!("{}", Green.paint("Successfully deleted node_modules"));
    } else {
        println!("{}", Red.paint("Folder node_modules not found"));
    }
    if should {
        delete_stuff("package-lock.json").await;
        delete_stuff("yarn.lock").await;
    }
}

async fn delete_stuff(path: &str) {
    if Path::new(path).exists() {
        delete_file_async(path).await
            .unwrap_or_else(|e| {
                eprintln!("{}", Red.paint("Error: "));
                eprintln!("{}", e);
            });
        println!("{}", Green.paint("Successfully deleted lockfiles"));
    }
}