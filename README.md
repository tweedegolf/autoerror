Derive basic error type infrastruture for enum types.

Supports unnamed and unit enum variants, and uses the type definition
to derive `std::fmt::Display` and `std::error:Error` for the error type,
as well as `std::from::From<T>` for any unnamed variant with one parameter
inferred to be an error type (currently determined by whether it's type
name is Error).

Default behaviour can be overridden with the auto_error attribute
- format_str takes a string which becomes the format string for that
  variant
- make_from forces derivation of std::from::From when set to true
- err forces the std::error::Error implementation to return the inner
  type during calls to source, or in other words to treat the inner
  type as an error type.

From derivation and source returning work only for variants with a single field.

# Example

```
#[derive(AutoError)]
use autoerror::AutoError;

enum Error {
    #[auto_error(format_str="Document not found")]
    NotFound,
    IO(std::io::Error),
    #[auto_error(make_from=true)]
    Other(String),
}
```
