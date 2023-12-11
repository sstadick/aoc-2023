use std::{
    error::Error,
    fmt::{self, Debug, Display},
    fs::File,
    io::{BufRead, BufReader, Read},
    marker::PhantomData,
    ops::{Index, IndexMut},
    path::Path,
    str::FromStr,
};

use ordered_float::OrderedFloat;

#[derive(Debug, Clone)]
pub struct SlurpError {
    line: usize,
    msg: String,
}

impl fmt::Display for SlurpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error at line {}: {}", self.line, self.msg)
    }
}

impl Error for SlurpError {}

pub fn slurp_bytes<P>(path: P) -> Result<Vec<u8>, SlurpError>
where
    P: AsRef<Path>,
{
    let mut reader = BufReader::new(File::open(path).expect("Failed to open file"));
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes).map_err(|e| SlurpError { line: 0, msg: e.to_string() })?;
    Ok(bytes)
}

/// Slurp file will try to parse the string into `T` as long as T implements FromStr
#[allow(clippy::missing_errors_doc)]
pub fn slurp_file<P, T>(path: P) -> impl Iterator<Item = Result<T, <T as FromStr>::Err>>
where
    P: AsRef<Path>,
    T: FromStr,
    <T as FromStr>::Err: Error,
{
    let reader =
        BufReader::new(File::open(&path).map(BufReader::new).expect("Failed to open file"));

    FromStrIter { reader: reader, buffer: String::new(), phantom: PhantomData }
}

pub struct FromStrIter<R: BufRead, T: FromStr> {
    reader: R,
    buffer: String,
    phantom: PhantomData<T>,
}

impl<R: BufRead, T: FromStr> Iterator for FromStrIter<R, T> {
    type Item = Result<T, <T as FromStr>::Err>;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear();
        if let Ok(size) = self.reader.read_line(&mut self.buffer) {
            if size == 0 {
                return None;
            }
            Some(self.buffer.trim_end().parse::<T>())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    msg: String,
}
impl ParseError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error for command: {}", self.msg)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Grid<T>
where
    T: Clone + Debug,
{
    data: Vec<T>,
    pub row_size: usize,
    pub col_size: usize,
}

impl<T> Grid<T>
where
    T: Clone + Debug,
{
    /// Create the grid from bytes read from a file. This uses and handles the newlines.
    ///
    /// Example input:
    ///
    /// ```
    /// 01234\n
    /// 56789\n
    /// ```
    pub fn from_file_bytes(bytes: &[u8]) -> Self
    where
        T: From<u8>,
    {
        // Account for possible missing last newline
        let extra = if *bytes.last().expect("Given zero length bytes") == b'\n' { 0 } else { 1 };
        let row_size =
            bytes.iter().position(|b| *b == b'\n').expect("No newline found in passed in bytes.");
        let col_size = (bytes.len() + extra) / (row_size + 1);

        let mut data = Vec::with_capacity(row_size * col_size);
        for row in bytes.chunks(row_size + 1) {
            for value in row.into_iter().copied().filter(|v| *v != b'\n') {
                data.push(value.into());
            }
        }

        Self { data, row_size, col_size }
    }

    pub fn iter_rows<'a>(&'a self) -> GridRowIter<'a, T> {
        GridRowIter { grid: self, current_row: 0 }
    }

    pub fn iter_cols(&self) -> GridColIter<T> {
        GridColIter { grid: self, current_col: 0 }
    }

    pub fn iter_points(&self) -> PointIter<T> {
        PointIter { grid: self, current_point: Point { x: 0, y: 0 } }
    }

    /// Insert a row at the given index, the row at that current index will be moved down one.
    ///
    /// `insert_at` is the row index, or Y value.
    ///
    /// ```
    /// 0, 1, 2, 3, 4
    /// 5, 6, 7, 8, 9
    /// ```
    ///
    /// insert_at 1 `10, 11, 12, 13, 14`
    ///
    /// ```
    /// 0,  1,  2,  3,  4
    /// 10, 11, 12, 13, 14
    /// 5,  6,  7,  8,  9
    /// ```
    pub fn insert_row_at(&mut self, row: &[T], insert_at: usize) {
        assert_eq!(
            row.len(),
            self.row_size,
            "Can't insert a row whose size doesn't match the existing grid row sizes"
        );
        let (before, after) = self.data.split_at(insert_at * self.row_size);
        let mut new_data = Vec::with_capacity(before.len() + after.len() + row.len());
        new_data.extend_from_slice(before);
        new_data.extend_from_slice(row);
        new_data.extend_from_slice(after);

        self.data = new_data;
        self.col_size += 1;
    }

    /// Insert a column at the given index, the column at that current index will be moved one over.
    ///
    /// `insert_at` is the col index, or X value.
    ///
    /// ```
    /// 0, 1, 2, 3, 4
    /// 5, 6, 7, 8, 9
    /// ```
    ///
    /// insert_at 1 `10, 11`
    ///
    /// ```
    /// 0, 10, 1,  2,  3,  4
    /// 5, 11, 6,  7,  8,  9
    /// ```
    pub fn insert_col_at(&mut self, col: &[T], insert_at: usize) {
        assert_eq!(
            col.len(),
            self.col_size,
            "Can't insert a column whose size doesn't match the existing grid col sizes"
        );

        // Insert in reverse order to not throw off indexing
        for (row_index, col_value) in col.iter().cloned().enumerate().rev() {
            let index = (self.row_size * row_index) + insert_at;
            self.data.insert(index, col_value);
        }
        self.row_size += 1;
    }
}

impl<T> Display for Grid<T>
where
    T: Clone + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.iter_rows() {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}

pub struct PointIter<'a, T>
where
    T: Clone + Debug,
{
    grid: &'a Grid<T>,
    current_point: Point,
}

impl<'a, T> Iterator for PointIter<'a, T>
where
    T: Clone + Debug,
{
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_point.y >= self.grid.col_size {
            return None;
        }
        let p = self.current_point;
        if self.current_point.x + 1 == self.grid.row_size {
            self.current_point.x = 0;
            self.current_point.y += 1;
        } else {
            self.current_point.x += 1;
        }
        Some((p, &self.grid[p]))
    }
}

#[derive(Debug)]
pub struct GridRowIter<'a, T>
where
    T: Clone + Debug,
{
    grid: &'a Grid<T>,
    current_row: usize,
}

impl<'a, T> Iterator for GridRowIter<'a, T>
where
    T: Clone + Debug,
{
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.grid.row_size * self.current_row;
        if start >= self.grid.data.len() {
            None
        } else {
            self.current_row += 1;
            Some(&self.grid.data[start..start + self.grid.row_size])
        }
    }
}

#[derive(Debug)]
pub struct GridColIter<'a, T>
where
    T: Clone + Debug,
{
    grid: &'a Grid<T>,
    current_col: usize,
}

impl<'a, T> Iterator for GridColIter<'a, T>
where
    T: Clone + Debug,
{
    type Item = SingleColIter<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_col < self.grid.row_size {
            let col = self.current_col;
            self.current_col += 1;
            Some(SingleColIter { grid: self.grid, current_point: Point { x: col, y: 0 } })
        } else {
            None
        }
    }
}

pub struct SingleColIter<'a, T>
where
    T: Clone + Debug,
{
    grid: &'a Grid<T>,
    current_point: Point,
}

impl<'a, T> Iterator for SingleColIter<'a, T>
where
    T: Clone + Debug,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_point.y >= self.grid.col_size {
            None
        } else {
            let point = self.current_point;
            self.current_point.y += 1;
            Some(&self.grid[point])
        }
    }
}

impl<T> Index<Point> for Grid<T>
where
    T: Clone + Debug,
{
    type Output = T;

    /// Index by coordinate works by assuming the top left value is (0, 0):
    ///
    /// ```
    /// 0, 1, 2, 3, 4
    /// 5, 6, 7, 8, 9
    /// ```
    ///
    /// (3, 1) in this case is the value 8.
    fn index(&self, index: Point) -> &Self::Output {
        &self.data[(self.row_size * index.y) + index.x]
    }
}

impl<T> IndexMut<Point> for Grid<T>
where
    T: Clone + Debug,
{
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.data[(self.row_size * index.y) + index.x]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    /// Compute the distance between two points.
    pub fn distance(&self, other: &Self) -> f64 {
        ((other.x as f64 - self.x as f64).powi(2) + (other.y as f64 - self.y as f64).powi(2)).sqrt()
    }

    /// Find the minimal number of whole number steps to get from self to other
    pub fn steps_to(&self, other: &Self) -> usize {
        let mut current = self.clone();
        let mut steps = 0;
        let directions_to_try = &mut [current; 4];
        while current != *other {
            directions_to_try.fill(current);
            directions_to_try[0].x += 1;
            directions_to_try[1].x = directions_to_try[1].x.saturating_sub(1);
            directions_to_try[2].y += 1;
            directions_to_try[3].y = directions_to_try[3].y.saturating_sub(1);

            current =
                *directions_to_try.iter().min_by_key(|d| OrderedFloat(other.distance(*d))).unwrap();
            steps += 1;
        }
        steps
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(&[0, 1, 2, 3, 4, b'\n', 5, 6, 7, 8, 9, b'\n'], Grid { data: vec![0, 1, 2, 3, 4, 5, 6,7, 8, 9], row_size: 5, col_size: 2})]
    #[case(&[0, 1, 2, 3, 4, b'\n', 5, 6, 7, 8, 9], Grid { data: vec![0, 1, 2, 3, 4, 5, 6,7, 8, 9], row_size: 5, col_size: 2})]
    fn test_grid_from_bytes(#[case] input: &[u8], #[case] expected: Grid<u8>) {
        let grid = Grid::from_file_bytes(input);
        assert_eq!(grid, expected);
    }

    #[rstest]
    #[case(Grid { data: vec![0, 1, 2, 3, 4, 5, 6,7, 8, 9], row_size: 5, col_size: 2}, vec![5, 6, 7, 8, 9])]
    fn test_row_iter(#[case] input: Grid<u8>, #[case] expected: Vec<u8>) {
        assert_eq!(input.iter_rows().last().unwrap(), expected)
    }

    #[rstest]
    #[case(Grid { data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], row_size: 5, col_size: 2}, vec![0, 5, 1, 6, 2, 7, 3, 8, 4, 9])]
    fn test_col_iter(#[case] input: Grid<u8>, #[case] expected: Vec<u8>) {
        assert_eq!(
            input
                .iter_cols()
                .into_iter()
                .map(|c| c.into_iter().copied())
                .flatten()
                .collect::<Vec<_>>(),
            expected
        )
    }

    #[rstest]
    #[case(Point { x: 0, y: 0}, 0)]
    #[case(Point { x: 3, y: 1}, 8)]
    #[case(Point { x: 4, y: 0}, 4)]
    #[case(Point { x: 0, y: 1}, 5)]
    #[case(Point { x: 4, y: 1}, 9)]
    fn test_index(#[case] index: Point, #[case] expected: u8) {
        let grid = Grid { data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], row_size: 5, col_size: 2 };
        assert_eq!(grid[index], expected)
    }

    #[rstest]
    #[case((vec![10, 11, 12, 13, 14], 1), &[0, 1, 2, 3, 4, 10, 11, 12, 13, 14, 5, 6, 7, 8, 9])]
    fn test_insert_row(#[case] to_insert: (Vec<u8>, usize), #[case] expected: &[u8]) {
        let mut grid = Grid { data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], row_size: 5, col_size: 2 };
        grid.insert_row_at(&to_insert.0, to_insert.1);
        assert_eq!(&grid.data, expected);
        assert_eq!(&grid.iter_rows().flatten().copied().collect::<Vec<_>>(), expected)
    }

    #[rstest]
    #[case((vec![10, 11], 1), &[0, 10, 1, 2, 3, 4, 5, 11, 6, 7, 8, 9])]
    fn test_insert_col(#[case] to_insert: (Vec<u8>, usize), #[case] expected: &[u8]) {
        let mut grid = Grid { data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], row_size: 5, col_size: 2 };
        grid.insert_col_at(&to_insert.0, to_insert.1);
        assert_eq!(&grid.data, expected);
        assert_eq!(&grid.iter_rows().flatten().copied().collect::<Vec<_>>(), expected)
    }

    #[rstest]
    #[case(vec![
        (Point {x: 0, y: 0}, 0_u8),
        (Point {x: 1, y: 0}, 1_u8),
        (Point {x: 2, y: 0}, 2_u8),
        (Point {x: 3, y: 0}, 3_u8),
        (Point {x: 4, y: 0}, 4_u8),
        (Point {x: 0, y: 1}, 5_u8),
        (Point {x: 1, y: 1}, 6_u8),
        (Point {x: 2, y: 1}, 7_u8),
        (Point {x: 3, y: 1}, 8_u8),
        (Point {x: 4, y: 1}, 9_u8),
    ])]
    fn test_point_iter(#[case] expected: Vec<(Point, u8)>) {
        let grid = Grid { data: vec![0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9], row_size: 5, col_size: 2 };
        let found = grid.iter_points().collect::<Vec<_>>();
        assert_eq!(found.len(), expected.len());
        for (f, e) in found.into_iter().zip(expected.into_iter()) {
            assert_eq!(f.0, e.0);
            assert_eq!(*f.1, e.1);
        }
    }
}
