use std::fmt::{Debug, Formatter};
use std::hash::Hash;

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
    let debug_slice_length = exact_iter.len();

    if max_length == 0 {
        if debug_slice_length == 0 {
            f.debug_list().entries(exact_iter).finish()
        } else {
            write!(f, "(length={})", debug_slice_length)
        }
    } else if debug_slice_length <= max_length {
        f.debug_list().entries(exact_iter).finish()
    } else {
        write!(
            f,
            "(length={} truncated to {}) ",
            debug_slice_length, max_length
        )?;
        f.debug_list()
            .entries(exact_iter.into_iter().take(max_length))
            .finish()?;
        f.write_str("...")
    }
}

trait TruncateDebug<D: Debug + Eq + Hash> {
    const MAX_LENGTH: usize;

    fn format_debug_truncated_to_max_length(
        &self,
        f: &mut Formatter<'_>,
    ) -> Result<(), std::fmt::Error>;
}

pub struct ExactSizeIntoIterHolder<ESII> {
    exact_size_into_iter: ESII,
}

pub fn truncated_exact_size_debug_fmt<ESII>(
    exact_size_into_iter: ESII,
) -> ExactSizeIntoIterHolder<ESII> {
    ExactSizeIntoIterHolder {
        exact_size_into_iter,
    }
}

impl<'esii, ESII, ESI, D> TruncateDebug<D> for ExactSizeIntoIterHolder<&'esii ESII>
where
    D: Debug + Eq + Hash,
    ESI: ExactSizeIterator<Item = D>,
    &'esii ESII: IntoIterator<IntoIter = ESI>,
{
    const MAX_LENGTH: usize = 42;

    fn format_debug_truncated_to_max_length(
        &self,
        f: &mut Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        format_debug_exact_size_truncated_to_max_length(
            f,
            self.exact_size_into_iter,
            Self::MAX_LENGTH,
        )
    }
}

impl<'esii, ESII, ESI, D> Debug for ExactSizeIntoIterHolder<&'esii ESII>
where
    D: Debug + Eq + Hash,
    ESI: ExactSizeIterator<Item = D>,
    &'esii ESII: IntoIterator<IntoIter = ESI>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.format_debug_truncated_to_max_length(f)
    }
}

#[cfg(test)]
mod tests {
    mod the_function_with_an_into_iter_field {
        use crate::debug_exact_size_truncation_playground::truncated_exact_size_debug_fmt;
        use linked_hash_set::LinkedHashSet;
        use std::fmt::{Debug, Formatter};

        struct FormatTheFunction;

        impl Debug for FormatTheFunction {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
                {
                    let test_input = [1, 2, 3, 4, 5];
                    let test_hash_set = test_input.iter().cloned().collect::<LinkedHashSet<i32>>();

                    write!(
                        f,
                        "HashSet IntoIterator: {:?}",
                        truncated_exact_size_debug_fmt(&test_hash_set)
                    )?;

                    write!(
                        f,
                        ". Hi again borrowed HashSet IntoIterator: {:?}",
                        truncated_exact_size_debug_fmt(&test_hash_set)
                    )?;

                    write!(
                        f,
                        ". Hi borrowed array IntoIterator: {:?}",
                        truncated_exact_size_debug_fmt(&test_input)
                    )?;
                }

                Ok(())
            }
        }

        #[test]
        fn fmt_the_function_with_a_field() {
            let fmt_hash_set = FormatTheFunction;

            let debug_string = format!("{:?}", fmt_hash_set);

            println!("{}", debug_string);
            assert!(!debug_string.is_empty(), "{}", debug_string);
        }
    }

    mod the_function_with_an_into_iter_ref_field {
        use super::super::format_debug_exact_size_truncated_to_max_length;
        use linked_hash_set::LinkedHashSet;
        use std::fmt::{Debug, Formatter};
        use std::hash::Hash;

        struct FormatTheFunction;

        struct FormatTheFunctionWithAnIntoIterRefField<'esii, ESII> {
            field: &'esii ESII,
        }

        impl<'esii, ESII, ESI, D> Debug for FormatTheFunctionWithAnIntoIterRefField<'esii, ESII>
        where
            D: Debug + Eq + Hash,
            ESI: ExactSizeIterator<Item = D>,
            &'esii ESII: IntoIterator<IntoIter = ESI>,
        {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
                const MAX_LENGTH: usize = 5;
                {
                    let iter = self.field;

                    f.write_str("Hi consumed field: ")?;
                    format_debug_exact_size_truncated_to_max_length//::<D, ESI, ESII>
                        (f, iter, MAX_LENGTH)?;
                }
                Ok(())
            }
        }

        impl Debug for FormatTheFunction {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
                {
                    let test = [1, 2, 3, 4, 5]
                        .iter()
                        .cloned()
                        .collect::<LinkedHashSet<i32>>();
                    let format_the_function_with_a_field =
                        FormatTheFunctionWithAnIntoIterRefField { field: &test };

                    write!(
                        f,
                        "Hi consumed IntoIterator: {:?}",
                        format_the_function_with_a_field
                    )?;
                }

                Ok(())
            }
        }

        #[test]
        fn fmt_the_function_with_a_ref_field() {
            let fmt_hash_set = FormatTheFunction;

            let debug_string = format!("{:?}", fmt_hash_set);

            println!("{}", debug_string);
            assert!(!debug_string.is_empty(), "{}", debug_string);
        }
    }

    mod the_function_with_a_field {
        use super::super::format_debug_exact_size_truncated_to_max_length;
        use linked_hash_set::LinkedHashSet;
        use std::fmt::{Debug, Formatter};

        struct FormatTheFunction;

        struct FormatTheFunctionWithAField<'eii> {
            field: &'eii LinkedHashSet<i32>,
        }

        impl<'eii> Debug for FormatTheFunctionWithAField<'eii> {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
                const MAX_LENGTH: usize = 5;
                {
                    let iter = self.field;

                    f.write_str("Hi consumed field: ")?;
                    format_debug_exact_size_truncated_to_max_length(f, iter, MAX_LENGTH)?;
                }
                Ok(())
            }
        }

        impl Debug for FormatTheFunction {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
                {
                    let test = [1, 2, 3, 4, 5]
                        .iter()
                        .cloned()
                        .collect::<LinkedHashSet<i32>>();
                    let format_the_function_with_a_field =
                        FormatTheFunctionWithAField { field: &test };

                    write!(
                        f,
                        "Hi consumed HashSet: {:?}",
                        format_the_function_with_a_field
                    )?;
                }

                Ok(())
            }
        }

        #[test]
        fn fmt_the_function_with_a_field() {
            let fmt_hash_set = FormatTheFunction;

            let debug_string = format!("{:?}", fmt_hash_set);

            println!("{}", debug_string);
            assert!(!debug_string.is_empty(), "{}", debug_string);
        }
    }

    mod the_function {
        use super::super::format_debug_exact_size_truncated_to_max_length;
        use linked_hash_set::LinkedHashSet;
        use std::fmt::{Debug, Formatter};

        struct FormatTheFunction;
        impl Debug for FormatTheFunction {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
                const MAX_LENGTH: usize = 5;
                {
                    let test = [1, 2, 3, 4, 5]
                        .iter()
                        .cloned()
                        .collect::<LinkedHashSet<i32>>();
                    let iter = (&test).into_iter();

                    f.write_str("Hi consumed HashSet: ")?;
                    format_debug_exact_size_truncated_to_max_length(f, iter, MAX_LENGTH)?;
                    //let iter2 = test.into_iter(); // Nope, it's gone.
                }
                {
                    let test = [1, 2, 3, 4, 5]
                        .iter()
                        .cloned()
                        .collect::<LinkedHashSet<i32>>();
                    let iter = (&test).into_iter();

                    f.write_str(". Hi borrowed HashSet: ")?;
                    format_debug_exact_size_truncated_to_max_length(f, iter, MAX_LENGTH)?;

                    f.write_str(". I can still consume you afterwards... ")?;
                    let iter2 = test.into_iter();
                    format_debug_exact_size_truncated_to_max_length(f, iter2, MAX_LENGTH)?;
                }

                Ok(())
            }
        }

        #[test]
        fn fmt_the_function() {
            let fmt_hash_set = FormatTheFunction;

            let debug_string = format!("{:?}", fmt_hash_set);

            println!("{}", debug_string);
            assert!(!debug_string.is_empty(), "{}", debug_string);
        }
    }

    mod format_eii {
        use linked_hash_set::LinkedHashSet;
        use std::fmt::{Debug, Formatter};

        struct FormatHashSet;
        impl Debug for FormatHashSet {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
                {
                    let test = [1, 2, 3, 4, 5]
                        .iter()
                        .cloned()
                        .collect::<LinkedHashSet<i32>>();
                    let iter = test.into_iter();

                    f.write_str("Hi consumed HashSet: ")?;
                    f.debug_list().entries(iter).finish()?;
                    //let iter2 = test.into_iter(); // Nope, it's gone.
                }
                {
                    let test = [1, 2, 3, 4, 5]
                        .iter()
                        .cloned()
                        .collect::<LinkedHashSet<i32>>();
                    let iter = (&test).into_iter();

                    f.write_str(". Hi borrowed HashSet: ")?;
                    f.debug_list().entries(iter).finish()?;

                    f.write_str(". I can still consume you afterwards... ")?;
                    let iter2 = test.into_iter();
                    f.debug_list().entries(iter2).finish()?;
                }

                Ok(())
            }
        }
        #[test]
        fn fmt_hash_set() {
            let fmt_hash_set = FormatHashSet;

            let debug_string = format!("{:?}", fmt_hash_set);

            println!("{}", debug_string);
            assert!(!debug_string.is_empty(), "{}", debug_string);
        }
        // Nope, can't create a Formatter explicitly
        // #[test]
        // fn format_eii() -> Result<(), std::fmt::Error>{
        //     let mut output = String::with_capacity(100);
        //     let test = LinkedHashSet::<u32>::new();
        //     let args = format_args!("{:?}", test.into_iter());
        //     let mut formatter = Formatter {
        //         flags: 0,
        //         width: None,
        //         precision: None,
        //         buf: output,
        //         align: rt::v1::Alignment::Unknown,
        //         fill: ' ',
        //     };
        //
        //     output.write_fmt(args)?;
        //
        //     Ok(())
        // }
    }
}
