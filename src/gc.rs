//! # gc
//!
//! The `gc` subcommand


/// Arguments for the sliding window family of subcommands.
/// This allows us to use the same config for GC and CRI windows.
pub fn cli_sub_sliding(name: &'static str, about: &'static str) -> CliApp {
    SubCommand::with_name(name)
        .about(about)
        .arg(
            Arg::with_name("fasta")
                .help("The reference fasta to calculate windows over. \
                       Use '-' for stdin.")
                .index(1)
                .required(true)
                .validator(is_file),
        )
        .arg(
            Arg::with_name("window")
                .short("w")
                .long("size")
                .help("The size of the window")
                .default_value("5000")
                .takes_value(true)
                .validator(is_usize),
        )
        .arg(
            Arg::with_name("step")
                .short("s")
                .long("step")
                .help("The step")
                .default_value("1000")
                .takes_value(true)
                .validator(is_usize),
        )
        .arg(
            Arg::with_name("outfile")
                .short("o")
                .long("outfile")
                .help("Where to write output to. Use '-' for stdout (default).")
                .default_value("-")
                .takes_value(true)
                .validator(is_file),
        )
}
