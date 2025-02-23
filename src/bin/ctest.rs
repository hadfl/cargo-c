use cargo::util::command_prelude::*;
use cargo::CliResult;
use cargo::Config;

use cargo_c::build::*;
use cargo_c::cli::run_cargo_fallback;
use cargo_c::cli::subcommand_test;
use cargo_c::config::*;

fn main() -> CliResult {
    let mut config = Config::default()?;

    let subcommand = subcommand_test("ctest");

    let mut app = clap::command!()
        .setting(AppSettings::DeriveDisplayOrder)
        .dont_collapse_args_in_usage(true)
        .allow_external_subcommands(true)
        .subcommand(subcommand);

    let args = app.clone().get_matches();

    let subcommand_args = match args.subcommand() {
        Some(("ctest", args)) => args,
        Some((cmd, args)) => {
            return run_cargo_fallback(cmd, args);
        }
        _ => {
            // No subcommand provided.
            app.print_help()?;
            return Ok(());
        }
    };

    if subcommand_args.is_present("version") {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    config_configure(&mut config, subcommand_args)?;

    let mut ws = subcommand_args.workspace(&config)?;

    let (packages, compile_opts) = cbuild(&mut ws, &config, subcommand_args, "dev")?;

    ctest(&ws, &config, subcommand_args, &packages, compile_opts)
}
