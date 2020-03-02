extern crate clap;

use clap::{Arg, App, SubCommand, AppSettings};

fn main() {

    let matches = App::new("trishaw")
        .about("Processes spec source files into valid HTML.")
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(Arg::with_name("quiet")
            .short("q")
            .long("quiet")
            .multiple(true)
            .help("Silences one level of message, least-important first."))
        .arg(Arg::with_name("slient")
            .short("s")
            .long("slient")
            .help("Shorthand for 'as many -q as you need to shut it up'"))
        .arg(Arg::with_name("force")
            .short("f")
            .long("force")
            .help("Force the preprocessor to run to completion; fatal errors don't stop processing."))
        .arg(Arg::with_name("dry-run")
            .short("d")
            .long("dry-run")
            .help("Prevents the processor from actually saving anything to disk, but otherwise fully runs."))
        .arg(Arg::with_name("print")
            .long("print")
            .value_name("PRINTMODE")
            .takes_value(true)
            .help("Print mode. Options are 'plain' (just text), 'console' (colored with console color codes), 'markup', and 'json'. Defaults to 'console'."))
        .arg(Arg::with_name("die-on")
            .long("die-on")
            .help("Determines what sorts of errors cause Bikeshed to die (quit immediately with an error status code). Default is 'fatal'; the -f flag is a shorthand for 'nothing'"))
        .subcommand(SubCommand::with_name("spec")
            .about("Process a spec source file into a valid output file.")
            .arg(Arg::with_name("infile")
                .required(true)
                .index(1)
                .takes_value(true)
                .help("Path to the source file."))
            .arg(Arg::with_name("outfile")
                .required(true)
                .index(2)
                .takes_value(true)
                .help("Path to the output file."))
            .arg(Arg::with_name("debug")
                .long("debug")
                .help("Switches on some debugging tools. Don't use for production!"))
            .arg(Arg::with_name("gh-token")
                .long("gh-token")
                .value_name("GHTOKEN")
                .takes_value(true)
                .help("Switches on some debugging tools. Don't use for production!"))
            .arg(Arg::with_name("byos")
                .long("byos")
                .help("Bring-Your-Own-Spec: turns off all the Bikeshed auto-niceties, so you can piecemeal its features into your existing doc instead. Experimental, let me know if things get crashy or weird."))
            .arg(Arg::with_name("line-numbers")
                .short("l")
                .long("line-numbers")
                .help("Hacky support for outputting line numbers on all error messages. Disables output, as this is hacky and might mess up your source.")))
        // TODO: add argument for each subcommand
        .subcommand(SubCommand::with_name("echidna")
            .about("Process a spec source file into a valid output file and publish it according to certain automatic protocols."))
        .subcommand(SubCommand::with_name("watch")
            .about("Process a spec source file into a valid output file, automatically rebuilding when it changes."))
        .subcommand(SubCommand::with_name("serve")
            .about("Identical to 'watch', but also serves the folder on localhost."))
        .subcommand(SubCommand::with_name("update")
            .about("Update supporting files (those in /spec-data)."))
        .subcommand(SubCommand::with_name("issues-list")
            .about("Process a plain-text issues file into HTML. Call with no args to see an example input text."))
        .subcommand(SubCommand::with_name("debug")
            .about("Run various debugging commands."))
        .subcommand(SubCommand::with_name("refs")
            .about("Search Bikeshed's ref database."))
        .subcommand(SubCommand::with_name("source")
            .about("Tools for formatting the *source* document."))
        .subcommand(SubCommand::with_name("test")
            .about("Tools for running Bikeshed's testsuite."))
        .subcommand(SubCommand::with_name("profile")
            .about("Profiling Bikeshed. Needs graphviz, gprof2dot, and xdot installed."))
        .subcommand(SubCommand::with_name("template")
            .about("Outputs a skeleton .bs file for you to start with."))
        .subcommand(SubCommand::with_name("wpt")
            .about("Tools for writing Web Platform Tests."))
        .get_matches();

    match matches {
        _ => {},
    };
}
