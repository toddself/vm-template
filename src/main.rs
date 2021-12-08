use chrono::{DateTime, Local};

const DATE_FORMAT: &str = "%A, %b %e, %Y";

fn main() {
    let now: DateTime<Local> = Local::now();
    print!(
        "# {}\n\n## TODO\n* [ ]\n\n## In Progress\n*\n\n## Done\n*\n\n## Notes\n\n[back to index](index.md)",
        now.format(DATE_FORMAT).to_string()
    );
}
