//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// The
    /// `GCPhysicalInputElement`protocol is a base protocol for specific types
    /// of elements that represent controls on a device.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcphysicalinputelement?language=objc)
    pub unsafe trait GCPhysicalInputElement: NSObjectProtocol {
        /// The set of aliases that can be used to access this element with keyed subscript
        /// notation.
        #[method_id(@__retain_semantics Other aliases)]
        unsafe fn aliases(&self) -> Retained<NSSet<NSString>>;

        /// The element's localized display name.
        ///
        /// This is the string that your app should display in any on-screen messages
        /// instructing the user to interact with the control.  For example:
        ///
        /// "Press \(buttonA.localizedName) to jump!"
        ///
        /// Do not cache this value - it can change when the user remaps controls.
        #[method_id(@__retain_semantics Other localizedName)]
        unsafe fn localizedName(&self) -> Option<Retained<NSString>>;

        /// The SF Symbols name for the element.
        #[method_id(@__retain_semantics Other sfSymbolsName)]
        unsafe fn sfSymbolsName(&self) -> Option<Retained<NSString>>;
    }
);

extern_class!(
    /// An instance of
    /// `GCPhysicalInputElementCollection`contains the collection of
    /// input elements found in a device's physical input profile.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcphysicalinputelementcollection?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GCPhysicalInputElementCollection<
        Key: ?Sized = AnyObject,
        Element: ?Sized = AnyObject,
    >;
);

unsafe impl<Key: ?Sized, Element: ?Sized> NSFastEnumeration
    for GCPhysicalInputElementCollection<Key, Element>
{
}

unsafe impl<Key: ?Sized, Element: ?Sized> NSObjectProtocol
    for GCPhysicalInputElementCollection<Key, Element>
{
}

extern_methods!(
    unsafe impl<Key: Message, Element: Message> GCPhysicalInputElementCollection<Key, Element> {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// The number of elements in the collection.
        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        /// Returns the element associated with a given alias.
        ///
        ///
        /// Parameter `alias`: The alias for which to return the corresponding element.  Typically, you
        /// pass one of the constants defined in
        /// `GCInputNames.h.`
        /// Returns: The element associated with
        /// _alias,_or nil if no element is associated
        /// with
        /// _alias._
        #[method_id(@__retain_semantics Other elementForAlias:)]
        pub unsafe fn elementForAlias(&self, alias: &Key) -> Option<Retained<Element>>;

        #[method_id(@__retain_semantics Other objectForKeyedSubscript:)]
        pub unsafe fn objectForKeyedSubscript(&self, key: &Key) -> Option<Retained<Element>>;

        #[method_id(@__retain_semantics Other elementEnumerator)]
        pub unsafe fn elementEnumerator(&self) -> Retained<NSEnumerator<Element>>;
    }
);
