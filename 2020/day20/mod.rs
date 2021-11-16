use itertools::Itertools;
use std::iter;
use std::time::Instant;

use crate::utils::{self, *};

lazy_static! {
    static ref INPUT: Vec<String> = utils::read_input_lines("day20");
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Edge {
    forward: u32,
    backward: u32,
}

impl Edge {
    fn flip(self) -> Self {
        Edge {
            forward: self.backward,
            backward: self.forward,
        }
    }
}

#[derive(Debug)]
struct Tile {
    id: u32,
    pixels: Vec<Vec<char>>,
    // Top, right, bottom, left
    edges: [Edge; 4],
}

fn pixels_to_edge(s: &[char]) -> Edge {
    let mut forward = 0;
    let mut flip_bit = 1;
    let mut backward = 0;
    for c in s {
        forward <<= 1;
        if *c == '#' {
            forward |= 1;
            backward |= flip_bit;
        }
        flip_bit <<= 1;
    }
    Edge { forward, backward }
}

fn get_edges(pixels: &[Vec<char>]) -> [Edge; 4] {
    let edge1 = pixels_to_edge(&pixels[0]);
    let mut v = vec![' '; pixels.len()];
    for (i, p) in pixels.iter().enumerate() {
        v[i] = p[p.len() - 1];
    }
    let edge2 = pixels_to_edge(&v);
    let edge3 = pixels_to_edge(&pixels[pixels.len() - 1]);
    for (i, p) in pixels.iter().enumerate() {
        v[i] = p[0];
    }
    let edge4 = pixels_to_edge(&v);
    [edge1, edge2, edge3, edge4]
}

fn parse_id(line: &str) -> u32 {
    line[5..9].parse::<u32>().unwrap()
}

fn parse_tiles() -> Vec<Tile> {
    let mut ret = Vec::with_capacity((INPUT.len() + 1) / 12);
    let mut id: u32 = 0;
    let mut pixels: Vec<Vec<char>> = Vec::with_capacity(10);
    for line in INPUT.iter().chain(iter::once(&String::new())) {
        if line.is_empty() {
            let edges = get_edges(&pixels);
            ret.push(Tile { id, pixels, edges });
            id = 0;
            pixels = Vec::with_capacity(10);
            continue;
        }

        if id == 0 {
            id = parse_id(line);
        } else {
            pixels.push(line.chars().collect_vec());
        }
    }
    ret
}

fn find_corners_and_edges(
    tiles: &[Tile],
    tiles_by_edge: &AHashMap<u32, AHashSet<usize>>,
) -> (AHashSet<usize>, AHashSet<usize>) {
    let mut corners = AHashSet::new();
    let mut edges = AHashSet::new();
    for (i, t) in tiles.iter().enumerate() {
        let mut c = 0;
        for edge in t.edges.iter() {
            if tiles_by_edge[&edge.forward].len() == 1 && tiles_by_edge[&edge.backward].len() == 1 {
                c += 1
            }
        }
        if c == 1 {
            edges.insert(i);
        } else if c == 2 {
            corners.insert(i);
        } else if c != 0 {
            panic!("invalid piece: {} has {} non-matching edges", i, c);
        }
    }
    (corners, edges)
}

fn build_tiles_by_edge(tiles: &[Tile]) -> AHashMap<u32, AHashSet<usize>> {
    let mut tiles_by_edge: AHashMap<u32, AHashSet<usize>> =
        AHashMap::with_capacity(tiles.len() * 4);
    for (i, tile) in tiles.iter().enumerate() {
        for edge in &tile.edges {
            let mut e = tiles_by_edge.entry(edge.forward).or_insert(AHashSet::new());
            e.insert(i);
            e = tiles_by_edge
                .entry(edge.backward)
                .or_insert(AHashSet::new());
            e.insert(i);
        }
    }
    tiles_by_edge
}

pub fn part1() -> usize {
    let tiles = parse_tiles();

    let tiles_by_edge = build_tiles_by_edge(&tiles);

    let (corners, _) = find_corners_and_edges(&tiles, &tiles_by_edge);

    corners.iter().map(|&i| tiles[i].id as usize).product()
}

// Transformation matrix in the form
// a b
// c d
#[derive(Clone, Copy, PartialEq)]
struct Transformation {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

impl std::ops::Mul for Transformation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Transformation {
            a: self.a * rhs.a + self.b * rhs.c,
            b: self.a * rhs.b + self.b * rhs.d,
            c: self.c * rhs.a + self.d * rhs.c,
            d: self.c * rhs.b + self.d * rhs.d,
        }
    }
}

impl std::default::Default for Transformation {
    fn default() -> Self {
        Transformation {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
        }
    }
}

impl std::fmt::Debug for Transformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[[{} {}] [{} {}]]", self.a, self.b, self.c, self.d)
    }
}

impl Transformation {
    fn transform_grid(&self, (x, y): GridIndex, len: usize) -> GridIndex {
        let c = ((len - 1) as f32) / 2.0;
        let (new_x, new_y) = self.transform(((x as f32) - c, (y as f32) - c));
        ((new_x + c).round() as usize, (new_y + c).round() as usize)
    }
    fn transform(&self, (x, y): (f32, f32)) -> (f32, f32) {
        (x * self.a + y * self.b, x * self.c + y * self.d)
    }
}

#[derive(Debug, Clone, Default)]
struct PlacedTile {
    tile: usize,
    transformation: Transformation,
    edges: [u32; 4],
}

fn get_edge(pt: &PlacedTile, edge: GridDir) -> u32 {
    let edge_idx = match edge {
        GridDir::UP => 0,
        GridDir::RIGHT => 1,
        GridDir::DOWN => 2,
        GridDir::LEFT => 3,
        _ => unreachable!(),
    };
    pt.edges[edge_idx]
}

fn is_match(t1: &PlacedTile, t2: &PlacedTile, dir: GridDir) -> bool {
    get_edge(t1, dir) == get_edge(t2, dir.flip())
}

fn make_transformation(flip_x: bool, flip_y: bool, rotation: u8) -> Transformation {
    let mut t = if flip_x {
        Transformation {
            a: -1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
        }
    } else {
        Transformation::default()
    };
    if flip_y {
        t = Transformation {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: -1.0,
        } * t;
    }
    if rotation > 0 {
        let (sin_th, cos_th) = (-1.0 * (rotation as f32) * std::f32::consts::FRAC_PI_2).sin_cos();
        t = Transformation {
            a: cos_th,
            b: sin_th,
            c: -sin_th,
            d: cos_th,
        } * t;
    }
    return t;
}

fn tile_options(tiles: &[Tile], t: usize) -> Vec<PlacedTile> {
    let mut ret = Vec::with_capacity(8);
    let edges0 = tiles[t].edges.clone();
    for flip_x in [false, true] {
        let edges1 = if flip_x {
            [edges0[0].flip(), edges0[3], edges0[2].flip(), edges0[1]]
        } else {
            edges0
        };
        for flip_y in [false, true] {
            let mut edges = if flip_y {
                [edges1[2], edges1[1].flip(), edges1[0], edges1[3].flip()]
            } else {
                edges1
            };
            for rotation in 0..4 {
                if rotation > 0 {
                    edges = [edges[3].flip(), edges[0], edges[1].flip(), edges[2]];
                }
                ret.push(PlacedTile {
                    tile: t,
                    transformation: make_transformation(flip_x, flip_y, rotation),
                    edges: [
                        edges[0].forward,
                        edges[1].forward,
                        edges[2].forward,
                        edges[3].forward,
                    ],
                });
            }
        }
    }
    ret
}

fn try_place_tiles_in_dir(
    graph: &[Vec<Option<PlacedTile>>],
    current_cell: GridIndex,
    options: &mut Vec<PlacedTile>,
    dir: GridDir,
) {
    let cell = utils::increment_pos(graph, current_cell, dir)
        .and_then(|adj| utils::pos_index(graph, adj).as_ref());
    if let Some(pt) = cell {
        options.retain(|opt| is_match(opt, pt, dir));
    }
}

fn try_place_tile(
    graph: &[Vec<Option<PlacedTile>>],
    cached_options: &[Vec<PlacedTile>],
    current_cell: GridIndex,
    p: usize,
) -> Vec<PlacedTile> {
    let mut options = cached_options[p].clone();
    try_place_tiles_in_dir(graph, current_cell, &mut options, GridDir::UP);
    try_place_tiles_in_dir(graph, current_cell, &mut options, GridDir::RIGHT);
    try_place_tiles_in_dir(graph, current_cell, &mut options, GridDir::DOWN);
    try_place_tiles_in_dir(graph, current_cell, &mut options, GridDir::LEFT);
    options
}

fn get_next_index(graph: &[Vec<Option<PlacedTile>>], current_cell: GridIndex) -> GridIndex {
    for dir in [GridDir::RIGHT, GridDir::DOWN, GridDir::LEFT, GridDir::UP] {
        if let Some(next) = utils::add_pos(graph, current_cell, dir) {
            if let None = utils::pos_index(graph, next) {
                return next;
            }
        }
    }
    (0, current_cell.1 + 1)
}

// TODO: this could be optimized further by automatically checking for pieces
// that only have one match every time we place a new piece.
fn solve(
    current_cell: GridIndex,
    tiles: &[Tile],
    placed: Vec<bool>,
    graph: Vec<Vec<Option<PlacedTile>>>,
    cached_options: &[Vec<PlacedTile>],
    tiles_by_edge: &AHashMap<u32, AHashSet<usize>>,
    corners: &AHashSet<usize>,
    edges: &AHashSet<usize>,
) -> Option<Vec<Vec<Option<PlacedTile>>>> {
    let bound = graph.len() - 1;
    let (x, y) = current_cell;
    let is_corner = (x == 0 || x == bound) && (y == 0 || y == bound);
    let is_edge = !is_corner && (x == 0 || x == bound || y == 0 || y == bound);

    let unplaced_tiles = (0..tiles.len())
        .filter(|i| {
            if placed[*i] || is_corner != corners.contains(i) || is_edge != edges.contains(i) {
                return false;
            }
            for dir in [GridDir::UP, GridDir::RIGHT, GridDir::DOWN, GridDir::LEFT] {
                if let Some(above) = utils::increment_pos(&graph, current_cell, dir) {
                    if let Some(piece) = utils::pos_index(&graph, above) {
                        let above_bottom_edge = get_edge(piece, dir.flip());
                        if !tiles_by_edge[&above_bottom_edge].contains(i) {
                            return false;
                        }
                    }
                }
            }
            true
        })
        .collect::<Vec<_>>();
    if unplaced_tiles.len() == 0 {
        let num_placed = placed.iter().filter(|pt| **pt).count();
        return if num_placed < tiles.len() {
            None
        } else {
            Some(graph)
        };
    }
    for idx in unplaced_tiles {
        for pt in try_place_tile(&graph, cached_options, current_cell, idx) {
            let mut next_g = graph.clone();
            utils::set_pos(&mut next_g, current_cell, Some(pt.clone()));
            let mut next_placed = placed.clone();
            next_placed[idx] = true;
            let next_index = get_next_index(&next_g, current_cell);
            let solution = solve(
                next_index as GridIndex,
                tiles,
                next_placed,
                next_g,
                cached_options,
                tiles_by_edge,
                corners,
                edges,
            );
            if let Some(sol) = solution {
                return Some(sol);
            }
        }
    }
    None
}

fn transform_and_crop_pixels(pt: &PlacedTile, pixels: &[Vec<char>]) -> Vec<Vec<char>> {
    let cropped_len = pixels.len() - 2;
    let mut new_pixels = vec![vec![' '; cropped_len]; cropped_len];

    for (y, row) in pixels[1..pixels.len() - 1].iter().enumerate() {
        for (x, p) in row[1..row.len() - 1].iter().enumerate() {
            let new_coord = pt.transformation.transform_grid((x, y), cropped_len);
            utils::set_pos(&mut new_pixels, new_coord, *p);
        }
    }

    new_pixels
}

fn flatten_img(tiles: &[Tile], graph: &[Vec<PlacedTile>]) -> Vec<Vec<char>> {
    let mut ret = Vec::with_capacity(96);
    for g_row in graph.iter() {
        let mut rows = vec![vec![]; 8];
        for pt in g_row.iter() {
            let pixels = transform_and_crop_pixels(pt, &tiles[pt.tile].pixels);

            for (j, pixel_row) in pixels.into_iter().enumerate() {
                rows[j].extend(pixel_row);
            }
        }
        ret.extend(rows);
    }
    ret
}

static POSITIONS: [GridIndex; 15] = [
    (0, 1),
    (1, 2),
    (4, 2),
    (5, 1),
    (6, 1),
    (7, 2),
    (10, 2),
    (11, 1),
    (12, 1),
    (13, 2),
    (16, 2),
    (17, 1),
    (18, 0),
    (18, 1),
    (19, 1),
];

fn is_sea_monster(grid: &[Vec<char>], current_cell: GridIndex, positions: &[GridIndex]) -> bool {
    for pos in positions.iter() {
        if let Some(idx) =
            utils::add_pos(grid, current_cell, GridDir(pos.0 as isize, pos.1 as isize))
        {
            if *utils::pos_index(grid, idx) != '#' {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn find_all_sea_monsters(
    img: &[Vec<char>],
    positions: &[GridIndex],
    is_tall: bool,
) -> Vec<GridIndex> {
    let mut sea_monsters = vec![];
    let len = img.len();
    let (x_bound, y_bound) = if is_tall {
        (len - 2, len - 19)
    } else {
        (len - 19, len - 2)
    };

    for y in 0..y_bound {
        for x in 0..x_bound {
            if is_sea_monster(&img, (x, y), positions) {
                sea_monsters.push((x, y));
            }
        }
    }
    sea_monsters
}

pub fn part2() -> usize {
    let mut start = Instant::now();

    let tiles = parse_tiles();
    println!("parse_tiles: {:?}", Instant::now().duration_since(start));
    start = Instant::now();

    let tiles_by_edge = build_tiles_by_edge(&tiles);
    println!(
        "build_tiles_by_edge: {:?}",
        Instant::now().duration_since(start)
    );
    start = Instant::now();

    let placed = vec![false; tiles.len()];
    let bounds = (tiles.len() as f32).sqrt() as usize;
    let graph = vec![vec![None; bounds]; bounds];

    let (corners, edges) = find_corners_and_edges(&tiles, &tiles_by_edge);

    println!(
        "find_corners_and_edges: {:?}",
        Instant::now().duration_since(start)
    );
    start = Instant::now();

    let cached_options = (0..tiles.len())
        .map(|i| tile_options(&tiles, i))
        .collect_vec();

    let solution = solve(
        (0usize, 0usize) as GridIndex,
        &tiles,
        placed,
        graph,
        &cached_options,
        &tiles_by_edge,
        &corners,
        &edges,
    )
    .expect("failed to find a solution");

    println!("solve: {:?}", Instant::now().duration_since(start));
    start = Instant::now();

    let placed_tiles = solution
        .into_iter()
        .map(|row| row.into_iter().map(|t| t.unwrap()).collect_vec())
        .collect_vec();

    let img = flatten_img(&tiles, &placed_tiles);

    println!("flatten: {:?}", Instant::now().duration_since(start));
    start = Instant::now();

    let mut sea_monsters: Vec<GridIndex> = vec![];

    let bound = img.len();
    for flip_x in [false, true] {
        for flip_y in [false, true] {
            for rotation in 0..4 {
                let t = make_transformation(flip_x, flip_y, rotation);
                let positions = POSITIONS
                    .iter()
                    .map(|p| t.transform_grid(*p, bound))
                    .collect_vec();
                sea_monsters = find_all_sea_monsters(&img, &positions, rotation % 2 == 1);
                if sea_monsters.len() > 0 {
                    break;
                }
            }
        }
    }

    println!(
        "find_all_sea_monsters: {:?}",
        Instant::now().duration_since(start)
    );

    if sea_monsters.is_empty() {
        panic!("didn't find any sea monsters");
    }

    let mut count = 0;
    for row in img.iter() {
        for &c in row.iter() {
            if c == '#' {
                count += 1;
            }
        }
    }

    count - POSITIONS.len() * sea_monsters.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn make_tile(id: u32, pixels: Vec<Vec<char>>) -> Tile {
        let edges = get_edges(&pixels);
        Tile { id, pixels, edges }
    }

    fn place_tile(tiles: &[Tile], idx: usize) -> PlacedTile {
        let edges = tiles[idx].edges;
        PlacedTile {
            tile: idx,
            transformation: Transformation::default(),
            edges: edges.map(|e| e.forward),
        }
    }

    fn transform_tile(
        tiles: &[Tile],
        idx: usize,
        flip_x: bool,
        flip_y: bool,
        rotation: u8,
    ) -> PlacedTile {
        let opts = tile_options(tiles, idx);
        let mut opt_idx = 0;
        if flip_x {
            opt_idx += 8;
        }
        if flip_y {
            opt_idx += 4;
        }
        opt_idx += rotation as usize;
        opts[opt_idx].clone()
    }

    #[test]
    fn test_pixels_to_edge() {
        let e = pixels_to_edge(&['#', '.', '#', '#', '.']);
        assert_eq!(
            e,
            Edge {
                forward: 0b10110,
                backward: 0b01101,
            }
        )
    }

    #[test]
    fn mult_matrix_identity() {
        let orig = Transformation {
            a: 1.0,
            b: -2.0,
            c: 3.0,
            d: -4.0,
        };

        let t = orig * Transformation::default();

        assert_eq!(t, orig);
    }

    #[test]
    fn get_edge_simple() {
        let tiles = vec![make_tile(
            1,
            vec![
                vec!['.', '.', '#'],
                vec!['.', '.', '.'],
                vec!['.', '#', '.'],
            ],
        )];

        let pt = transform_tile(&tiles, 0, false, false, 0);

        let mut edge = get_edge(&pt, GridDir::UP);
        let mut want_edge = 1;
        assert_eq!(edge, want_edge, "got top edge {}, want {}", edge, want_edge);

        edge = get_edge(&pt, GridDir::RIGHT);
        want_edge = 4;
        assert_eq!(
            edge, want_edge,
            "got right edge {}, want {}",
            edge, want_edge
        );

        edge = get_edge(&pt, GridDir::DOWN);
        want_edge = 2;
        assert_eq!(
            edge, want_edge,
            "got bottom edge {}, want {}",
            edge, want_edge
        );

        edge = get_edge(&pt, GridDir::LEFT);
        want_edge = 0;
        assert_eq!(
            edge, want_edge,
            "got left edge {}, want {}",
            edge, want_edge
        );
    }

    #[test]
    fn get_edge_flip_x() {
        let tiles = vec![make_tile(
            1,
            vec![
                vec!['.', '.', '#'],
                vec!['#', '.', '.'],
                vec!['#', '.', '.'],
            ],
        )];

        let pt = transform_tile(&tiles, 0, true, false, 0);

        let mut edge = get_edge(&pt, GridDir::UP);
        let mut want_edge = 4;
        assert_eq!(edge, want_edge, "got top edge {}, want {}", edge, want_edge);

        edge = get_edge(&pt, GridDir::RIGHT);
        want_edge = 3;
        assert_eq!(
            edge, want_edge,
            "got right edge {}, want {}",
            edge, want_edge
        );

        edge = get_edge(&pt, GridDir::DOWN);
        want_edge = 1;
        assert_eq!(
            edge, want_edge,
            "got bottom edge {}, want {}",
            edge, want_edge
        );

        edge = get_edge(&pt, GridDir::LEFT);
        want_edge = 4;
        assert_eq!(
            edge, want_edge,
            "got left edge {}, want {}",
            edge, want_edge
        );
    }

    #[test]
    fn get_edge_flip_y() {
        let tiles = vec![make_tile(
            1,
            vec![
                vec!['.', '#', '#'],
                vec!['.', '.', '.'],
                vec!['#', '.', '.'],
            ],
        )];

        let pt = transform_tile(&tiles, 0, false, true, 0);

        let mut edge = get_edge(&pt, GridDir::UP);
        let mut want_edge = 4;
        assert_eq!(edge, want_edge, "got top edge {}, want {}", edge, want_edge);

        edge = get_edge(&pt, GridDir::RIGHT);
        want_edge = 1;
        assert_eq!(
            edge, want_edge,
            "got right edge {}, want {}",
            edge, want_edge
        );

        edge = get_edge(&pt, GridDir::DOWN);
        want_edge = 3;
        assert_eq!(
            edge, want_edge,
            "got bottom edge {}, want {}",
            edge, want_edge
        );

        edge = get_edge(&pt, GridDir::LEFT);
        want_edge = 4;
        assert_eq!(
            edge, want_edge,
            "got left edge {}, want {}",
            edge, want_edge
        );
    }

    #[test]
    fn get_edge_rotate() {
        let tiles = vec![make_tile(
            1,
            vec![
                vec!['.', '.', '#'],
                vec!['.', '.', '.'],
                vec!['.', '#', '.'],
            ],
        )];

        let pt = transform_tile(&tiles, 0, false, false, 1);

        let mut edge = get_edge(&pt, GridDir::UP);
        let mut want_edge = 0;
        assert_eq!(edge, want_edge, "got top edge {}, want {}", edge, want_edge);

        edge = get_edge(&pt, GridDir::RIGHT);
        want_edge = 1;
        assert_eq!(
            edge, want_edge,
            "got right edge {}, want {}",
            edge, want_edge
        );

        edge = get_edge(&pt, GridDir::DOWN);
        want_edge = 1;
        assert_eq!(
            edge, want_edge,
            "got bottom edge {}, want {}",
            edge, want_edge
        );

        edge = get_edge(&pt, GridDir::LEFT);
        want_edge = 2;
        assert_eq!(
            edge, want_edge,
            "got left edge {}, want {}",
            edge, want_edge
        );

        let pt = transform_tile(&tiles, 0, false, false, 3);

        edge = get_edge(&pt, GridDir::UP);
        want_edge = 4;
        assert_eq!(edge, want_edge, "got top edge {}, want {}", edge, want_edge);

        edge = get_edge(&pt, GridDir::RIGHT);
        want_edge = 2;
        assert_eq!(
            edge, want_edge,
            "got right edge {}, want {}",
            edge, want_edge
        );

        edge = get_edge(&pt, GridDir::DOWN);
        want_edge = 0;
        assert_eq!(
            edge, want_edge,
            "got bottom edge {}, want {}",
            edge, want_edge
        );

        edge = get_edge(&pt, GridDir::LEFT);
        want_edge = 4;
        assert_eq!(
            edge, want_edge,
            "got left edge {}, want {}",
            edge, want_edge
        );
    }

    #[test]
    fn can_place_valid() {
        let tiles = vec![
            make_tile(
                1,
                vec![
                    vec!['.', '.', '#'],
                    vec!['.', '.', '.'],
                    vec!['#', '.', '.'],
                ],
            ),
            make_tile(
                2,
                vec![
                    vec!['.', '.', '.'],
                    vec!['.', '.', '.'],
                    vec!['.', '.', '#'],
                ],
            ),
            make_tile(
                3,
                vec![
                    vec!['#', '.', '.'],
                    vec!['.', '.', '.'],
                    vec!['.', '.', '.'],
                ],
            ),
            make_tile(
                4,
                vec![
                    vec!['#', '.', '.'],
                    vec!['.', '.', '.'],
                    vec!['.', '.', '.'],
                ],
            ),
            make_tile(
                5,
                vec![
                    vec!['.', '.', '.'],
                    vec!['.', '.', '.'],
                    vec!['.', '.', '#'],
                ],
            ),
        ];

        let pt1 = place_tile(&tiles, 0);
        let pt2 = place_tile(&tiles, 1);
        let pt3 = place_tile(&tiles, 2);
        let pt4 = place_tile(&tiles, 3);
        let pt5 = place_tile(&tiles, 4);

        assert!(is_match(&pt1, &pt2, GridDir::UP), "top match");
        assert!(is_match(&pt1, &pt3, GridDir::RIGHT), "right match");
        assert!(is_match(&pt1, &pt4, GridDir::DOWN), "bottom match");
        assert!(is_match(&pt1, &pt5, GridDir::LEFT), "left match");
    }

    #[test]
    fn transformation_rotate() {
        let bound = 4;
        let p: GridIndex = (2, 2);

        let t = make_transformation(false, false, 0);
        assert_eq!(t.transform_grid(p, bound), p);

        let t = make_transformation(false, false, 1);
        assert_eq!(t.transform_grid(p, bound), (2, 1));

        let t = make_transformation(false, false, 2);
        assert_eq!(t.transform_grid(p, bound), (1, 1));

        let t = make_transformation(false, false, 3);
        assert_eq!(t.transform_grid(p, bound), (1, 2));
    }

    #[test]
    fn transformation_flip() {
        let bound = 4;
        let p: GridIndex = (2, 2);

        let t = make_transformation(true, false, 0);

        assert_eq!(t.transform_grid(p, bound), (1, 2));
        assert_eq!(t.transform_grid((0, 0), bound), (3, 0));

        let t = make_transformation(false, true, 0);

        assert_eq!(t.transform_grid(p, bound), (2, 1));
        assert_eq!(t.transform_grid((0, 0), bound), (0, 3));
    }

    #[test]
    fn flatten() {
        let tiles = vec![make_tile(
            1,
            vec![
                vec!['.', '.', '#'],
                vec!['.', '.', '.'],
                vec!['.', '#', '.'],
            ],
        )];

        let pt = transform_tile(&tiles, 0, false, false, 1);

        let img = transform_and_crop_pixels(&pt, &tiles[0].pixels);

        assert_eq!(
            img,
            vec![
                vec!['.', '.', '.'],
                vec!['#', '.', '.'],
                vec!['.', '.', '#'],
            ]
        );

        let pt = transform_tile(&tiles, 0, true, false, 0);

        let img = transform_and_crop_pixels(&pt, &tiles[0].pixels);

        assert_eq!(
            img,
            vec![
                vec!['#', '.', '.'],
                vec!['.', '.', '.'],
                vec!['.', '#', '.'],
            ]
        );

        let pt = transform_tile(&tiles, 0, false, true, 0);

        let img = transform_and_crop_pixels(&pt, &tiles[0].pixels);

        assert_eq!(
            img,
            vec![
                vec!['.', '#', '.'],
                vec!['.', '.', '.'],
                vec!['.', '.', '#'],
            ]
        );
    }

    #[test]
    fn run_part1() {
        assert_eq!(part1(), 11788777383197);
    }

    #[test]
    fn run_part2() {
        assert_eq!(part2(), 2242);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part2);
    }
}
