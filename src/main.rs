fn main() {
    let cmd = clap::Command::new("envdb")
        .bin_name("envdb")
        .subcommand_required(true)
        .subcommand(
            clap::command!("get").arg(
                clap::arg!(--"target-env" <PATH>)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .default_value(".env"),
            ),
        )
        .subcommand(
            clap::command!("put").arg(
                clap::arg!(--"target-env" <PATH>)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .default_value(".env"),
            ),
        )
        .subcommand(
            clap::command!("scan").arg(
                clap::arg!(--"target-env" <PATH>)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .default_value(".env"),
            ),
        );

    let matches = cmd.get_matches();
    let matches = match matches.subcommand() {
        Some(("get", matches)) => matches,
        Some(("put", matches)) => matches,
        Some(("scan", matches)) => matches,
        _ => unreachable!("clap should ensure we don't get here"),
    };

    let target_env_path = matches.get_one::<std::path::PathBuf>("target-env");
    println!("{:?}", target_env_path);
}
