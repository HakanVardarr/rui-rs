pub trait Component {
    fn draw(&self) -> super::Result<()>;
}
