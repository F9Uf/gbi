use std::{error::Error, io};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use gbi::git::{branch, constants};
use ratatui::{
    Terminal, backend::CrosstermBackend, layout::{Constraint, Layout}, style::{Modifier, Style}, widgets::{Block, Borders, List, ListItem, ListState, Paragraph}
};

fn main() -> Result<(), Box<dyn Error>> {
    // --- Terminal setup ---
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    // --- Restore terminal ---
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{err}");
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn Error>> {
    let items = branch::list_branches(constants::CURRENT_REPO)?;

    let mut state = ListState::default();
    state.select(Some(0));

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .constraints([Constraint::Min(1), Constraint::Length(3)])
                .split(f.size());

            let body_area = chunks[0];
            let footer_area = chunks[1];

            let items: Vec<ListItem> =
                items.iter().map(|i| ListItem::new(i.as_str())).collect();

            let list = List::new(items)
                // .block(Block::default().borders(Borders::ALL).title("Select Branch"))
                .highlight_style(
                    Style::default().add_modifier(Modifier::REVERSED),
                )
                .highlight_symbol("➜ ");

            // f.render_widget(widget, area);
            let footer = Paragraph::new("Use ↓↑ or jk to move, ENTER to select, q to quit")
                .block(Block::default().borders(Borders::ALL));

            f.render_stateful_widget(list, body_area, &mut state);
            f.render_widget(footer, footer_area);
        })?;

        // --- Input handling ---
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Up | KeyCode::Char('k') => {
                    let i = match state.selected() {
                        Some(i) if i > 0 => i - 1,
                        _ => 0,
                    };
                    state.select(Some(i));
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    let i = match state.selected() {
                        Some(i) if i < items.len() - 1 => i + 1,
                        _ => items.len() - 1,
                    };
                    state.select(Some(i));
                }
                KeyCode::Enter => {
                    if let Some(i) = state.selected() {
                        let selected = &items[i];
                        println!("Selected: {selected}");
                        branch::checkout_branch(constants::CURRENT_REPO, selected)?;
                        return Ok(());
                    }
                }
                _ => {}
            }
        }
    }
}
