// Copyright 2025 Fernando Borretti
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::PathBuf;

use clap::Parser;
use rusqlite::Connection;
use rusqlite::Result;

#[derive(Parser)]
#[command(name = "koboexport")]
#[command(about = "Export bookmarks from a Kobo e-reader.", long_about = None)]
struct Cli {
    /// Path to the Kobo directory.
    #[arg(value_name = "DIR")]
    dir: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let db_path = cli.dir.join(".kobo").join("KoboReader.sqlite");
    let conn = Connection::open(&db_path).map_err(|e| {
        eprintln!("Error opening database at {:?}: {}", db_path, e);
        e
    })?;
    let sql =
        "select VolumeID, Text from Bookmark where Text is not null order by DateCreated asc;";
    let mut stmt = conn.prepare(sql)?;
    let bookmarks = stmt.query_map([], |row| {
        let volume_id: String = row.get(0)?;
        let text: String = row.get(1)?;
        let title = volume_id.rsplit('/').next().unwrap().to_owned();
        Ok((title, text))
    })?;
    let mut first = true;
    for bookmark in bookmarks {
        let (title, text) = bookmark?;
        if !first {
            println!("\n\n---\n");
        }
        print!("Title: {title}\n\n{}", text);
        first = false;
    }
    println!();
    Ok(())
}
