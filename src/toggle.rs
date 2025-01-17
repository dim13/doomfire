pub struct Toggle<T>(T, T);

impl<T: Copy> Toggle<T> {
    pub fn new(a: T, b: T) -> Self {
        Toggle(a, b)
    }
    pub fn toggle(&mut self) -> T {
        *self = Toggle(self.1, self.0);
        self.0
    }
}
