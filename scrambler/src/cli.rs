use clap::{App, AppSettings, Arg};

pub fn build_cli() -> App<'static, 'static> {
    let table_arg = Arg::with_name("table-name")
        .long("table-name")
        .help("The name of the table to use")
        .global(true)
        .takes_value(true);

    App::new("scrambler")
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::VersionlessSubcommands)
        // .after_help(
        //     "To find more information about a subcommand run `scrambler SUBCOMMAND -h`.",
        // )
        .arg(table_arg)
}
