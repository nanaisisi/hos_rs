use clap::{Parser, Subcommand};
use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event, KeyCode},
    style::{self, Color},
    terminal::{self, Clear, ClearType},
};
use std::io::{Write, stdout};
use std::process;
use std::{thread, time::Duration};

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
            display_os_logo();
        }
        Some(Commands::Babel) => {
            if let Err(e) = display_babel_stream() {
                eprintln!("エラーが発生しました: {:?}", e);
            }
        }
        _none => {
            println!("コマンドが指定されていません。--help でヘルプを表示します。");
            process::exit(1);
        }
    }
}

fn display_os_logo() {
    // Windows用のロゴ (実際の環境に応じて表示を変更できます)
    println!(
        "
    ██╗    ██╗██╗███╗   ██╗██████╗  ██████╗ ██╗    ██╗███████╗
    ██║    ██║██║████╗  ██║██╔══██╗██╔═══██╗██║    ██║██╔════╝
    ██║ █╗ ██║██║██╔██╗ ██║██║  ██║██║   ██║██║ █╗ ██║███████╗
    ██║███╗██║██║██║╚██╗██║██║  ██║██║   ██║██║███╗██║╚════██║
    ╚███╔███╔╝██║██║ ╚████║██████╔╝╚██████╔╝╚███╔███╔╝███████║
     ╚══╝╚══╝ ╚═╝╚═╝  ╚═══╝╚═════╝  ╚═════╝  ╚══╝╚══╝ ╚══════╝
                                                              
    "
    );

    println!("OS: Windows");
    println!("バージョン: 10/11");
    println!("アーキテクチャ: x86_64");
}

fn display_babel_stream() -> std::io::Result<()> {
    // ターミナルを代替スクリーンバッファモードに切り替え
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?; // カーソルを非表示に

    println!("「q」キーを押すと終了します...");
    thread::sleep(Duration::from_secs(2)); // 説明を表示する時間

    // メインループ
    let mut should_quit = false;
    while !should_quit {
        // ターミナルサイズを取得
        let (width, height) = terminal::size()?;

        // 画面をクリア
        stdout.execute(Clear(ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;

        // babelを画面全体に配置
        for y in 0..height {
            for x in 0..width.saturating_sub(5) {
                // "babel"の長さ分余裕を持たせる
                if x % 6 == 0 {
                    // カーソル位置を設定
                    stdout.execute(cursor::MoveTo(x, y))?;

                    // 赤色で"babel"を表示 - 修正部分
                    stdout.execute(style::SetForegroundColor(Color::Red))?;
                    stdout.write_all(b"babel")?;
                    stdout.execute(style::ResetColor)?;
                }
            }
        }

        stdout.flush()?;

        // キー入力をチェック（ノンブロッキング）
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    should_quit = true;
                }
            }
        }
    }

    // 終了処理：ターミナル設定を元に戻す
    stdout.execute(cursor::Show)?; // カーソルを再表示
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
