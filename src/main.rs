use clap::{Parser, Subcommand};
use std::process;
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
    /// OSのロゴを表示するコマンド（sixel/chafa）
    HosChafa,
    /// babelという文字を流し続けるコマンド
    Babel,
    /// OSのロゴをImageMagickでASCIIアート表示するコマンド
    HosMagick,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::HosChafa) => {
            hos::display_os_logo();
        }
        Some(Commands::Babel) => {
            if let Err(e) = babel::display_babel_stream() {
                eprintln!("エラーが発生しました: {:?}", e);
            }
        }
        Some(Commands::HosMagick) => {
            hos::display_os_logo_magick();
        }
        _none => {
            println!("コマンドが指定されていません。--help でヘルプを表示します。");
            process::exit(1);
        }
    }
}
