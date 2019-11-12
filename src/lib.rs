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

const HUND_SETTINGS: &str = ".hund";
const HUND_CONFIG: &str = "config";
const HUND_TOML: &str = "hund.toml";

pub use error::HundError;
