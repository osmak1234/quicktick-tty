use quicktick_tty::api;
use quicktick_tty::api::user::{sign_in, Credentials};
use quicktick_tty::app::{App, AppResult};
use quicktick_tty::event::{Event, EventHandler};
use quicktick_tty::handler::handle_key_events;
use quicktick_tty::helper::stateful_list::StatefulList;
use quicktick_tty::tui::Tui;
use std::{fs, io};
use tui::backend::CrosstermBackend;
use tui::Terminal;

#[tokio::main]
async fn main() -> AppResult<()> {
    let mut app = App::new();

    // Try to sign in with a cookie
    let config_dir = dirs::config_dir().unwrap().join("quicktick-tty");
    let cookie_file = config_dir.join("cookie.txt");

    if cookie_file.exists() {
        let uuid = fs::read_to_string(cookie_file).unwrap();
        println!("Found cookie: {}", uuid);
        println!("Signing in with cookie...");
        let res = sign_in(
            &mut app,
            Credentials {
                email: "cookie".to_string(),
                password: "cookie".to_string(),
            },
            Some(uuid),
        )
        .await;

        if let Some(user) = res {
            app.user = Some(user);
            app.task_data = crate::api::task::get_all_tasks(&mut app).await.unwrap();
            app.tasks.next();
            app.boards =
                StatefulList::with_items(api::board::get_all_user_boards(&mut app).await.unwrap());
            app.boards.next();

            println!("Signed in with cookie!");
        }
    }
    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app, &mut tui).await?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
