#[macro_export]
macro_rules! result_assert {
    ($x:expr) => {{
        result_assert!($x, format!("False in '{}'", stringify!($x)))
    }};
    ($x:expr, $str:expr) => {{
        match $x {
            true => Ok(()),
            false => Err($str),
        }
    }};
}

#[macro_export]
macro_rules! result_assert_eq {
    ($x:expr, $y:expr) => {
        crate::result_assert!($x == $y)
    };
    ($x:expr, $y:expr, $str:expr) => {
        crate::result_assert!($x == $y, $str)
    };
}

#[cfg(test)]
mod tests {
    mod result_assert {
        #[test]
        fn expression() {
            assert_eq!(result_assert!(2 == 2), Ok(()));
            assert_eq!(result_assert!(2 != 2), Err("False in '2 != 2'".to_string()));
        }

        #[test]
        fn expression_with_err() {
            assert_eq!(result_assert!(2 == 2, 4), Ok(()));
            assert_eq!(result_assert!(2 != 2, 4), Err(4));
        }
    }

    mod result_assert_eq {
        #[test]
        fn expression() {
            assert_eq!(result_assert_eq!(2, 2), Ok(()));
            assert_eq!(
                result_assert_eq!(2, 3),
                Err("False in '2 == 3'".to_string())
            );
        }

        #[test]
        fn expression_with_err() {
            assert_eq!(result_assert_eq!(2, 2, 4), Ok(()));
            assert_eq!(result_assert_eq!(2, 3, 4), Err(4));
        }
    }
}
