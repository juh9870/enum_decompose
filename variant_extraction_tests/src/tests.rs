#![allow(dead_code)]
//! Tests for `extract_variants` macro
//!
//! # Tests for compile failure
//!
//! ## Non-public enum
//! ```compile_fail
//! mod inner {
//!     # use variant_extraction::extract_variants;
//!     #[extract_variants]
//!     #[derive(Debug, Clone)]
//!     enum Invisible {
//!         A(usize),
//!         B(usize),
//!     }
//!     fn check_a(_arg: InvisibleA) {}
//!     fn check_b(_arg: InvisibleB) {}
//! }
//! fn fail_a(_arg: inner::InvisibleA) {}
//! fn fail_b(_arg: inner::InvisibleB) {}
//! ```
//!
//! ## Visibility modifier
//! ```compile_fail
//! mod inner {
//!     # use variant_extraction::extract_variants;
//!     #[extract_variants(vis="")]
//!     #[derive(Debug, Clone)]
//!     pub enum Invisible {
//!         A(usize),
//!         B(usize),
//!     }
//!     fn check_a(_arg: InvisibleA) {}
//!     fn check_b(_arg: InvisibleB) {}
//! }
//! fn fail_a(_arg: inner::InvisibleA) {}
//! fn fail_b(_arg: inner::InvisibleB) {}
//! ```
//!
//! ## Visibility modifier on the individual field
//! ```compile_fail
//! mod inner {
//!     # use variant_extraction::extract_variants;
//!     #[extract_variants]
//!     #[derive(Debug, Clone)]
//!     pub enum Invisible {
//!         A(usize),
//!         #[extract_variants(vis="")]
//!         B(usize),
//!     }
//!     fn check_a(_arg: InvisibleA) {}
//!     fn check_b(_arg: InvisibleB) {}
//! }
//! fn success_a(_arg: inner::InvisibleA) {}
//! fn fail_b(_arg: inner::InvisibleB) {}
//! ```
//!
//! ## Visibility modifier on the individual field and the struct
//! ```compile_fail
//! mod inner {
//!     # use variant_extraction::extract_variants;
//!     #[extract_variants(vis="pub")]
//!     #[derive(Debug, Clone)]
//!     enum Invisible {
//!         A(usize),
//!         #[extract_variants(vis="")]
//!         B(usize),
//!     }
//!     fn check_a(_arg: InvisibleA) {}
//!     fn check_b(_arg: InvisibleB) {}
//! }
//! fn success_a(_arg: inner::InvisibleA) {}
//! fn fail_b(_arg: inner::InvisibleB) {}
//! ```
//!
//! ## Fields visibility
//! ```compile_fail
//! mod inner {
//!     # use variant_extraction::extract_variants;
//!     #[extract_variants(fields_vis="")]
//!     #[derive(Debug, Clone)]
//!     pub enum Invisible {
//!         A(usize),
//!         B(usize),
//!     }
//!     fn check_a(_arg: InvisibleA) {}
//!     fn check_b(_arg: InvisibleB) {}
//! }
//! fn fail_a(arg: inner::InvisibleA) {
//!     println!("{}", arg.0);
//! }
//! fn fail_b(arg: inner::InvisibleB) {
//!     println!("{}", arg.0);
//! }
//! ```
//!
//! ## Individual fields visibility
//! ```compile_fail
//! mod inner {
//!     # use variant_extraction::extract_variants;
//!     #[extract_variants()]
//!     #[derive(Debug, Clone)]
//!     pub enum Invisible {
//!         A(usize),
//!         #[extract_variants(fields_vis="")]
//!         B(usize),
//!     }
//!     fn check_a(_arg: InvisibleA) {}
//!     fn check_b(_arg: InvisibleB) {}
//! }
//! fn success_a(arg: inner::InvisibleA) {
//!     println!("{}", arg.0);
//! }
//! fn fail_b(arg: inner::InvisibleB) {
//!     println!("{}", arg.0);
//! }
//! ```
//!
//! ## Skipped field
//! ```compile_fail
//! # use variant_extraction::extract_variants;
//! #[extract_variants]
//! #[derive(Debug, Clone)]
//! enum FieldSkip {
//!     A(usize),
//!     #[extract_variants(skip)]
//!     B(usize),
//! }
//! fn success_a(_arg: FieldSkipA) {}
//! fn fail_b(_arg: FieldSkipB) {}
//! ```
//!
//! ## Skipping unit fields
//! ```compile_fail
//! # use variant_extraction::extract_variants;
//! #[extract_variants]
//! #[derive(Debug, Clone)]
//! enum FieldSkip {
//!     A(usize),
//!     B,
//! }
//! fn success_a(_arg: FieldSkipA) {}
//! fn fail_b(_arg: FieldSkipB) {}
//! ```
//!
//! ## Skipping empty tuple fields
//! ```compile_fail
//! # use variant_extraction::extract_variants;
//! #[extract_variants]
//! #[derive(Debug, Clone)]
//! enum FieldSkip {
//!     A(usize),
//!     B(),
//! }
//! fn success_a(_arg: FieldSkipA) {}
//! fn fail_b(_arg: FieldSkipB) {}
//! ```
//!
//! ## Skipping empty struct fields
//! ```compile_fail
//! # use variant_extraction::extract_variants;
//! #[extract_variants]
//! #[derive(Debug, Clone)]
//! enum FieldSkip {
//!     A(usize),
//!     B {},
//! }
//! fn success_a(_arg: FieldSkipA) {}
//! fn fail_b(_arg: FieldSkipB) {}
//! ```

use std::fmt::Debug;
use variant_extraction::extract_variants;

mod visibility_test {
    mod inner {
        use variant_extraction::extract_variants;
        #[extract_variants]
        #[derive(Debug, Clone)]
        pub enum Invisible {
            A(usize),
            B(usize),
        }
        fn check_a(_arg: InvisibleA) {}
        fn check_b(_arg: InvisibleB) {}
    }
    fn success_a(_arg: inner::InvisibleA) {}
    fn success_b(_arg: inner::InvisibleB) {}
}

mod forced_visibility_test {
    mod inner {
        use variant_extraction::extract_variants;
        #[extract_variants(vis = "pub")]
        #[derive(Debug, Clone)]
        enum Invisible {
            A(usize),
            B(usize),
        }
        fn check_a(_arg: InvisibleA) {}
        fn check_b(_arg: InvisibleB) {}
    }
    fn success_a(_arg: inner::InvisibleA) {}
    fn success_b(_arg: inner::InvisibleB) {}
}

mod partial_forced_visibility_test {
    mod inner {
        use variant_extraction::extract_variants;
        #[extract_variants(vis = "pub")]
        #[derive(Debug, Clone)]
        enum Invisible {
            A(usize),
            #[extract_variants(vis = "")]
            B(usize),
        }
        fn check_a(_arg: InvisibleA) {}
        fn check_b(_arg: InvisibleB) {}
    }
    fn success_a(_arg: inner::InvisibleA) {}
}

mod partial_visibility_test {
    mod inner {
        use variant_extraction::extract_variants;
        #[extract_variants]
        #[derive(Debug, Clone)]
        pub enum Invisible {
            A(usize),
            #[extract_variants(vis = "")]
            B(usize),
        }
        fn check_a(_arg: InvisibleA) {}
        fn check_b(_arg: InvisibleB) {}
    }
    fn success_a(_arg: inner::InvisibleA) {}
}

#[extract_variants]
#[derive(Debug, Clone, Eq, PartialEq)]
enum Test {
    VariantA(usize, i64),
}

#[extract_variants(derive = "Debug, Clone, Default")]
#[derive(Debug, Clone)]
enum TestDerive {
    VariantA(usize, f64),
    #[extract_variants(derive = "Debug, Clone, Default, Eq, PartialEq")]
    VariantB {
        value: String,
    },
}

#[extract_variants]
#[derive(Debug, Clone)]
enum TestDeriveFieldOnly {
    VariantA(usize, f64),
    #[extract_variants(derive = "Debug, Clone, Default")]
    VariantB {
        value: String,
    },
}

#[extract_variants(prefix = "Renamed", suffix = "Struct")]
#[derive(Debug, Clone)]
enum TestRename {
    VariantA(usize, i64),
    #[extract_variants(rename = "ForceRenamed")]
    VariantB {
        value: String,
    },
}

#[extract_variants]
#[derive(Debug, Clone)]
enum TestFieldSkip {
    VariantA(usize),
    #[extract_variants(skip)]
    VariantB(usize),
}

#[extract_variants(skip_empty = false)]
#[derive(Debug, Clone)]
enum TestEmptyFieldNoSkip {
    VariantA(usize),
    VariantB,
    VariantC(),
    VariantD {},
}

fn assert_clone<T: Clone>() {}
fn assert_debug<T: Debug>() {}
fn assert_default<T: Default>() {}
fn assert_eq<T: Eq>() {}

fn assert_clone_debug<T: Clone + Debug>() {}

#[test]
fn type_checks() {
    // Normal Derive
    assert_clone_debug::<Test>();
    assert_clone_debug::<TestVariantA>();

    // Derive with only field Override
    assert_clone_debug::<TestDeriveFieldOnly>(); // enum
    assert_clone_debug::<TestDeriveFieldOnlyVariantA>(); // VariantA
    assert_clone_debug::<TestDeriveFieldOnlyVariantB>(); // VariantB
    assert_default::<TestDeriveFieldOnlyVariantB>();

    // Derive with field and global override
    assert_clone_debug::<TestDerive>(); // enum
    assert_clone_debug::<TestDeriveVariantA>(); // VariantA
    assert_default::<TestDeriveVariantA>();
    assert_clone_debug::<TestDeriveVariantB>(); // VariantB
    assert_default::<TestDeriveVariantB>();
    assert_eq::<TestDeriveVariantB>();

    // Renaming
    assert_clone_debug::<RenamedVariantAStruct>();
    assert_clone_debug::<ForceRenamed>();

    // Field skipping
    assert_clone_debug::<TestFieldSkip>();
    assert_clone_debug::<TestFieldSkipVariantA>();

    // Field skipping
    assert_clone_debug::<TestEmptyFieldNoSkip>();
    assert_clone_debug::<TestEmptyFieldNoSkipVariantA>();
    assert_clone_debug::<TestEmptyFieldNoSkipVariantB>();
    assert_clone_debug::<TestEmptyFieldNoSkipVariantC>();
    assert_clone_debug::<TestEmptyFieldNoSkipVariantD>();
}

#[test]
fn check_into() {
    assert_eq!(
        Test::VariantA(TestVariantA(0, 16)),
        TestVariantA(0, 16).into()
    )
}
