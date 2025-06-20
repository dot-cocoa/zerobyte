use anyhow::{Ok, Result};
use std::fs::{OpenOptions, metadata};
use std::io::{Write, Seek, SeekFrom};
use rand::Rng;

pub fn bytewriter(filename:&str) -> Result<()>{
    let filesize = metadata(filename)?.len() as usize;
    let mut file = OpenOptions::new()
        .write(true)
        .open(filename)?;

    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..filesize).map(|_| rng.r#gen()).collect();
    file.seek(SeekFrom::Start(0))?;
    file.write_all(&random_bytes)?;
    file.flush()?;


    file.seek(SeekFrom::Start(0))?;
    let zeros = vec![0u8; filesize];
    file.write_all(&zeros)?;
    file.flush()?;

    file.seek(SeekFrom::Start(0))?;
    let ones = vec![0xFFu8; filesize];
    file.write_all(&ones)?;
    file.flush()?;

    Ok(())
}