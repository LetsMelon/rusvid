pub trait MetricsSize {
    fn bytes(&self) -> usize;
}

pub trait MetricsVideo {
    fn frames(&self) -> usize;
    fn pixels(&self) -> usize;
}
