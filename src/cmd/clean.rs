use crate::{get_book_dir, get_build_opts};
use super::command_prelude::*;
use anyhow::Context;
use mdbook_spacewizards::MDBook;
use std::fs;
use std::path::PathBuf;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("clean")
        .about("Deletes a built book")
        .arg_dest_dir()
        .arg_root_dir()
        .arg_language()
}

// Clean command implementation
pub fn execute(args: &ArgMatches) -> mdbook_spacewizards::errors::Result<()> {
    let book_dir = get_book_dir(args);
    let build_opts = get_build_opts(args);
    let book = MDBook::load_with_build_opts(&book_dir, build_opts)?;

    let dir_to_remove = match args.get_one::<PathBuf>("dest-dir") {
        Some(dest_dir) => dest_dir.into(),
        None => book.root.join(&book.config.build.build_dir),
    };

    if dir_to_remove.exists() {
        fs::remove_dir_all(&dir_to_remove)
            .with_context(|| "Unable to remove the build directory")?;
    }

    Ok(())
}
