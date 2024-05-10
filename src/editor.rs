use std::{
    fmt::format,
    io::{stdout, Write},
};

use anyhow::Ok;
use crossterm::{
    cursor,
    event::{self, read},
    style::{self},
    terminal, ExecutableCommand, QueueableCommand,
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

#[derive(Debug)]
enum Mode {
    Normal,
    Insert,
}

pub struct Editor {
    size: (u16, u16),
    cx: u16,
    cy: u16,
    mode: Mode,
    stdout: std::io::Stdout,
}

impl Drop for Editor {
    fn drop(&mut self) {
        _ = self.stdout.flush();
        _ = self.stdout.execute(terminal::LeaveAlternateScreen);
        _ = terminal::disable_raw_mode();
    }
}

impl Editor {
    pub fn new() -> anyhow::Result<Self> {
        let mut stdout = stdout();
        terminal::enable_raw_mode()?;
        stdout
            .execute(terminal::EnterAlternateScreen)?
            .execute(terminal::Clear(terminal::ClearType::All))?;
        Ok(Editor {
            size: terminal::size()?,
            cx: 0,
            cy: 0,
            mode: Mode::Normal,
            stdout,
        })
    }

    pub fn draw(&mut self) -> anyhow::Result<()> {
        self.draw_statusline()?;
        self.stdout.queue(cursor::MoveTo(self.cx, self.cy))?;
        self.stdout.flush()?;

        Ok(())
    }

    pub fn draw_statusline(&mut self) -> anyhow::Result<()> {
        self.stdout.queue(cursor::MoveTo(0, self.size.1 - 2))?;
        self.stdout
            .queue(style::Print(format!("{:?}", self.mode)))?;

        Ok(())
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            self.draw()?;

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
                        self.stdout.queue(cursor::MoveTo(self.cx, self.cy))?;
                        self.stdout.queue(style::Print(c))?;
                        self.cx += 1;
                    }
                    Actions::NewLine => {
                        self.cx = 0;
                        self.cy += 1;
                    }
                }
            }
        }

        Ok(())
    }
    fn handle_event(&mut self, ev: event::Event) -> anyhow::Result<Option<Actions>> {
        match self.mode {
            Mode::Insert => self.handle_insert_event(ev),
            Mode::Normal => self.handle_normal_event(ev),
        }
    }
    fn handle_normal_event(&self, ev: event::Event) -> anyhow::Result<Option<Actions>> {
        let action = match ev {
            event::Event::Key(event) => match event.code {
                event::KeyCode::Char('q') => Some(Actions::Quit),
                event::KeyCode::Up | event::KeyCode::Char('k') => Some(Actions::MoveUp),
                event::KeyCode::Down | event::KeyCode::Char('j') => Some(Actions::MoveDown),
                event::KeyCode::Left | event::KeyCode::Char('h') => Some(Actions::MoveLeft),
                event::KeyCode::Right | event::KeyCode::Char('l') => Some(Actions::MoveRight),
                event::KeyCode::Char('i') => Some(Actions::EnterMode(Mode::Insert)),

                _ => None,
            },

            _ => None,
        };

        Ok(action)
    }
    fn handle_insert_event(&self, ev: event::Event) -> anyhow::Result<Option<Actions>> {
        let action = match ev {
            event::Event::Key(event) => match event.code {
                event::KeyCode::Esc => Some(Actions::EnterMode(Mode::Normal)),
                event::KeyCode::Enter => Some(Actions::NewLine),
                event::KeyCode::Char(c) => Some(Actions::AddChar(c)),
                _ => None,
            },
            _ => None,
        };

        Ok(action)
    }
}
