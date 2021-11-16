#[allow(unused_mut)]
use itertools::Itertools;

use crate::utils::{self, *};

lazy_static! {
    static ref INPUT: Vec<String> = utils::read_input_lines("day20");
}

#[derive(Debug)]
struct Tile {
    id: u32,
    pixels: Vec<Vec<char>>,
    // Top, right, bottom, left
    edges: Vec<u32>,
}

fn pixels_to_edge(s: &[char]) -> u32 {
    let mut edge = 0;
    for c in s {
        edge <<= 1;
        edge |= if *c == '#' { 1 } else { 0 };
    }
    edge
}

fn flip_edge(mut edge: u32, n_bits: usize) -> u32 {
    let mut flipped = 0;
    for _ in 0..n_bits {
        flipped <<= 1;
        flipped |= edge & 1;
        edge >>= 1;
    }
    flipped
}

fn get_edges(pixels: &[Vec<char>]) -> Vec<u32> {
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
    vec![edge1, edge2, edge3, edge4]
}

fn parse_id(line: &str) -> u32 {
    let mut parts = line.split_whitespace();
    parts.next();
    let id = parts.next().unwrap();
    id.strip_suffix(":").unwrap().parse::<u32>().unwrap()
}

fn parse_tiles() -> Vec<Tile> {
    utils::group_lines(&INPUT)
        .into_iter()
        .map(|g| {
            let mut lines = g.into_iter().cloned().collect_vec();
            let id = parse_id(&lines.remove(0));
            let pixels = lines.iter().map(|s| s.chars().collect_vec()).collect_vec();
            let edges = get_edges(&pixels);
            Tile { id, pixels, edges }
        })
        .collect()
}

fn find_corners_and_edges(
    tiles: &[Tile],
    tiles_by_edge: &AHashMap<u32, Vec<usize>>,
) -> (AHashSet<usize>, AHashSet<usize>) {
    let mut corners = AHashSet::new();
    let mut edges = AHashSet::new();
    for (i, t) in tiles.iter().enumerate() {
        let mut c = 0;
        for edge in &t.edges {
            if tiles_by_edge[&edge].len() == 1
                && tiles_by_edge[&flip_edge(*edge, t.pixels.len())].len() == 1
            {
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

fn build_tiles_by_edge(tiles: &[Tile]) -> AHashMap<u32, Vec<usize>> {
    let mut tiles_by_edge: AHashMap<u32, Vec<usize>> = AHashMap::new();
    for (i, tile) in tiles.iter().enumerate() {
        for edge in &tile.edges {
            let mut e = tiles_by_edge.entry(*edge).or_insert(vec![]);
            e.push(i);
            e = tiles_by_edge
                .entry(flip_edge(*edge, tile.pixels.len()))
                .or_insert(vec![]);
            e.push(i);
        }
    }
    tiles_by_edge
}

fn match_tiles() {
    let tiles = parse_tiles();

    let mut tiles_by_edge = build_tiles_by_edge(&tiles);

    let mut adj: Vec<Vec<u32>> = vec![vec![0; tiles.len()]; tiles.len()];

    let mut marked = vec![0; tiles.len()];
    let mut done_tiles = vec![false; tiles.len()];
    loop {
        for (&edge, tiles) in tiles_by_edge.iter() {
            if let [t1, t2] = tiles[..] {
                adj[t1][t2] = edge;
                adj[t2][t1] = edge;
                marked[t1] += 1;
                marked[t2] += 1;
            }
        }
        let mut found = false;
        for (i, done) in done_tiles.iter_mut().enumerate() {
            if !*done && marked[i] == 3 {
                *done = true;
                found = true;
                for edge in &tiles[i].edges {
                    tiles_by_edge.get_mut(edge).unwrap().retain(|&t| t != i);
                }
            }
        }
        if !found {
            break;
        }
    }
}

#[derive(Debug, Clone, Default)]
struct PlacedTile {
    tile: usize,
    flip_x: bool,
    flip_y: bool,
    rotation: usize,
}

fn get_edge(tiles: &[Tile], pt: &PlacedTile, edge: GridDir) -> u32 {
    let t = &tiles[pt.tile];
    let n_bits = t.pixels.len();
    let mut edges = t.edges.clone();
    if pt.flip_x {
        (edges[1], edges[3]) = (edges[3], edges[1]);
        edges[0] = flip_edge(edges[0], n_bits);
        edges[2] = flip_edge(edges[2], n_bits);
    }
    if pt.flip_y {
        (edges[0], edges[2]) = (edges[2], edges[0]);
        edges[1] = flip_edge(edges[1], n_bits);
        edges[3] = flip_edge(edges[3], n_bits);
    }
    for _ in 0..pt.rotation {
        let last = edges.pop().unwrap();
        edges.insert(0, last);
        edges[0] = flip_edge(edges[0], n_bits);
        edges[2] = flip_edge(edges[2], n_bits);
    }
    match edge {
        GridDir::UP => edges[0],
        GridDir::RIGHT => edges[1],
        GridDir::DOWN => edges[2],
        GridDir::LEFT => edges[3],
        _ => unreachable!(),
    }
}

fn is_match(tiles: &[Tile], t1: &PlacedTile, t2: &PlacedTile, dir: GridDir) -> bool {
    get_edge(tiles, t1, dir) == get_edge(tiles, t2, dir.flip())
}

fn tile_options(t: usize) -> Vec<PlacedTile> {
    let mut ret = vec![];
    for flip_x in [false, true] {
        for flip_y in [false, true] {
            for rotation in 0..4 {
                ret.push(PlacedTile {
                    tile: t,
                    flip_x,
                    flip_y,
                    rotation,
                });
            }
        }
    }
    ret
}

fn try_place_tiles_in_dir(
    graph: &[Vec<Option<PlacedTile>>],
    tiles: &[Tile],
    current_cell: GridIndex,
    options: &mut Vec<PlacedTile>,
    dir: GridDir,
) {
    let cell = utils::increment_pos(graph, current_cell, dir)
        .and_then(|adj| utils::pos_index(graph, adj).as_ref());
    if let Some(pt) = cell {
        options.retain(|opt| is_match(tiles, opt, pt, dir));
    }
}

fn try_place_tile(
    graph: &[Vec<Option<PlacedTile>>],
    tiles: &[Tile],
    current_cell: GridIndex,
    p: usize,
) -> Vec<PlacedTile> {
    let mut options = tile_options(p);
    try_place_tiles_in_dir(graph, tiles, current_cell, &mut options, GridDir::UP);
    try_place_tiles_in_dir(graph, tiles, current_cell, &mut options, GridDir::RIGHT);
    try_place_tiles_in_dir(graph, tiles, current_cell, &mut options, GridDir::DOWN);
    try_place_tiles_in_dir(graph, tiles, current_cell, &mut options, GridDir::LEFT);
    options
}

fn solve(
    current_cell: GridIndex,
    tiles: &[Tile],
    placed: Vec<bool>,
    graph: Vec<Vec<Option<PlacedTile>>>,
    corners: &AHashSet<usize>,
    edges: &AHashSet<usize>,
) -> Option<Vec<Vec<Option<PlacedTile>>>> {
    let bound = graph.len() - 1;
    let (x, y) = current_cell;
    let is_corner = (x == 0 || x == bound) && (y == 0 || y == bound);
    let is_edge = !is_corner && (x == 0 || x == bound || y == 0 || y == bound);

    let unplaced_tiles = (0..tiles.len())
        .filter(|i| !placed[*i] && is_corner == corners.contains(i) && is_edge == edges.contains(i))
        .collect::<Vec<_>>();
    if unplaced_tiles.len() == 0 {
        return Some(graph);
    }
    for idx in unplaced_tiles {
        for pt in try_place_tile(&graph, tiles, current_cell, idx) {
            let mut next_g = graph.clone();
            utils::set_pos(&mut next_g, current_cell, Some(pt.clone()));
            let mut next_placed = placed.clone();
            next_placed[idx] = true;
            let next_index = utils::increment_pos(&next_g, current_cell, GridDir::RIGHT)
                .unwrap_or((0, current_cell.1 + 1));
            let solution = solve(
                next_index as GridIndex,
                tiles,
                next_placed,
                next_g,
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

fn transform_and_crop_pixels(
    pt: &PlacedTile,
    pixels: &[Vec<char>],
    with_crop: bool,
) -> Vec<Vec<char>> {
    let mut new_pixels = pixels.clone().to_vec();

    utils::flip_grid_mut(&mut new_pixels, pt.flip_x, pt.flip_y);

    if pt.rotation > 0 {
        new_pixels = utils::rotate_grid(
            &new_pixels,
            (pt.rotation as f32) * std::f32::consts::FRAC_PI_2,
        );
    }

    // Crop off one pixel.
    if with_crop {
        new_pixels = new_pixels[1..new_pixels.len() - 1].to_vec();
        for row in new_pixels.iter_mut() {
            *row = row[1..row.len() - 1].to_vec();
        }
    }

    new_pixels
}

fn flatten_img(tiles: &[Tile], graph: &[Vec<PlacedTile>], with_crop: bool) -> Vec<Vec<char>> {
    let mut ret = vec![];
    for g_row in graph.iter() {
        let mut rows = vec![];
        for pt in g_row.iter() {
            let pixels = transform_and_crop_pixels(pt, &tiles[pt.tile].pixels, with_crop);

            for (j, pixel_row) in pixels.into_iter().enumerate() {
                if j == rows.len() {
                    rows.push(vec![]);
                }
                rows[j].extend(pixel_row);
                if !with_crop {
                    rows[j].push(' ');
                }
            }
        }
        ret.extend(rows);
        if !with_crop {
            ret.push(vec![' '; ret[0].len()]);
        }
    }
    ret
}

pub fn part1() -> usize {
    let tiles = parse_tiles();

    let tiles_by_edge = build_tiles_by_edge(&tiles);

    let (corners, _) = find_corners_and_edges(&tiles, &tiles_by_edge);

    corners.iter().map(|&i| tiles[i].id as usize).product()
}

static POSITIONS: [GridDir; 15] = [
    GridDir(0, 0),
    GridDir(1, 1),
    GridDir(4, 1),
    GridDir(5, 0),
    GridDir(6, 0),
    GridDir(7, 1),
    GridDir(10, 1),
    GridDir(11, 0),
    GridDir(12, 0),
    GridDir(13, 1),
    GridDir(16, 1),
    GridDir(17, 0),
    GridDir(18, -1),
    GridDir(18, 0),
    GridDir(19, 0),
];

fn is_sea_monster(grid: &[Vec<char>], current_cell: GridIndex) -> bool {
    for pos in POSITIONS.iter() {
        if let Some(idx) = utils::add_pos(grid, current_cell, *pos) {
            if *utils::pos_index(grid, idx) != '#' {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn find_all_sea_monsters(img: &[Vec<char>]) -> Vec<GridIndex> {
    let mut sea_monsters = vec![];
    for y in 1..(img.len() - 1) {
        for x in 0..(img[y].len() - 19) {
            if is_sea_monster(&img, (x, y)) {
                sea_monsters.push((x, y) as GridIndex);
            }
        }
    }
    sea_monsters
}

pub fn part2() -> usize {
    let tiles = parse_tiles();

    let tiles_by_edge = build_tiles_by_edge(&tiles);

    let placed = vec![false; tiles.len()];
    let bounds = (tiles.len() as f32).sqrt() as usize;
    let mut graph = vec![];
    for _ in 0..bounds {
        graph.push(vec![None; bounds]);
    }

    let (corners, edges) = find_corners_and_edges(&tiles, &tiles_by_edge);

    let solution = solve(
        (0usize, 0usize) as GridIndex,
        &tiles,
        placed,
        graph,
        &corners,
        &edges,
    )
    .unwrap();

    let placed_tiles = solution
        .into_iter()
        .map(|row| row.into_iter().map(|t| t.unwrap()).collect_vec())
        .collect_vec();

    let img = flatten_img(&tiles, &placed_tiles, true);

    let mut img_with_sea_monsters: Vec<Vec<char>> = vec![];
    let mut sea_monsters: Vec<GridIndex> = vec![];

    for flip_x in [false, true] {
        for flip_y in [false, true] {
            for rotation in 0..4 {
                img_with_sea_monsters =
                    utils::rotate_grid(&img, (rotation as f32) * std::f32::consts::FRAC_PI_2);
                utils::flip_grid_mut(&mut img_with_sea_monsters, flip_x, flip_y);

                sea_monsters = find_all_sea_monsters(&img_with_sea_monsters);
                if sea_monsters.len() > 0 {
                    break;
                }
            }
        }
    }

    for sea_monster in sea_monsters.iter() {
        for pos in POSITIONS.iter() {
            let p = utils::add_pos(&img_with_sea_monsters, *sea_monster, *pos).unwrap();
            utils::set_pos(&mut img_with_sea_monsters, p, 'O');
        }
    }

    let mut c = 0;
    for y in 0..img_with_sea_monsters.len() {
        for x in 0..img_with_sea_monsters[y].len() {
            if *utils::pos_index(&img_with_sea_monsters, (x, y)) == '#' {
                c += 1;
            }
        }
    }

    c
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn make_tile(id: u32, pixels: Vec<Vec<char>>) -> Tile {
        let edges = get_edges(&pixels);
        Tile { id, pixels, edges }
    }

    fn place_tile(idx: usize) -> PlacedTile {
        PlacedTile {
            tile: idx,
            flip_x: false,
            flip_y: false,
            rotation: 0,
        }
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

        let pt = place_tile(0);

        let mut edge = get_edge(&tiles, &pt, GridDir::UP);
        let mut want_edge = 1;
        assert_eq!(edge, want_edge, "got top edge {}, want {}", edge, want_edge);

        edge = get_edge(&tiles, &pt, GridDir::RIGHT);
        want_edge = 4;
        assert_eq!(
            edge, want_edge,
            "got right edge {}, want {}",
            edge, want_edge
        );

        edge = get_edge(&tiles, &pt, GridDir::DOWN);
        want_edge = 2;
        assert_eq!(
            edge, want_edge,
            "got bottom edge {}, want {}",
            edge, want_edge
        );

        edge = get_edge(&tiles, &pt, GridDir::LEFT);
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

        let pt = PlacedTile {
            tile: 0,
            flip_x: true,
            flip_y: false,
            rotation: 0,
        };

        let mut edge = get_edge(&tiles, &pt, GridDir::UP);
        let mut want_edge = 4;
        assert_eq!(edge, want_edge, "got top edge {}, want {}", edge, want_edge);

        edge = get_edge(&tiles, &pt, GridDir::RIGHT);
        want_edge = 3;
        assert_eq!(
            edge, want_edge,
            "got right edge {}, want {}",
            edge, want_edge
        );

        edge = get_edge(&tiles, &pt, GridDir::DOWN);
        want_edge = 1;
        assert_eq!(
            edge, want_edge,
            "got bottom edge {}, want {}",
            edge, want_edge
        );

        edge = get_edge(&tiles, &pt, GridDir::LEFT);
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

        let pt = PlacedTile {
            tile: 0,
            flip_x: false,
            flip_y: true,
            rotation: 0,
        };

        let mut edge = get_edge(&tiles, &pt, GridDir::UP);
        let mut want_edge = 4;
        assert_eq!(edge, want_edge, "got top edge {}, want {}", edge, want_edge);

        edge = get_edge(&tiles, &pt, GridDir::RIGHT);
        want_edge = 1;
        assert_eq!(
            edge, want_edge,
            "got right edge {}, want {}",
            edge, want_edge
        );

        edge = get_edge(&tiles, &pt, GridDir::DOWN);
        want_edge = 3;
        assert_eq!(
            edge, want_edge,
            "got bottom edge {}, want {}",
            edge, want_edge
        );

        edge = get_edge(&tiles, &pt, GridDir::LEFT);
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

        let pt = PlacedTile {
            tile: 0,
            flip_x: false,
            flip_y: false,
            rotation: 1,
        };

        let mut edge = get_edge(&tiles, &pt, GridDir::UP);
        let mut want_edge = 0;
        assert_eq!(edge, want_edge, "got top edge {}, want {}", edge, want_edge);

        edge = get_edge(&tiles, &pt, GridDir::RIGHT);
        want_edge = 1;
        assert_eq!(
            edge, want_edge,
            "got right edge {}, want {}",
            edge, want_edge
        );

        edge = get_edge(&tiles, &pt, GridDir::DOWN);
        want_edge = 1;
        assert_eq!(
            edge, want_edge,
            "got bottom edge {}, want {}",
            edge, want_edge
        );

        edge = get_edge(&tiles, &pt, GridDir::LEFT);
        want_edge = 2;
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

        let pt1 = place_tile(0);
        let pt2 = place_tile(1);
        let pt3 = place_tile(2);
        let pt4 = place_tile(3);
        let pt5 = place_tile(4);

        assert!(is_match(&tiles, &pt1, &pt2, GridDir::UP), "top match");
        assert!(is_match(&tiles, &pt1, &pt3, GridDir::RIGHT), "right match");
        assert!(is_match(&tiles, &pt1, &pt4, GridDir::DOWN), "bottom match");
        assert!(is_match(&tiles, &pt1, &pt5, GridDir::LEFT), "left match");
    }

    #[test]
    fn run_part1() {
        assert_eq!(part1(), 111);
    }

    #[test]
    fn run_part2() {
        assert_eq!(part2(), 343);
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
