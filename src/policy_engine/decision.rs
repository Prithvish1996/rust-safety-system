#[derive(Debug)]
pub enum Decision {
    Safe,
    Warning(String),
    Critical(String),
}