use std::{
    char,
    io::{self, stdout, Read},
};

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen,
    },
    ExecutableCommand,
};

fn main() -> io::Result<()> {
    // Enable raw mode
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    // let mut buffer = [0; 1]; // 单字节的缓冲区
    // let stdin = io::stdin();
    // let mut handle = stdin.lock(); // 锁定标准输入以提高效率
    loop {
        let _ = editor_refresh_screen();
        match editor_process_keypress() {
            Ok(result) => {}
            Err(e) => {}
        }
    }
    // Ok(())
}

/// Reset the terminal before exit
fn reset_terminal_before_exit() {
    let _ = stdout().execute(LeaveAlternateScreen);
    let _ = disable_raw_mode();
}

/// Deals with low-level terminal input.
fn editor_read_key() -> Result<char, io::Error> {
    let mut buffer = [0; 1];
    let nread = io::stdin().read(&mut buffer)?;
    if nread != 1 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to read a character",
        ));
    }
    Ok(buffer[0] as char)
}

/// Waits for a keypress,and then handle it.
/// Deals with mapping keys to editor functions as a higher level
fn editor_process_keypress() -> io::Result<()> {
    match editor_read_key() {
        Ok(c) => match c {
            'q' => {
                reset_terminal_before_exit();
                std::process::exit(0)
            }

            _ => Ok(()),
        },
        Err(e) => {
            eprint!("Error: { }", e);
            Err(e)
        }
    }
}

fn editor_refresh_screen() -> io::Result<()> {
    execute!(io::stdout(), Clear(crossterm::terminal::ClearType::All))?;
    execute!(io::stdout(), MoveTo(0, 0))?;
    Ok(())
}
