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
    let tasks_to_display = app
        .task_data
        .iter()
        .filter(|task| {
            app.boards
                .selected()
                .is_some_and(|board| board.special.is_some_and(|special| special == 1))
                || app
                    .boards
                    .selected()
                    .is_some_and(|board| board.uuid == task.board_uuid)
        })
        .cloned()
        .collect::<Vec<_>>();

    app.tasks.items = tasks_to_display;
    let items = app
        .tasks
        .items
        .iter()
        .map(|task| {
            let mut task_name = task.name.clone();
            if task.completed {
                task_name = format!(" {}", task_name);
            } else {
                task_name = format!(" {}", task_name);
            }
            ListItem::new(task_name)
        })
        .collect::<Vec<_>>();

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
        .highlight_symbol(" ");

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
        .highlight_symbol(" ");

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

    let input_content = app.input_content.clone();

    let input_fields = input_content.ui_to_render();

    // based on input_fields.len() render the input input_fields

    let layout_input = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            input_fields
                .iter()
                .map(|_| Constraint::Length(3))
                .collect::<Vec<_>>()
                .as_ref(),
        )
        .split(Rect::new(
            screen_size.width / 2 - 20,
            screen_size.height / 2 - 5,
            50,
            match input_fields.len() {
                0 => 0,
                1 => 3,
                2 => 6,
                _ => 9,
            },
        ));

    //  let input = Paragraph::new(format!("\n{}", app.input.value))
    //      .block(
    //          Block::default()
    //              .title("New task")
    //              .borders(Borders::ALL)
    //              .border_type(BorderType::Rounded),
    //      )
    //      .style(Style::default().bg(Color::Indexed(235)).fg(Color::White));

    if app.input_content.visible {
        for (index, input) in input_fields.iter().enumerate() {
            frame.render_widget(input.clone(), layout_input[index]);
        }
    }

    match app.input_content.visible {
        true => {
            frame.set_cursor(
                layout_input[0].x + app.input_content.selected_input_len() as u16 + 1,
                layout_input[app.input_content.selected_input].y + 1,
            );
        }
        false => {}
    }
}
