#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub source_start: i64,
    pub source_end: i64,
    pub destination_start: i64,
    pub destination_end: i64,
    pub range: i64,
}
