use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub struct EnvPair {
    pub key: String,
    pub value: String,
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
                println!("Failed to read a line");
            }
        }
        Err(format!("Not found key in env file: {}", target_env_path_str))
    } else {
        Err(format!("Failed to open the file: {}", target_env_path_str))
    }
}
