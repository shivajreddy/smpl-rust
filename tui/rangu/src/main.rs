use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::Block;

fn foo(flag: bool) -> Block<'static> {
    if flag {
        return Block::new().bg(Color::Red);
    } else {
        Block::bordered().bg(Color::Blue)
    }
}

fn run_app() -> anyhow::Result<()> {
    let mut flag = true;
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| frame.render_widget(foo(flag), frame.area()))?;
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
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
    let _ = run_app();
}
