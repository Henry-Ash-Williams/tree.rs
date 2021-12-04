#[macro_use]
pub mod ps {
    #[macro_export]
    macro_rules! pretty_assert_eq {
        ($lhs:expr, $rhs:expr) => {{
            let _lhs = &$lhs;
            let _rhs = &$rhs;
            assert!(
                _lhs == _rhs,
                "Failed assertion lhs == rhs, lhs: \n{:#?}",
                _lhs
            );
            assert!(
                _rhs == _lhs,
                "Failed assertion rhs == lhs, rhs: \n{:#?}",
                _lhs
            )
        }};
    }
    
    #[macro_export]
    macro_rules! pretty_assert {
        ($x:expr) => {{
            let _x = $x; 
            assert!(
                _x,
                "Failed assertion: \n{:#?}",
                _x
            );
        }};
    }

    #[macro_export]
    macro_rules! echo {
        ($x:expr) => {{
            assert!(false, "State:\n{:#?}")
        }};
    }
}
pub use ps::*;
