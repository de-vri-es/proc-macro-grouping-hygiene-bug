use proc_macro_grouping_hygiene_bug::*;

macro_rules! foo1 {
	($expr:expr) => { identity!($expr * $expr) };
}

macro_rules! foo2 {
	($expr:expr) => { manual_identity!($expr * $expr) };
}

#[test]
fn test_identity() {
	assert_eq!(foo1!(1 + 1), 4); // OK, gives 4.
	assert_eq!(foo2!(1 + 1), 4); // FAILS, gives 3.
}
