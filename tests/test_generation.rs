use autoerror::AutoError;

mod e1 {
    #[derive(Debug)]
    pub struct Error {}

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str("test")
        }
    }
    
    impl std::error::Error for Error {}
}

mod e2 {
    #[derive(Debug)]
    pub struct Error {}

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str("test")
        }
    }
    
    impl std::error::Error for Error {}
}

mod e3 {
    #[derive(Debug)]
    pub struct NotError {}

    impl std::fmt::Display for NotError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str("test")
        }
    }
    
    impl std::error::Error for NotError {}
}

mod e4 {
    #[derive(Debug)]
    pub struct NotError {}

    impl std::fmt::Display for NotError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str("test")
        }
    }
    
    impl std::error::Error for NotError {}
}

#[derive(Debug, AutoError)]
enum Error {
    A(e1::Error),
    #[auto_error(err=false, make_from=false, format_str = "Error {}")]
    B(e2::Error),
    #[auto_error(err=true, make_from=true)]
    C(e3::NotError),
    #[auto_error(make_from=true)]
    D(e4::NotError),
    E(String, isize),
    F(),
}

impl From<e2::Error> for Error {
    fn from (e: e2::Error) -> Error {
        Error::B(e)
    }
}

use std::error::Error as StdError;

pub fn main() {
    let a = Error::from(e1::Error{});
    assert_eq!(format!("{}", a), "test");
    assert!(a.source().is_some());

    let b = Error::from(e2::Error{});
    assert_eq!(format!("{}", b), "Error test");
    assert!(b.source().is_none());

    let c = Error::from(e3::NotError{});
    assert_eq!(format!("{}", c), "test");
    assert!(c.source().is_some());

    let d = Error::from(e4::NotError{});
    assert_eq!(format!("{}", d), "test");
    assert!(d.source().is_none());

    let e = Error::E("bla".to_string(), 5);
    assert_eq!(format!("{}", e), "bla 5");
    assert!(e.source().is_none());

    let f = Error::F();
    assert_eq!(format!("{}", f), "");
    assert!(f.source().is_none());
}
