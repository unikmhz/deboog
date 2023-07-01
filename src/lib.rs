//! Derive macro for extended debug formatting.
//!
//! Currently allows skipping fields and masking field values using various strategies, with more to follow.
//!
//! ## Basic usage
//!
//! Skip serializing a field:
//!
//! ```rust
//! use deboog::Deboog;
//!
//! #[derive(Deboog)]
//! struct Data {
//!     shown: i32,
//!     #[deboog(skip)]
//!     skipped: i32,
//! }
//!
//! assert_eq!(
//!     format!("{:?}", Data { shown: 123, skipped: 234 }),
//!     r#"Data { shown: 123 }"#
//! );
//! ```
//!
//! Also works with tuple structs:
//!
//! ```rust
//! use deboog::Deboog;
//!
//! #[derive(Deboog)]
//! struct Data(i32, #[deboog(skip)] i32);
//!
//! assert_eq!(
//!     format!("{:?}", Data(123, 234)),
//!     r#"Data(123)"#
//! );
//! ```
//!
//! And in enums too:
//!
//! ```rust
//! use deboog::Deboog;
//!
//! #[derive(Deboog)]
//! enum Data {
//!     One,
//!     Two {
//!         shown: i32,
//!         #[deboog(skip)]
//!         skipped: i32,
//!     },
//! }
//!
//! assert_eq!(
//!     format!("{:?}", Data::Two { shown: 123, skipped: 234 }),
//!     r#"Two { shown: 123 }"#
//! );
//! ```

/// Field conversion trait impls
pub mod field;
/// String masking utilities
pub mod masking;

/// #[derive(Debug)] with extra features
pub use deboog_derive::Deboog;
