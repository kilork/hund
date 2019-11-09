use failure::Fail;

#[derive(Fail, Debug)]
enum HundError {
    #[fail(display = "I'm hungry! Woof!")]
    Hungry,
}
