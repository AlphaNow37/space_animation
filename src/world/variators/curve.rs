pub trait Curve<T> {
    fn pt(&self, t: f32) -> T;
    fn speed(&self, t: f32) -> T;
}

// pub struct Line()
