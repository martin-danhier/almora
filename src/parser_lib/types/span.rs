use super::Location;

#[derive(Debug, Clone)]
pub struct Span {
    pub start: Location,
    pub end: Location,
}