use chrono::{DateTime, Local};
use std::env;

const DATE_FORMAT: &str = "%A, %b %e, %Y";
const DATE_FORMAT_DISPLAY: &str = "%B %d";
const DATE_FORMAT_LINK: &str = "%Y-%m-%d";

fn main() {
    let args: Vec<String> = env::args().collect();
    let now: DateTime<Local> = Local::now();

    let flag = args.get(1);
    match flag {
        Some(e) => 
            match e.as_str() {
                "--link" | "-l" => print!(
                        "* [{}]({}.md)",
                        now.format(DATE_FORMAT_DISPLAY),
                        now.format(DATE_FORMAT_LINK)
                    ),
                _ => print!("NOT SUPPORTED")
            }
        None => print!(
                "# {}\n\n## TODO\n* [ ]\n\n## In Progress\n\n## Done\n\n## Notes\n\n[back to index](index.md)",
                now.format(DATE_FORMAT).to_string()
            ),

    };
}
