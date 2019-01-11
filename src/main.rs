extern crate clap;
extern crate riprap;

mod cli;
use cli::Config;
use cli::SNPConfig;
use cli::WindowConfig;

use riprap::runner;

use std::env;
use std::process;

fn main() {
    let app = cli::build_cli();
    let matches = cli::eval_cli(app, env::args_os());

    let result = match matches.subcommand() {
        ("gc", Some(m)) => {
            WindowConfig::parse_clap(m)
                .and_then(|c| runner::run_gc(&c.fasta, c.window, c.step))
            },
        ("cri", Some(m)) => {
            WindowConfig::parse_clap(m)
                .and_then(|c| runner::run_cri(&c.fasta, c.window, c.step))
            },
        ("snp", Some(m)) => {
            SNPConfig::parse_clap(m)
                .and_then(|c| runner::run_ripsnp(&c.fasta, &c.vcf))
            },
        ("", None) => {
            println!("no subcommand");
            Ok(())
        },
        _ => unreachable!() // Ok(()),
    };

    process::exit(match result {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("Error: {:?}", err);
            1
        }
    });
}
