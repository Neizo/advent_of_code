use std::error::Error;
use std::fs::read_to_string;

const FILE_PATH: &str = "./inputs/aoc_2024/inputs_day9.txt";

pub fn get_response() -> Result<(usize, usize), Box<dyn Error>> {
    let input = read_to_string(&FILE_PATH).expect("Unable to read file");

    // Étape 1 : Parse l'entrée
    let mut disk = parse_disk(input.as_str());
    let mut disk_p2 = disk.clone();

    // Étape 2 : Compacte les fichiers
    compact_disk(&mut disk);
    compact_disk_whole_files(&mut disk_p2);

    Ok((calculate_checksum(&disk),calculate_checksum(&disk_p2)))
}

// Parse la chaîne d'entrée en une représentation de disque
fn parse_disk(input: &str) -> Vec<Option<usize>> {
    let mut disk = Vec::new();
    let mut file_id = 0;

    let digits: Vec<usize> = input.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    for (i, &len) in digits.iter().enumerate() {
        if i % 2 == 0 {
            // Fichiers
            disk.extend(vec![Some(file_id); len]);
            file_id += 1;
        } else {
            // Espaces libres
            disk.extend(vec![None; len]);
        }
    }
    disk
}

// Compacte les fichiers en déplaçant les blocs vers la gauche
fn compact_disk(disk: &mut Vec<Option<usize>>) {
    let mut write_pos = 0;
    let mut read_pos = disk.len() -1;

    while write_pos < read_pos {
        if disk[write_pos] != None {write_pos += 1;}
        if disk[read_pos] == None {read_pos -= 1;}

        if disk[read_pos] != None && disk[write_pos] == None {
            disk[write_pos] = disk[read_pos];
            disk[read_pos] = None;
        }
    }
}

fn compact_disk_whole_files(disk: &mut Vec<Option<usize>>) {
    let mut read_pos = disk.len() -1;
    let mut free_space = find_free_space_spans(disk, read_pos);

    while read_pos > 0 {
        if disk[read_pos] == None {read_pos -= 1;}

        if disk[read_pos] != None {
            let file_size = disk.iter().filter(|&&b| b == disk[read_pos]).count();

            for (idx_free_space, (free_start, free_end)) in free_space.iter().enumerate() {
                if *free_end > read_pos {break}
                let free_space_size = (free_end - free_start) + 1;
                if file_size <= free_space_size {
                    for index in *free_start..(free_start + file_size) {
                        disk[index] = disk[read_pos];
                    }

                    let file_start = read_pos - if read_pos > file_size {file_size} else {read_pos};
                    for index in (file_start+1)..=read_pos {
                        disk[index] = None;
                    }

                    let new_free_start = free_start + file_size;
                    if new_free_start > *free_end {
                        free_space.remove(idx_free_space);
                    } else {
                        free_space[idx_free_space] = (new_free_start, *free_end);
                    }

                    break;
                }
            }

            read_pos -= if read_pos > file_size {file_size} else {read_pos};
        }
    }
}

// Trouve les plages d'espace libre sur le disque
fn find_free_space_spans(disk: &Vec<Option<usize>>, read_pos:usize) -> Vec<(usize, usize)> {
    let mut free_spaces = Vec::new();
    let mut start = None;

    for (i, block) in disk.iter().enumerate() {
        if i > read_pos {break}
        match (start, block) {
            (None, None) => start = Some(i), // Début d'une plage libre
            (Some(s), Some(_)) => {
                free_spaces.push((s, i - 1));
                start = None;
            }
            _ => (),
        }
    }

    // Ajoute la dernière plage si elle se termine en fin de disque
    if let Some(s) = start {
        free_spaces.push((s, disk.len() - 1));
    }

    free_spaces
}

// Calcule la somme de contrôle
fn calculate_checksum(disk: &Vec<Option<usize>>) -> usize {
    disk.iter()
        .enumerate()
        .filter_map(|(pos, &file_id)| file_id.map(|id| pos * id))
        .sum()
}