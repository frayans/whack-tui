use std::time;

use anyhow::Result;
use rand::{thread_rng, Rng};
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

        let mut current_msg = handle_event(&model)?;

        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        }
    }

    ratatui::restore();
    Ok(())
}

#[derive(Debug, Default)]
struct Model {
    pub cells: [bool; 4],
    pub whack_count: usize,
    pub wrong_whack_count: usize,
    pub state: State,
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

impl MoleCell {
    fn as_usize(&self) -> usize {
        match self {
            MoleCell::TopLeft => 0,
            MoleCell::TopRight => 1,
            MoleCell::BotLeft => 2,
            MoleCell::BotRight => 3,
        }
    }

    fn from_usize(n: usize) -> Option<Self> {
        match n {
            0 => Some(MoleCell::TopLeft),
            1 => Some(MoleCell::TopRight),
            2 => Some(MoleCell::BotLeft),
            3 => Some(MoleCell::BotRight),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Message {
    GameWhack(MoleCell),
    GameGenerate,
    GameGenerateCleanup(MoleCell),
    GameStart,
    GameLosing,
    GameWinning,
    Quit,
}

fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::GameWhack(cell) => {
            model.whack_count += 1;
            if model.whack_count >= 10 && model.wrong_whack_count < 3 {
                return Some(Message::GameWinning);
            }

            if model.cells[cell.as_usize()] {
                return Some(Message::GameGenerateCleanup(cell));
            } else {
                return Some(Message::GameLosing);
            }
        }
        Message::GameGenerate => {
            let mut rng = thread_rng();
            let mole_idx = rng.gen_range(0..4);
            model.cells[mole_idx] = true;
            model.state = State::Game(MoleCell::from_usize(mole_idx));
        }
        Message::GameGenerateCleanup(cell) => {
            model.cells[cell.as_usize()] = false;
            return Some(Message::GameGenerate);
        }
        Message::GameStart => {
            return Some(Message::GameGenerate);
        }
        Message::GameLosing => {
            model.state = State::GameLose;
        }
        Message::GameWinning => {
            model.state = State::GameWin;
        }
        Message::Quit => {
            model.state = State::Exit;
        }
    }

    None
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
            let [left, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(f.area());
            let [top_left, bot_left] = Layout::vertical([Constraint::Fill(1); 2]).areas(left);
            let [top_right, bot_right] = Layout::vertical([Constraint::Fill(1); 2]).areas(right);

            let cell = Block::bordered();
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
        State::GameLose => todo!(),
        State::GameWin => todo!(),
        State::Exit => todo!(),
    }
}

fn handle_event(model: &Model) -> Result<Option<Message>> {
    if event::poll(time::Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                return Ok(handle_key(model, key));
            }
        }
    }

    Ok(None)
}

fn handle_key(model: &Model, key: event::KeyEvent) -> Option<Message> {
    match model.state {
        State::Menu => match key.code {
            KeyCode::Char('q') => Some(Message::Quit),
            KeyCode::Char('p') => Some(Message::GameStart),
            _ => None,
        },
        State::Game(_) => match key.code {
            KeyCode::Esc => Some(Message::Quit),
            KeyCode::Char('q') => Some(Message::GameWhack(MoleCell::TopLeft)),
            KeyCode::Char('w') => Some(Message::GameWhack(MoleCell::TopRight)),
            KeyCode::Char('a') => Some(Message::GameWhack(MoleCell::BotLeft)),
            KeyCode::Char('s') => Some(Message::GameWhack(MoleCell::BotRight)),
            _ => None,
        },
        _ => None,
    }
}
