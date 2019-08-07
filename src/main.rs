#[macro_use]
extern crate failure;

mod errors;
mod runner;
mod snp;
mod stats;
mod cli;

use cli::Config;
use cli::SNPConfig;
use cli::WindowConfig;

use std::env;
use std::process;

fn main() {
    let app = cli::build_cli();
    let matches = cli::eval_cli(app, env::args_os());

    let result = match matches.subcommand() {
        ("gc", Some(m)) => {
            WindowConfig::parse_clap(m).and_then(|c| {
                runner::run_gc(&c.fasta, &c.outfile, c.window, c.step)
            })
        },
        ("cri", Some(m)) => {
            WindowConfig::parse_clap(m).and_then(|c| {
                runner::run_cri(&c.fasta, &c.outfile, c.window, c.step)
            })
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


/*
        ("snp", Some(m)) => {
            SNPConfig::parse_clap(m)
                .and_then(|c| runner::run_ripsnp(&c.fasta, &c.vcf))
        },
        ("", None) => {
            println!("no subcommand");
            Ok(())
        },
        */
