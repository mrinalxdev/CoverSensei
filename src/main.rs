use std::io::{stdout, Write};

use anyhow::Ok;
use crossterm::{
    cursor, event::{self, read}, style, terminal, ExecutableCommand, QueueableCommand
};

enum Actions {
    Quit,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,

    EnterMode(Mode),
}

enum Mode {
    Normal,
    Insert,
}

fn handle_event(mode: &Mode, stdout: &mut std::io::Stdout, ev: event::Event) -> anyhow::Result<Option<Actions>> {
    match mode {
        Mode::Insert => handle_insert_event(stdout, ev),
        Mode::Normal => handle_normal_event(ev),
    }
}

fn handle_normal_event(ev: event::Event) -> anyhow::Result<Option<Actions>> {
    match ev {
        event::Event::Key(event) => match event.code {
            event::KeyCode::Char('q') => Ok(Some(Actions::Quit)),
            event::KeyCode::Up | event::KeyCode::Char('i') => Ok(Some(Actions::MoveUp)),
            event::KeyCode::Down | event::KeyCode::Char('k') => Ok(Some(Actions::MoveDown)),
            event::KeyCode::Left | event::KeyCode::Char('j') => Ok(Some(Actions::MoveLeft)),
            event::KeyCode::Right | event::KeyCode::Char('l') => Ok(Some(Actions::MoveRight)),
            event::KeyCode::Char('u') => Ok(Some(Actions::EnterMode(Mode::Insert))),
            _ => Ok(None),
        },
        _ => Ok(None),
    }
}
fn handle_insert_event(stdout: &mut std::io::Stdout, ev: event::Event) -> anyhow::Result<Option<Actions>> {
    match ev {
        event::Event::Key(event) => match event.code {
            event::KeyCode::Esc => Ok(Some(Actions::EnterMode(Mode::Normal))),
            event::KeyCode::Char(c) => {
                stdout.queue(style::Print(c))?;
                // cx += 1;
                Ok(None)
            }
            _ => Ok(None),
        }
        event::Event::FocusGained => todo!(),
        event::Event::FocusLost => todo!(),
        event::Event::Mouse(_) => todo!(),
        event::Event::Paste(_) => todo!(),
        event::Event::Resize(_, _) => todo!(),
    }
}

fn main() -> anyhow::Result<()> {
    let mut stdout = stdout();
    let mut mode = Mode::Normal;
    let mut cx = 0;
    let mut cy = 0;

    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    loop {
        stdout.queue(cursor::MoveTo(cx, cy))?;
        stdout.flush()?;

        if let Some(action) = handle_event(&mode, &mut stdout, read()?)? {
            match action {
                Actions::Quit => break,
                Actions::MoveUp => {
                    cy = cy.saturating_sub(1);
                }
                Actions::MoveDown => {
                    cy += 1u16;
                }
                Actions::MoveLeft => {
                    cx = cx.saturating_sub(1);
                }
                Actions::MoveRight => {
                    cx += 1u16;
                }
                Actions::EnterMode(new_mode) => {
                    mode = new_mode;
                },
            }
        }
    }

    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
