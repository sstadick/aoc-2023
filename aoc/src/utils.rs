use std::{
    error::Error,
    fmt::{self, Debug},
    fs::File,
    io::{BufRead, BufReader, Read},
    marker::PhantomData,
    path::Path,
    str::FromStr,
};

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
