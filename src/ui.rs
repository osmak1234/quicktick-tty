use tui::{
    backend::Backend,
    prelude::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // Create a horizontal line widget for the navbar

    // check if the user exists if not set "username" to "Guest"

    // USER
    let username = match &app.user {
        Some(user) => user.name.clone(),
        None => String::from("Guest"),
    };

    let navbar = Paragraph::new(username).block(
        Block::default()
            .title("QuickTick")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );
    // Adjust the style as needed

    // TASKS

    let items: Vec<_> = app
        .tasks
        .items
        .iter()
        .map(|task| {
            ListItem::new(format!(
                "{} {}",
                if task.completed { "" } else { "" },
                task.name.clone(),
            ))
        })
        .collect();

    let task_widget = List::new(items)
        .block(
            Block::default()
                .title("Tasks")
                .borders(Borders::ALL)
                .border_type(if app.selected_widget {
                    BorderType::Plain
                } else {
                    BorderType::Double
                }),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");

    // BOARDS

    let items: Vec<_> = app
        .boards
        .items
        .iter()
        .map(|board| ListItem::new(board.name.clone()))
        .collect();

    let board_widget = List::new(items)
        .block(
            Block::default()
                .title("Boards")
                .borders(Borders::ALL)
                .border_type(if app.selected_widget {
                    BorderType::Double
                } else {
                    BorderType::Plain
                }),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");

    // Create your layout with the navbar and horizontal split for "Tasks" and "Boards"
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Percentage(100)].as_ref()) // Navbar and horizontal split
        .split(frame.size());

    // Create a horizontal split layout for "Tasks" and "Boards"
    let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(layout[1]); // Use the second constraint for horizontal split

    // Render the navbar
    frame.render_widget(navbar, layout[0]);

    // Render "Tasks" and "Boards" paragraphs in the horizontal split layout
    frame.render_stateful_widget(
        task_widget,
        horizontal_layout[0], // Render "Tasks" in the left half of the split
        &mut app.tasks.state,
    );

    frame.render_stateful_widget(
        board_widget,
        horizontal_layout[1], // Render "Boards" in the right half of the split
        &mut app.boards.state,
    );

    let screen_size = frame.size();

    // place the input in the middle of the screen using layout_input variable name,
    let layout_input = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(
            if app.input.visible && screen_size.width > 60 && screen_size.height > 6 {
                Rect::new(
                    screen_size.width / 2 - 30,
                    screen_size.height / 2 - 3,
                    60,
                    5,
                )
            } else {
                Rect::new(0, 0, 0, 0)
            },
        );

    let input = Paragraph::new(format!("\n{}", app.input.value))
        .block(
            Block::default()
                .title("New task")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().bg(Color::Indexed(235)).fg(Color::White));

    if app.input.visible {
        frame.render_widget(input, layout_input[0]);
    }

    match app.input.visible {
        true => {
            frame.set_cursor(
                layout_input[0].x + app.input.value.to_string().len() as u16 + 1,
                layout_input[0].y + 2,
            );
        }
        false => {}
    }
}
