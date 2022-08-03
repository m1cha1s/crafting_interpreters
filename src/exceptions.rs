#[derive(Debug)]
pub enum Exce {
    UnknownChar { line: u64, column: u64 },
}
