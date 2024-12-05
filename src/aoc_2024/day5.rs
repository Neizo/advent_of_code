use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;

const FILE_PATH: &str = "./inputs/aoc_2024/inputs_day5.txt";

pub fn get_response() -> Result<(i64, i64), Box<dyn Error>> {
    // Charger les règles d'ordre et les mises à jour depuis l'entrée
    let (rules, updates) = parse_input(FILE_PATH);

    // Construire le graphe à partir des règles
    let graph = build_graph(&rules);

    Ok((enigme1(&graph, &updates), enigme2(&graph, &updates)))
}

fn enigme1(graph: &HashMap<usize, Vec<usize>>, updates: &Vec<Vec<usize>>) -> i64 {
    let mut total_middle_pages = 0i64;
    for update in updates {
        if is_update_valid(&graph, &update) {
            if let Some(middle_page) = find_middle_page(&update) {
                total_middle_pages += middle_page as i64;
            }
        }
    }

    total_middle_pages
}
fn parse_input(filename: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let input = std::fs::read_to_string(filename).expect("Unable to read file");
    let mut sections = input.split("---");

    let section1 = sections.next().unwrap_or("").trim(); // Partie avant la ligne vide
    let section2 = sections.next().unwrap_or("").trim(); // Partie après la ligne vide

    // Parse the rules
    let rules = section1
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split('|').map(|x| x.parse::<usize>().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();

    // Parse the updates
    let updates = section2
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    (rules, updates)
}

fn build_graph(rules: &Vec<(usize, usize)>) -> HashMap<usize, Vec<usize>> {
    let mut graph = HashMap::new();
    for &(x, y) in rules {
        graph.entry(x).or_insert_with(Vec::new).push(y);
    }
    graph
}

fn is_update_valid(graph: &HashMap<usize, Vec<usize>>, update: &Vec<usize>) -> bool {

    for (index, page) in update.iter().enumerate() {
        match graph.get(page) {
            None => {}
            Some(pages_graph) => {
                for cpt in 0..index {
                    if is_right_order(pages_graph, &update[cpt]) == false {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn is_right_order(pages_graph:&Vec<usize>, page:&usize) -> bool {
    !(pages_graph.iter().filter(|value| page == *value).count() > 0)
}

fn find_middle_page(update: &Vec<usize>) -> Option<usize> {
    if update.is_empty() {
        None
    } else {
        Some(update[update.len() / 2])
    }
}

fn enigme2(graph: &HashMap<usize, Vec<usize>>, updates: &Vec<Vec<usize>>) -> i64 {
    let mut total_middle_pages = 0i64;

    for update in updates {
        if !is_update_valid(graph, update) {
            let sorted_update = topological_sort(graph, update);
            if let Some(middle_page) = find_middle_page(&sorted_update) {
                total_middle_pages += middle_page as i64;
            }
        }
    }

    total_middle_pages
}

fn topological_sort(graph: &HashMap<usize, Vec<usize>>, update: &Vec<usize>) -> Vec<usize> {
    let mut in_degree = HashMap::new();
    let mut order = Vec::new();
    let mut queue = VecDeque::new();

    // Restreindre le graphe aux pages de la mise à jour
    let pages_set: HashSet<_> = update.iter().cloned().collect();

    for &page in update {
        in_degree.insert(page, 0);
    }

    /*
     Pour chaque page 𝑋, on parcourt ses dépendances (les pages 𝑌 pour lesquelles 𝑋∣𝑌) et on augmente le degré entrant de 𝑌
     Pour chaque page, on vérifie le nombre de page présent dans la ligne qui devrais être après celle ci, on augmente le degré d'autant de page que touver,
     Mais on augmente pas le degré de la page parcourus mais celle trouvé dans la ligne
     exemple
        graph : {53: [29, 13], 61: [13, 53, 29], 29: [13], 75: [29, 53, 47, 61, 13], 47: [53, 13, 61, 29], 97: [13, 61, 47, 29, 53, 75]} (par exemple les page 29 et 13 doivent se trouver après la page 53)
        updates : [61, 13, 29]

        ici, dans le graph on sais que 13 doit être après 61 et après 29 donc on augmente son degré de 2, ce qui fait qu'il seras à la fin
    */
    for &page in update {
        if let Some(dependencies) = graph.get(&page) {
            for &dep in dependencies {
                if pages_set.contains(&dep) {
                    *in_degree.entry(dep).or_insert(0) += 1;
                }
            }
        }
    }

    /*Toute page avec un degré entrant de 0 peut être placée en premier dans l’ordre.*/
    for &page in update {
        if in_degree[&page] == 0 {
            queue.push_back(page);
        }
    }

    /*Retirer une page de la file et l’ajouter à l’ordre final*/
    while let Some(page) = queue.pop_front() {
        order.push(page);
        /*
        * Pour chaque page 𝑌 dépendante de 𝑋, on décrémente son degré entrant (car X a déjà été placé dans l’ordre).
        * Si le degré entrant de Y devient 0, cela signifie qu’elle est prête à être placée (aucune dépendance restante), donc on l’ajoute à la file.
        */
        if let Some(dependencies) = graph.get(&page) {
            for &dep in dependencies {
                if pages_set.contains(&dep) {
                    let count = in_degree.get_mut(&dep).unwrap();
                    *count -= 1;
                    if *count == 0 {
                        queue.push_back(dep);
                    }
                }
            }
        }
    }

    order
}