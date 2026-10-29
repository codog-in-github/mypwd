mod cli;
mod hashing;
mod salt;

use clap::Parser;
use std::io;

use cli::Args;
use hashing::{ensure_character_mix, md5_to_charset, md5_with_salt};
use salt::{clear_salt_history, init_salt};

fn main() -> io::Result<()> {
    let args = Args::parse();
    if args.history_clear {
        return clear_salt_history();
    }
    let salt = init_salt(args.init.as_deref(), args.force)?;
    if let Some(text) = args.text {
        let digest = md5_with_salt(&text, &salt);
        let password = ensure_character_mix(md5_to_charset(digest), digest);
        println!("{password}");
    }

    Ok(())
}
