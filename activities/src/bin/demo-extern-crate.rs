use humantime::format_duration;
use std::time::Duration;

fn main() {
    let d = Duration::from_secs(4901);
    println!("{}", format_duration(d));
}
