use std::fmt::{Debug, Formatter};
use std::hash::Hash;

/// Write an `IntoIterator<IntoIter=ExactSizeIterator>>` of `Debug` to `Formatter`,
/// truncating to the specified length.
pub fn format_debug_exact_size_truncated_to_max_length<D, ESI, ESII>(
    f: &mut Formatter<'_>,
    exact_into_iter: ESII,
    max_length: usize,
) -> Result<(), std::fmt::Error>
where
    D: Debug + Eq + Hash,
    ESI: ExactSizeIterator<Item = D>,
    ESII: IntoIterator<IntoIter = ESI>,
{
    let exact_iter = exact_into_iter.into_iter();
    let debug_exact_iter_length = exact_iter.len();

    if max_length == 0 {
        if debug_exact_iter_length == 0 {
            f.debug_list().entries(exact_iter).finish()
        } else {
            write!(f, "(length={})", debug_exact_iter_length)
        }
    } else if debug_exact_iter_length <= max_length {
        f.debug_list().entries(exact_iter).finish()
    } else {
        write!(
            f,
            "(length={} truncated to {}) ",
            debug_exact_iter_length, max_length
        )?;
        f.debug_list()
            .entries(exact_iter.into_iter().take(max_length))
            .finish()?;
        f.write_str("...")
    }
}

trait TruncateDebug<D: Debug + Eq + Hash, const SIZE: usize> {
    const MAX_LENGTH: usize = SIZE;

    fn format_debug_truncated_to_max_length(
        &self,
        f: &mut Formatter<'_>,
    ) -> Result<(), std::fmt::Error>;

    /// Helper function for `IntoIterator < IntoIter=Iterator< Item=Debug > >` fixing the length.
    fn format_debug_exact_size_truncated_to_max_length<ESI, ESII>(
        f: &mut Formatter<'_>,
        exact_into_iter: ESII,
    ) -> Result<(), std::fmt::Error>
    where
        ESI: ExactSizeIterator<Item = D>,
        ESII: IntoIterator<IntoIter = ESI>,
    {
        format_debug_exact_size_truncated_to_max_length(f, exact_into_iter, Self::MAX_LENGTH)
    }
}

pub struct ExactSizeIntoIterHolder<ESII, const SIZE: usize> {
    exact_size_into_iter: ESII,
}

impl<'esii, ESII, ESI, D, const SIZE: usize> TruncateDebug<D, SIZE>
    for ExactSizeIntoIterHolder<&'esii ESII, SIZE>
where
    D: Debug + Eq + Hash,
    ESI: ExactSizeIterator<Item = D>,
    &'esii ESII: IntoIterator<IntoIter = ESI>,
{
    fn format_debug_truncated_to_max_length(
        &self,
        f: &mut Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        Self::format_debug_exact_size_truncated_to_max_length(f, self.exact_size_into_iter)
    }
}

impl<'esii, ESII, ESI, D, const SIZE: usize> Debug for ExactSizeIntoIterHolder<&'esii ESII, SIZE>
where
    D: Debug + Eq + Hash,
    ESI: ExactSizeIterator<Item = D>,
    &'esii ESII: IntoIterator<IntoIter = ESI>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.format_debug_truncated_to_max_length(f)
    }
}

/// Arbitrary constant used to truncate number of items in debug string.
pub const DEFAULT_EXACT_SIZE_DEBUG_FMT_MAX_ITEMS: usize = 42;

/// Truncates `IntoIterator` item length in debug string where underlying iterator is an `ExactSizeIterator`
/// to maximum of `DEFAULT_EXACT_SIZE_DEBUG_FMT_MAX_ITEMS`.
/// Example which doesn't truncate:
/// ```rust
/// use strings::debug_exact_size_truncation::truncated_exact_size_debug_fmt;
/// let data = [1, 2, 3, 4];
/// let debug_string = format!("result: {:?}", truncated_exact_size_debug_fmt(&data));
/// assert_eq!(debug_string, "result: [1, 2, 3, 4]");
/// ```
/// Example which does truncate:
/// ```rust
/// use strings::debug_exact_size_truncation::{truncated_exact_size_debug_fmt, DEFAULT_EXACT_SIZE_DEBUG_FMT_MAX_ITEMS};
/// let data = (1..=DEFAULT_EXACT_SIZE_DEBUG_FMT_MAX_ITEMS + 1).collect::<Vec<_>>();
/// let debug_string = format!("result: {:?}", truncated_exact_size_debug_fmt(&data));
/// assert!(debug_string.starts_with("result: (length=43 truncated to 42) [1, 2, 3"));
/// assert!(debug_string.ends_with("..."));
/// ```
pub fn truncated_exact_size_debug_fmt<ESII>(
    exact_size_into_iter: ESII,
) -> ExactSizeIntoIterHolder<ESII, DEFAULT_EXACT_SIZE_DEBUG_FMT_MAX_ITEMS> {
    ExactSizeIntoIterHolder {
        exact_size_into_iter,
    }
}

#[cfg(test)]
mod tests {
    mod test_truncated_exact_size_iterator {
        use super::super::ExactSizeIntoIterHolder;
        use linked_hash_set::LinkedHashSet;
        use std::fmt::{Debug, Formatter};
        use std::hash::Hash;

        const MAX_TEST_ITEMS: usize = 5;

        /// Truncates `IntoIterator` item length in debug string where underlying iterator is an `ExactSizeIterator`
        /// to maximum of `MAX_TEST_ITEMS`.
        fn test_truncated_debug<ESII>(
            exact_size_into_iter: ESII,
        ) -> ExactSizeIntoIterHolder<ESII, MAX_TEST_ITEMS> {
            ExactSizeIntoIterHolder::<ESII, MAX_TEST_ITEMS> {
                exact_size_into_iter,
            }
        }

        /// Example struct which has a LinkedHashSet field and formats using `test_truncated_debug()`.
        struct StructWithAHashSet<D> {
            hash_set: LinkedHashSet<D>,
        }

        impl<D> StructWithAHashSet<D> {
            fn new(hash_set: LinkedHashSet<D>) -> Self {
                Self { hash_set }
            }
        }

        impl<D: Debug + Eq + Hash> Debug for StructWithAHashSet<D> {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(
                    f,
                    "test_hash_set: {:?}",
                    test_truncated_debug(&self.hash_set)
                )
            }
        }

        /// Helper function to turn input into a hash set, populate a struct, and then capture debug string.
        fn truncate_debug_string_for(inputs: &[i32]) -> String {
            let hash_set = inputs.iter().cloned().collect::<LinkedHashSet<_>>();
            let input = StructWithAHashSet::new(hash_set);
            let result = format!("{:?}", input);

            result
        }

        #[test]
        fn truncated_exact_size_length_input_length_greater_than_max_length() {
            let result = truncate_debug_string_for(&[1, 2, 3, 4, 5, 6]);

            assert_eq!(
                result,
                "test_hash_set: (length=6 truncated to 5) [1, 2, 3, 4, 5]..."
            );
        }

        #[test]
        fn truncated_exact_size_length_input_length_equal_to_max_length() {
            let result = truncate_debug_string_for(&[1, 2, 3, 4, 5]);

            assert_eq!(result, "test_hash_set: [1, 2, 3, 4, 5]");
        }

        #[test]
        fn truncated_exact_size_length_input_length_less_then_max_length() {
            let result = truncate_debug_string_for(&[1, 2, 3, 4]);

            assert_eq!(result, "test_hash_set: [1, 2, 3, 4]");
        }
    }

    mod test_debug_exact_size_iterator_truncated_to_max_length {
        use super::super::format_debug_exact_size_truncated_to_max_length;
        use linked_hash_set::LinkedHashSet;
        use std::fmt::{Debug, Formatter};
        use std::hash::Hash;

        struct StructWithAHashSet<D> {
            hash_set: LinkedHashSet<D>,
            max_length: usize,
        }

        impl<D> StructWithAHashSet<D> {
            fn new(hash_set: LinkedHashSet<D>, max_length: usize) -> Self {
                Self {
                    hash_set,
                    max_length,
                }
            }
        }

        impl<D: Debug + Hash + Eq> Debug for StructWithAHashSet<D> {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
                format_debug_exact_size_truncated_to_max_length(f, &self.hash_set, self.max_length)
            }
        }

        fn truncate_debug_string_for(inputs: &[i32], max_length: usize) -> String {
            let hash_set = inputs.iter().cloned().collect::<LinkedHashSet<_>>();
            let input = StructWithAHashSet::new(hash_set, max_length);
            let result = format!("{:?}", input);

            result
        }

        #[test]
        fn debug_exact_size_iterator_truncated_to_max_length_longer_than_max() {
            let result = truncate_debug_string_for(&[1, 2, 3, 4, 5], 4);

            assert_eq!(result, "(length=5 truncated to 4) [1, 2, 3, 4]...");
        }

        #[test]
        fn debug_exact_size_iterator_truncated_to_max_length_shorter_than_max() {
            let result = truncate_debug_string_for(&[1, 2, 3, 4, 5], 6);

            assert_eq!(result, "[1, 2, 3, 4, 5]");
        }

        #[test]
        fn debug_exact_size_iterator_truncated_to_max_length_equal_max() {
            let result = truncate_debug_string_for(&[1, 2, 3, 4, 5], 5);

            assert_eq!(result, "[1, 2, 3, 4, 5]");
        }

        #[test]
        fn debug_exact_size_iterator_truncated_to_max_length_zero() {
            let result = truncate_debug_string_for(&[], 0);

            assert_eq!(result, "[]");
        }

        #[test]
        fn empty_debug_exact_size_iterator_truncated_to_max_length_zero() {
            let result = truncate_debug_string_for(&[1, 2, 3, 4, 5], 0);

            assert_eq!(result, "(length=5)");
        }
    }
}
