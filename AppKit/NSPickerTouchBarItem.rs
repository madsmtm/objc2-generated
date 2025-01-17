//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspickertouchbaritemselectionmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPickerTouchBarItemSelectionMode(pub NSInteger);
impl NSPickerTouchBarItemSelectionMode {
    #[doc(alias = "NSPickerTouchBarItemSelectionModeSelectOne")]
    pub const SelectOne: Self = Self(0);
    #[doc(alias = "NSPickerTouchBarItemSelectionModeSelectAny")]
    pub const SelectAny: Self = Self(1);
    #[doc(alias = "NSPickerTouchBarItemSelectionModeMomentary")]
    pub const Momentary: Self = Self(2);
}

unsafe impl Encode for NSPickerTouchBarItemSelectionMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSPickerTouchBarItemSelectionMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspickertouchbaritemcontrolrepresentation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPickerTouchBarItemControlRepresentation(pub NSInteger);
impl NSPickerTouchBarItemControlRepresentation {
    #[doc(alias = "NSPickerTouchBarItemControlRepresentationAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "NSPickerTouchBarItemControlRepresentationExpanded")]
    pub const Expanded: Self = Self(1);
    #[doc(alias = "NSPickerTouchBarItemControlRepresentationCollapsed")]
    pub const Collapsed: Self = Self(2);
}

unsafe impl Encode for NSPickerTouchBarItemControlRepresentation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSPickerTouchBarItemControlRepresentation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspickertouchbaritem?language=objc)
    #[unsafe(super(NSTouchBarItem, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSTouchBarItem")]
    pub struct NSPickerTouchBarItem;
);

#[cfg(feature = "NSTouchBarItem")]
unsafe impl NSCoding for NSPickerTouchBarItem {}

#[cfg(feature = "NSTouchBarItem")]
unsafe impl NSObjectProtocol for NSPickerTouchBarItem {}

extern_methods!(
    #[cfg(feature = "NSTouchBarItem")]
    unsafe impl NSPickerTouchBarItem {
        #[unsafe(method_family(none))]
        #[method_id(pickerTouchBarItemWithIdentifier:labels:selectionMode:target:action:)]
        pub unsafe fn pickerTouchBarItemWithIdentifier_labels_selectionMode_target_action(
            identifier: &NSTouchBarItemIdentifier,
            labels: &NSArray<NSString>,
            selection_mode: NSPickerTouchBarItemSelectionMode,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "NSImage")]
        #[unsafe(method_family(none))]
        #[method_id(pickerTouchBarItemWithIdentifier:images:selectionMode:target:action:)]
        pub unsafe fn pickerTouchBarItemWithIdentifier_images_selectionMode_target_action(
            identifier: &NSTouchBarItemIdentifier,
            images: &NSArray<NSImage>,
            selection_mode: NSPickerTouchBarItemSelectionMode,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method(controlRepresentation)]
        pub unsafe fn controlRepresentation(&self) -> NSPickerTouchBarItemControlRepresentation;

        /// Setter for [`controlRepresentation`][Self::controlRepresentation].
        #[method(setControlRepresentation:)]
        pub unsafe fn setControlRepresentation(
            &self,
            control_representation: NSPickerTouchBarItemControlRepresentation,
        );

        #[unsafe(method_family(none))]
        #[method_id(collapsedRepresentationLabel)]
        pub unsafe fn collapsedRepresentationLabel(&self) -> Retained<NSString>;

        /// Setter for [`collapsedRepresentationLabel`][Self::collapsedRepresentationLabel].
        #[method(setCollapsedRepresentationLabel:)]
        pub unsafe fn setCollapsedRepresentationLabel(
            &self,
            collapsed_representation_label: &NSString,
        );

        #[cfg(feature = "NSImage")]
        #[unsafe(method_family(none))]
        #[method_id(collapsedRepresentationImage)]
        pub unsafe fn collapsedRepresentationImage(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        /// Setter for [`collapsedRepresentationImage`][Self::collapsedRepresentationImage].
        #[method(setCollapsedRepresentationImage:)]
        pub unsafe fn setCollapsedRepresentationImage(
            &self,
            collapsed_representation_image: Option<&NSImage>,
        );

        #[method(selectedIndex)]
        pub unsafe fn selectedIndex(&self) -> NSInteger;

        /// Setter for [`selectedIndex`][Self::selectedIndex].
        #[method(setSelectedIndex:)]
        pub unsafe fn setSelectedIndex(&self, selected_index: NSInteger);

        #[cfg(feature = "NSColor")]
        #[unsafe(method_family(none))]
        #[method_id(selectionColor)]
        pub unsafe fn selectionColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSColor")]
        /// Setter for [`selectionColor`][Self::selectionColor].
        #[method(setSelectionColor:)]
        pub unsafe fn setSelectionColor(&self, selection_color: Option<&NSColor>);

        #[method(selectionMode)]
        pub unsafe fn selectionMode(&self) -> NSPickerTouchBarItemSelectionMode;

        /// Setter for [`selectionMode`][Self::selectionMode].
        #[method(setSelectionMode:)]
        pub unsafe fn setSelectionMode(&self, selection_mode: NSPickerTouchBarItemSelectionMode);

        #[method(numberOfOptions)]
        pub unsafe fn numberOfOptions(&self) -> NSInteger;

        /// Setter for [`numberOfOptions`][Self::numberOfOptions].
        #[method(setNumberOfOptions:)]
        pub unsafe fn setNumberOfOptions(&self, number_of_options: NSInteger);

        #[cfg(feature = "NSImage")]
        #[method(setImage:atIndex:)]
        pub unsafe fn setImage_atIndex(&self, image: Option<&NSImage>, index: NSInteger);

        #[cfg(feature = "NSImage")]
        #[unsafe(method_family(none))]
        #[method_id(imageAtIndex:)]
        pub unsafe fn imageAtIndex(&self, index: NSInteger) -> Option<Retained<NSImage>>;

        #[method(setLabel:atIndex:)]
        pub unsafe fn setLabel_atIndex(&self, label: &NSString, index: NSInteger);

        #[unsafe(method_family(none))]
        #[method_id(labelAtIndex:)]
        pub unsafe fn labelAtIndex(&self, index: NSInteger) -> Option<Retained<NSString>>;

        #[unsafe(method_family(none))]
        #[method_id(target)]
        pub unsafe fn target(&self) -> Option<Retained<AnyObject>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`target`][Self::target].
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&AnyObject>);

        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        /// Setter for [`action`][Self::action].
        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        /// Setter for [`isEnabled`][Self::isEnabled].
        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(setEnabled:atIndex:)]
        pub unsafe fn setEnabled_atIndex(&self, enabled: bool, index: NSInteger);

        #[method(isEnabledAtIndex:)]
        pub unsafe fn isEnabledAtIndex(&self, index: NSInteger) -> bool;

        /// The localized string labelling this item during user customization. The default value is empty string.
        #[unsafe(method_family(none))]
        #[method_id(customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Retained<NSString>;

        /// Setter for [`customizationLabel`][Self::customizationLabel].
        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customization_label: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTouchBarItem`
    #[cfg(feature = "NSTouchBarItem")]
    unsafe impl NSPickerTouchBarItem {
        #[unsafe(method_family(init))]
        #[method_id(initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSTouchBarItem")]
    unsafe impl NSPickerTouchBarItem {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
