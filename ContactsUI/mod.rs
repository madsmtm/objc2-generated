// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[link(name = "ContactsUI", kind = "framework")]
extern "C" {}

#[cfg(feature = "CNContactPicker")]
#[path = "CNContactPicker.rs"]
mod __CNContactPicker;
#[cfg(feature = "CNContactPickerDelegate")]
#[path = "CNContactPickerDelegate.rs"]
mod __CNContactPickerDelegate;
#[cfg(feature = "CNContactViewController")]
#[path = "CNContactViewController.rs"]
mod __CNContactViewController;

#[cfg(feature = "CNContactPicker")]
pub use self::__CNContactPicker::CNContactPicker;
#[cfg(feature = "CNContactPickerDelegate")]
pub use self::__CNContactPickerDelegate::CNContactPickerDelegate;
#[cfg(all(feature = "CNContactViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
pub use self::__CNContactViewController::CNContactViewController;
