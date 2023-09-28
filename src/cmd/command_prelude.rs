//! Helpers for building the command-line arguments for commands.

pub use clap::{arg, Arg, ArgMatches, Command};
use clap::builder::NonEmptyStringValueParser;
use std::path::PathBuf;

pub trait CommandExt: Sized {
    fn _arg(self, arg: Arg) -> Self;

    fn arg_dest_dir(self) -> Self {
        self._arg(
            Arg::new("dest-dir")
                .short('d')
                .long("dest-dir")
                .value_name("dest-dir")
                .value_parser(clap::value_parser!(PathBuf))
                .help(
                    "Output directory for the book\n\
                    Relative paths are interpreted relative to the book's root directory.\n\
                    If omitted, mdBook uses build.build-dir from book.toml \
                    or defaults to `./book`.",
                ),
        )
    }

    fn arg_language(self) -> Self {
        self._arg(
            Arg::new("language")
                .short('l')
                .long("language")
                .value_name("language")
                .value_parser(NonEmptyStringValueParser::new())
                .help(
                    "Only valid if the [language] table in the config is not empty.\n\
                    If omitted, builds all translations and provides a menu in the generated output for switching between them."
                )
        )
    }

    fn arg_root_dir(self) -> Self {
        self._arg(
            Arg::new("dir")
                .help(
                    "Root directory for the book\n\
                    (Defaults to the current directory when omitted)",
                )
                .value_parser(clap::value_parser!(PathBuf)),
        )
    }

    fn arg_open(self) -> Self {
        self._arg(arg!(-o --open "Opens the compiled book in a web browser"))
    }
}

impl CommandExt for Command {
    fn _arg(self, arg: Arg) -> Self {
        self.arg(arg)
    }
}
