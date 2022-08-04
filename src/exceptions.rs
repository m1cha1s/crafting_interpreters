#[derive(Debug)]
pub enum Exce {
    UnexpectedChar { line: u32, column: u32 },
    UnterminatedString { line: u32, column: u32 },
}
