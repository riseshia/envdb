use std::process::exit;

use exitcode;
use clap::Arg;

fn main() {
    let cmd = clap::Command::new("envdb")
        .bin_name("envdb")
        .subcommand_required(true)
        .subcommand(
            clap::command!("get")
                .arg(
                    Arg::new("target-env")
                        .long("target-env")
                        .value_name("PATH")
                        .value_parser(clap::value_parser!(std::path::PathBuf))
                        .default_value(".env"),
                )
                .arg(
                    Arg::new("key")
                        .required(true)
                )
        )
        .subcommand(
            clap::command!("put")
                .arg(
                    Arg::new("target-env")
                        .long("target-env")
                        .value_name("PATH")
                        .value_parser(clap::value_parser!(std::path::PathBuf))
                        .default_value(".env")
                )
                .arg(
                    Arg::new("key").required(true)
                )
                .arg(
                    Arg::new("value").required(true)
                )
        )
        .subcommand(
            clap::command!("scan")
                .arg(
                    Arg::new("target-env")
                        .long("target-env")
                        .value_name("PATH")
                        .value_parser(clap::value_parser!(std::path::PathBuf))
                        .default_value(".env")
                )
                .arg(
                    Arg::new("key_prefix").required(true)
                )
        )
        .subcommand(
            clap::command!("delete")
            .arg(
                Arg::new("target-env")
                .long("target-env")
                .value_name("PATH")
                .value_parser(clap::value_parser!(std::path::PathBuf))
                .default_value(".env")
                )
            .arg(
                Arg::new("key").required(true)
                )
        );

    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("get", matches)) => {
            let target_env_path = matches.get_one::<std::path::PathBuf>("target-env").unwrap();
            let key = matches.get_one::<String>("key").unwrap();
            match envdb::get(target_env_path, key) {
                Ok(env_pair) => {
                    println!("{}", env_pair.value);
                    exit(exitcode::OK);
                },
                Err(err_msg) => {
                    eprintln!("{}", err_msg);
                    exit(1); // XXX: could be better?
                }
            }
        },
        Some(("put", matches)) => {
            let target_env_path = matches.get_one::<std::path::PathBuf>("target-env").unwrap();
            let key = matches.get_one::<String>("key").unwrap();
            let value = matches.get_one::<String>("value").unwrap();
            match envdb::put(target_env_path, key, value) {
                Ok(_) => {
                    exit(exitcode::OK);
                },
                Err(err_msg) => {
                    eprintln!("{}", err_msg);
                    exit(1); // XXX: could be better?
                }
            }
        },
        Some(("scan", matches)) => {
            let target_env_path = matches.get_one::<std::path::PathBuf>("target-env").unwrap();
            let key_prefix = matches.get_one::<String>("key_prefix").unwrap();
            match envdb::scan(target_env_path, key_prefix) {
                Ok(env_pairs) => {
                    if env_pairs.is_empty() {
                        exit(1);
                    } else {
                        for env_pair in env_pairs {
                            println!("{}", env_pair.to_line());
                        }
                        exit(exitcode::OK);
                    }
                },
                Err(err_msg) => {
                    eprintln!("{}", err_msg);
                    exit(1); // XXX: could be better?
                }
            }
        },
        Some(("delete", matches)) => {
            let target_env_path = matches.get_one::<std::path::PathBuf>("target-env").unwrap();
            let key = matches.get_one::<String>("key").unwrap();
            match envdb::delete(target_env_path, key) {
                Ok(_) => {
                    exit(exitcode::OK);
                },
                Err(err_msg) => {
                    eprintln!("{}", err_msg);
                    exit(1); // XXX: could be better?
                }
            }
        },
        _ => unreachable!("clap should ensure we don't get here"),
    }
}
