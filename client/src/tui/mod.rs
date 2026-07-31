use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame, buffer::Buffer, layout::{
        Constraint::{self, Percentage},
        Flex, Layout, Rect,
    }, symbols::block, text::Line, widgets::{Block, List, Paragraph, Widget, Wrap},
};

use crate::tui::Screen::{Chat, Login};

#[derive(Debug)]
enum Screen {
    Login,
    Chat
}

#[derive(Debug)]
pub struct App {
    exit: bool,
    input: String,
    output: String,
    screen: Screen
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            input: String::new(),
            output: String::new(),
            screen: Login
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().await?
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    async fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event).await
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Char(c) => self.input.push(c),
            KeyCode::Backspace => {
                self.input.pop();
            },
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn render_login(&self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .title("Login")
            .render(area, buf);
        
        let [display_area, input_area] = 
            Layout::vertical([
                Constraint::Percentage(70),
                Constraint::Percentage(30)
            ])
            .areas(area);

        Paragraph::new("Type username to login")
                .block(Block::bordered())
                .render(display_area, buf);

        Paragraph::new(self.input.as_str())
                .block(Block::bordered())
                .render(input_area, buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .render(area, buf);
        match self.screen {
            Login => self.render_login(area, buf),
            Chat => todo!()
        }
    }
}
