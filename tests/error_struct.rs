use autoerror::AutoError;

#[derive(Debug, AutoError)]
struct Error {
    A: std::io::Error,
    B: std::fmt::Error,
}

pub fn main() {
}
