//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-ui-kit")]
use objc2_ui_kit::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkinterfaceobjecthorizontalalignment?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKInterfaceObjectHorizontalAlignment(pub NSInteger);
impl WKInterfaceObjectHorizontalAlignment {
    #[doc(alias = "WKInterfaceObjectHorizontalAlignmentLeft")]
    pub const Left: Self = Self(0);
    #[doc(alias = "WKInterfaceObjectHorizontalAlignmentCenter")]
    pub const Center: Self = Self(1);
    #[doc(alias = "WKInterfaceObjectHorizontalAlignmentRight")]
    pub const Right: Self = Self(2);
}

unsafe impl Encode for WKInterfaceObjectHorizontalAlignment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKInterfaceObjectHorizontalAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkinterfaceobjectverticalalignment?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKInterfaceObjectVerticalAlignment(pub NSInteger);
impl WKInterfaceObjectVerticalAlignment {
    #[doc(alias = "WKInterfaceObjectVerticalAlignmentTop")]
    pub const Top: Self = Self(0);
    #[doc(alias = "WKInterfaceObjectVerticalAlignmentCenter")]
    pub const Center: Self = Self(1);
    #[doc(alias = "WKInterfaceObjectVerticalAlignmentBottom")]
    pub const Bottom: Self = Self(2);
}

unsafe impl Encode for WKInterfaceObjectVerticalAlignment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKInterfaceObjectVerticalAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkinterfaceobject?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKInterfaceObject;
);

unsafe impl NSObjectProtocol for WKInterfaceObject {}

extern_methods!(
    unsafe impl WKInterfaceObject {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setAlpha:)]
        pub unsafe fn setAlpha(&self, alpha: CGFloat);

        #[cfg(feature = "WKInterfaceDevice")]
        #[method(setSemanticContentAttribute:)]
        pub unsafe fn setSemanticContentAttribute(
            &self,
            semantic_content_attribute: WKInterfaceSemanticContentAttribute,
        );

        #[method(setHorizontalAlignment:)]
        pub unsafe fn setHorizontalAlignment(
            &self,
            horizontal_alignment: WKInterfaceObjectHorizontalAlignment,
        );

        #[method(setVerticalAlignment:)]
        pub unsafe fn setVerticalAlignment(
            &self,
            vertical_alignment: WKInterfaceObjectVerticalAlignment,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setHeight:)]
        pub unsafe fn setHeight(&self, height: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setRelativeWidth:withAdjustment:)]
        pub unsafe fn setRelativeWidth_withAdjustment(&self, width: CGFloat, adjustment: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setRelativeHeight:withAdjustment:)]
        pub unsafe fn setRelativeHeight_withAdjustment(&self, height: CGFloat, adjustment: CGFloat);

        #[method(sizeToFitWidth)]
        pub unsafe fn sizeToFitWidth(&self);

        #[method(sizeToFitHeight)]
        pub unsafe fn sizeToFitHeight(&self);

        #[unsafe(method_family(none))]
        #[method_id(interfaceProperty)]
        pub unsafe fn interfaceProperty(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKInterfaceObject {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// WKAccessibility
    unsafe impl WKInterfaceObject {
        #[method(setAccessibilityIdentifier:)]
        pub unsafe fn setAccessibilityIdentifier(
            &self,
            accessibility_identifier: Option<&NSString>,
        );

        #[method(setAccessibilityLabel:)]
        pub unsafe fn setAccessibilityLabel(&self, accessibility_label: Option<&NSString>);

        #[method(setAccessibilityHint:)]
        pub unsafe fn setAccessibilityHint(&self, accessibility_hint: Option<&NSString>);

        #[method(setAccessibilityValue:)]
        pub unsafe fn setAccessibilityValue(&self, accessibility_value: Option<&NSString>);

        #[method(setIsAccessibilityElement:)]
        pub unsafe fn setIsAccessibilityElement(&self, is_accessibility_element: bool);

        #[cfg(feature = "objc2-ui-kit")]
        #[method(setAccessibilityTraits:)]
        pub unsafe fn setAccessibilityTraits(&self, accessibility_traits: UIAccessibilityTraits);

        #[method(setAccessibilityImageRegions:)]
        pub unsafe fn setAccessibilityImageRegions(
            &self,
            accessibility_image_regions: &NSArray<WKAccessibilityImageRegion>,
        );
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkaccessibilityimageregion?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKAccessibilityImageRegion;
);

unsafe impl NSObjectProtocol for WKAccessibilityImageRegion {}

extern_methods!(
    unsafe impl WKAccessibilityImageRegion {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(frame)]
        pub unsafe fn frame(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`frame`][Self::frame].
        #[method(setFrame:)]
        pub unsafe fn setFrame(&self, frame: CGRect);

        #[unsafe(method_family(none))]
        #[method_id(label)]
        pub unsafe fn label(&self) -> Retained<NSString>;

        /// Setter for [`label`][Self::label].
        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKAccessibilityImageRegion {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
