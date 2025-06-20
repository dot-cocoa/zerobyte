use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH, Duration},
    error::Error,
};

use anyhow::Result;
use rand::{
    distributions::Alphanumeric,
    thread_rng,
    RngCore, Rng,
};
use filetime::{FileTime, set_file_times};
use pbkdf2::pbkdf2;
use hmac::Hmac;
use sha2::Sha256;
use aes_gcm::{
    aead::{Aead, KeyInit, generic_array::GenericArray},
    Aes256Gcm, Nonce,
};
use chacha20poly1305::{ChaCha20Poly1305};

use crate::module::{
    bytewriter::{self, bytewriter},
    encrypt::{self, crpy},
    fakefile::create_fake_file,
};

mod module;

#[derive(Clone, Copy, Debug)]
enum Lang {
    Jp,
    En,
}

fn random_string(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

fn clear_screen() {
    if cfg!(windows) {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

fn print_header(lang: Lang) {
    let header = [
        " _____              ____        _               ",
        "|__  /___ _ __ ___ | __ ) _   _| |_ ___         ",
        "  / // _ \\ '__/ _ \\|  _ \\| | | | __/ _ \\    ",
        " / /|  __/ | | (_) | |_) | |_| | ||  __/        ",
        "/____\\___|_|  \\___/|____/ \\__, |\\__\\___|   ",
        "                          |___/                 ",
    ];
    let color_code = "\x1b[1;35m";
    let reset_code = "\x1b[0m";
    for line in header {
        println!("{}{}{}", color_code, line, reset_code);
    }
    println!();
    match lang {
        Lang::Jp => println!("\x1b[1;34m  多層暗号痕跡消去処理システム\x1b[0m \x1b[1;32mZeroByte\x1b[0m\n"),
        Lang::En => println!("\x1b[1;34m  Multi-layer Encrypted Trace Erasure System\x1b[0m \x1b[1;32mZeroByte\x1b[0m\n"),
    }
}

fn prompt_existing_file(lang: Lang) -> String {
    loop {
        let mut input = String::new();
        match lang {
            Lang::Jp => print!("ファイル名: "),
            Lang::En => print!("Filename: "),
        }
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input_trim = input.trim();
        if Path::new(input_trim).exists() {
            return input_trim.to_string();
        } else {
            match lang {
                Lang::Jp => println!("ファイルを見つけることができませんでした。再入力してください。"),
                Lang::En => println!("File not found. Please try again."),
            }
        }
    }
}

fn rename_file_multiple_times(mut current_filename: String) -> String {
    let mut current_path = PathBuf::from(&current_filename);
    for _ in 0..1000 {
        let newfilename = random_string(20);
        let new_path = current_path.with_file_name(&newfilename);
        if fs::rename(&current_path, &new_path).is_ok() {
            current_path = new_path;
        } else {
            break;
        }
    }
    current_path
        .file_name()
        .map(|os_str| os_str.to_string_lossy().to_string())
        .unwrap_or(current_filename)
}

fn encrypted_erasure(lang: Lang) {
    clear_screen();
    print_header(lang);

    let filename = prompt_existing_file(lang);
    let filemeta = fs::metadata(&filename).expect("Failed to get metadata");
    let modified = filemeta.modified().expect("Failed to get modified time");
    let duration = modified.duration_since(UNIX_EPOCH).unwrap();

    let final_filename = rename_file_multiple_times(filename);
    create_fake_file(filemeta.len(), duration).unwrap();
    match lang {
        Lang::Jp => {
            println!("メタデータコピーファイルの生成に成功しました。");
            println!("数千回のリネームに成功しました。");
        }
        Lang::En => {
            println!("Fake metadata files created successfully.");
            println!("Renamed thousands of times successfully.");
        }
    }

    crpy(&final_filename).unwrap();

    match lang {
        Lang::Jp => println!("複数層暗号化に成功しました。"),
        Lang::En => println!("Multi-layer encryption succeeded."),
    }

    bytewriter(&final_filename).unwrap();

    match lang {
        Lang::Jp => println!("ランダムバイトで数回の上書きに成功しました。"),
        Lang::En => println!("Overwritten multiple times with random bytes successfully."),
    }

    std::fs::remove_file(&final_filename);
    match lang {
        Lang::Jp => println!("すべての処理が正常に終了しました！！！"),
        Lang::En => println!("successfully:)"),
    }
}

fn mainscreen(lang: Lang) {
    print_header(lang);
    match lang {
        Lang::Jp => {
            println!("1. ファイルの完全暗号消去\n");
            print!("モードを選択してください: ");
        }
        Lang::En => {
            println!("1. Full Encrypted File Erasure\n");
            print!("Please select a mode: ");
        }
    }
    io::stdout().flush().unwrap();

    let mut modeselect = String::new();
    io::stdin().read_line(&mut modeselect).ok();

    if modeselect.trim() == "1" {
        encrypted_erasure(lang);
    }
}

fn configload() -> Result<Lang, Box<dyn Error>> {
    let config_file = "config.json";
    match fs::read_to_string(config_file) {
        Ok(cf) => {
            let data: HashMap<String, String> = serde_json::from_str(&cf)?;
            let lang_str = data.get("Language").map(|s| s.as_str()).unwrap_or("En");
            Ok(match lang_str {
                "Jp" => Lang::Jp,
                _ => Lang::En,
            })
        }
        Err(_) => {
            println!("Config file not found. Select Language:\n1. Japanese\n2. English");
            loop {
                let mut langnum = String::new();
                io::stdin().read_line(&mut langnum).ok();
                let (lang_alp, lang_code) = match langnum.trim() {
                    "1" => (Lang::Jp, "Jp"),
                    "2" => (Lang::En, "En"),
                    _ => {
                        println!("Invalid selection.");
                        continue;
                    }
                };
                let mut inputdata = HashMap::new();
                inputdata.insert("Language", lang_code);
                let json_string = serde_json::to_string_pretty(&inputdata)?;
                fs::write(config_file, json_string)?;
                return Ok(lang_alp);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lang = configload()?;
    mainscreen(lang);
    Ok(())
}
