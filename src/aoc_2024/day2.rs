use std::cmp::PartialEq;
use std::error::Error;
use std::fs::read_to_string;

const FILE_PATH: &str = "./inputs/aoc_2024/inputs_day2.txt";


fn parse_file() -> Result<Vec<Vec<i64>>, Box<dyn Error>> {
    let datas = read_to_string(FILE_PATH)?;

    let reports = datas
        .lines()
        .map(|line| {
            let mut line_value = vec![];
            for value in line.trim().split_whitespace().collect::<Vec<&str>>() {
                match value.parse::<i64>() {
                    Ok(data) => {line_value.push(data)}
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }

            line_value
        }).collect::<Vec<Vec<i64>>>();

    Ok(reports)
}

pub fn get_response() -> Result<(i64, i64), Box<dyn Error>> {
    match parse_file() {
        Ok(reports) => {
            Ok((enigme1(&reports), enigme2(&reports)))
        }
        Err(e) => {
            return Err(e);
        }
    }
}

fn enigme1(reports: &Vec<Vec<i64>>) -> i64 {
    reports
        .iter()
        .filter(|line| {
            is_safe_line(line) == None
        }) // Filtrer les lignes sûres
        .count() as i64 // Compter les lignes sûres
}

fn is_safe_line(line: &Vec<i64>) -> Option<usize> {
    let size_line = line.len();

    #[derive(PartialEq, Eq)]
    enum Levels {
        INCREASING,
        DECREASING
    }

    // Déterminer si la ligne est sûre
    let mut levels = Levels::INCREASING;
    for (cpt, _) in line.iter().enumerate() {
        if cpt == 0 {
            levels = if line[cpt] < line[cpt + 1] {
                Levels::DECREASING
            } else {
                Levels::INCREASING
            };
        } else if cpt + 1 < size_line {
            // Vérifier le niveau (incrément/décrément)
            if (line[cpt] < line[cpt + 1] && levels == Levels::INCREASING)
                || (line[cpt] > line[cpt + 1] && levels == Levels::DECREASING)
            {
                return Some(cpt); // Retourne l'index global ici
            }
        }

        // Vérifier la différence entre les valeurs
        if cpt + 1 < size_line {
            let diff = (line[cpt] - line[cpt + 1]).abs();
            if diff < 1 || diff > 3 {
                return Some(cpt); // Retourne l'index global ici
            }
        }
    }

    None // La ligne est sûre
}

fn enigme2(reports: &Vec<Vec<i64>>) -> i64 {
    reports
        .iter()
        .filter(|line| {
            match is_safe_line(line) {
                None => return true, // Ligne initialement sûre
                Some(_) => {
                    for cpt in 0..line.len() {
                        let mut new_line = (*line).clone();
                        new_line.remove(cpt);
                        if is_safe_line(&new_line).is_none() {return true;}
                    }
                    return false;
                }
            }
        })
        .count() as i64
}