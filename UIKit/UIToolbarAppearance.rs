//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitoolbarappearance?language=objc)
    #[unsafe(super(UIBarAppearance, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIBarAppearance")]
    pub struct UIToolbarAppearance;
);

#[cfg(feature = "UIBarAppearance")]
unsafe impl NSCoding for UIToolbarAppearance {}

#[cfg(feature = "UIBarAppearance")]
unsafe impl NSCopying for UIToolbarAppearance {}

#[cfg(feature = "UIBarAppearance")]
unsafe impl CopyingHelper for UIToolbarAppearance {
    type Result = Self;
}

#[cfg(feature = "UIBarAppearance")]
unsafe impl NSObjectProtocol for UIToolbarAppearance {}

#[cfg(feature = "UIBarAppearance")]
unsafe impl NSSecureCoding for UIToolbarAppearance {}

extern_methods!(
    #[cfg(feature = "UIBarAppearance")]
    unsafe impl UIToolbarAppearance {
        #[cfg(feature = "UIBarButtonItemAppearance")]
        /// The appearance for plain-style bar button items
        #[unsafe(method_family(none))]
        #[method_id(buttonAppearance)]
        pub unsafe fn buttonAppearance(&self) -> Retained<UIBarButtonItemAppearance>;

        #[cfg(feature = "UIBarButtonItemAppearance")]
        /// Setter for [`buttonAppearance`][Self::buttonAppearance].
        #[method(setButtonAppearance:)]
        pub unsafe fn setButtonAppearance(&self, button_appearance: &UIBarButtonItemAppearance);

        #[cfg(feature = "UIBarButtonItemAppearance")]
        /// The appearance for done-style bar button items
        #[unsafe(method_family(none))]
        #[method_id(doneButtonAppearance)]
        pub unsafe fn doneButtonAppearance(&self) -> Retained<UIBarButtonItemAppearance>;

        #[cfg(feature = "UIBarButtonItemAppearance")]
        /// Setter for [`doneButtonAppearance`][Self::doneButtonAppearance].
        #[method(setDoneButtonAppearance:)]
        pub unsafe fn setDoneButtonAppearance(
            &self,
            done_button_appearance: &UIBarButtonItemAppearance,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `UIBarAppearance`
    #[cfg(feature = "UIBarAppearance")]
    unsafe impl UIToolbarAppearance {
        /// Constructs a new bar appearance, configured with default values and targeting the device idiom.
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "UIDevice")]
        /// Constructs a new bar appearance, targeting the passed-in idiom as a hint. Not all platforms support all available idioms. See the idiom property to determine the resolved idiom.
        #[unsafe(method_family(init))]
        #[method_id(initWithIdiom:)]
        pub unsafe fn initWithIdiom(
            this: Allocated<Self>,
            idiom: UIUserInterfaceIdiom,
        ) -> Retained<Self>;

        /// Constructs a new bar appearance, copying all relevant properties from the given appearance object. This initializer is useful for migrating configuration between UIBarAppearance subclasses. For example, you can initialize a UINavigationBarAppearance with a UIToolbarAppearance instance, and shared attributes will be identical between the two.
        #[unsafe(method_family(init))]
        #[method_id(initWithBarAppearance:)]
        pub unsafe fn initWithBarAppearance(
            this: Allocated<Self>,
            bar_appearance: &UIBarAppearance,
        ) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIBarAppearance")]
    unsafe impl UIToolbarAppearance {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
