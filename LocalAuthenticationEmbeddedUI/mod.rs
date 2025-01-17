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
#![allow(clippy::doc_lazy_continuation)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::unportable_markdown)]
#![allow(rustdoc::invalid_html_tags)]

#[link(name = "LocalAuthenticationEmbeddedUI", kind = "framework")]
extern "C" {}

#[cfg(feature = "LAAuthenticationView")]
#[path = "LAAuthenticationView.rs"]
mod __LAAuthenticationView;
#[cfg(feature = "LAPresentationContext")]
#[path = "LAPresentationContext.rs"]
mod __LAPresentationContext;
#[cfg(feature = "LARight_UI")]
#[path = "LARight_UI.rs"]
mod __LARight_UI;

#[cfg(all(feature = "LAAuthenticationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
pub use self::__LAAuthenticationView::LAAuthenticationView;
#[cfg(all(feature = "LAPresentationContext", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
pub use self::__LAPresentationContext::LAPresentationContext;
#[cfg(feature = "LARight_UI")]
pub use self::__LARight_UI::LARightUI;
