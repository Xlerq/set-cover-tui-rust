use ratatui::{prelude::*, widgets::*};
use crate::MenuOption;

pub fn draw(frame: &mut Frame, app: &crate::App) 
{
    // Podziel ekran na dwie części: menu (góra) i wyniki (dół)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5), // Menu – minimum 5 wierszy
            Constraint::Min(3), // Wyniki – minimum 3 wiersze (reszta ekranu)
        ])
        .split(frame.area());

    // Rysuj menu w górnej części
    let items = vec![
        ListItem::new("1. Załaduj dane"),
        ListItem::new("2. Algorytm optymalny"),
        ListItem::new("3. Algorytm zachłanny"),
        ListItem::new("4. Wyjście"),
    ];

    let list = List::new(items)
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Yellow))
        .highlight_symbol("> ");

    let selected_index = match app.selected 
    {
        MenuOption::LoadFile => 0,
        MenuOption::OptimalAlgorithm => 1,
        MenuOption::GreedyAlgorithm => 2,
        MenuOption::Exit => 3,
    };

    let mut state = ListState::default();
    state.select(Some(selected_index));

    frame.render_stateful_widget(list, chunks[0], &mut state);

    if let Some(ref message) = app.message 
    {
        let result_area = chunks[1]; // Dolna część ekranu
        let result_text = Paragraph::new(message.as_str())
            .block(Block::default().title("Wyniki").borders(Borders::ALL))
            .wrap(Wrap { trim: false }); // Zachowaj formatowanie (np. nowe linie)

        frame.render_widget(result_text, result_area);
    }
}