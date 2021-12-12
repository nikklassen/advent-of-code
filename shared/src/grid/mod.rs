use std::{
    fmt::Display,
    iter::repeat,
    num::ParseIntError,
    slice::{ChunksExact, ChunksExactMut},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridIndex(pub usize, pub usize);

impl Display for GridIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl From<(usize, usize)> for GridIndex {
    fn from((x, y): (usize, usize)) -> Self {
        GridIndex(x, y)
    }
}

impl FromStr for GridIndex {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .collect();

        let x_fromstr = coords[0].parse::<usize>()?;
        let y_fromstr = coords[1].parse::<usize>()?;

        Ok(GridIndex(x_fromstr, y_fromstr))
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct GridDir(pub isize, pub isize);

impl GridDir {
    pub const UP: GridDir = GridDir(0, -1);
    pub const RIGHT: GridDir = GridDir(1, 0);
    pub const DOWN: GridDir = GridDir(0, 1);
    pub const LEFT: GridDir = GridDir(-1, 0);

    pub fn flip(&self) -> GridDir {
        GridDir(self.0 * -1, self.1 * -1)
    }
}

pub static ADJACENT_DIRS: [GridDir; 8] = [
    GridDir(-1, -1),
    GridDir::UP,
    GridDir(-1, 1),
    GridDir::LEFT,
    GridDir::RIGHT,
    GridDir(1, -1),
    GridDir::DOWN,
    GridDir(1, 1),
];

pub static CARDINAL_DIRS: [GridDir; 4] =
    [GridDir::UP, GridDir::LEFT, GridDir::RIGHT, GridDir::DOWN];

#[derive(Clone, PartialEq)]
pub struct Grid<T> {
    cells: Vec<T>,
    height: usize,
    width: usize,
}

impl<T> Grid<T> {
    pub fn from_vec(v: Vec<Vec<T>>) -> Self {
        let height = v.len();
        let width = v.first().map(|row| row.len()).unwrap_or(0);
        Grid {
            cells: v.into_iter().flatten().collect(),
            height,
            width,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, pos: GridIndex) -> Option<&T> {
        self.add_offset(pos, GridDir(0, 0)).map(|pos| &self[pos])
    }

    pub fn add_offset(
        &self,
        GridIndex(pos_x, pos_y): GridIndex,
        GridDir(dir_x, dir_y): GridDir,
    ) -> Option<GridIndex> {
        let new_x = (pos_x as isize) + dir_x;
        let new_y = (pos_y as isize) + dir_y;

        if new_y < 0 || new_y >= self.height as isize || new_x < 0 || new_x >= self.width as isize {
            None
        } else {
            Some(GridIndex(new_x as usize, new_y as usize))
        }
    }

    pub fn rows(&self) -> ChunksExact<T> {
        self.cells.chunks_exact(self.width)
    }

    pub fn rows_mut(&mut self) -> ChunksExactMut<T> {
        self.cells.chunks_exact_mut(self.width)
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, T> {
        self.cells.iter()
    }

    pub fn enumerate_cells<'a>(&'a self) -> impl std::iter::Iterator<Item = (GridIndex, &'a T)> {
        self.indexes().map(|idx| (idx, &self[idx]))
    }

    pub fn indexes(&self) -> impl std::iter::Iterator<Item = GridIndex> + '_ {
        (0..self.height).flat_map(|y| (0..self.width).zip(repeat(y)).map(|(x, y)| GridIndex(x, y)))
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, T> {
        self.cells.iter_mut()
    }
}

impl<T: Default> Grid<T> {
    pub fn new(size: usize) -> Self {
        Grid::with_bounds(size, size)
    }

    pub fn with_bounds(width: usize, height: usize) -> Self {
        let mut cells = Vec::with_capacity(width * height);
        unsafe {
            cells.set_len(width * height);
        }
        cells.fill_with(T::default);
        Grid {
            cells,
            height,
            width,
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn from_elem(elem: T, size: usize) -> Self {
        Grid {
            cells: vec![elem; size * size],
            height: size,
            width: size,
        }
    }
}

impl<T> std::ops::Index<GridIndex> for Grid<T> {
    type Output = T;

    fn index(&self, GridIndex(x, y): GridIndex) -> &Self::Output {
        &self.cells[y * self.width + x]
    }
}

impl<T> std::ops::IndexMut<GridIndex> for Grid<T> {
    fn index_mut(&mut self, GridIndex(x, y): GridIndex) -> &mut Self::Output {
        &mut self.cells[y * self.width + x]
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.into_iter()
    }
}

impl<T: Display> std::fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows() {
            for e in row {
                write!(f, "{}", e)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows() {
            for e in row {
                write!(f, "{:?}", e)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
