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
    pub fn to_line(&self) -> String {
        format!("{}={}", self.key, self.value)
    }
}

enum EnvPairParseError {
    Comment,
    Unknown,
}

fn line_to_env_pair(line: &str) -> Result<EnvPair, EnvPairParseError> {
    if let Some((key, value)) = line.split_once('=') {
        Ok(EnvPair {
            key: key.to_string(),
            value: value.to_string()
        })
    } else if line.starts_with('#') {
        Err(EnvPairParseError::Comment)
    } else {
        Err(EnvPairParseError::Unknown)
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

                    match env_pair {
                        Ok(env_pair) => {
                            return Ok(env_pair)
                        },
                        Err(EnvPairParseError::Comment) => {
                            continue
                        },
                        Err(EnvPairParseError::Unknown) => {
                            eprintln!("Skip parse line: {}", line)
                        },
                    }
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

pub fn scan(target_env_path: &Path, key_prefix: &str) -> Result<Vec<EnvPair>, String> {
    let target_env_path_str = target_env_path.to_str().expect("Fail to convert env path to string");

    if let Ok(file) = fs::File::open(target_env_path) {
        let reader = io::BufReader::new(file);

        let mut matched_pairs = vec![];

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with(&key_prefix) {
                    let env_pair = line_to_env_pair(&line);

                    match env_pair {
                        Ok(env_pair) => {
                            matched_pairs.push(env_pair)
                        },
                        Err(EnvPairParseError::Comment) => {
                            continue
                        },
                        Err(EnvPairParseError::Unknown) => {
                            eprintln!("Skip parse line: {}", line)
                        },
                    }
                }
            } else {
                return Err(format!("Failed to read a line in env file: {}", target_env_path_str));
            }
        }

        Ok(matched_pairs)
    } else {
        Err(format!("Failed to open the file: {}", target_env_path_str))
    }
}

pub fn put(target_env_path: &Path, key: &str, new_value: &str) -> Result<(), String> {
    let target_env_path_str = target_env_path.to_str().expect("Fail to convert env path to string");

    if let Ok(file) = fs::File::open(target_env_path) {
        let reader = io::BufReader::new(file);
        let mut to_be_replaced = false;

        let mut new_lines = vec![];

        for line in reader.lines() {
            if let Ok(line) = line {
                let env_pair = line_to_env_pair(&line);

                match env_pair {
                    Ok(mut env_pair) => {
                        if env_pair.key == key {
                            to_be_replaced = true;
                            env_pair.value = new_value.to_string();
                            new_lines.push(env_pair.to_line())
                        } else {
                            new_lines.push(line)
                        }
                    },
                    Err(EnvPairParseError::Comment) => {
                        new_lines.push(line);
                    },
                    Err(EnvPairParseError::Unknown) => {
                        eprintln!("Skip parse line: {}", line);
                        new_lines.push(line)
                    },
                }
            } else {
                return Err(format!("Failed to read a line in env file: {}", target_env_path_str));
            }
        }

        if !to_be_replaced {
            let new_pair = EnvPair { key: key.to_string(), value: new_value.to_string() };
            new_lines.push(new_pair.to_line());
        }

        if let Ok(mut tmpfile) = NamedTempFile::new() {
            for new_line in new_lines {
                let res = writeln!(tmpfile, "{}", new_line);

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

pub fn delete(target_env_path: &std::path::PathBuf, key: &str) -> Result<(), String> {
    let target_env_path_str = target_env_path.to_str().expect("Fail to convert env path to string");

    if let Ok(file) = fs::File::open(target_env_path) {
        let reader = io::BufReader::new(file);

        let mut new_lines = vec![];

        for line in reader.lines() {
            if let Ok(line) = line {
                let env_pair = line_to_env_pair(&line);

                match env_pair {
                    Ok(env_pair) => {
                        if env_pair.key != key {
                            new_lines.push(line)
                        }
                    },
                    Err(EnvPairParseError::Comment) => {
                        new_lines.push(line);
                    },
                    Err(EnvPairParseError::Unknown) => {
                        eprintln!("Skip parse line: {}", line);
                        new_lines.push(line)
                    },
                }
            } else {
                return Err(format!("Failed to read a line in env file: {}", target_env_path_str));
            }
        }

        if let Ok(mut tmpfile) = NamedTempFile::new() {
            for new_line in new_lines {
                let res = writeln!(tmpfile, "{}", new_line);

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
