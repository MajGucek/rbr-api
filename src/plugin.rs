pub trait RbrPlugin {
    const ID: &'static str;
    const NAME: &'static str;

    fn new() -> Self;
}