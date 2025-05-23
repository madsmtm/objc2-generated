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

#[link(name = "TVServices", kind = "framework")]
extern "C" {}

#[cfg(feature = "NSUserActivity_TVServices")]
#[path = "NSUserActivity_TVServices.rs"]
mod __NSUserActivity_TVServices;
#[cfg(feature = "TVAppProfileDescriptor")]
#[path = "TVAppProfileDescriptor.rs"]
mod __TVAppProfileDescriptor;
#[cfg(feature = "TVContentIdentifier")]
#[path = "TVContentIdentifier.rs"]
mod __TVContentIdentifier;
#[cfg(feature = "TVContentItem")]
#[path = "TVContentItem.rs"]
mod __TVContentItem;
#[cfg(feature = "TVServicesDefines")]
#[path = "TVServicesDefines.rs"]
mod __TVServicesDefines;
#[cfg(feature = "TVTopShelfAction")]
#[path = "TVTopShelfAction.rs"]
mod __TVTopShelfAction;
#[cfg(feature = "TVTopShelfCarouselContent")]
#[path = "TVTopShelfCarouselContent.rs"]
mod __TVTopShelfCarouselContent;
#[cfg(feature = "TVTopShelfCarouselItem")]
#[path = "TVTopShelfCarouselItem.rs"]
mod __TVTopShelfCarouselItem;
#[cfg(feature = "TVTopShelfContent")]
#[path = "TVTopShelfContent.rs"]
mod __TVTopShelfContent;
#[cfg(feature = "TVTopShelfContentProvider")]
#[path = "TVTopShelfContentProvider.rs"]
mod __TVTopShelfContentProvider;
#[cfg(feature = "TVTopShelfInsetContent")]
#[path = "TVTopShelfInsetContent.rs"]
mod __TVTopShelfInsetContent;
#[cfg(feature = "TVTopShelfItem")]
#[path = "TVTopShelfItem.rs"]
mod __TVTopShelfItem;
#[cfg(feature = "TVTopShelfItemCollection")]
#[path = "TVTopShelfItemCollection.rs"]
mod __TVTopShelfItemCollection;
#[cfg(feature = "TVTopShelfNamedAttribute")]
#[path = "TVTopShelfNamedAttribute.rs"]
mod __TVTopShelfNamedAttribute;
#[cfg(feature = "TVTopShelfObject")]
#[path = "TVTopShelfObject.rs"]
mod __TVTopShelfObject;
#[cfg(feature = "TVTopShelfProvider")]
#[path = "TVTopShelfProvider.rs"]
mod __TVTopShelfProvider;
#[cfg(feature = "TVTopShelfSectionedContent")]
#[path = "TVTopShelfSectionedContent.rs"]
mod __TVTopShelfSectionedContent;
#[cfg(feature = "TVTopShelfSectionedItem")]
#[path = "TVTopShelfSectionedItem.rs"]
mod __TVTopShelfSectionedItem;
#[cfg(feature = "TVUserManager")]
#[path = "TVUserManager.rs"]
mod __TVUserManager;

#[cfg(feature = "NSUserActivity_TVServices")]
pub use self::__NSUserActivity_TVServices::TVUserActivityTypeBrowsingChannelGuide;
#[cfg(feature = "NSUserActivity_TVServices")]
pub use self::__NSUserActivity_TVServices::TVUserActivityTypeBrowsingEntertainmentContent;
#[cfg(feature = "TVAppProfileDescriptor")]
pub use self::__TVAppProfileDescriptor::TVAppProfileDescriptor;
#[cfg(feature = "TVContentIdentifier")]
pub use self::__TVContentIdentifier::TVContentIdentifier;
#[cfg(feature = "TVContentItem")]
pub use self::__TVContentItem::TVContentItem;
#[cfg(feature = "TVContentItem")]
pub use self::__TVContentItem::TVContentItemImageShape;
#[cfg(feature = "TVContentItem")]
pub use self::__TVContentItem::TVContentItemImageTrait;
#[cfg(feature = "TVTopShelfAction")]
pub use self::__TVTopShelfAction::TVTopShelfAction;
#[cfg(feature = "TVTopShelfCarouselContent")]
pub use self::__TVTopShelfCarouselContent::TVTopShelfCarouselContent;
#[cfg(feature = "TVTopShelfCarouselContent")]
pub use self::__TVTopShelfCarouselContent::TVTopShelfCarouselContentStyle;
#[cfg(all(
    feature = "TVTopShelfCarouselItem",
    feature = "TVTopShelfItem",
    feature = "TVTopShelfObject"
))]
pub use self::__TVTopShelfCarouselItem::TVTopShelfCarouselItem;
#[cfg(feature = "TVTopShelfCarouselItem")]
pub use self::__TVTopShelfCarouselItem::TVTopShelfCarouselItemMediaOptions;
#[cfg(feature = "TVTopShelfContent")]
pub use self::__TVTopShelfContent::TVTopShelfContent;
#[cfg(feature = "TVTopShelfContentProvider")]
pub use self::__TVTopShelfContentProvider::TVTopShelfContentProvider;
#[cfg(feature = "TVTopShelfInsetContent")]
pub use self::__TVTopShelfInsetContent::TVTopShelfInsetContent;
#[cfg(all(feature = "TVTopShelfItem", feature = "TVTopShelfObject"))]
pub use self::__TVTopShelfItem::TVTopShelfItem;
#[cfg(feature = "TVTopShelfItem")]
pub use self::__TVTopShelfItem::TVTopShelfItemImageTraits;
#[cfg(all(feature = "TVTopShelfItemCollection", feature = "TVTopShelfObject"))]
pub use self::__TVTopShelfItemCollection::TVTopShelfItemCollection;
#[cfg(feature = "TVTopShelfNamedAttribute")]
pub use self::__TVTopShelfNamedAttribute::TVTopShelfNamedAttribute;
#[cfg(feature = "TVTopShelfObject")]
pub use self::__TVTopShelfObject::TVTopShelfObject;
#[cfg(feature = "TVTopShelfProvider")]
pub use self::__TVTopShelfProvider::TVTopShelfContentStyle;
#[cfg(all(
    feature = "TVContentItem",
    feature = "TVTopShelfProvider",
    feature = "objc2-core-foundation"
))]
pub use self::__TVTopShelfProvider::TVTopShelfImageSizeForShape;
#[cfg(feature = "TVTopShelfProvider")]
pub use self::__TVTopShelfProvider::TVTopShelfItemsDidChangeNotification;
#[cfg(feature = "TVTopShelfProvider")]
pub use self::__TVTopShelfProvider::TVTopShelfProvider;
#[cfg(feature = "TVTopShelfSectionedContent")]
pub use self::__TVTopShelfSectionedContent::TVTopShelfSectionedContent;
#[cfg(all(
    feature = "TVTopShelfItem",
    feature = "TVTopShelfObject",
    feature = "TVTopShelfSectionedItem"
))]
pub use self::__TVTopShelfSectionedItem::TVTopShelfSectionedItem;
#[cfg(feature = "TVTopShelfSectionedItem")]
pub use self::__TVTopShelfSectionedItem::TVTopShelfSectionedItemImageShape;
#[cfg(feature = "TVUserManager")]
pub use self::__TVUserManager::TVUserIdentifier;
#[cfg(feature = "TVUserManager")]
pub use self::__TVUserManager::TVUserManager;
#[cfg(feature = "TVUserManager")]
pub use self::__TVUserManager::TVUserManagerCurrentUserIdentifierDidChangeNotification;
