#![allow(unused)]

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    style::{Color, Stylize},
    widgets::Widget,
};

fn run_app() -> anyhow::Result<()> {
    println!("FPS & color boxes");
    // let f = ratatui::Frame::count(&self)

    let mut flag = true;

    let w1 = ratatui::widgets::Block::new().bg(Color::Red);
    let w2 = ratatui::widgets::Block::bordered().bg(Color::Blue);
    let w = if flag {
        // flat = !flat;
        w1
    } else {
        // flat = !flat;
        w2
    };

    let r = ratatui::widgets::canvas::Rectangle::new(
        0.0,
        0.0,
        30.0,
        30.0,
        ratatui::style::Color::Green,
    );
    ratatui::run(|mut terminal| {
        loop {
            terminal.draw(|frame| {
                frame.render_widget(
                    // "Hi",
                    // w.clone(),
                    if flag {
                        ratatui::widgets::Block::new().bg(Color::Red)
                    } else {
                        ratatui::widgets::Block::bordered().bg(Color::Blue)
                    },
                    ratatui::layout::Rect {
                        x: 10,
                        y: 0,
                        width: 10,
                        height: 5,
                    },
                );
                // frame.render_widget(
                //     "Hello",
                //     ratatui::layout::Rect {
                //         x: 30,
                //         y: 30,
                //         width: 90,
                //         height: 90,
                //     },
                // );
            })?;
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Esc => break Ok(()),
                        KeyCode::Char('j') => flag = !flag,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    })
}

fn main() {
    run_app();
}
