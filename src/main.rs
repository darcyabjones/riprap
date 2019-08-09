extern crate failure;

mod bedgraph;
mod cli;
mod errors;
mod runner;
mod stats;

use std::io;
use std::env;
use std::process;
use failure::{Fail, Error};

use cli::Config;
use cli::WindowConfig;
use errors::UnitResult;


fn main() {
    if let Err(err) = try_main() {
        let ecode = err.ecode();

        // A pipe error occurs when the consumer of this process's output has
        // hung up. This is a normal event, and we should quit gracefully.
        //if is_pipe_error(&err) {
        //    process::exit(0);
        //}

        eprintln!("{}", err.pretty_error());

        // If we get a non-empty backtrace (e.g., RUST_BACKTRACE=1 is set),
        // then show it.
        let backtrace = err
            .backtrace()
            .map(|b| b.to_string())
            .unwrap_or_else(|| "".to_string());

        if !backtrace.trim().is_empty() {
            eprintln!("{}", backtrace);
        }

        process::exit(err.ecode());
    }
}


fn try_main() -> UnitResult {
    let app = cli::build_cli();
    let matches = cli::eval_cli(app, env::args_os());

    match matches.subcommand() {
        ("gc", Some(m)) => {
            WindowConfig::parse_clap(m)
                .and_then(|c| {
                    runner::run_gc(&c.fasta, &c.outfile, c.window, c.step)
                })
        },
        ("cri", Some(m)) => {
            WindowConfig::parse_clap(m)
                .and_then(|c| {
                    runner::run_cri(&c.fasta, &c.outfile, c.window, c.step)
                })
        },
        _ => unreachable!() // Ok(()),
    }
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
