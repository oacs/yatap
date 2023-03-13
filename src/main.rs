use std::{fs::ReadDir, path::PathBuf};

use clap::Parser;
use config::load_config;

mod config;

/// Project launcher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to config fail
    #[arg(short, long)]
    config: PathBuf,
}

fn main() {
    let args = Args::parse();
    println!("{:?}!", args.config);
    let config = load_config(args.config);
    println!("{:?}", config.paths);

    let paths = config.paths;
    let paths = get_dir_paths(paths);
    println!("{:?}", paths);
    let search = "r";
    let searched_dirs = search_dirs(search, paths);
    println!("{:?}", searched_dirs);
}

/// Grabs a Vec<String> and map it to ReadDir
fn get_dir_paths(paths: Vec<String>) -> Vec<ReadDir> {
    let dir_paths: Vec<ReadDir> = paths
        .iter()
        .map(|path| std::fs::read_dir(path))
        .filter_map(|dir| dir.ok())
        .collect();
    return dir_paths;
}

fn search_dirs(search_text: &str, dirs: Vec<ReadDir>) -> Vec<String> {
    dirs.into_iter()
        .flat_map(|dir| dir.filter_map(Result::ok))
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| {
            let path = entry.path();
            let file_name = path.file_name()?.to_str()?;
            if file_name.contains(search_text) {
                Some(path.to_str().unwrap().to_string())
            } else {
                None
            }
        })
        .collect()
}
