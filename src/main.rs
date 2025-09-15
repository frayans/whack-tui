use std::time::Duration;

use anyhow::Result;
use rand::{rng, Rng};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    prelude::*,
    widgets::*,
};

fn main() -> Result<()> {
    let mut term = ratatui::init();
    let mut model = Model::default();

    while model.state != State::Exit {
        term.draw(|frame| view(&model, frame))?;

        let mut current_msg = handle_event(&model);

        while let Some(msg) = current_msg {
            current_msg = update(&mut model, msg);
        }
    }

    ratatui::restore();
    Ok(())
}

#[derive(Debug)]
struct Model {
    pub cells: [bool; 4],
    pub whack_count: usize,
    pub wrong_whack_count: usize,
    pub state: State,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            cells: [false; 4],
            whack_count: 0,
            wrong_whack_count: 0,
            state: State::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
enum State {
    #[default]
    Menu,
    Game(Option<MoleCell>),
    GameLose,
    GameWin,
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MoleCell {
    TopLeft,
    TopRight,
    BotLeft,
    BotRight,
}

impl From<MoleCell> for usize {
    fn from(value: MoleCell) -> Self {
        match value {
            MoleCell::TopLeft => 0,
            MoleCell::TopRight => 1,
            MoleCell::BotLeft => 2,
            MoleCell::BotRight => 3,
        }
    }
}

impl From<usize> for MoleCell {
    fn from(value: usize) -> Self {
        match value {
            0 => MoleCell::TopLeft,
            1 => MoleCell::TopRight,
            2 => MoleCell::BotLeft,
            3 => MoleCell::BotRight,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Message {
    GameWhack(MoleCell),
    GameGenerate,
    GameGenerateCleanup,
    GameStart,
    GameLosing,
    GameWinning,
    Quit,
}

fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::GameWhack(cell) => {
            if model.whack_count >= 10 && model.wrong_whack_count < 3 {
                return Some(Message::GameWinning);
            }

            if model.wrong_whack_count >= 3 {
                return Some(Message::GameLosing);
            }

            let idx: usize = cell.into();
            if model.cells[idx] {
                model.whack_count += 1;
            } else {
                model.wrong_whack_count += 1;
            }

            Some(Message::GameGenerateCleanup)
        }
        Message::GameGenerate => {
            let mut rng = rng();
            let mole_idx = rng.random_range(0..4);
            model.cells[mole_idx] = true;
            model.state = State::Game(Some(mole_idx.into()));
            None
        }
        Message::GameGenerateCleanup => {
            model.cells = [false; 4];
            Some(Message::GameGenerate)
        }
        Message::GameStart => Some(Message::GameGenerate),
        Message::GameLosing => {
            model.state = State::GameLose;
            None
        }
        Message::GameWinning => {
            model.state = State::GameWin;
            None
        }
        Message::Quit => {
            model.state = State::Exit;
            None
        }
    }
}

fn view(model: &Model, f: &mut Frame) {
    match model.state {
        State::Menu => f.render_widget(
            Paragraph::new("Press 'p' to play")
                .block(Block::bordered())
                .centered(),
            f.area(),
        ),
        State::Game(mole_idx) => {
            let game_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(15),
                    Constraint::Percentage(70),
                    Constraint::Percentage(15),
                ])
                .split(f.area());
            let [left, right] =
                Layout::horizontal([Constraint::Percentage(50); 2]).areas(game_layout[1]);
            let [top_left, bot_left] =
                Layout::vertical([Constraint::Percentage(50); 2]).areas(left);
            let [top_right, bot_right] =
                Layout::vertical([Constraint::Percentage(50); 2]).areas(right);

            let whack_text = Span::styled(
                format!("Successful Whack: {}", model.whack_count),
                Style::default(),
            );
            let wrong_whack_text = Span::styled(
                format!("Wrong Whack: {}", model.wrong_whack_count),
                Style::default(),
            );
            let cell = Block::bordered();
            f.render_widget(whack_text, game_layout[0]);
            f.render_widget(wrong_whack_text, game_layout[2]);
            f.render_widget(&cell, top_left);
            f.render_widget(&cell, top_right);
            f.render_widget(&cell, bot_left);
            f.render_widget(&cell, bot_right);

            if let Some(idx) = mole_idx {
                match idx {
                    MoleCell::TopLeft => f.render_widget(&cell.bg(Color::Green), top_left),
                    MoleCell::TopRight => f.render_widget(&cell.bg(Color::Green), top_right),
                    MoleCell::BotLeft => f.render_widget(&cell.bg(Color::Green), bot_left),
                    MoleCell::BotRight => f.render_widget(&cell.bg(Color::Green), bot_right),
                }
            }
        }
        State::GameLose => f.render_widget(
            Paragraph::new(vec![
                Line::from("You lost!"),
                Line::from("Press 'q' to quit."),
            ])
            .block(Block::bordered())
            .centered(),
            f.area(),
        ),
        State::GameWin => f.render_widget(
            Paragraph::new(vec![
                Line::from("You won!"),
                Line::from("Press 'q' to quit."),
            ])
            .block(Block::bordered())
            .centered(),
            f.area(),
        ),
        State::Exit => {}
    }
}

fn handle_event(model: &Model) -> Option<Message> {
    if let Ok(_) = event::poll(Duration::from_millis(250)) {
        if let Ok(Event::Key(key)) = event::read() {
            if key.kind == KeyEventKind::Press {
                return handle_key(model, key);
            }
        }
    }

    None
}

fn handle_key(model: &Model, key_ev: event::KeyEvent) -> Option<Message> {
    match model.state {
        State::Menu => match key_ev.code {
            KeyCode::Char('q') => Some(Message::Quit),
            KeyCode::Char('p') => Some(Message::GameStart),
            _ => None,
        },
        State::Game(_) => match key_ev.code {
            KeyCode::Esc => Some(Message::Quit),
            KeyCode::Char('q') => Some(Message::GameWhack(MoleCell::TopLeft)),
            KeyCode::Char('w') => Some(Message::GameWhack(MoleCell::TopRight)),
            KeyCode::Char('a') => Some(Message::GameWhack(MoleCell::BotLeft)),
            KeyCode::Char('s') => Some(Message::GameWhack(MoleCell::BotRight)),
            _ => None,
        },
        State::GameWin | State::GameLose => match key_ev.code {
            KeyCode::Char('q') => Some(Message::Quit),
            _ => None,
        },
        _ => None,
    }
}
