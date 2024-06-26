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

#[link(name = "AutomaticAssessmentConfiguration", kind = "framework")]
extern "C" {}

#[cfg(feature = "AEAssessmentApplication")]
#[path = "AEAssessmentApplication.rs"]
mod __AEAssessmentApplication;
#[cfg(feature = "AEAssessmentConfiguration")]
#[path = "AEAssessmentConfiguration.rs"]
mod __AEAssessmentConfiguration;
#[cfg(feature = "AEAssessmentParticipantConfiguration")]
#[path = "AEAssessmentParticipantConfiguration.rs"]
mod __AEAssessmentParticipantConfiguration;
#[cfg(feature = "AEAssessmentSession")]
#[path = "AEAssessmentSession.rs"]
mod __AEAssessmentSession;
#[cfg(feature = "AEAssessmentSessionDelegate")]
#[path = "AEAssessmentSessionDelegate.rs"]
mod __AEAssessmentSessionDelegate;
#[cfg(feature = "AEErrors")]
#[path = "AEErrors.rs"]
mod __AEErrors;
#[cfg(feature = "AEVisibility")]
#[path = "AEVisibility.rs"]
mod __AEVisibility;

#[cfg(feature = "AEAssessmentApplication")]
pub use self::__AEAssessmentApplication::AEAssessmentApplication;
#[cfg(feature = "AEAssessmentConfiguration")]
pub use self::__AEAssessmentConfiguration::AEAssessmentConfiguration;
#[cfg(feature = "AEAssessmentConfiguration")]
pub use self::__AEAssessmentConfiguration::AEAutocorrectMode;
#[cfg(feature = "AEAssessmentParticipantConfiguration")]
pub use self::__AEAssessmentParticipantConfiguration::AEAssessmentParticipantConfiguration;
#[cfg(feature = "AEAssessmentSession")]
pub use self::__AEAssessmentSession::AEAssessmentSession;
#[cfg(feature = "AEAssessmentSessionDelegate")]
pub use self::__AEAssessmentSessionDelegate::AEAssessmentSessionDelegate;
#[cfg(feature = "AEErrors")]
pub use self::__AEErrors::AEAssessmentErrorCode;
#[cfg(feature = "AEErrors")]
pub use self::__AEErrors::AEAssessmentErrorDomain;
