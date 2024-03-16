pub mod always;
pub mod and;
pub mod buffer;
pub mod error;
pub mod nand;
pub mod never;
pub mod nor;
pub mod not;
pub mod or;
pub mod xnor;
pub mod xor;

// and Creates a Rule where all child Rules must be Met
// at_least Creates a Rule where n child Rules must be Met
// bool_equals Creates a rule for boolean comparison.
// float_contains
// float_does_not_contain
// float_equals Creates a rule for float comparison.
// float_greater_than
// float_greater_than_inclusive
// float_in
// float_in_range
// float_less_than
// float_less_than_inclusive
// float_not_equals
// float_not_in
// float_not_in_range
// int_contains
// int_contains_all
// int_contains_any
// int_does_not_contain
// int_does_not_contain_any
// int_equals Creates a rule for int comparison.
// int_greater_than
// int_greater_than_inclusive
// int_in
// int_in_range
// int_less_than
// int_less_than_inclusive
// int_not_equals
// int_not_in
// int_not_in_range
// or Creates a Rule where any child Rule must be Met
// string_contains
// string_contains_all
// string_contains_any
// string_does_not_contain
// string_does_not_contain_any
// string_equals Creates a rule for string comparison
// string_in
// string_not_equals
// string_not_in
//
