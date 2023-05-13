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
                    Arg::new("prefix").required(true)
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
                },
                Err(err_msg) => {
                    eprintln!("{}", err_msg);
                    exit(1); // XXX: could be better?
                }
            }
        },
        Some(("put", matches)) => {
            exit(exitcode::OK);
        },
        Some(("scan", matches)) => {
            exit(exitcode::OK);
        },
        _ => unreachable!("clap should ensure we don't get here"),
    }
}
