use std::{
    fs::{File, remove_file},
    time::{SystemTime, UNIX_EPOCH, Duration},
};
use filetime::{FileTime, set_file_times};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn random_string(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn create_fake_file(size: u64, timestamp: Duration) -> std::io::Result<()> {
    for i in 0..60 {
        let filename = format!("{}{}", i, random_string(23));
        let mut file = File::create(&filename)?;
        file.set_len(size)?;
        let secs = timestamp.as_secs() as i64;
        let mtime = FileTime::from_unix_time(secs, 0);
        let atime = FileTime::from_system_time(SystemTime::now());
        set_file_times(&filename, atime, mtime)?;
        remove_file(filename)?;
    }
    Ok(())
}
