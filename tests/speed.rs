use std::{
    fs::{read_dir, remove_file},
    time::Instant,
};

use ftlog::appender::{Duration, FileAppender, Period};

pub fn setup() -> () {
    let logger = ftlog::Builder::new()
        .bounded(10000, true)
        .root(FileAppender::new("./root.log"))
        .filter("rotate", "rotate", None)
        .appender("rotate", FileAppender::rotate("rotate.log", Period::Minute))
        .filter("expire", "expire", None)
        .appender(
            "expire",
            FileAppender::rotate_with_expire("expire.log", Period::Minute, Duration::seconds(30)),
        )
        .build()
        .expect("logger build failed");
    logger.init().expect("set logger failed");
}

fn clean(dir: &str) {
    for file in read_dir(dir)
        .unwrap()
        .into_iter()
        .filter_map(|x| x.ok())
        .filter(|f| f.file_type().unwrap().is_file())
    {
        if file
            .path()
            .extension()
            .map(|x| x.to_string_lossy())
            .unwrap_or_default()
            == "log"
        {
            remove_file(file.path()).unwrap();
        }
    }
}
#[test]
fn test_speed() {
    // ~80MB
    setup();
    {
        // file
        let now = Instant::now();
        for i in 1..=1_000_000 {
            ftlog::info!("file log {}", i);
        }
        ftlog::logger().flush();
        let elapsed = now.elapsed();
        println!("File elapsed: {}s", elapsed.as_secs_f64());
        clean("./");
        assert!(elapsed.as_secs() < 8);
    }

    {
        // file with rotate
        let now = Instant::now();
        for i in 1..=1_000_000 {
            ftlog::info!(target:"rotate", "file log {}", i);
        }
        ftlog::logger().flush();
        let elapsed = now.elapsed();
        println!("Rotate file elapsed: {}s", elapsed.as_secs_f64());
        clean("./");
        assert!(elapsed.as_secs() < 8);
    }

    {
        // file with rotate with expire
        let now = Instant::now();
        for i in 1..=1_000_000 {
            ftlog::info!(target:"expire", "file log {}", i);
        }
        ftlog::logger().flush();
        let elapsed = now.elapsed();
        println!(
            "Rotate file with expire elapsed: {}s",
            elapsed.as_secs_f64()
        );
        clean("./");
        assert!(elapsed.as_secs() < 8);
    }
}
