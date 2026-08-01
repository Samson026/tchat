use std::io::{self, Error};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Widget},
};

use self::Screen::{Chat, Login, Users};
use crate::{
    api::models::{ChatMessage, User},
    client::ClientApp,
};

#[derive(Debug)]
enum Screen {
    Login,
    #[allow(dead_code)]
    Chat,
    Users,
}

enum Command {
    Chat { User: String },
    Users,
}

pub struct App {
    exit: bool,
    input: String,
    output: String,
    screen: Screen,
    client: ClientApp,
    _chat: Vec<ChatMessage>,
    users: Option<Vec<User>>,
}

fn parse_cmd(cmd: &str) -> Result<Command, String> {
    let args = shell_words::split(cmd).map_err(|error| error.to_string())?;

    match args.as_slice() {
        [command, username] if command == "/chat" => Ok(Command::Chat {
            User: username.to_string(),
        }),
        [command] if command == "/users" => Ok(Command::Users),
        [] => Err("Enter a command".to_string()),
        [command, ..] => Err(format!("Unknown command: {command}")),
    }
}

impl App {
    pub fn new(client: ClientApp) -> Self {
        Self {
            exit: false,
            input: String::new(),
            output: String::new(),
            screen: Login,
            client,
            _chat: Vec::<ChatMessage>::with_capacity(10),
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
        match key_event.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Char(c) => self.input.push(c),
            KeyCode::Enter => match self.screen {
                Login => {
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
                Chat | Users => self.handle_command().await,
            },
            KeyCode::Backspace => {
                self.input.pop();
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    async fn handle_command(&mut self) {
        let cmd = parse_cmd(&self.input);

        match cmd {
            Ok(cmd) => match cmd {
                Command::Chat { User: username } => {
                    self.screen = Chat;
                }
                Command::Users => {
                    self.users = match self.client.get_users().await {
                        Ok(users) => Some(users),
                        Err(_) => None,
                    };
                    self.screen = Users;
                }
                _ => todo!(),
            },
            Err(error) => {
                self.output = error;
            }
        }
        self.input = String::new();
    }

    fn render_login(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title("Login");
        let inner = block.inner(area);
        block.render(area, buf);

        let text = format!("Type username to login:\n\n{}", self.output);
        Paragraph::new(text).render(inner, buf);
    }

    fn render_chat(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title("Chat");
        let inner = block.inner(area);
        block.render(area, buf);

        let [header_area, messages_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).areas(inner);

        let text = format!("Welcome, {}", self.client.user.as_ref().unwrap().username);
        Paragraph::new(text)
            .block(Block::bordered())
            .render(header_area, buf);

        let mut constraints = Vec::<Constraint>::new();

        for _ in 0..10 {
            constraints.push(Constraint::Length(3))
        }

        let areas = Layout::vertical(constraints).split(messages_area);

        for area in areas.iter() {
            Paragraph::new("test")
                .block(Block::default().borders(Borders::BOTTOM))
                .render(*area, buf);
        }
    }

    fn render_users(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title("Users");
        let inner = block.inner(area);
        block.render(area, buf);

        if let Some(users) = self.users.as_ref() {
            if users.is_empty() {
                Paragraph::new("No other users found.").render(inner, buf);
            } else {
                let constraints: Vec<Constraint> =
                    users.iter().map(|_| Constraint::Length(3)).collect();

                let areas = Layout::vertical(constraints).split(inner);

                for (user, area) in users.iter().zip(areas.iter()) {
                    Paragraph::new(user.username.as_str())
                        .block(Block::default().borders(Borders::BOTTOM))
                        .render(*area, buf);
                }
            }
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered();
        let inner = block.inner(area);
        block.render(area, buf);

        let [display_area, input_area] =
            Layout::vertical([Constraint::Percentage(90), Constraint::Percentage(10)]).areas(inner);

        match self.screen {
            Login => self.render_login(display_area, buf),
            Chat => self.render_chat(display_area, buf),
            Users => self.render_users(display_area, buf),
        }

        Paragraph::new(self.input.as_str())
            .block(Block::bordered())
            .render(input_area, buf);
    }
}
