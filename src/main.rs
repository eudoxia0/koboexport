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
    let sql = "select Text from Bookmark where Text is not null order by DateCreated asc;";
    let mut stmt = conn.prepare(sql)?;
    let bookmarks = stmt.query_map([], |row| {
        let text: String = row.get(0)?;
        Ok(text)
    })?;
    let mut first = true;
    for bookmark in bookmarks {
        let text = bookmark?;
        if !first {
            println!("\n\n---\n");
        }
        print!("{}", text);
        first = false;
    }
    println!();
    Ok(())
}
