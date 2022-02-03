use clap::{Arg, App};
use delete::{rapid_delete_dir_all};


fn main() {
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

    if should {
        println!("deleting lock");
        lol();
    }

    println!("not deleting lock");
    lol();
}

async fn lol() {
    rapid_delete_dir_all("node_modules", None, None).await;
}