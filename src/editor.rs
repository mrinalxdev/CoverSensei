use std::io::{stdout, Write};

use anyhow::Ok;
use crossterm::{
    cursor,
    event::{self, read},
    style::{self, style}, terminal, ExecutableCommand, QueueableCommand,
};

enum Actions {
    Quit,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,

    AddChar(char),
    NewLine,

    EnterMode(Mode),
}

enum Mode {
    Normal,
    Insert,
}

pub struct Editor {
    size : (u16, u16),
    cx: u16,
    cy: u16,
    mode: Mode,
}

impl Editor {
    pub fn new() -> Self {
        let mut stdout = stdout();
        Editor {
            size: terminal::size()?,
            cx: 0,
            cy: 0,
            mode: Mode::Normal,
        }
    }

    pub fn draw(&self, stdout: &mut std::io::Stdout) -> anyhow::Result<()> {
        self.draw_statusline(stdout);
        stdout.queue(cursor::MoveTo(self.cx, self.cy))?;
        stdout.flush()?;

        Ok(())
    }

    pub fn draw_statusline(&self, stdout: &mut std::io::Stdout) -> anyhow::Result<()> {
        stdout.queue(cursor::MoveTo(0, self.size.1 - 2))?;
        stdout.queue(style::Print("Status Line"))?;

        Ok(())
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        let mut stdout = stdout();
        terminal::enable_raw_mode()?;
        stdout
            .execute(terminal::EnterAlternateScreen)?
            .execute(terminal::Clear(terminal::ClearType::All))?;


        loop {
           self.draw(&mut stdout)?;

            if let Some(action) = self.handle_event(read()?)? {
                match action {
                    Actions::Quit => break,
                    Actions::MoveUp => {
                        self.cy = self.cy.saturating_sub(1);
                    }
                    Actions::MoveDown => {
                        self.cy += 1u16;
                    }
                    Actions::MoveLeft => {
                        self.cx = self.cx.saturating_sub(1);
                    }
                    Actions::MoveRight => {
                        self.cx += 1u16;
                    }
                    Actions::EnterMode(new_mode) => {
                        self.mode = new_mode;
                    }
                    Actions::AddChar(c) => {
                        stdout.queue(cursor::MoveTo(self.cx, self.cy))?;
                        stdout.queue(style::Print(c))?;
                        self.cx += 1;
                    }
                    Actions::NewLine => {
                        self.cx = 0;
                        self.cy += 1;
                    }
                }
            }
        }

        stdout.execute(terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        Ok(())
    }
}

fn handle_event(
    &mut self, ev: event::Event
) -> anyhow::Result<Option<Actions>> {
    match self.mode {
        Mode::Insert => self.handle_insert_event(stdout, ev),
        Mode::Normal => self.handle_normal_event(ev),
    }
}

fn handle_normal_event(&self, ev: event::Event) -> anyhow::Result<Option<Actions>> {
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
fn handle_insert_event(
    stdout: &mut std::io::Stdout,
    ev: event::Event,
) -> anyhow::Result<Option<Actions>> {
    match ev {
        event::Event::Key(event) => match event.code {
            event::KeyCode::Esc => Ok(Some(Actions::EnterMode(Mode::Normal))),
            event::KeyCode::Char(c) => {
                stdout.queue(style::Print(c))?;
                // cx += 1;
                Ok(None)
            }
            _ => Ok(None),
        },
        event::Event::FocusGained => todo!(),
        event::Event::FocusLost => todo!(),
        event::Event::Mouse(_) => todo!(),
        event::Event::Paste(_) => todo!(),
        event::Event::Resize(_, _) => todo!(),
    }
}
