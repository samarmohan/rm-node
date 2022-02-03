use clap::{Arg, App};
use delete::{delete_folder, delete_file};
use std::path::Path;
use ansi_term::Colour::{Red, Green};
use std::thread;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};


fn main() {
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

    pb.set_message("Deleting...");
    thread::sleep(Duration::from_secs(3));
    if Path::new("node_modules").exists() {
        delete_folder("node_modules").unwrap_or(println!("{}", Green.paint("Deleted node_modules successfully!")));
    } else {
        println!("{}", Red.paint("Folder node_modules not found"));
    }
    if should {
        delete_stuff("package-lock.json");
        delete_stuff("yarn.lock");
    }
    pb.finish_with_message("Done!");
}

fn delete_stuff(path: &str) {
    if Path::new(path).exists() {
        delete_file(path).unwrap_or(println!("{}", Green.paint("Deleted lockfiles successfully!")));
    } else {
        println!("{}", Red.paint("Lockfiles not found"));
    }
}