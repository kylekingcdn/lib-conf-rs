use example1_lib::Example1Config;
use std::time::Duration;

fn main() {
    let config = Example1Config::builder()
        .log_file_path(Some("/tmp/test.log".to_string()))
        // despite our best doc efforts we still mixed the units up - our logs keep getting wiped
        // if only we could override this setting at runtime..
        .log_rotate_interval(Duration::from_secs(1))
        .build();

    println!("Library config: {config:#?}");
}
