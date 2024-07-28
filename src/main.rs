#![allow(unused)]  // Beginning only.

use crate::prelude::*;
use std::fs::{read_dir, ReadDir};

mod error;
mod prelude;
mod utils;

fn main() -> Result<()> {
    println!("\n=== Hello, world! ===\n");

    for entry in read_dir("./")?.filter_map(
        |e: std::result::Result<std::fs::DirEntry, std::io::Error>| e.ok()
    ) {
        let entry: String = W(&entry).try_into()?;
        println!("{entry:?}");  // :? -> Debug
    }

    Ok(())
}
