//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

mod private_NSObjectNSAccessibility {
    pub trait Sealed {}
}

/// Category "NSAccessibility" on [`NSObject`].
/// * Accessibility Informal Protocol **
#[doc(alias = "NSAccessibility")]
pub unsafe trait NSObjectNSAccessibility:
    ClassType + Sized + private_NSObjectNSAccessibility::Sealed
{
    extern_methods!(
        #[cfg(feature = "NSAccessibilityConstants")]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[unsafe(method(accessibilityAttributeNames))]
        #[unsafe(method_family = none)]
        unsafe fn accessibilityAttributeNames(
            &self,
        ) -> Retained<NSArray<NSAccessibilityAttributeName>>;

        #[cfg(feature = "NSAccessibilityConstants")]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[unsafe(method(accessibilityAttributeValue:))]
        #[unsafe(method_family = none)]
        unsafe fn accessibilityAttributeValue(
            &self,
            attribute: &NSAccessibilityAttributeName,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSAccessibilityConstants")]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[unsafe(method(accessibilityIsAttributeSettable:))]
        #[unsafe(method_family = none)]
        unsafe fn accessibilityIsAttributeSettable(
            &self,
            attribute: &NSAccessibilityAttributeName,
        ) -> bool;

        #[cfg(feature = "NSAccessibilityConstants")]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[unsafe(method(accessibilitySetValue:forAttribute:))]
        #[unsafe(method_family = none)]
        unsafe fn accessibilitySetValue_forAttribute(
            &self,
            value: Option<&AnyObject>,
            attribute: &NSAccessibilityAttributeName,
        );

        #[cfg(feature = "NSAccessibilityConstants")]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[unsafe(method(accessibilityParameterizedAttributeNames))]
        #[unsafe(method_family = none)]
        unsafe fn accessibilityParameterizedAttributeNames(
            &self,
        ) -> Retained<NSArray<NSAccessibilityParameterizedAttributeName>>;

        #[cfg(feature = "NSAccessibilityConstants")]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[unsafe(method(accessibilityAttributeValue:forParameter:))]
        #[unsafe(method_family = none)]
        unsafe fn accessibilityAttributeValue_forParameter(
            &self,
            attribute: &NSAccessibilityParameterizedAttributeName,
            parameter: Option<&AnyObject>,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSAccessibilityConstants")]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[unsafe(method(accessibilityActionNames))]
        #[unsafe(method_family = none)]
        unsafe fn accessibilityActionNames(&self) -> Retained<NSArray<NSAccessibilityActionName>>;

        #[cfg(feature = "NSAccessibilityConstants")]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[unsafe(method(accessibilityActionDescription:))]
        #[unsafe(method_family = none)]
        unsafe fn accessibilityActionDescription(
            &self,
            action: &NSAccessibilityActionName,
        ) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSAccessibilityConstants")]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[unsafe(method(accessibilityPerformAction:))]
        #[unsafe(method_family = none)]
        unsafe fn accessibilityPerformAction(&self, action: &NSAccessibilityActionName);

        #[deprecated = "Use isAccessibilityElement instead"]
        #[unsafe(method(accessibilityIsIgnored))]
        #[unsafe(method_family = none)]
        unsafe fn accessibilityIsIgnored(&self) -> bool;

        #[unsafe(method(accessibilityHitTest:))]
        #[unsafe(method_family = none)]
        unsafe fn accessibilityHitTest(&self, point: NSPoint) -> Option<Retained<AnyObject>>;

        #[unsafe(method(accessibilityFocusedUIElement))]
        #[unsafe(method_family = none)]
        unsafe fn accessibilityFocusedUIElement(&self) -> Option<Retained<AnyObject>>;

        #[unsafe(method(accessibilityIndexOfChild:))]
        #[unsafe(method_family = none)]
        unsafe fn accessibilityIndexOfChild(&self, child: &AnyObject) -> NSUInteger;

        #[cfg(feature = "NSAccessibilityConstants")]
        #[unsafe(method(accessibilityArrayAttributeCount:))]
        #[unsafe(method_family = none)]
        unsafe fn accessibilityArrayAttributeCount(
            &self,
            attribute: &NSAccessibilityAttributeName,
        ) -> NSUInteger;

        #[cfg(feature = "NSAccessibilityConstants")]
        #[unsafe(method(accessibilityArrayAttributeValues:index:maxCount:))]
        #[unsafe(method_family = none)]
        unsafe fn accessibilityArrayAttributeValues_index_maxCount(
            &self,
            attribute: &NSAccessibilityAttributeName,
            index: NSUInteger,
            max_count: NSUInteger,
        ) -> Retained<NSArray>;

        #[unsafe(method(accessibilityNotifiesWhenDestroyed))]
        #[unsafe(method_family = none)]
        unsafe fn accessibilityNotifiesWhenDestroyed(&self) -> bool;
    );
}

impl private_NSObjectNSAccessibility::Sealed for NSObject {}
unsafe impl NSObjectNSAccessibility for NSObject {}

/// NSWorkspaceAccessibilityDisplay.
#[cfg(feature = "NSWorkspace")]
impl NSWorkspace {
    extern_methods!(
        #[unsafe(method(accessibilityDisplayShouldIncreaseContrast))]
        #[unsafe(method_family = none)]
        pub unsafe fn accessibilityDisplayShouldIncreaseContrast(&self) -> bool;

        #[unsafe(method(accessibilityDisplayShouldDifferentiateWithoutColor))]
        #[unsafe(method_family = none)]
        pub unsafe fn accessibilityDisplayShouldDifferentiateWithoutColor(&self) -> bool;

        #[unsafe(method(accessibilityDisplayShouldReduceTransparency))]
        #[unsafe(method_family = none)]
        pub unsafe fn accessibilityDisplayShouldReduceTransparency(&self) -> bool;

        #[unsafe(method(accessibilityDisplayShouldReduceMotion))]
        #[unsafe(method_family = none)]
        pub unsafe fn accessibilityDisplayShouldReduceMotion(&self) -> bool;

        #[unsafe(method(accessibilityDisplayShouldInvertColors))]
        #[unsafe(method_family = none)]
        pub unsafe fn accessibilityDisplayShouldInvertColors(&self) -> bool;
    );
}

/// NSWorkspaceAccessibility.
#[cfg(feature = "NSWorkspace")]
impl NSWorkspace {
    extern_methods!(
        #[unsafe(method(isVoiceOverEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isVoiceOverEnabled(&self) -> bool;

        #[unsafe(method(isSwitchControlEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isSwitchControlEnabled(&self) -> bool;
    );
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsworkspaceaccessibilitydisplayoptionsdidchangenotification?language=objc)
    pub static NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification:
        &'static NSNotificationName;
}

extern "C-unwind" {
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    pub fn NSAccessibilityFrameInView(parent_view: &NSView, frame: NSRect) -> NSRect;
}

extern "C-unwind" {
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    pub fn NSAccessibilityPointInView(parent_view: &NSView, point: NSPoint) -> NSPoint;
}

#[inline]
pub unsafe extern "C-unwind" fn NSAccessibilitySetMayContainProtectedContent(flag: bool) -> bool {
    extern "C-unwind" {
        fn NSAccessibilitySetMayContainProtectedContent(flag: Bool) -> Bool;
    }
    unsafe { NSAccessibilitySetMayContainProtectedContent(Bool::new(flag)) }.as_bool()
}

#[cfg(feature = "NSAccessibilityConstants")]
#[inline]
pub unsafe extern "C-unwind" fn NSAccessibilityRoleDescription(
    role: &NSAccessibilityRole,
    subrole: Option<&NSAccessibilitySubrole>,
) -> Option<Retained<NSString>> {
    extern "C-unwind" {
        fn NSAccessibilityRoleDescription(
            role: &NSAccessibilityRole,
            subrole: Option<&NSAccessibilitySubrole>,
        ) -> *mut NSString;
    }
    let ret = unsafe { NSAccessibilityRoleDescription(role, subrole) };
    unsafe { Retained::retain_autoreleased(ret) }
}

#[inline]
pub unsafe extern "C-unwind" fn NSAccessibilityRoleDescriptionForUIElement(
    element: &AnyObject,
) -> Option<Retained<NSString>> {
    extern "C-unwind" {
        fn NSAccessibilityRoleDescriptionForUIElement(element: &AnyObject) -> *mut NSString;
    }
    let ret = unsafe { NSAccessibilityRoleDescriptionForUIElement(element) };
    unsafe { Retained::retain_autoreleased(ret) }
}

#[cfg(feature = "NSAccessibilityConstants")]
#[inline]
pub unsafe extern "C-unwind" fn NSAccessibilityActionDescription(
    action: &NSAccessibilityActionName,
) -> Option<Retained<NSString>> {
    extern "C-unwind" {
        fn NSAccessibilityActionDescription(action: &NSAccessibilityActionName) -> *mut NSString;
    }
    let ret = unsafe { NSAccessibilityActionDescription(action) };
    unsafe { Retained::retain_autoreleased(ret) }
}

extern "C-unwind" {
    #[cfg(feature = "NSAccessibilityConstants")]
    #[deprecated = "Exceptions are no longer appropriate for indicating errors in accessibility API. Unexpected values should be handled through appropriate type checking."]
    pub fn NSAccessibilityRaiseBadArgumentException(
        element: Option<&AnyObject>,
        attribute: Option<&NSAccessibilityAttributeName>,
        value: Option<&AnyObject>,
    );
}

/// * Ignored UIElements Utilities **
#[inline]
pub unsafe extern "C-unwind" fn NSAccessibilityUnignoredAncestor(
    element: &AnyObject,
) -> Option<Retained<AnyObject>> {
    extern "C-unwind" {
        fn NSAccessibilityUnignoredAncestor(element: &AnyObject) -> *mut AnyObject;
    }
    let ret = unsafe { NSAccessibilityUnignoredAncestor(element) };
    unsafe { Retained::retain_autoreleased(ret) }
}

#[inline]
pub unsafe extern "C-unwind" fn NSAccessibilityUnignoredDescendant(
    element: &AnyObject,
) -> Option<Retained<AnyObject>> {
    extern "C-unwind" {
        fn NSAccessibilityUnignoredDescendant(element: &AnyObject) -> *mut AnyObject;
    }
    let ret = unsafe { NSAccessibilityUnignoredDescendant(element) };
    unsafe { Retained::retain_autoreleased(ret) }
}

#[inline]
pub unsafe extern "C-unwind" fn NSAccessibilityUnignoredChildren(
    original_children: &NSArray,
) -> Retained<NSArray> {
    extern "C-unwind" {
        fn NSAccessibilityUnignoredChildren(original_children: &NSArray) -> *mut NSArray;
    }
    let ret = unsafe { NSAccessibilityUnignoredChildren(original_children) };
    unsafe { Retained::retain_autoreleased(ret) }
        .expect("function was marked as returning non-null, but actually returned NULL")
}

#[inline]
pub unsafe extern "C-unwind" fn NSAccessibilityUnignoredChildrenForOnlyChild(
    original_child: &AnyObject,
) -> Retained<NSArray> {
    extern "C-unwind" {
        fn NSAccessibilityUnignoredChildrenForOnlyChild(original_child: &AnyObject)
            -> *mut NSArray;
    }
    let ret = unsafe { NSAccessibilityUnignoredChildrenForOnlyChild(original_child) };
    unsafe { Retained::retain_autoreleased(ret) }
        .expect("function was marked as returning non-null, but actually returned NULL")
}

extern "C-unwind" {
    /// * Posting Notifications **
    #[cfg(feature = "NSAccessibilityConstants")]
    pub fn NSAccessibilityPostNotification(
        element: &AnyObject,
        notification: &NSAccessibilityNotificationName,
    );
}
