use crate::{SetCoverData, Subset};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_data(file_path: &str) -> Result<SetCoverData, String> {
    let file = File::open(file_path).map_err(|e| format!("Błąd otwarcia pliku: {}", e))?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let universe_line = lines.next().ok_or("Brak linii z zbiorem uniwersalnym")?;
    let universe_line = universe_line.map_err(|e| format!("Błąd odczytu linii: {}", e))?;
    if !universe_line.starts_with('[') || !universe_line.ends_with(']') {
        return Err("Zbiór uniwersalny musi być w formacie [1 2 3 ...]".to_string());
    }
    let universe: Vec<usize> = universe_line[1..universe_line.len() - 1]
        .split_whitespace()
        .map(|num| {
            num.parse()
                .map_err(|e| format!("Błąd parsowania liczby w zbiorze uniwersalnym: {}", e))
        })
        .collect::<Result<Vec<usize>, String>>()?;

    let min_val = *universe
        .iter()
        .min()
        .ok_or("Zbiór uniwersalny jest pusty")?;
    if min_val != 1 {
        return Err("Zbiór uniwersalny musi zaczynać się od 1".to_string());
    }
    let universe: Vec<usize> = universe.iter().map(|&x| x - 1).collect();

    let mut subsets = Vec::new();
    for (i, line) in lines.enumerate() {
        let line = line.map_err(|e| format!("Błąd odczytu linii {}: {}", i + 2, e))?;
        if line.trim().is_empty() {
            continue;
        }
        if !line.starts_with("[[") || !line.ends_with(']') {
            return Err(format!(
                "Linia {} musi być w formacie [[elementy] koszt]",
                i + 2
            ));
        }

        let content = &line[1..line.len() - 1];
        let parts: Vec<&str> = content.split(']').collect();
        if parts.len() != 2 {
            return Err(format!(
                "Linia {}: Nieprawidłowy format, oczekiwano [[elementy] koszt]",
                i + 2
            ));
        }

        let elements_str = parts[0]
            .trim()
            .strip_prefix('[')
            .ok_or(format!("Linia {}: Brak [ w elementach", i + 2))?;
        let elements: Vec<usize> = elements_str
            .split_whitespace()
            .map(|num| {
                num.parse()
                    .map_err(|e| format!("Błąd parsowania elementu w linii {}: {}", i + 2, e))
            })
            .collect::<Result<Vec<usize>, String>>()?;
        let elements: Vec<usize> = elements.iter().map(|&x| x - 1).collect();

        let cost: usize = parts[1]
            .trim()
            .parse()
            .map_err(|e| format!("Błąd parsowania kosztu w linii {}: {}", i + 2, e))?;

        subsets.push(Subset { elements, cost });
    }

    Ok(SetCoverData { universe, subsets })
}
