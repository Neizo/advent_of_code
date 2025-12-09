use std::collections::{BTreeMap, HashSet};
use num::abs;
use crate::utils::utils_files::{afficher_resultats, mesurer};
use std::fs::File;
use std::io::Write;

const FILE_PATH_TEST: &str = "./inputs/aoc_2025/day9/inputs_test.txt";
const FILE_PATH_E1: &str = "./inputs/aoc_2025/day9/inputs_e1.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

/// Segment horizontal compact
#[derive(Debug, Clone)]
pub struct Segment {
    pub y: i64,
    pub x_start: i64,
    pub x_end: i64,
}

fn parse_input(_file_path: &str) -> Vec<(i64, i64)> {
    let content = std::fs::read_to_string(_file_path)
        .expect("Failed to read file");

    content
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(',');
            Some((
                parts.next()?.trim().parse().ok()?,
                parts.next()?.trim().parse().ok()?
            ))
        })
        .collect()
}

pub fn get_response() {
    let (enigme1_result, time_e1) = mesurer(enigme1);
    let (enigme2_result, time_e2) = mesurer(enigme2);

    afficher_resultats(9, enigme1_result, time_e1, enigme2_result, time_e2);
}

pub fn enigme1() -> i64 {
    let coords = parse_input(FILE_PATH_E1);
    let mut area = Vec::new();

    for (idx, (col, row)) in coords.iter().enumerate() {
        for (col2, row2) in coords.iter().skip(idx) {
            let length = abs(col - col2) + 1;
            let width = abs(row - row2) + 1;
            area.push(length * width);
        }
    }

    *area.iter().max().unwrap()
}

pub fn enigme2() -> i64 {
    let coords = parse_input(FILE_PATH_E1);
    let (_, green_segments) = find_all_green_tiles(&coords);

    create_svg_from_segments(&coords, &green_segments, "../test.svg");
    0
}

/// Étape 1 & 2a : Tracer les lignes entre les points rouges consécutifs
/// Retourne un HashSet de toutes les tuiles vertes sur le périmètre
fn trace_perimeter(red_tiles: &[(i64, i64)]) -> HashSet<Point> {
    let mut green_tiles = HashSet::new();

    for i in 0..red_tiles.len() {
        let start = red_tiles[i];
        let end = red_tiles[(i + 1) % red_tiles.len()]; // Boucle vers le premier

        // Tracer la ligne entre start et end
        let line_points = draw_line(start, end);

        // Ajouter tous les points sauf les extrémités (qui sont rouges)
        for point in line_points {
            if point != start && point != end {
                green_tiles.insert(Point::new(point.0, point.1));
            }
        }
    }

    green_tiles
}

/// Trace une ligne droite entre deux points (horizontale ou verticale)
fn draw_line(start: (i64, i64), end: (i64, i64)) -> Vec<(i64, i64)> {
    let mut points = Vec::new();

    if start.0 == end.0 {
        // Ligne verticale
        let x = start.0;
        let min_y = start.1.min(end.1);
        let max_y = start.1.max(end.1);
        for y in min_y..=max_y {
            points.push((x, y));
        }
    } else if start.1 == end.1 {
        // Ligne horizontale
        let y = start.1;
        let min_x = start.0.min(end.0);
        let max_x = start.0.max(end.0);
        for x in min_x..=max_x {
            points.push((x, y));
        }
    } else {
        panic!("Les points ne sont pas alignés horizontalement ou verticalement");
    }

    points
}

pub fn find_green_tiles_optimized(red_tiles: &[(i64, i64)]) -> (Vec<Segment>, i64) {
    // Construire une structure de données efficace
    // Map: y -> liste de segments (x_start, x_end)
    let mut segments_by_line: BTreeMap<i64, Vec<(i64, i64)>> = BTreeMap::new();

    // Étape 1: Tracer le périmètre
    let mut perimeter_count = 0;

    for i in 0..red_tiles.len() {
        let start_pt = red_tiles[i];
        let end_pt = red_tiles[(i + 1) % red_tiles.len()];

        if start_pt.1 == end_pt.1 {
            // Segment horizontal
            let y = start_pt.1;
            let x1 = start_pt.0.min(end_pt.0);
            let x2 = start_pt.0.max(end_pt.0);

            // Ajouter les points du périmètre (sauf extrémités)
            if x2 > x1 + 1 {
                segments_by_line.entry(y)
                    .or_insert_with(Vec::new)
                    .push((x1 + 1, x2 - 1));
                perimeter_count += x2 - x1 - 1;
            }
        }
    }

    // Étape 2: Trouver toutes les lignes Y
    let min_y = red_tiles.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = red_tiles.iter().map(|&(_, y)| y).max().unwrap();

    // Étape 3: Pour chaque ligne, calculer les intersections
    let mut interior_count = 0;

    for y in min_y..=max_y {
        // Trouver les intersections avec les segments verticaux
        let mut intersections: Vec<i64> = Vec::new();

        for i in 0..red_tiles.len() {
            let p1 = red_tiles[i];
            let p2 = red_tiles[(i + 1) % red_tiles.len()];

            // Segment vertical
            if p1.0 == p2.0 {
                let x = p1.0;
                let y1 = p1.1.min(p2.1);
                let y2 = p1.1.max(p2.1);

                if y1 <= y && y <= y2 {
                    intersections.push(x);
                }
            }
        }

        if intersections.is_empty() {
            continue;
        }

        intersections.sort_unstable();

        // Créer des segments entre les paires
        let line_segments = segments_by_line.entry(y).or_insert_with(Vec::new);

        for chunk in intersections.chunks(2) {
            if chunk.len() == 2 {
                let x1 = chunk[0];
                let x2 = chunk[1];

                if x2 > x1 + 1 {
                    line_segments.push((x1 + 1, x2 - 1));
                    interior_count += x2 - x1 - 1;
                }
            }
        }
    }

    // Convertir en liste de segments
    let mut all_segments = Vec::new();
    for (y, segs) in segments_by_line.iter() {
        for &(x1, x2) in segs {
            all_segments.push(Segment {
                y: *y,
                x_start: x1,
                x_end: x2,
            });
        }
    }

    (all_segments, perimeter_count + interior_count)
}

/// Fonction principale qui combine toutes les étapes
fn find_all_green_tiles(red_tiles: &[(i64, i64)]) -> (HashSet<Point>, Vec<Segment>) {
    let (segments, _) = find_green_tiles_optimized(&red_tiles);
    (trace_perimeter(red_tiles), segments)
}

pub fn create_svg_from_segments(
    red_tiles: &[(i64, i64)],
    segments: &[Segment],
    output_path: &str,
) -> std::io::Result<()> {
    let min_x = red_tiles.iter().map(|&(x, _)| x).min().unwrap_or(0);
    let min_y = red_tiles.iter().map(|&(_, y)| y).min().unwrap_or(0);
    let max_x = red_tiles.iter().map(|&(x, _)| x).max().unwrap_or(0);
    let max_y = red_tiles.iter().map(|&(_, y)| y).max().unwrap_or(0);

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    let mut file = File::create(output_path)?;

    // En-tête SVG
    writeln!(file, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
    writeln!(file, r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{} {} {} {}" width="1200" height="1200">"#,
             min_x, min_y, width, height)?;

    writeln!(file, r#"  <rect x="{}" y="{}" width="{}" height="{}" fill="white"/>"#,
             min_x, min_y, width, height)?;

    // Style CSS
    writeln!(file, r#"  <style>"#)?;
    writeln!(file, r#"    .green-segment {{ fill: lime; stroke: none; }}"#)?;
    writeln!(file, r#"    .red-border {{ fill: none; stroke: red; stroke-width: 3; }}"#)?;
    writeln!(file, r#"    .red-point {{ fill: red; }}"#)?;
    writeln!(file, r#"  </style>"#)?;

    // Dessiner tous les segments verts
    writeln!(file, r#"  <g id="green_segments" class="green-segment">"#)?;

    for (_, seg) in segments.iter().enumerate() {
        let seg_width = seg.x_end - seg.x_start + 1;
        writeln!(file, r#"    <rect x="{}" y="{}" width="{}" height="1"/>"#,
                 seg.x_start, seg.y, seg_width)?;
    }
    writeln!(file, "  </g>")?;
    println!("  ✓ Tous les segments écrits");

    // Dessiner le périmètre rouge
    writeln!(file, r#"  <polygon points=""#)?;
    for &(x, y) in red_tiles {
        write!(file, "{},{} ", x, y)?;
    }
    writeln!(file, r#"" class="red-border"/>"#)?;

    // Dessiner les points rouges
    writeln!(file, r#"  <g id="red_points" class="red-point">"#)?;
    for &(x, y) in red_tiles {
        writeln!(file, r#"    <rect x="{}" y="{}" width="3" height="3"/>"#, x, y)?;
    }
    writeln!(file, "  </g>")?;

    // Titre
    let total_tiles: i64 = segments.iter()
        .map(|seg| seg.x_end - seg.x_start + 1)
        .sum();

    let font_size = 150;
    writeln!(file, r#"  <text x="{}" y="{}" font-size="{}" fill="black" font-family="Arial" font-weight="bold">"#,
             min_x + 100, min_y + font_size + 100, font_size)?;
    writeln!(file, r#"    Grid: {}x{} | Green tiles: {} | Segments: {}"#,
             width, height, total_tiles, segments.len())?;
    writeln!(file, r#"  </text>"#)?;

    writeln!(file, "</svg>")?;
    Ok(())
}
