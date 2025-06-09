// babel.rs
use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event, KeyCode},
    terminal::{self, Clear, ClearType},
};
use std::io::{Write, stdout};
use std::{thread, time::Duration};

pub fn display_babel_stream() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    // 説明を表示して一時停止
    stdout.execute(Clear(ClearType::All))?;
    println!("「q」キーを押すと終了します...");
    stdout.flush()?;
    thread::sleep(Duration::from_secs(2));
    stdout.execute(Clear(ClearType::All))?;

    let mut should_quit = false;
    let mut offset = 0;
    let (width, height) = terminal::size()?;
    // ここでscreen_linesを1行多く用意し、最初のpopで説明行を消す
    let mut screen_lines = vec![Vec::new(); (height + 1) as usize];
    for y in 0..(height + 1) {
        let row_offset = y % 2;
        for x in 0..(width / 6) {
            let x_pos = x * 6 + row_offset;
            if x_pos < width.saturating_sub(5) {
                screen_lines[y as usize].push(x_pos);
            }
        }
    }
    let mut screen_buffer = String::with_capacity((width * height * 6) as usize);
    while !should_quit {
        screen_buffer.clear();
        for y in 0..height {
            screen_buffer.push_str(&format!("\x1b[{};0H", y + 1));
            screen_buffer.push_str("\x1b[2K");
            for &x_pos in &screen_lines[(y + 1) as usize] {
                // 1行ずらして参照
                screen_buffer.push_str(&format!("\x1b[{};{}H", y + 1, x_pos + 1));
                screen_buffer.push_str("\x1b[31m");
                screen_buffer.push_str("babel");
                screen_buffer.push_str("\x1b[0m");
            }
        }
        stdout.write_all(screen_buffer.as_bytes())?;
        stdout.flush()?;
        let _last_line = screen_lines.remove(0);
        // 新しい行を生成
        let bottom_y = height - 1;
        let row_offset = (bottom_y + offset) % 2;
        let mut new_line = Vec::new();
        for x in 0..(width / 6) {
            let x_pos = x * 6 + row_offset;
            if x_pos < width.saturating_sub(5) {
                new_line.push(x_pos);
            }
        }
        screen_lines.push(new_line);
        thread::sleep(Duration::from_millis(150));
        offset = (offset + 1) % 2;
        if event::poll(Duration::from_millis(50))?
            && let Event::Key(key) = event::read()?
            && key.code == KeyCode::Char('q')
        {
            should_quit = true;
        }
    }
    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
