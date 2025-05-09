//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsslider?language=objc)
    #[unsafe(super(NSControl, NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    pub struct NSSlider;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
extern_conformance!(
    unsafe impl NSAccessibility for NSSlider {}
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
extern_conformance!(
    unsafe impl NSAccessibilityElementProtocol for NSSlider {}
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
extern_conformance!(
    unsafe impl NSAccessibilitySlider for NSSlider {}
);

#[cfg(all(
    feature = "NSAnimation",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
extern_conformance!(
    unsafe impl NSAnimatablePropertyContainer for NSSlider {}
);

#[cfg(all(
    feature = "NSAppearance",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
extern_conformance!(
    unsafe impl NSAppearanceCustomization for NSSlider {}
);

#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
extern_conformance!(
    unsafe impl NSCoding for NSSlider {}
);

#[cfg(all(
    feature = "NSControl",
    feature = "NSDragging",
    feature = "NSResponder",
    feature = "NSView"
))]
extern_conformance!(
    unsafe impl NSDraggingDestination for NSSlider {}
);

#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
extern_conformance!(
    unsafe impl NSObjectProtocol for NSSlider {}
);

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
extern_conformance!(
    unsafe impl NSUserInterfaceItemIdentification for NSSlider {}
);

#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
impl NSSlider {
    extern_methods!(
        #[cfg(feature = "NSSliderCell")]
        #[unsafe(method(sliderType))]
        #[unsafe(method_family = none)]
        pub unsafe fn sliderType(&self) -> NSSliderType;

        #[cfg(feature = "NSSliderCell")]
        /// Setter for [`sliderType`][Self::sliderType].
        #[unsafe(method(setSliderType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSliderType(&self, slider_type: NSSliderType);

        #[unsafe(method(minValue))]
        #[unsafe(method_family = none)]
        pub unsafe fn minValue(&self) -> c_double;

        /// Setter for [`minValue`][Self::minValue].
        #[unsafe(method(setMinValue:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMinValue(&self, min_value: c_double);

        #[unsafe(method(maxValue))]
        #[unsafe(method_family = none)]
        pub unsafe fn maxValue(&self) -> c_double;

        /// Setter for [`maxValue`][Self::maxValue].
        #[unsafe(method(setMaxValue:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMaxValue(&self, max_value: c_double);

        #[unsafe(method(altIncrementValue))]
        #[unsafe(method_family = none)]
        pub unsafe fn altIncrementValue(&self) -> c_double;

        /// Setter for [`altIncrementValue`][Self::altIncrementValue].
        #[unsafe(method(setAltIncrementValue:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAltIncrementValue(&self, alt_increment_value: c_double);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(knobThickness))]
        #[unsafe(method_family = none)]
        pub unsafe fn knobThickness(&self) -> CGFloat;

        #[cfg(feature = "NSEvent")]
        #[unsafe(method(acceptsFirstMouse:))]
        #[unsafe(method_family = none)]
        pub unsafe fn acceptsFirstMouse(&self, event: Option<&NSEvent>) -> bool;

        /// Setter for [`isVertical`][Self::isVertical].
        #[unsafe(method(setVertical:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setVertical(&self, vertical: bool);

        #[cfg(feature = "NSColor")]
        #[unsafe(method(trackFillColor))]
        #[unsafe(method_family = none)]
        pub unsafe fn trackFillColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSColor")]
        /// Setter for [`trackFillColor`][Self::trackFillColor].
        #[unsafe(method(setTrackFillColor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTrackFillColor(&self, track_fill_color: Option<&NSColor>);
    );
}

/// Methods declared on superclass `NSControl`.
#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
impl NSSlider {
    extern_methods!(
        #[unsafe(method(initWithFrame:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSResponder`.
#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
impl NSSlider {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
impl NSSlider {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

/// NSSliderVerticalGetter.
#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
impl NSSlider {
    extern_methods!();
}

/// NSTickMarkSupport.
#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
impl NSSlider {
    extern_methods!(
        #[unsafe(method(numberOfTickMarks))]
        #[unsafe(method_family = none)]
        pub unsafe fn numberOfTickMarks(&self) -> NSInteger;

        /// Setter for [`numberOfTickMarks`][Self::numberOfTickMarks].
        #[unsafe(method(setNumberOfTickMarks:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setNumberOfTickMarks(&self, number_of_tick_marks: NSInteger);

        #[cfg(feature = "NSSliderCell")]
        #[unsafe(method(tickMarkPosition))]
        #[unsafe(method_family = none)]
        pub unsafe fn tickMarkPosition(&self) -> NSTickMarkPosition;

        #[cfg(feature = "NSSliderCell")]
        /// Setter for [`tickMarkPosition`][Self::tickMarkPosition].
        #[unsafe(method(setTickMarkPosition:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTickMarkPosition(&self, tick_mark_position: NSTickMarkPosition);

        #[unsafe(method(allowsTickMarkValuesOnly))]
        #[unsafe(method_family = none)]
        pub unsafe fn allowsTickMarkValuesOnly(&self) -> bool;

        /// Setter for [`allowsTickMarkValuesOnly`][Self::allowsTickMarkValuesOnly].
        #[unsafe(method(setAllowsTickMarkValuesOnly:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAllowsTickMarkValuesOnly(&self, allows_tick_mark_values_only: bool);

        #[unsafe(method(tickMarkValueAtIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn tickMarkValueAtIndex(&self, index: NSInteger) -> c_double;

        #[unsafe(method(rectOfTickMarkAtIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn rectOfTickMarkAtIndex(&self, index: NSInteger) -> NSRect;

        #[unsafe(method(indexOfTickMarkAtPoint:))]
        #[unsafe(method_family = none)]
        pub unsafe fn indexOfTickMarkAtPoint(&self, point: NSPoint) -> NSInteger;

        #[unsafe(method(closestTickMarkValueToValue:))]
        #[unsafe(method_family = none)]
        pub unsafe fn closestTickMarkValueToValue(&self, value: c_double) -> c_double;
    );
}

/// NSSliderConvenience.
#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
impl NSSlider {
    extern_methods!(
        /// Creates a continuous horizontal slider over the range 0.0 to 1.0. The default value is 0.0.
        ///
        /// Parameter `target`: The target object that receives action messages from the control.
        ///
        /// Parameter `action`: The action message sent by the control.
        ///
        /// Returns: An initialized slider control.
        #[unsafe(method(sliderWithTarget:action:))]
        #[unsafe(method_family = none)]
        pub unsafe fn sliderWithTarget_action(
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        /// Creates a continuous horizontal slider that represents values over a specified range.
        ///
        /// Parameter `value`: The initial value displayed by the control.
        ///
        /// Parameter `minValue`: The minimum value represented by the control.
        ///
        /// Parameter `maxValue`: The maximum value represented by the control.
        ///
        /// Parameter `target`: The target object that receives action messages from the control.
        ///
        /// Parameter `action`: The action message sent by the control.
        ///
        /// Returns: An initialized slider control.
        #[unsafe(method(sliderWithValue:minValue:maxValue:target:action:))]
        #[unsafe(method_family = none)]
        pub unsafe fn sliderWithValue_minValue_maxValue_target_action(
            value: c_double,
            min_value: c_double,
            max_value: c_double,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;
    );
}

/// NSSliderDeprecated.
#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
impl NSSlider {
    extern_methods!(
        #[cfg(feature = "NSCell")]
        #[deprecated = "-setTitleCell: had no effect since 10.0"]
        #[unsafe(method(setTitleCell:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTitleCell(&self, cell: Option<&NSCell>);

        #[deprecated = "-titleCell has returned nil since 10.0"]
        #[unsafe(method(titleCell))]
        #[unsafe(method_family = none)]
        pub unsafe fn titleCell(&self) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSColor")]
        #[deprecated = "-setTitleColor: had no effect since 10.0"]
        #[unsafe(method(setTitleColor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTitleColor(&self, new_color: Option<&NSColor>);

        #[cfg(feature = "NSColor")]
        #[deprecated = "-titleColor has returned nil since 10.0"]
        #[unsafe(method(titleColor))]
        #[unsafe(method_family = none)]
        pub unsafe fn titleColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSFont")]
        #[deprecated = "-setTitleFont: had no effect since 10.0"]
        #[unsafe(method(setTitleFont:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTitleFont(&self, font_obj: Option<&NSFont>);

        #[cfg(feature = "NSFont")]
        #[deprecated = "-titleFont has returned nil since 10.0"]
        #[unsafe(method(titleFont))]
        #[unsafe(method_family = none)]
        pub unsafe fn titleFont(&self) -> Option<Retained<NSFont>>;

        #[deprecated = "-title has returned nil since 10.0"]
        #[unsafe(method(title))]
        #[unsafe(method_family = none)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        #[deprecated = "-setTitle: had no effect since 10.0"]
        #[unsafe(method(setTitle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTitle(&self, string: Option<&NSString>);

        #[cfg(feature = "objc2-core-foundation")]
        #[deprecated = "-knobThickness has returned 0 since 10.0"]
        #[unsafe(method(setKnobThickness:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setKnobThickness(&self, thickness: CGFloat);

        #[cfg(feature = "NSImage")]
        #[deprecated = "-setImage: had no effect since 10.0"]
        #[unsafe(method(setImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setImage(&self, background_image: Option<&NSImage>);

        #[cfg(feature = "NSImage")]
        #[deprecated = "-image has returned nil since 10.0"]
        #[unsafe(method(image))]
        #[unsafe(method_family = none)]
        pub unsafe fn image(&self) -> Option<Retained<NSImage>>;
    );
}
