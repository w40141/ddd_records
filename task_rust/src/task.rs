pub trait Task {
    fn id(&self) -> &i64;
    fn title(&self) -> &String;
}
