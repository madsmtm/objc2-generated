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

#[link(name = "ScreenSaver", kind = "framework")]
extern "C" {}

#[cfg(feature = "ScreenSaverDefaults")]
#[path = "ScreenSaverDefaults.rs"]
mod __ScreenSaverDefaults;
#[cfg(feature = "ScreenSaverView")]
#[path = "ScreenSaverView.rs"]
mod __ScreenSaverView;

#[cfg(feature = "ScreenSaverDefaults")]
pub use self::__ScreenSaverDefaults::ScreenSaverDefaults;
#[cfg(all(feature = "ScreenSaverView", feature = "objc2-app-kit"))]
pub use self::__ScreenSaverView::ScreenSaverView;
