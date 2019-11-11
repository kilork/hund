/*!
# hund description

## Features

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
hund = "0.1"
```

*/

pub(crate) mod app;
pub mod command;
pub(crate) mod config;
pub mod error;

const SETTINGS: &str = ".hund/config";

pub use error::HundError;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
