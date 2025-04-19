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
    /// 互い違いのパターンでbabelを表示するコマンド
    BabelSimple,
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
        Some(Commands::BabelSimple) => {
            if let Err(e) = display_babel_simple() {
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

    // ターミナルサイズを取得
    let (width, height) = terminal::size()?;

    // 画面をクリア
    stdout.execute(Clear(ClearType::All))?;

    // メインループ
    let mut should_quit = false;
    let mut frame = 0;

    // 全画面にbabelを配置
    let mut screen_lines = vec![Vec::new(); height as usize];
    for y in 0..height {
        for x in 0..width.saturating_sub(5) {
            if (x + frame) % 6 == 0 {
                screen_lines[y as usize].push((x, "babel"));
            }
        }
    }

    // ダブルバッファリング用の文字列を用意
    let mut screen_buffer = String::with_capacity((width * height * 6) as usize);

    while !should_quit {
        // バッファをクリア
        screen_buffer.clear();

        // 全画面の内容をバッファに一括で追加
        for y in 0..height {
            // 行の先頭に移動する制御シーケンス
            screen_buffer.push_str(&format!("\x1b[{};0H", y + 1));
            // 行をクリアする制御シーケンス
            screen_buffer.push_str("\x1b[2K");

            for &(x, text) in &screen_lines[y as usize] {
                // 位置移動の制御シーケンス
                screen_buffer.push_str(&format!("\x1b[{};{}H", y + 1, x + 1));
                // 赤色の制御シーケンス
                screen_buffer.push_str("\x1b[31m");
                // テキスト
                screen_buffer.push_str(text);
                // 色リセットの制御シーケンス
                screen_buffer.push_str("\x1b[0m");
            }
        }

        // バッファを一度に出力
        stdout.write_all(screen_buffer.as_bytes())?;
        stdout.flush()?;

        // 1行ずつ上にシフト
        let last_line = screen_lines.remove(0);
        screen_lines.push(last_line);

        // 最下段の行は新しいフレームに基づいて更新
        screen_lines[height as usize - 1].clear();
        for x in 0..width.saturating_sub(5) {
            if (x + frame) % 6 == 0 {
                screen_lines[height as usize - 1].push((x, "babel"));
            }
        }

        // ウェイト
        thread::sleep(Duration::from_millis(150));

        // フレームを進める
        frame = (frame + 1) % 6;

        // キー入力をチェック（ノンブロッキング）
        if event::poll(Duration::from_millis(50))? {
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

fn display_babel_simple() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    println!("「q」キーを押すと終了します...");
    thread::sleep(Duration::from_secs(2));

    let mut should_quit = false;
    let mut offset = 0;

    // ターミナルサイズを取得
    let (width, height) = terminal::size()?;

    // 画面をクリア
    stdout.execute(Clear(ClearType::All))?;

    // 全画面のbabelパターンを作成（互い違いのパターンで）
    let mut screen_lines = vec![Vec::new(); height as usize];
    for y in 0..height {
        let row_offset = (y + offset) % 2;
        for x in 0..(width / 6) {
            let x_pos = x * 6 + row_offset * 3;
            if x_pos < width.saturating_sub(5) {
                screen_lines[y as usize].push(x_pos);
            }
        }
    }

    // ダブルバッファリング用の文字列を用意
    let mut screen_buffer = String::with_capacity((width * height * 6) as usize);

    while !should_quit {
        // バッファをクリア
        screen_buffer.clear();

        // 全画面の内容をバッファに一括で追加
        for y in 0..height {
            // 行の先頭に移動する制御シーケンス
            screen_buffer.push_str(&format!("\x1b[{};0H", y + 1));
            // 行をクリアする制御シーケンス
            screen_buffer.push_str("\x1b[2K");

            for &x_pos in &screen_lines[y as usize] {
                // 位置移動の制御シーケンス
                screen_buffer.push_str(&format!("\x1b[{};{}H", y + 1, x_pos + 1));
                // 赤色の制御シーケンス
                screen_buffer.push_str("\x1b[31m");
                // テキスト
                screen_buffer.push_str("babel");
                // 色リセットの制御シーケンス
                screen_buffer.push_str("\x1b[0m");
            }
        }

        // バッファを一度に出力
        stdout.write_all(screen_buffer.as_bytes())?;
        stdout.flush()?;

        // 1行ずつ上にシフト
        let last_line = screen_lines.remove(0);
        screen_lines.push(Vec::new()); // 新しい空の行を追加

        // 最下段の行を新しいパターンで更新
        let bottom_y = height - 1;
        let row_offset = (bottom_y + offset) % 2;
        for x in 0..(width / 6) {
            let x_pos = x * 6 + row_offset * 3;
            if x_pos < width.saturating_sub(5) {
                screen_lines[height as usize - 1].push(x_pos);
            }
        }

        // ウェイト
        thread::sleep(Duration::from_millis(150));

        // 一定間隔でパターンを切り替え
        offset = (offset + 1) % 2;

        // キー入力をチェック（ノンブロッキング）
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    should_quit = true;
                }
            }
        }
    }

    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
