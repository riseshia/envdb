use std::fs;
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;

use tempfile::NamedTempFile;

pub struct EnvPair {
    pub key: String,
    pub value: String,
}

impl EnvPair {
    fn to_line(&self) -> String {
        format!("{}={}", self.key, self.value)
    }
}

fn line_to_env_pair(line: &str) -> EnvPair {
    if let Some((key, value)) = line.split_once('=') {
        EnvPair {
            key: key.to_string(),
            value: value.to_string()
        }
    } else {
        panic!("Fail to parse env pair string");
    }
}

pub fn get(target_env_path: &Path, key: &str) -> Result<EnvPair, String> {
    let key_with_equal = format!("{}=", key);
    let target_env_path_str = target_env_path.to_str().expect("Fail to convert env path to string");

    if let Ok(file) = fs::File::open(target_env_path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with(&key_with_equal) {
                    let env_pair = line_to_env_pair(&line);
                    return Ok(env_pair);
                }
            } else {
                return Err(format!("Failed to read a line in env file: {}", target_env_path_str));
            }
        }
        Err(format!("Not found key in env file: {}", target_env_path_str))
    } else {
        Err(format!("Failed to open the file: {}", target_env_path_str))
    }
}

pub fn put(target_env_path: &Path, key: &str, new_value: &str) -> Result<(), String> {
    let target_env_path_str = target_env_path.to_str().expect("Fail to convert env path to string");

    if let Ok(file) = fs::File::open(target_env_path) {
        let reader = io::BufReader::new(file);
        let mut to_be_replaced = false;

        let mut pairs = vec![];

        for line in reader.lines() {
            if let Ok(line) = line {
                let mut env_pair = line_to_env_pair(&line);
                if env_pair.key == key {
                    to_be_replaced = true;
                    env_pair.value = new_value.to_string();
                }

                pairs.push(env_pair);
            } else {
                return Err(format!("Failed to read a line in env file: {}", target_env_path_str));
            }
        }

        if !to_be_replaced {
            let new_pair = EnvPair { key: key.to_string(), value: new_value.to_string() };
            pairs.push(new_pair);
        }

        if let Ok(mut tmpfile) = NamedTempFile::new() {
            for pair in pairs {
                let res = writeln!(tmpfile, "{}", pair.to_line());

                if let Err(err) = res {
                    return Err(format!("Failed to write the file for rewrite: {}", err));
                }
            }

            let tmpfile_path = tmpfile.into_temp_path();

            if let Err(err) = fs::copy(tmpfile_path, target_env_path) {
                return Err(format!("Failed to write the file for rewrite: {}", err));
            }
        } else {
            return Err("Failed to open the tempfile for rewrite".to_string());
        }

        Ok(())
    } else {
        Err(format!("Failed to open the file: {}", target_env_path_str))
    }
}
