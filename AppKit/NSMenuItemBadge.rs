//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// The badge type is used to specify one of the pre-defined or custom
/// string portions of a menu item badge, ensuring appropriate localization
/// and pluralization behaviors automatically when using a pre-defined type.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/appkit/nsmenuitembadgetype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSMenuItemBadgeType(pub NSInteger);
impl NSMenuItemBadgeType {
    /// The badge should have no string portion.
    #[doc(alias = "NSMenuItemBadgeTypeNone")]
    pub const None: Self = Self(0);
    /// The badge represents the number of available updates.
    #[doc(alias = "NSMenuItemBadgeTypeUpdates")]
    pub const Updates: Self = Self(1);
    /// The badge represents the number of new items.
    #[doc(alias = "NSMenuItemBadgeTypeNewItems")]
    pub const NewItems: Self = Self(2);
    /// The badge represents the number of alerts.
    #[doc(alias = "NSMenuItemBadgeTypeAlerts")]
    pub const Alerts: Self = Self(3);
}

unsafe impl Encode for NSMenuItemBadgeType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSMenuItemBadgeType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// A badge used to provide additional quantitative information specific
    /// to the menu item, such as the number of available updates.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/appkit/nsmenuitembadge?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMenuItemBadge;
);

unsafe impl NSCopying for NSMenuItemBadge {}

unsafe impl CopyingHelper for NSMenuItemBadge {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSMenuItemBadge {}

extern_methods!(
    unsafe impl NSMenuItemBadge {
        /// Creates a badge with an integer count and a label representing
        /// the number of available updates.
        #[unsafe(method_family(none))]
        #[method_id(updatesWithCount:)]
        pub unsafe fn updatesWithCount(item_count: NSInteger) -> Retained<Self>;

        /// Creates a badge with an integer count and a label representing
        /// the number of new items.
        #[unsafe(method_family(new))]
        #[method_id(newItemsWithCount:)]
        pub unsafe fn newItemsWithCount(item_count: NSInteger) -> Retained<Self>;

        /// Creates a badge with an integer count and a label representing
        /// the number of alerts.
        #[unsafe(method_family(none))]
        #[method_id(alertsWithCount:)]
        pub unsafe fn alertsWithCount(item_count: NSInteger) -> Retained<Self>;

        /// Initializes the badge with a count and a pre-defined badge type.
        #[unsafe(method_family(init))]
        #[method_id(initWithCount:type:)]
        pub unsafe fn initWithCount_type(
            this: Allocated<Self>,
            item_count: NSInteger,
            r#type: NSMenuItemBadgeType,
        ) -> Retained<Self>;

        /// Initializes the badge with an integer count and an empty string.
        #[unsafe(method_family(init))]
        #[method_id(initWithCount:)]
        pub unsafe fn initWithCount(this: Allocated<Self>, item_count: NSInteger)
            -> Retained<Self>;

        /// Initializes the badge with the provided custom string.
        #[unsafe(method_family(init))]
        #[method_id(initWithString:)]
        pub unsafe fn initWithString(this: Allocated<Self>, string: &NSString) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// The count of items the badge displays. If a custom string was used
        /// to create a badge, the value is 0.
        #[method(itemCount)]
        pub unsafe fn itemCount(&self) -> NSInteger;

        /// The type of items the badge displays. If a custom string was used
        /// to create a badge, this value is
        /// `NSMenuItemBadgeTypeNone.`
        #[method(type)]
        pub unsafe fn r#type(&self) -> NSMenuItemBadgeType;

        /// The string representation of the badge as it would appear when the
        /// badge is displayed.
        #[unsafe(method_family(none))]
        #[method_id(stringValue)]
        pub unsafe fn stringValue(&self) -> Option<Retained<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSMenuItemBadge {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
