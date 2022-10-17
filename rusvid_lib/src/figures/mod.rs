pub mod circle;
pub mod rect;
pub mod triangle;

pub mod prelude {
    pub use super::circle::circle;
    pub use super::rect::rect;
    pub use super::triangle::equilateral_triangle;
}
