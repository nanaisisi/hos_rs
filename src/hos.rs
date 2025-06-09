// hos.rs
pub fn display_os_logo() {
    use crossterm::{ExecutableCommand, cursor, terminal};
    use std::fs;
    use std::io::{self, Write};
    use std::path::Path;
    use std::process::Command;
    use std::thread;
    use std::time::Duration;

    let frame_dir = Path::new("frames");
    let Ok(entries) = fs::read_dir(frame_dir) else {
        println!("framesディレクトリがありません。画像を配置してください。");
        return;
    };
    let mut frames: Vec<_> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.extension()
                .map(|ext| ext == "png" || ext == "jpg" || ext == "svg")
                .unwrap_or(false)
        })
        .collect();
    frames.sort();

    if frames.is_empty() {
        println!("framesディレクトリに画像がありません。");
        return;
    }

    let mut stdout = io::stdout();
    let (width, height) = terminal::size().unwrap_or((80, 24));
    stdout.execute(terminal::EnterAlternateScreen).ok();
    stdout.execute(cursor::Hide).ok();
    // 説明を表示して一時停止
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .ok();
    println!("「q」キーを押すと終了します...");
    stdout.flush().ok();
    thread::sleep(Duration::from_secs(2));
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .ok();
    let mut should_quit = false;
    while !should_quit {
        for frame_path in frames.iter() {
            // 画面クリア
            print!("\x1b[2J\x1b[H");
            stdout.flush().ok();
            // chafaで画像をANSIアートに変換して表示
            let output = Command::new("chafa")
                .arg(frame_path)
                .arg(format!("--size={width}x{height}"))
                .arg("--symbols=block")
                .output();
            if let Ok(output) = output
                && output.status.success()
            {
                let _ = stdout.write_all(&output.stdout);
            }
            stdout.flush().ok();
            // qキーで終了
            if crossterm::event::poll(Duration::from_millis(100)).unwrap_or(false)
                && let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read()
                && let crossterm::event::KeyCode::Char('q') = key.code
            {
                should_quit = true;
                break;
            }
        }
    }
    stdout.execute(cursor::Show).ok();
    stdout.execute(terminal::LeaveAlternateScreen).ok();
}
