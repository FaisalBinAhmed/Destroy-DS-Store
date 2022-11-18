use clap::{crate_version, App, Arg};

pub fn command_line_app() -> App<'static, 'static> {
    App::new("destroy_ds_store")
        .about("Delete the annoying .DS_Store files in MacOS")
        .version(crate_version!()) //this macro returns the version number from cargo.toml
        .arg(
            Arg::with_name("dir")
                .help("specify directory name")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("yesToAll")
                .short("y")
                .help("delete all found files without asking"),
        )
}
