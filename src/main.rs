use anyhow::Result;
use crossterm::{
    cursor::{self, MoveTo},
    event::{self, Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Print,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, size, Clear, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ropey::Rope;
use std::{
    char,
    fs::File,
    io::{self, stdout, Read, Write},
};
pub mod commands;
pub mod custom_event;
pub mod keymap;
struct EditorConfig {
    screen_rows: u16,
    screen_cols: u16,
    // x: u16,
    // y: u16,
    /// Save the `document`
    /// Able to get the content by line number
    text: Rope,
}

// NOTE: 使用了Rust 中的 newType 模式来绕过孤儿规则
// struct MyEvent(crossterm::event::KeyEvent);

impl EditorConfig {
    fn new() -> EditorConfig {
        let text = Rope::from_str("");
        EditorConfig {
            screen_rows: 0,
            screen_cols: 0,
            text,
        }
    }

    fn editor_open(&mut self) -> Result<()> {
        // let content = "Hello world!";
        // self.text.insert(0, content);
        // TODO: Cannot recongise `~`. Need to parse manually
        let mut file = File::open("/Users/crowds/Scripts/orgmode/index.norg")?;
        // 创建一个字符串缓冲区来存储文件内容
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // 将字符串转换为 Rope
        self.text = Rope::from_str(&contents);
        // self.text.len_lines();

        Ok(())
    }

    /// Enters editor mode, usually involving disabling line buffering
    /// and echo
    fn enter_editor_mode(&self) -> io::Result<()> {
        execute!(stdout(), EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        Ok(())
    }

    /// Leave editor mode,reset the terminal
    fn leave_editor_mode(&self) {
        let _ = stdout().execute(LeaveAlternateScreen);
        let _ = disable_raw_mode();
        std::process::exit(0)
    }

    /// Get and save windows size
    fn get_window_size(&mut self) {
        match crossterm::terminal::size() {
            Ok((cols, rows)) => {
                self.screen_rows = rows;
                self.screen_cols = cols;
            }
            Err(e) => {
                eprintln!("Failed to get windows size: {}", e);
            }
        }
        // let (cols, rows) = crossterm::terminal::size().unwrap();
    }

    /// Draw `~` characters in each empty line
    fn editor_draw_rows(&self) -> io::Result<()> {
        // TODO: 不输出超出屏幕的部分
        // let lines = self.text.len_lines();
        let mut line_iter = self.text.lines();
        for l in 0..self.screen_cols {
            if let Some(line) = line_iter.next() {
                execute!(io::stdout(), MoveTo(0, l))?;
                execute!(io::stdout(), Print(line.to_string()))?;
            }
        }
        // for l in 0..self.screen_rows {
        //     execute!(io::stdout(), MoveTo(0, l as u16))?;
        //     execute!(io::stdout(), Print("~"))?;
        //     if l < self.screen_rows - 1 {
        //         execute!(io::stdout(), Print("\r\n"))?;
        //     }
        // }
        // 刷新屏幕确保内容被正确打印
        stdout().flush()?;
        Ok(())
    }

    fn editor_refresh_screen(&self) -> io::Result<()> {
        execute!(io::stdout(), Clear(crossterm::terminal::ClearType::All))?;
        execute!(io::stdout(), MoveTo(0, 0))?;
        self.editor_draw_rows()?;
        // 曲线绘制完成后,重新定位到开头位置
        execute!(io::stdout(), MoveTo(0, 0))?;

        Ok(())
    }

    fn get_cursor_position(&self) -> Result<(u16, u16)> {
        let (x, y) = cursor::position()?;
        Ok((x, y))
    }
}

/// enter_editor_mode: Init Terminal
/// get_windwos_size
/// editor_open
fn main() -> io::Result<()> {
    let mut editor_config = EditorConfig::new();
    editor_config.enter_editor_mode()?;
    editor_config.get_window_size();
    editor_config.editor_open().unwrap();
    // execute!(cursor::Hide).unwrap();
    // 这里可以添加一些屏幕绘制的操作
    // execute!(cursor::Show).unwrap();
    editor_config.editor_refresh_screen().unwrap();
    // let mut buffer = [0; 1]; // 单字节的缓冲区
    // let stdin = io::stdin();
    // let mut handle = stdin.lock(); // 锁定标准输入以提高效率
    loop {
        if let event::Event::Key(key_event) = event::read()? {
            editor_key_event(key_event);
        }
        // let _ = editor_refresh_screen();
        // match editor_process_keypress() {
        //     Ok(result) => {}
        //     Err(e) => {}
        // }
    }
    // Ok(())
}

/// Reset the terminal before exit
fn reset_terminal_and_exit() {
    let _ = stdout().execute(LeaveAlternateScreen);
    let _ = disable_raw_mode();
    std::process::exit(0)
}

/// Crossterm handle keyboard event
fn editor_key_event(key_event: KeyEvent) {
    match (key_event.code, key_event.modifiers) {
        // Exit directly
        (KeyCode::Char('q'), KeyModifiers::NONE) => {
            reset_terminal_and_exit();
        }
        // Print Char
        (KeyCode::Char(c), KeyModifiers::NONE) => {
            // 这里的 `c` 表示 传入的 key_event 中的 KeyCode 是一个 Char
            // 这个 Char 用 c 表示
            execute!(io::stdout(), Print(c as char)).unwrap();
        }
        // Ctrl
        (KeyCode::Char('c'), KeyModifiers::CONTROL) => {}

        (_, KeyModifiers::CONTROL) => {}
        (KeyCode::Backspace, _) => {
            // execute!(io::stdout(),)
            // cursor::MoveToNextLine
        }
        _ => {}
    }
}

fn append_content() {
    // let
}
