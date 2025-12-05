use crate::utils::utils_files::{afficher_resultats, mesurer};

//const FILE_PATH_TEST: &str = "./inputs/aoc_2025/day2/inputs_test.txt";
const FILE_PATH_E1: &str = "./inputs/aoc_2025/day2/inputs_e1.txt";

fn parse_input(_file_path: &str) -> Vec<(usize, usize)> {
    let content = std::fs::read_to_string(_file_path)
        .expect("Failed to read file");

    content
    .split(",")
    .map(|ids| {
        let (start, end) = ids.split_once('-').unwrap();
        let start: usize = start.parse().unwrap();
        let end: usize = end.parse().unwrap();
        (start, end)
    }).collect()
}

pub fn get_response() {
    let (enigme1_result, time_e1) = mesurer(enigme1);
    let (enigme2_result, time_e2) = mesurer(enigme2);

    afficher_resultats(2, enigme1_result, time_e1, enigme2_result,time_e2);
}

pub fn enigme1() -> i64 {
    let plages = parse_input(FILE_PATH_E1);

    let mut count = 0;
    for (debut, fin) in plages {
        let range =  debut ..= fin;
        for id in range {
            let id_str = id.to_string();
            if id_str.len() % 2 != 0 {
                continue;
            }
            if &id_str[..id_str.len() / 2] == &id_str[id_str.len() / 2..] {
                count += id;
            }
        }
    }

    count as i64
}

pub fn genere_invalide(debut:usize, fin:usize) -> Vec<u128>{
    /*Exemple : genere_invalide(95, 115) → cherche les invalides entre 95 et 115*/

    /*
        Calcule le nombre de chiffres du début et de la fin.
        Exemple : debut = 95 → "95" → longueur = 2
        Exemple : fin = 1012 → "1012" → longueur = 4
     */
    let longueur_min = debut.to_string().len();
    let longueur_max = fin.to_string().len();
    let mut nombres_invalides: Vec<u128> = Vec::new();


    /*
        Itère sur chaque longueur possible de nombres invalides.
        Si plage = [95, 1012], on teste les longueurs : 2, 3, 4
        On va chercher les invalides à 2 chiffres (11, 22, 99...), puis à 3 chiffres (111, 121...), puis à 4 chiffres (1010, 1212...)
     */
    for longueur_totale in longueur_min..longueur_max + 1 {
        /*
            Itère sur chaque longueur de motif possible
            Si longueur_totale = 4, on teste les motifs de longueur 1 et 2
            Longueur 1 : 1111 = "1" répété 4 fois
            Longueur 2 : 1212 = "12" répété 2 fois

            Pourquoi /2 ? Un motif ne peut pas être plus long que la moitié du nombre total (sinon on ne peut pas le répéter 2 fois minimum).
         */
        for longueur_motif in 1..longueur_totale / 2 + 1{
            if longueur_totale % longueur_motif != 0 {continue}

            let nb_repetitions = longueur_totale / longueur_motif;

            if nb_repetitions < 2 {continue}

            // Calcul du multiplicateur pour répétition
            // Exemple : pour répéter 3 fois un motif de longueur 2
            // nombre = motif × (10^4 + 10^2 + 1) = motif × 10101
            // Formule générale : motif × (somme de 10^(L×i) pour i de 0 à K-1)
            // C'est une série géométrique : (10^(L×K) - 1) / (10^L - 1)
            let puissance = 10_u128.pow(longueur_motif as u32);
            let multiplicateur;
            if puissance == 1 {
                multiplicateur = nb_repetitions as u128
            } else {
                multiplicateur = (puissance.pow(nb_repetitions as u32) - 1) / (puissance - 1);
            }

            /*
                Premier motif possible de cette longueur.

                Si longueur_motif = 1 → motif_min = 1
                Si longueur_motif = 2 → motif_min = 10¹ = 10
                Si longueur_motif = 3 → motif_min = 10² = 100
            */
            let motif_min = if longueur_motif > 1 {10_i32.pow((longueur_motif - 1)  as u32)} else {1};
            let motif_max = 10_i32.pow(longueur_motif  as u32) - 1;

            // Optimisation : calculer directement les motifs valides
            /*
                Calcule le plus petit motif qui peut générer un nombre >= debut.
                Exemple : debut = 95, multiplicateur = 11 (pour XX répété 2 fois)

                nombre_min_pour_motif = (95 + 11 - 1) / 11 = 105 / 11 = 9 (division entière)
                Donc on commence avec le motif 9 → 9 × 11 = 99 ✓

                Le + multiplicateur - 1 c'est pour arrondir vers le haut (ceiling division).
             */
            let nombre_min_pour_motif = (debut + multiplicateur  as usize - 1) / multiplicateur  as usize;
            /*
                Calcule le plus grand motif qui peut générer un nombre <= fin.
                Exemple : fin = 115, multiplicateur = 11

                nombre_max_pour_motif = 115 / 11 = 10
                Donc on s'arrête au motif 10 → 10 × 11 = 110 ✓
             */
            let nombre_max_pour_motif = fin / multiplicateur  as usize;

            let motif_debut = motif_min.max(nombre_min_pour_motif as i32) as usize;
            let motif_fin = motif_max.min(nombre_max_pour_motif  as i32) as usize;

            for motif in motif_debut..motif_fin + 1 {
                let nombre = (motif as u128) * multiplicateur;
                if (debut as u128) <= nombre && nombre <= (fin  as u128) {
                    if nombres_invalides.contains(&nombre) == false {
                        nombres_invalides.push(nombre);
                    }
                }
            }
        }
    }

    nombres_invalides
}
pub fn enigme2() -> u128 {
    let ranges = parse_input(FILE_PATH_E1);

    let mut count:u128 = 0;
    for (debut, fin) in ranges {
        count += genere_invalide(debut, fin).iter().sum::<u128>();
    }

    count
}
