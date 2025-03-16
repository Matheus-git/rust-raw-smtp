pub trait UserAgent {
    fn conn(&mut self);
    fn hello(&mut self);
    fn from(&mut self, from: String);
    fn to(&mut self, to: String);
    fn data(&mut self, data: String);
    fn quit(&mut self);
}