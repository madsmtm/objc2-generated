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

#[link(name = "IdentityLookup", kind = "framework")]
extern "C" {}

#[cfg(feature = "ILBase")]
#[path = "ILBase.rs"]
mod __ILBase;
#[cfg(feature = "ILCallClassificationRequest")]
#[path = "ILCallClassificationRequest.rs"]
mod __ILCallClassificationRequest;
#[cfg(feature = "ILCallCommunication")]
#[path = "ILCallCommunication.rs"]
mod __ILCallCommunication;
#[cfg(feature = "ILClassificationActions")]
#[path = "ILClassificationActions.rs"]
mod __ILClassificationActions;
#[cfg(feature = "ILClassificationRequest")]
#[path = "ILClassificationRequest.rs"]
mod __ILClassificationRequest;
#[cfg(feature = "ILClassificationResponse")]
#[path = "ILClassificationResponse.rs"]
mod __ILClassificationResponse;
#[cfg(feature = "ILCommunication")]
#[path = "ILCommunication.rs"]
mod __ILCommunication;
#[cfg(feature = "ILMessageClassificationRequest")]
#[path = "ILMessageClassificationRequest.rs"]
mod __ILMessageClassificationRequest;
#[cfg(feature = "ILMessageCommunication")]
#[path = "ILMessageCommunication.rs"]
mod __ILMessageCommunication;
#[cfg(feature = "ILMessageFilterAction")]
#[path = "ILMessageFilterAction.rs"]
mod __ILMessageFilterAction;
#[cfg(feature = "ILMessageFilterCapabilitiesQueryHandling")]
#[path = "ILMessageFilterCapabilitiesQueryHandling.rs"]
mod __ILMessageFilterCapabilitiesQueryHandling;
#[cfg(feature = "ILMessageFilterCapabilitiesQueryRequest")]
#[path = "ILMessageFilterCapabilitiesQueryRequest.rs"]
mod __ILMessageFilterCapabilitiesQueryRequest;
#[cfg(feature = "ILMessageFilterCapabilitiesQueryResponse")]
#[path = "ILMessageFilterCapabilitiesQueryResponse.rs"]
mod __ILMessageFilterCapabilitiesQueryResponse;
#[cfg(feature = "ILMessageFilterError")]
#[path = "ILMessageFilterError.rs"]
mod __ILMessageFilterError;
#[cfg(feature = "ILMessageFilterExtension")]
#[path = "ILMessageFilterExtension.rs"]
mod __ILMessageFilterExtension;
#[cfg(feature = "ILMessageFilterExtensionContext")]
#[path = "ILMessageFilterExtensionContext.rs"]
mod __ILMessageFilterExtensionContext;
#[cfg(feature = "ILMessageFilterQueryHandling")]
#[path = "ILMessageFilterQueryHandling.rs"]
mod __ILMessageFilterQueryHandling;
#[cfg(feature = "ILMessageFilterQueryRequest")]
#[path = "ILMessageFilterQueryRequest.rs"]
mod __ILMessageFilterQueryRequest;
#[cfg(feature = "ILMessageFilterQueryResponse")]
#[path = "ILMessageFilterQueryResponse.rs"]
mod __ILMessageFilterQueryResponse;
#[cfg(feature = "ILNetworkResponse")]
#[path = "ILNetworkResponse.rs"]
mod __ILNetworkResponse;

#[cfg(all(
    feature = "ILCallClassificationRequest",
    feature = "ILClassificationRequest"
))]
pub use self::__ILCallClassificationRequest::ILCallClassificationRequest;
#[cfg(all(feature = "ILCallCommunication", feature = "ILCommunication"))]
pub use self::__ILCallCommunication::ILCallCommunication;
#[cfg(feature = "ILClassificationActions")]
pub use self::__ILClassificationActions::ILClassificationAction;
#[cfg(feature = "ILClassificationRequest")]
pub use self::__ILClassificationRequest::ILClassificationRequest;
#[cfg(feature = "ILClassificationResponse")]
pub use self::__ILClassificationResponse::ILClassificationResponse;
#[cfg(feature = "ILCommunication")]
pub use self::__ILCommunication::ILCommunication;
#[cfg(all(
    feature = "ILClassificationRequest",
    feature = "ILMessageClassificationRequest"
))]
pub use self::__ILMessageClassificationRequest::ILMessageClassificationRequest;
#[cfg(all(feature = "ILCommunication", feature = "ILMessageCommunication"))]
pub use self::__ILMessageCommunication::ILMessageCommunication;
#[cfg(feature = "ILMessageFilterAction")]
pub use self::__ILMessageFilterAction::ILMessageFilterAction;
#[cfg(feature = "ILMessageFilterAction")]
pub use self::__ILMessageFilterAction::ILMessageFilterSubAction;
#[cfg(feature = "ILMessageFilterCapabilitiesQueryHandling")]
pub use self::__ILMessageFilterCapabilitiesQueryHandling::ILMessageFilterCapabilitiesQueryHandling;
#[cfg(feature = "ILMessageFilterCapabilitiesQueryRequest")]
pub use self::__ILMessageFilterCapabilitiesQueryRequest::ILMessageFilterCapabilitiesQueryRequest;
#[cfg(feature = "ILMessageFilterCapabilitiesQueryResponse")]
pub use self::__ILMessageFilterCapabilitiesQueryResponse::ILMessageFilterCapabilitiesQueryResponse;
#[cfg(feature = "ILMessageFilterError")]
pub use self::__ILMessageFilterError::ILMessageFilterError;
#[cfg(feature = "ILMessageFilterError")]
pub use self::__ILMessageFilterError::ILMessageFilterErrorDomain;
#[cfg(feature = "ILMessageFilterExtension")]
pub use self::__ILMessageFilterExtension::ILMessageFilterExtension;
#[cfg(feature = "ILMessageFilterExtensionContext")]
pub use self::__ILMessageFilterExtensionContext::ILMessageFilterExtensionContext;
#[cfg(feature = "ILMessageFilterQueryHandling")]
pub use self::__ILMessageFilterQueryHandling::ILMessageFilterQueryHandling;
#[cfg(feature = "ILMessageFilterQueryRequest")]
pub use self::__ILMessageFilterQueryRequest::ILMessageFilterQueryRequest;
#[cfg(feature = "ILMessageFilterQueryResponse")]
pub use self::__ILMessageFilterQueryResponse::ILMessageFilterQueryResponse;
#[cfg(feature = "ILNetworkResponse")]
pub use self::__ILNetworkResponse::ILNetworkResponse;
