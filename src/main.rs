use std::{error::Error, io};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use gbi::git::{branch, constants};
use ratatui::{
    Frame, Terminal, backend::CrosstermBackend, layout::{Constraint, Layout}, style::{Modifier, Style}, widgets::{Block, Borders, List, ListItem, ListState, Paragraph}
};

#[derive(Default)]
struct App {
    exit: bool,
    current_branch_index: usize,
}

impl App {
    fn run(mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), Box<dyn Error>> {
        let branch_names = branch::list_branches(constants::CURRENT_REPO)?;
        let currrent_branch_name = branch::get_current_branch(constants::CURRENT_REPO)?;

        let current_index = branch_names
            .iter()
            .position(|name| name == &currrent_branch_name)
            .unwrap_or(0);

        self.current_branch_index = current_index;

        let mut state: ListState = ListState::default();
        state.select(Some(current_index));
        
        while !self.exit {
            terminal.draw(|f|
                self.render(f, branch_names.clone(), &mut state))?;
            
            self.handle_input_events(branch_names.clone(), &mut state)?;
        }

        Ok(())
    }

    fn render(&self, frame: &mut Frame<CrosstermBackend<io::Stdout>>, branch_names: Vec<String>, state: &mut ListState) {
        let layout =Layout::default()
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(frame.size());

        let main_area = layout[0];
        let footer_area = layout[1];

        self.render_branch_list(frame, main_area, branch_names, state);
        self.render_footer_instruction(frame, footer_area);
    }

    fn render_branch_list(&self, frame: &mut Frame<CrosstermBackend<io::Stdout>>, area: ratatui::layout::Rect, branch_names: Vec<String>, state: &mut ListState) {
        let items: Vec<ListItem> = branch_names
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let display_name = if i == self.current_branch_index {
                    format!("{} *", name)
                } else {
                    name.to_string()
                };
                ListItem::new(display_name)
            })
            .collect();

        let branches_list = List::new(items)
            .highlight_style(
                Style::default().add_modifier(Modifier::REVERSED),
            )
            .highlight_symbol("➜ ");

        frame.render_stateful_widget(branches_list, area, state);
    }

    fn render_footer_instruction(&self, frame: &mut Frame<CrosstermBackend<io::Stdout>>, area: ratatui::layout::Rect) {
        let footer = Paragraph::new("Use ↓↑ or jk to move, ENTER to select, q to quit")
            .block(Block::default().borders(Borders::TOP));
        frame.render_widget(footer, area);

    }

    fn handle_input_events(&mut self, branch_names: Vec<String>, state: &mut ListState) -> Result<(), Box<dyn Error>> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    self.exit = true;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    let i = match state.selected() {
                        Some(i) if i > 0 => i - 1,
                        _ => 0,
                    };
                    state.select(Some(i));
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    let i = match state.selected() {
                        Some(i) if i < branch_names.len() - 1 => i + 1,
                        _ => branch_names.len() - 1,
                    };
                    state.select(Some(i));
                }
                KeyCode::Enter => {
                    if let Some(i) = state.selected() {
                        let selected = &branch_names[i];
                        branch::checkout_branch(constants::CURRENT_REPO, selected)?;
                        self.current_branch_index = i;
                        return Ok(());
                    }
                }
                _ => {}
            }

        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // --- Terminal setup ---
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let app = App::default().run(&mut terminal);

    // --- Restore terminal ---
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = app {
        eprintln!("{err}");
    }

    Ok(())
}
