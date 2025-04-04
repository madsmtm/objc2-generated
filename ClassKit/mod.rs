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

#[link(name = "ClassKit", kind = "framework")]
extern "C" {}

#[cfg(feature = "CLSActivity")]
#[path = "CLSActivity.rs"]
mod __CLSActivity;
#[cfg(feature = "CLSActivityItem")]
#[path = "CLSActivityItem.rs"]
mod __CLSActivityItem;
#[cfg(feature = "CLSBinaryItem")]
#[path = "CLSBinaryItem.rs"]
mod __CLSBinaryItem;
#[cfg(feature = "CLSContext")]
#[path = "CLSContext.rs"]
mod __CLSContext;
#[cfg(feature = "CLSContextProvider")]
#[path = "CLSContextProvider.rs"]
mod __CLSContextProvider;
#[cfg(feature = "CLSDataStore")]
#[path = "CLSDataStore.rs"]
mod __CLSDataStore;
#[cfg(feature = "CLSDefines")]
#[path = "CLSDefines.rs"]
mod __CLSDefines;
#[cfg(feature = "CLSObject")]
#[path = "CLSObject.rs"]
mod __CLSObject;
#[cfg(feature = "CLSProgressReportingCapability")]
#[path = "CLSProgressReportingCapability.rs"]
mod __CLSProgressReportingCapability;
#[cfg(feature = "CLSQuantityItem")]
#[path = "CLSQuantityItem.rs"]
mod __CLSQuantityItem;
#[cfg(feature = "CLSScoreItem")]
#[path = "CLSScoreItem.rs"]
mod __CLSScoreItem;
#[cfg(feature = "NSUserActivity_CLSDeepLinks")]
#[path = "NSUserActivity_CLSDeepLinks.rs"]
mod __NSUserActivity_CLSDeepLinks;

#[cfg(all(feature = "CLSActivity", feature = "CLSObject"))]
pub use self::__CLSActivity::CLSActivity;
#[cfg(all(feature = "CLSActivityItem", feature = "CLSObject"))]
pub use self::__CLSActivityItem::CLSActivityItem;
#[cfg(all(
    feature = "CLSActivityItem",
    feature = "CLSBinaryItem",
    feature = "CLSObject"
))]
pub use self::__CLSBinaryItem::CLSBinaryItem;
#[cfg(feature = "CLSBinaryItem")]
pub use self::__CLSBinaryItem::CLSBinaryValueType;
#[cfg(all(feature = "CLSContext", feature = "CLSObject"))]
pub use self::__CLSContext::CLSContext;
#[cfg(feature = "CLSContext")]
pub use self::__CLSContext::CLSContextTopic;
#[cfg(feature = "CLSContext")]
pub use self::__CLSContext::CLSContextTopicArtsAndMusic;
#[cfg(feature = "CLSContext")]
pub use self::__CLSContext::CLSContextTopicComputerScienceAndEngineering;
#[cfg(feature = "CLSContext")]
pub use self::__CLSContext::CLSContextTopicHealthAndFitness;
#[cfg(feature = "CLSContext")]
pub use self::__CLSContext::CLSContextTopicLiteracyAndWriting;
#[cfg(feature = "CLSContext")]
pub use self::__CLSContext::CLSContextTopicMath;
#[cfg(feature = "CLSContext")]
pub use self::__CLSContext::CLSContextTopicScience;
#[cfg(feature = "CLSContext")]
pub use self::__CLSContext::CLSContextTopicSocialScience;
#[cfg(feature = "CLSContext")]
pub use self::__CLSContext::CLSContextTopicWorldLanguage;
#[cfg(feature = "CLSContext")]
pub use self::__CLSContext::CLSContextType;
#[cfg(feature = "CLSContextProvider")]
pub use self::__CLSContextProvider::CLSContextProvider;
#[cfg(feature = "CLSDataStore")]
pub use self::__CLSDataStore::CLSDataStore;
#[cfg(feature = "CLSDataStore")]
pub use self::__CLSDataStore::CLSDataStoreDelegate;
#[cfg(feature = "CLSDefines")]
pub use self::__CLSDefines::CLSErrorCode;
#[cfg(feature = "CLSDefines")]
pub use self::__CLSDefines::CLSErrorCodeDomain;
#[cfg(feature = "CLSDefines")]
pub use self::__CLSDefines::CLSErrorObjectKey;
#[cfg(feature = "CLSDefines")]
pub use self::__CLSDefines::CLSErrorSuccessfulObjectsKey;
#[cfg(feature = "CLSDefines")]
pub use self::__CLSDefines::CLSErrorUnderlyingErrorsKey;
#[cfg(feature = "CLSDefines")]
pub use self::__CLSDefines::CLSErrorUserInfoKey;
#[cfg(feature = "CLSDefines")]
pub use self::__CLSDefines::CLSPredicateKeyPath;
#[cfg(feature = "CLSDefines")]
pub use self::__CLSDefines::CLSPredicateKeyPathDateCreated;
#[cfg(feature = "CLSDefines")]
pub use self::__CLSDefines::CLSPredicateKeyPathIdentifier;
#[cfg(feature = "CLSDefines")]
pub use self::__CLSDefines::CLSPredicateKeyPathParent;
#[cfg(feature = "CLSDefines")]
pub use self::__CLSDefines::CLSPredicateKeyPathTitle;
#[cfg(feature = "CLSDefines")]
pub use self::__CLSDefines::CLSPredicateKeyPathTopic;
#[cfg(feature = "CLSDefines")]
pub use self::__CLSDefines::CLSPredicateKeyPathUniversalLinkURL;
#[cfg(feature = "CLSObject")]
pub use self::__CLSObject::CLSObject;
#[cfg(all(feature = "CLSObject", feature = "CLSProgressReportingCapability"))]
pub use self::__CLSProgressReportingCapability::CLSProgressReportingCapability;
#[cfg(feature = "CLSProgressReportingCapability")]
pub use self::__CLSProgressReportingCapability::CLSProgressReportingCapabilityKind;
#[cfg(all(
    feature = "CLSActivityItem",
    feature = "CLSObject",
    feature = "CLSQuantityItem"
))]
pub use self::__CLSQuantityItem::CLSQuantityItem;
#[cfg(all(
    feature = "CLSActivityItem",
    feature = "CLSObject",
    feature = "CLSScoreItem"
))]
pub use self::__CLSScoreItem::CLSScoreItem;
#[cfg(feature = "NSUserActivity_CLSDeepLinks")]
pub use self::__NSUserActivity_CLSDeepLinks::NSUserActivityCLSDeepLinks;
