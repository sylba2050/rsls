use std::env;
use std::fs;
use filetime::FileTime;
use chrono::{DateTime, Local, TimeZone};


fn main() {
    let paths = env::current_dir().unwrap();

    for path in fs::read_dir(paths).unwrap() {
        let p = path.unwrap().path();
        println!("{}", p.display());

        let metadata = fs::metadata(p).unwrap();
        println!("{:?}", metadata.file_type());

        let mtime = FileTime::from_last_modification_time(&metadata);
        let dt1: DateTime<Local> = Local.timestamp(mtime.unix_seconds(), 0);
        println!("modified at {}", dt1);

        let atime = FileTime::from_creation_time(&metadata);
        if atime.is_some() {
            let dt2: DateTime<Local> = Local.timestamp(atime.unwrap().unix_seconds(), 0);
            println!("created at {}", dt2);
        } else if atime.is_none() {
            println!("unti");
        }
    }
}
