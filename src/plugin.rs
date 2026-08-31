pub trait RbrPlugin {
    const NAME: &'static str;

    fn new() -> Self;
}