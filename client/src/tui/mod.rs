use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use self::Screen::{Chat, Login, Users};
use crate::{
    api::models::{ChatMessage, User},
    client::ClientApp,
};

#[derive(Debug)]
enum Screen {
    Login,
    Chat,
    Users,
}

pub struct App {
    exit: bool,
    input: String,
    output: String,
    screen: Screen,
    client: ClientApp,
    chat: Vec<ChatMessage>,
    users: Option<Vec<User>>,
}

impl App {
    pub fn new(client: ClientApp) -> Self {
        Self {
            exit: false,
            input: String::new(),
            output: String::new(),
            screen: Login,
            client: client,
            chat: Vec::<ChatMessage>::with_capacity(10),
            users: None,
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
        match self.screen {
            Login => self.login_handle_key_event(key_event).await,
            Chat => todo!(),
            Users => self.users_handle_key_event(key_event),
        }
    }

    async fn login_handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Char(c) => self.input.push(c),
            KeyCode::Enter => {
                let username = self.input.clone();
                self.input = String::new();
                match self.client.login(&username).await {
                    Ok(()) => match self.client.get_users().await {
                        Ok(users) => {
                            let current_user_id = self.client.user.as_ref().map(|user| user.id);
                            self.users = Some(
                                users
                                    .into_iter()
                                    .filter(|user| Some(user.id) != current_user_id)
                                    .collect(),
                            );
                            self.screen = Users;
                        }
                        Err(error) => {
                            self.output = format!("Error: {}", error);
                        }
                    },
                    Err(error) => self.output = format!("Error, {}", error),
                }
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            _ => {}
        }
    }

    fn users_handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.code == KeyCode::Esc {
            self.exit();
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn render_login(&self, area: Rect, buf: &mut Buffer) {
        Block::bordered().title("Login").render(area, buf);

        let [display_area, input_area] =
            Layout::vertical([Constraint::Percentage(90), Constraint::Percentage(10)]).areas(area);

        let text = format!("Type username to login:\n\n{}", self.output.to_string());
        Paragraph::new(text)
            .block(Block::bordered())
            .render(display_area, buf);

        let block = Block::bordered();
        let inner_area = block.inner(input_area);
        block.render(input_area, buf);

        Paragraph::new(self.input.as_str())
            .wrap(Wrap { trim: false })
            .render(inner_area, buf)
    }

    fn render_chat(&self, area: Rect, buf: &mut Buffer) {
        Block::bordered().title("Users").render(area, buf);

        let [display_area, input_area] =
            Layout::vertical([Constraint::Percentage(90), Constraint::Percentage(10)]).areas(area);

        let text = format!("Welcome, {}", self.client.user.as_ref().unwrap().username);
        Paragraph::new(text)
            .block(Block::bordered())
            .render(display_area, buf);

        let mut constraints = Vec::<Constraint>::new();

        for i in 0..10 {
            constraints.push(Constraint::Length(3))
        }

        let areas = Layout::vertical(constraints).split(display_area);

        for area in areas.iter() {
            Paragraph::new("test")
                .block(Block::default().borders(Borders::BOTTOM))
                .render(*area, buf);
        }

        Paragraph::new(self.input.as_str())
            .block(Block::bordered())
            .render(input_area, buf);
    }

    fn render_users(&self, area: Rect, buf: &mut Buffer) {
        Block::bordered().title("Users").render(area, buf);

        let [display_area, input_area] =
            Layout::vertical([Constraint::Percentage(90), Constraint::Percentage(10)]).areas(area);

        if let Some(users) = self.users.as_ref() {
            if users.is_empty() {
                Paragraph::new("No other users found.")
                    .block(Block::bordered())
                    .render(display_area, buf);
            } else {
                let constraints: Vec<Constraint> =
                    users.iter().map(|_| Constraint::Length(3)).collect();

                let areas = Layout::vertical(constraints).split(display_area);

                for (user, area) in users.iter().zip(areas.iter()) {
                    Paragraph::new(user.username.as_str())
                        .block(Block::default().borders(Borders::BOTTOM))
                        .render(*area, buf);
                }
            }
        }

        Paragraph::new(self.input.as_str())
            .block(Block::bordered())
            .render(input_area, buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered().render(area, buf);
        match self.screen {
            Login => self.render_login(area, buf),
            Chat => self.render_chat(area, buf),
            Users => self.render_users(area, buf),
        }
    }
}
