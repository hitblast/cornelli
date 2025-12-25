use std::{env, path::Path, process::exit};

use clap::Parser;
use cornelli::{
    cli::{Args, args::Command, atomic::set_accept_all},
    commands::Runnable,
    core::ChristmasDB,
    log_err,
};

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let args = Args::parse();

    // set global flags' values
    set_accept_all(args.accept_all);

    // create the database
    let pass = match env::var("CORNELLI_PASS") {
        Ok(var) => var,
        Err(_) => {
            log_err!("Password wasn't provided with CORNELLI_PASS.");
            exit(1)
        }
    };

    let db_path = match dirs::config_dir() {
        Some(path) => path.join("cornelli/christmas.json"),
        None => {
            log_err!("Config directory couldn't be determined, using current directory...");
            Path::new("christmas.json").to_path_buf()
        }
    };

    let mut db = match ChristmasDB::load_from_pass(pass, db_path.to_path_buf()).await {
        Ok(db) => db,
        Err(_) => {
            log_err!("ChristmasDB failed to initialize.");
            exit(1);
        }
    };

    // command invocation
    let result = match &args.command {
        Command::Keep(cmd) => cmd.run(&mut db).await,
        Command::Mailbox(cmd) => cmd.run(&mut db).await,
    };

    if let Err(err) = result {
        log_err!("{err}");
        exit(1);
    }
}
