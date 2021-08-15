use std::fmt::{Debug, Formatter};

/// Write a slice of `Debug` to `Formatter`, truncating to the specified length.
pub fn format_debug_slice_truncated_to_max_length(
    f: &mut Formatter<'_>,
    debug_slice: &[impl Debug],
    max_length: usize,
) -> Result<(), std::fmt::Error> {
    if max_length == 0 {
        if debug_slice.is_empty() {
            write!(f, "{:?}", debug_slice)
        } else {
            write!(f, "(length={})", debug_slice.len())
        }
    } else if debug_slice.len() <= max_length {
        write!(f, "{:?}", debug_slice)
    } else {
        write!(
            f,
            "(length={} truncated to {}) {:?}...",
            debug_slice.len(),
            max_length,
            &debug_slice[0..max_length]
        )
    }
}

/// Trait fixing the slice truncation length.
trait TruncateSliceDebug<'slice, D: Debug> {
    const MAX_LENGTH: usize;

    fn format_debug_truncated_to_max_length(
        &self,
        f: &mut Formatter<'_>,
        debug_slice: &'slice [D],
    ) -> Result<(), std::fmt::Error> {
        format_debug_slice_truncated_to_max_length(f, debug_slice, Self::MAX_LENGTH)
    }
}

/// Truncate slices to `MAX_LENGTH` declared in impl.
pub struct TruncatedDebug<'slice, D: Debug> {
    slice: &'slice [D],
}

impl<'slice, D: Debug> TruncateSliceDebug<'slice, D> for TruncatedDebug<'slice, D> {
    const MAX_LENGTH: usize = 25;
}

impl<'slice, D: Debug> Debug for TruncatedDebug<'slice, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.format_debug_truncated_to_max_length(f, self.slice)
    }
}

#[allow(dead_code)]
/// Helper function to wrap a slice in a struct that will truncate its debug output.
pub fn truncated_debug<D: Debug>(slice: &[D]) -> TruncatedDebug<D> {
    TruncatedDebug { slice }
}

#[cfg(test)]
mod tests {
    mod test_truncated_slice {
        use crate::debug_slice_truncation::TruncateSliceDebug;
        use std::fmt::{Debug, Formatter};
        //        use linked_hash_set::LinkedHashSet;

        /// Test struct truncating to MAX_LENGTH: 5
        pub struct TestTruncatedDebug<'slice, D: Debug> {
            slice: &'slice [D],
        }

        impl<'slice, D: Debug> TruncateSliceDebug<'slice, D> for TestTruncatedDebug<'slice, D> {
            const MAX_LENGTH: usize = 5;
        }

        impl<'slice, D: Debug> Debug for TestTruncatedDebug<'slice, D> {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
                self.format_debug_truncated_to_max_length(f, self.slice)
            }
        }

        pub fn test_truncated_debug<D: Debug>(slice: &[D]) -> TestTruncatedDebug<D> {
            TestTruncatedDebug { slice }
        }

        /// Example struct which has a struct field
        struct StructWithASlice<'slice, D: Debug> {
            slice: &'slice [D],
        }

        impl<'slice, D: Debug> StructWithASlice<'slice, D> {
            fn new(slice: &'slice [D]) -> Self {
                Self { slice }
            }
        }

        impl<'slice, D: Debug> Debug for StructWithASlice<'slice, D> {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(f, "test_slice: {:?}", test_truncated_debug(self.slice))
            }
        }

        fn truncate_debug_string_for(inputs: &[i32]) -> String {
            let input = StructWithASlice::new(&inputs);
            let result = format!("{:?}", input);

            result
        }

        #[test]
        fn truncated_slice_length_input_length_greater_than_max_length() {
            let result = truncate_debug_string_for(&[1, 2, 3, 4, 5, 6]);

            assert_eq!(
                result,
                "test_slice: (length=6 truncated to 5) [1, 2, 3, 4, 5]..."
            );
        }

        #[test]
        fn truncated_slice_length_input_length_equal_to_max_length() {
            let result = truncate_debug_string_for(&[1, 2, 3, 4, 5]);

            assert_eq!(result, "test_slice: [1, 2, 3, 4, 5]");
        }

        #[test]
        fn truncated_slice_length_input_length_less_then_max_length() {
            let result = truncate_debug_string_for(&[1, 2, 3, 4]);

            assert_eq!(result, "test_slice: [1, 2, 3, 4]");
        }
    }

    mod test_debug_slice_truncated_to_max_length {
        struct StructWithASlice<'slice, D: Debug> {
            slice: &'slice [D],
            max_length: usize,
        }

        impl<'slice, D: Debug> StructWithASlice<'slice, D> {
            fn new(slice: &'slice [D], max_length: usize) -> Self {
                Self { slice, max_length }
            }
        }

        impl<'slice, D: Debug> Debug for StructWithASlice<'slice, D> {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
                format_debug_slice_truncated_to_max_length(f, self.slice, self.max_length)
            }
        }

        use crate::debug_slice_truncation::format_debug_slice_truncated_to_max_length;
        use std::fmt::{Debug, Formatter};

        fn truncate_debug_string_for(inputs: &[i32], max_length: usize) -> String {
            let input = StructWithASlice::new(&inputs, max_length);
            let result = format!("{:?}", input);

            result
        }

        #[test]
        fn debug_slice_truncated_to_max_length_longer_than_max() {
            let result = truncate_debug_string_for(&[1, 2, 3, 4, 5], 4);

            assert_eq!(result, "(length=5 truncated to 4) [1, 2, 3, 4]...");
        }

        #[test]
        fn debug_slice_truncated_to_max_length_shorter_than_max() {
            let result = truncate_debug_string_for(&[1, 2, 3, 4, 5], 6);

            assert_eq!(result, "[1, 2, 3, 4, 5]");
        }

        #[test]
        fn debug_slice_truncated_to_max_length_equal_max() {
            let result = truncate_debug_string_for(&[1, 2, 3, 4, 5], 5);

            assert_eq!(result, "[1, 2, 3, 4, 5]");
        }

        #[test]
        fn debug_slice_truncated_to_max_length_zero() {
            let result = truncate_debug_string_for(&[], 0);

            assert_eq!(result, "[]");
        }

        #[test]
        fn empty_debug_slice_truncated_to_max_length_zero() {
            let result = truncate_debug_string_for(&[1, 2, 3, 4, 5], 0);

            assert_eq!(result, "(length=5)");
        }
    }
}
