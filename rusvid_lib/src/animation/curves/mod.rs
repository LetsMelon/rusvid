use anyhow::Result;

pub mod linear;

/// ```rust
/// use rusvid_lib::animation::curves::linear::Linear;
/// use rusvid_lib::animation::curves::Function;
///
/// let linear: Linear<f64> = Linear::new(30, 90, 100.0, 300.0).unwrap();
///
/// assert_eq!(linear.calc(30), 100.0);
/// assert_eq!(linear.calc(90), 300.0);
///
/// assert_eq!(linear.calc(20), 100.0);
/// assert_eq!(linear.calc(100), 300.0);
/// ```
pub trait Function {
    type Value;

    fn new(
        start_frame: usize,
        end_frame: usize,
        start: Self::Value,
        end: Self::Value,
    ) -> Result<Self>
    where
        Self: Sized;

    fn start_frame(&self) -> usize;
    fn end_frame(&self) -> usize;
    fn start(&self) -> Self::Value;
    fn end(&self) -> Self::Value;

    fn calc_raw(&self, frame_number: usize) -> Self::Value;
    fn calc(&self, frame_number: usize) -> Self::Value {
        if frame_number < self.start_frame() {
            return self.start();
        } else if frame_number > self.end_frame() {
            return self.end();
        }
        self.calc_raw(frame_number)
    }
}
