use failure::Fail;

#[derive(Fail, Debug)]
pub enum HundError {
    #[fail(display = "I'm hungry! Woof!")]
    Hungry,
    #[fail(display = "Path already exists: {}", _0)]
    PathExists(String),
}
