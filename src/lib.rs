#![doc = include_str!("../README.md")]

pub const ORDERING: core::sync::atomic::Ordering = core::sync::atomic::Ordering::SeqCst;

/// See the [crate level documentation](index.html) for more information.
#[macro_export]
macro_rules! flip_flop {
    (($start:expr)..($end:expr)) => {
        $crate::flip_flop!(@go $start, $end, true)
    };

    (($start:expr)...($end:expr)) => {
        $crate::flip_flop!(@go $start, $end, false)
    };

    (@go $start:expr, $end:expr, $double:expr) => {{
        static FLIP_FLOP: core::sync::atomic::AtomicBool =
            core::sync::atomic::AtomicBool::new(false);

        if !FLIP_FLOP.load($crate::ORDERING) {
            if $start {
                if !$double || !$end {
                    FLIP_FLOP.store(true, $crate::ORDERING);
                }
                true
            } else {
                false
            }
        } else {
            if $end {
                FLIP_FLOP.store(false, $crate::ORDERING);
            }
            true
        }
    }};
}
