use clap::{Parser, Subcommand};
use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event, KeyCode},
    terminal::{self, Clear, ClearType},
};
use std::io::{Write, stdout};
use std::process;
use std::{thread, time::Duration};

mod babel;
mod hos;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// OSのロゴを表示するコマンド
    Hos,
    /// babelという文字を流し続けるコマンド
    Babel,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Hos) => {
            hos::display_os_logo();
        }
        Some(Commands::Babel) => {
            if let Err(e) = babel::display_babel_stream() {
                eprintln!("エラーが発生しました: {:?}", e);
            }
        }
        _none => {
            println!("コマンドが指定されていません。--help でヘルプを表示します。");
            process::exit(1);
        }
    }
}
