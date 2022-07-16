use anyhow::{bail, Result};

use crate::animation::curves::Function;
use crate::types::Point2d;

#[derive(Debug, Copy, Clone)]
pub struct Linear<T> {
    start_frame: usize,
    end_frame: usize,
    start: T,
    end: T,

    // f(x) = kx+d
    k: T,
    d: T,
}

impl Function for Linear<f64> {
    type Value = f64;

    fn new(
        start_frame: usize,
        end_frame: usize,
        start: Self::Value,
        end: Self::Value,
    ) -> Result<Self> {
        if start_frame > end_frame {
            bail!("`start_frame` has to be smaller than `end_frame`");
        }

        let d = 0.0;
        let k = start / start_frame as f64;

        Ok(Linear {
            start_frame,
            end_frame,
            start,
            end,
            d,
            k,
        })
    }

    fn calc(&self, frame_number: usize) -> Self::Value {
        if frame_number < self.start_frame {
            return self.start;
        } else if frame_number > self.end_frame {
            return self.end;
        }
        self.k * (frame_number as f64) + self.d
    }
}

impl Function for Linear<Point2d> {
    type Value = Point2d;

    fn new(
        start_frame: usize,
        end_frame: usize,
        start: Self::Value,
        end: Self::Value,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        if start_frame > end_frame {
            bail!("`start_frame` has to be smaller than `end_frame`");
        }

        let d = (0.0, 0.0);
        let mut k = (0.0, 0.0);
        k.0 = start.0 / start_frame as f64;
        k.1 = start.1 / start_frame as f64;

        Ok(Linear {
            start_frame,
            end_frame,
            start,
            end,
            d,
            k,
        })
    }

    fn calc(&self, frame_number: usize) -> Self::Value {
        if frame_number < self.start_frame {
            return self.start;
        } else if frame_number > self.end_frame {
            return self.end;
        }
        (
            self.k.0 * (frame_number as f64) + self.d.0,
            self.k.1 * (frame_number as f64) + self.d.1,
        )
    }
}

#[cfg(test)]
mod tests {

    mod f64 {
        use crate::animation::curves::linear::Linear;
        use crate::animation::curves::Function;

        #[test]
        fn just_works() {
            let linear: Linear<f64> = Linear::new(30, 90, 100.0, 300.0).unwrap();

            assert_eq!(linear.calc(30), 100.0);
            assert_eq!(linear.calc(60), 200.0);
            assert_eq!(linear.calc(90), 300.0);

            assert_eq!(linear.calc(20), 100.0);
            assert_eq!(linear.calc(100), 300.0);
        }

        mod error {
            use super::Linear;
            use crate::animation::curves::Function;
            use anyhow::Result;

            #[test]
            fn in_constructor() {
                let linear: Result<Linear<f64>> = Linear::new(90, 30, 100.0, 300.0);

                assert_eq!(
                    linear.err().unwrap().to_string(),
                    "`start_frame` has to be smaller than `end_frame`"
                );
            }
        }
    }

    mod Point2d {
        use crate::animation::curves::linear::Linear;
        use crate::animation::curves::Function;
        use crate::types::Point2d;

        #[test]
        fn just_works() {
            let linear: Linear<Point2d> =
                Linear::new(30, 90, (100.0, 100.0), (300.0, 300.0)).unwrap();

            assert_eq!(linear.calc(30), (100.0, 100.0));
            assert_eq!(linear.calc(60), (200.0, 200.0));
            assert_eq!(linear.calc(90), (300.0, 300.0));

            assert_eq!(linear.calc(20), (100.0, 100.0));
            assert_eq!(linear.calc(100), (300.0, 300.0));
        }

        mod error {
            use crate::animation::curves::linear::Linear;
            use crate::animation::curves::Function;
            use crate::types::Point2d;
            use anyhow::Result;

            #[test]
            fn in_constructor() {
                let linear: Result<Linear<Point2d>> =
                    Linear::new(90, 30, (100.0, 100.0), (300.0, 300.0));

                assert_eq!(
                    linear.err().unwrap().to_string(),
                    "`start_frame` has to be smaller than `end_frame`"
                );
            }
        }
    }
}
