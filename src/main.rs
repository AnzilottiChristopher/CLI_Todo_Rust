use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, List, ListItem, ListState, Padding, Paragraph, Widget},
};

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
    list_state: ListState,
    is_add_new: bool,
    input_value: String,
}

#[derive(Debug, Default)]
struct TodoItem {
    is_done: bool,
    description: String,
}

fn main() -> Result<()> {
    let mut state = AppState::default();
    state.is_add_new = false;

    state.items.push(TodoItem {
        is_done: false,
        description: String::from("Finish Application"),
    });
    state.items.push(TodoItem {
        is_done: false,
        description: String::from("Descrip 1"),
    });
    state.items.push(TodoItem {
        is_done: false,
        description: String::from("Descrip 2"),
    });

    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal, &mut state);

    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        //Rendering
        terminal.draw(|f| render(f, app_state))?;

        //Input Handling
        if let Event::Key(key) = event::read()? {
            if cfg!(target_os = "linux") {
                if app_state.is_add_new {
                    if handle_add_new(key, app_state) {
                        app_state.is_add_new = false;
                    }
                } else if key.code == KeyCode::Esc {
                    break;
                } else {
                    handle_input(key, app_state);
                }
            } else if cfg!(target_os = "windows") {
                if key.kind == KeyEventKind::Press {
                    if app_state.is_add_new {
                        if handle_add_new(key, app_state) {
                            app_state.is_add_new = false;
                        }
                    } else if key.code == KeyCode::Esc {
                        break;
                    } else {
                        handle_input(key, app_state);
                    }
                }
            }
        }
    }
    Ok(())
}

fn handle_add_new(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        KeyCode::Char(c) => {
            app_state.input_value.push(c);
        }
        KeyCode::Backspace => {
            app_state.input_value.pop();
        }
        KeyCode::Enter => {
            return true;
        }
        KeyCode::Esc => {
            return true;
        }
        _ => {}
    }
    false
}
fn handle_input(key: KeyEvent, app_state: &mut AppState) {
    match key.code {
        KeyCode::Char(char) => match char {
            'A' => {
                app_state.is_add_new = true;
            }
            'D' => {
                if let Some(index) = app_state.list_state.selected() {
                    app_state.items.remove(index);
                }
                app_state.list_state.select_previous();
            }
            'k' => {
                app_state.list_state.select_previous();
            }
            'j' => {
                app_state.list_state.select_next();
            }
            _ => {}
        },
        _ => {}
    }
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
    if app_state.is_add_new {
        Paragraph::new(app_state.input_value.as_str())
            .block(
                Block::bordered()
                    .fg(Color::Green)
                    .padding(Padding::uniform(1))
                    .border_type(BorderType::Rounded),
            )
            .render(frame.area(), frame.buffer_mut());
    } else {
        let [border_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(frame.area());

        let [inner_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(border_area);

        Block::bordered()
            .border_type(BorderType::Rounded)
            .fg(Color::Yellow)
            .render(border_area, frame.buffer_mut());

        let list = List::new(
            app_state
                .items
                .iter()
                .map(|x| ListItem::from(x.description.as_str())),
        )
        .highlight_symbol(">")
        .highlight_style(Style::default().fg(Color::Green));

        frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
    }
}
