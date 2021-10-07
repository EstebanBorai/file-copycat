extern crate notify;

use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::{read, write, File};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

const OUTPUT_FILE_DEFAULT_NAME: &str = "./file_copycat_output";

pub type ReplacerFn = Box<dyn Fn(Vec<u8>) -> Vec<u8>>;

pub fn watch<P>(
    input: P,
    output: P,
    replacer: ReplacerFn,
) -> std::result::Result<(), Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    let (tx, rx) = channel();

    get_or_create_output_file(&input, &output)?;

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1))?;

    watcher.watch(&input, RecursiveMode::Recursive)?;

    loop {
        match rx.recv() {
            Ok(event) => {
                if matches!(event, DebouncedEvent::Write(_)) {
                    let bytes = read(&input)?;
                    let bytes = replacer(bytes);

                    write(&output, bytes)?;
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn get_or_create_output_file<P>(
    input: P,
    output: P,
) -> std::result::Result<File, Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    let output_file_bytes = read(input)?;
    if output_file_bytes.is_empty() {
        let file = File::open(output)?;

        return Ok(file);
    }

    println!(
        "No output file found. Creating a new file with name {}",
        OUTPUT_FILE_DEFAULT_NAME
    );
    let output_file = File::create(OUTPUT_FILE_DEFAULT_NAME)?;

    return Ok(output_file);
}
