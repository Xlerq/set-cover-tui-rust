use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io::{self, stdout, Write};
use std::time::Instant;

mod data;
mod ui;
mod algorithms;

#[derive(Clone, Copy, PartialEq)]
pub enum MenuOption {
    LoadFile,
    OptimalAlgorithm,
    GreedyAlgorithm,
    Exit,
}

pub struct App {
    selected: MenuOption,
    data: Option<SetCoverData>,
    message: Option<String>,
    input_mode: bool,
    input_path: String,
}

pub struct SetCoverData {
    pub universe: Vec<usize>,
    pub subsets: Vec<Subset>,
}

pub struct Subset {
    pub elements: Vec<usize>,
    pub cost: usize,
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = App {
        selected: MenuOption::LoadFile,
        data: None,
        message: None,
        input_mode: false,
        input_path: String::new(),
    };

    loop {
        if app.input_mode {
            disable_raw_mode()?;
            stdout().execute(LeaveAlternateScreen)?;
            print!("Podaj ścieżkę do pliku: ");
            stdout().flush()?;

            let mut path = String::new();
            io::stdin().read_line(&mut path)?;
            let path = path.trim();

            enable_raw_mode()?;
            stdout().execute(EnterAlternateScreen)?;
            terminal.clear()?;

            match data::get_data(path) {
                Ok(data) => {
                    app.data = Some(data);
                    if let Some(ref d) = app.data {
                        let universe_adjusted: Vec<usize> = d.universe.iter().map(|&x| x + 1).collect();
                        let subsets_adjusted: Vec<(Vec<usize>, usize)> = d.subsets.iter().map(|s| {
                            let adjusted_elements: Vec<usize> = s.elements.iter().map(|&x| x + 1).collect();
                            (adjusted_elements, s.cost)
                        }).collect();
                        let universe_str = format!("Zbiór uniwersalny: {:?}", universe_adjusted);
                        let subsets_str = format!("Podzbiory: {:?}", subsets_adjusted);
                        app.message = Some(format!("{}\n{}", universe_str, subsets_str));
                    }
                }
                Err(e) => {
                    app.message = Some(format!("Błąd: {}", e));
                }
            }
            app.input_mode = false;
            app.input_path.clear();
        } else {
            terminal.draw(|frame| ui::draw(frame, &app))?;
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Up => {
                        app.selected = match app.selected {
                            MenuOption::LoadFile => MenuOption::LoadFile,
                            MenuOption::OptimalAlgorithm => MenuOption::LoadFile,
                            MenuOption::GreedyAlgorithm => MenuOption::OptimalAlgorithm,
                            MenuOption::Exit => MenuOption::GreedyAlgorithm,
                        };
                    }
                    KeyCode::Down => {
                        app.selected = match app.selected {
                            MenuOption::LoadFile => MenuOption::OptimalAlgorithm,
                            MenuOption::OptimalAlgorithm => MenuOption::GreedyAlgorithm,
                            MenuOption::GreedyAlgorithm => MenuOption::Exit,
                            MenuOption::Exit => MenuOption::Exit,
                        };
                    }
                    KeyCode::Enter => {
                        match app.selected {
                            MenuOption::LoadFile => {
                                app.input_mode = true;
                            }
                            MenuOption::OptimalAlgorithm => {
                                if let Some(ref data) = app.data {
                                    let start = Instant::now();
                                    let result = algorithms::optimal_algorithm(data);
                                    let duration = start.elapsed();
                                    let total_cost: usize = result.iter().map(|&idx| data.subsets[idx].cost).sum();
                                    let adjusted_indices: Vec<usize> = result.iter().map(|&idx| idx + 1).collect();
                                    let selected_details: Vec<String> = result
                                    .iter()
                                    .map(|&idx| {
                                        let adjusted_elements: Vec<usize> = data.subsets[idx].elements.iter().map(|&x| x + 1).collect();
                                        format!("Podzbiór {}: {:?}, koszt: {}", idx + 1, adjusted_elements, data.subsets[idx].cost)
                                    })
                                    .collect();
                                    app.message = Some(format!(
                                        "Wynik optymalny:\nIndeksy: {:?}\nCałkowity koszt: {}\nCzas wykonania: {:.3} ms\n{}",
                                        adjusted_indices,
                                        total_cost,
                                        duration.as_secs_f64() * 1000.0,
                                                               selected_details.join("\n")
                                    ));
                                } else {
                                    app.message = Some("Najpierw wczytaj dane!".to_string());
                                }
                            }
                            MenuOption::GreedyAlgorithm => {
                                if let Some(ref data) = app.data {
                                    let start = Instant::now();
                                    let result = algorithms::greedy_algorithm(data);
                                    let duration = start.elapsed();
                                    let total_cost: usize = result.iter().map(|&idx| data.subsets[idx].cost).sum();
                                    let adjusted_indices: Vec<usize> = result.iter().map(|&idx| idx + 1).collect();
                                    let selected_details: Vec<String> = result
                                    .iter()
                                    .map(|&idx| {
                                        let adjusted_elements: Vec<usize> = data.subsets[idx].elements.iter().map(|&x| x + 1).collect();
                                        format!("Podzbiór {}: {:?}, koszt: {}", idx + 1, adjusted_elements, data.subsets[idx].cost)
                                    })
                                    .collect();
                                    app.message = Some(format!(
                                        "Wynik zachłanny:\nIndeksy: {:?}\nCałkowity koszt: {}\nCzas wykonania: {:.3} ms\n{}",
                                        adjusted_indices,
                                        total_cost,
                                        duration.as_secs_f64() * 1000.0,
                                                               selected_details.join("\n")
                                    ));
                                } else {
                                    app.message = Some("Najpierw wczytaj dane!".to_string());
                                }
                            }
                            MenuOption::Exit => break,
                        }
                    }
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
