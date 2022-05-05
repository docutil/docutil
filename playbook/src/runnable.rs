pub trait Runnable {
    fn name(&self);
    fn run(&self) -> Result<(), Box<dyn std::error::Error>>;
}
