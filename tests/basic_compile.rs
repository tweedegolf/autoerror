use autoerror::AutoError;

#[derive(Debug, AutoError)]
enum Error {
    #[auto_error(err=true)]
    A(std::io::Error),
    B(std::fmt::Error),
}

pub fn main() {
}
