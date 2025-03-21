//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipickerviewaccessibilitydelegate?language=objc)
    #[cfg(feature = "UIPickerView")]
    pub unsafe trait UIPickerViewAccessibilityDelegate:
        UIPickerViewDelegate + MainThreadOnly
    {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[unsafe(method(pickerView:accessibilityLabelForComponent:))]
        #[unsafe(method_family = none)]
        unsafe fn pickerView_accessibilityLabelForComponent(
            &self,
            picker_view: &UIPickerView,
            component: NSInteger,
        ) -> Option<Retained<NSString>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[unsafe(method(pickerView:accessibilityHintForComponent:))]
        #[unsafe(method_family = none)]
        unsafe fn pickerView_accessibilityHintForComponent(
            &self,
            picker_view: &UIPickerView,
            component: NSInteger,
        ) -> Option<Retained<NSString>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[unsafe(method(pickerView:accessibilityUserInputLabelsForComponent:))]
        #[unsafe(method_family = none)]
        unsafe fn pickerView_accessibilityUserInputLabelsForComponent(
            &self,
            picker_view: &UIPickerView,
            component: NSInteger,
        ) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[unsafe(method(pickerView:accessibilityAttributedLabelForComponent:))]
        #[unsafe(method_family = none)]
        unsafe fn pickerView_accessibilityAttributedLabelForComponent(
            &self,
            picker_view: &UIPickerView,
            component: NSInteger,
        ) -> Option<Retained<NSAttributedString>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[unsafe(method(pickerView:accessibilityAttributedHintForComponent:))]
        #[unsafe(method_family = none)]
        unsafe fn pickerView_accessibilityAttributedHintForComponent(
            &self,
            picker_view: &UIPickerView,
            component: NSInteger,
        ) -> Option<Retained<NSAttributedString>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[unsafe(method(pickerView:accessibilityAttributedUserInputLabelsForComponent:))]
        #[unsafe(method_family = none)]
        unsafe fn pickerView_accessibilityAttributedUserInputLabelsForComponent(
            &self,
            picker_view: &UIPickerView,
            component: NSInteger,
        ) -> Retained<NSArray<NSAttributedString>>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiscrollviewaccessibilitydelegate?language=objc)
    #[cfg(feature = "UIScrollView")]
    pub unsafe trait UIScrollViewAccessibilityDelegate:
        UIScrollViewDelegate + MainThreadOnly
    {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[unsafe(method(accessibilityScrollStatusForScrollView:))]
        #[unsafe(method_family = none)]
        unsafe fn accessibilityScrollStatusForScrollView(
            &self,
            scroll_view: &UIScrollView,
        ) -> Option<Retained<NSString>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[unsafe(method(accessibilityAttributedScrollStatusForScrollView:))]
        #[unsafe(method_family = none)]
        unsafe fn accessibilityAttributedScrollStatusForScrollView(
            &self,
            scroll_view: &UIScrollView,
        ) -> Option<Retained<NSAttributedString>>;
    }
);

/// UIAccessibilityInvertColors.
#[cfg(all(feature = "UIResponder", feature = "UIView"))]
impl UIView {
    extern_methods!(
        #[unsafe(method(accessibilityIgnoresInvertColors))]
        #[unsafe(method_family = none)]
        pub unsafe fn accessibilityIgnoresInvertColors(&self) -> bool;

        /// Setter for [`accessibilityIgnoresInvertColors`][Self::accessibilityIgnoresInvertColors].
        #[unsafe(method(setAccessibilityIgnoresInvertColors:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAccessibilityIgnoresInvertColors(
            &self,
            accessibility_ignores_invert_colors: bool,
        );
    );
}

/// UIAccessibility.
#[cfg(feature = "UIColor")]
impl UIColor {
    extern_methods!(
        #[unsafe(method(accessibilityName))]
        #[unsafe(method_family = none)]
        pub unsafe fn accessibilityName(&self) -> Retained<NSString>;
    );
}
