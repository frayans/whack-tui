use std::time;

use anyhow::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    prelude::*,
    widgets::*,
};

mod whack;

fn main() -> Result<()> {
    let mut term = ratatui::init();
    let mut model = Model::default();

    while model.state != State::Exit {
        term.draw(|frame| view(&mut model, frame))?;

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
    pub cells: [bool; 9],
    pub whack_count: usize,
    pub wrong_whack_count: usize,
    pub state: State,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum State {
    #[default]
    Menu,
    Game,
    GameLose,
    GameWin,
    Exit,
}

#[derive(Debug, PartialEq)]
enum Message {
    GameWhack(usize),
    GameIsOccupied(usize),
    GameStart,
    GameLosing,
    GameWinning,
    Quit,
}

fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::GameWhack(idx) => {
            model.whack_count += 1;
            if model.whack_count >= 10 && model.wrong_whack_count < 3 {
                return Some(Message::GameWinning);
            }

            if model.cells[idx] {
                return Some(Message::GameIsOccupied(idx));
            } else {
                model.cells[idx] = true;
            }
        }
        Message::GameIsOccupied(_) => {
            model.wrong_whack_count += 1;
            if model.wrong_whack_count >= 3 {
                return Some(Message::GameLosing);
            }
        }
        Message::GameStart => {
            model.state = State::Game;
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

fn view(model: &mut Model, f: &mut Frame) {
    let grid = Layout::default()
        .constraints(Constraint::from_percentages([25, 25, 25, 25, 25, 25]))
        .split(f.area());

    match model.state {
        State::Menu => f.render_widget(Block::bordered(), f.area()),
        State::Game => {
            f.render_widget(Block::bordered().bg(Color::Green), grid[0]);
            f.render_widget(Block::bordered().bg(Color::Green), grid[1]);
            f.render_widget(Block::bordered().bg(Color::Green), grid[2]);
            f.render_widget(Block::bordered().bg(Color::Red), grid[3]);
            f.render_widget(Block::bordered().bg(Color::Red), grid[4]);
            f.render_widget(Block::bordered().bg(Color::Red), grid[5]);
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
        State::Game => match key.code {
            KeyCode::Esc => Some(Message::Quit),
            KeyCode::Char('q') => Some(Message::GameWhack(0)),
            _ => None,
        },
        _ => None,
    }
}
