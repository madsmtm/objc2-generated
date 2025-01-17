//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsrulerorientation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSRulerOrientation(pub NSUInteger);
impl NSRulerOrientation {
    #[doc(alias = "NSHorizontalRuler")]
    pub const HorizontalRuler: Self = Self(0);
    #[doc(alias = "NSVerticalRuler")]
    pub const VerticalRuler: Self = Self(1);
}

unsafe impl Encode for NSRulerOrientation {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSRulerOrientation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsrulerviewunitname?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type NSRulerViewUnitName = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsrulerviewunitinches?language=objc)
    pub static NSRulerViewUnitInches: &'static NSRulerViewUnitName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsrulerviewunitcentimeters?language=objc)
    pub static NSRulerViewUnitCentimeters: &'static NSRulerViewUnitName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsrulerviewunitpoints?language=objc)
    pub static NSRulerViewUnitPoints: &'static NSRulerViewUnitName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsrulerviewunitpicas?language=objc)
    pub static NSRulerViewUnitPicas: &'static NSRulerViewUnitName;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsrulerview?language=objc)
    #[unsafe(super(NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    pub struct NSRulerView;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSRulerView {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSRulerView {}

#[cfg(all(feature = "NSAnimation", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAnimatablePropertyContainer for NSRulerView {}

#[cfg(all(feature = "NSAppearance", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAppearanceCustomization for NSRulerView {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSCoding for NSRulerView {}

#[cfg(all(feature = "NSDragging", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSDraggingDestination for NSRulerView {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSObjectProtocol for NSRulerView {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSRulerView {}

extern_methods!(
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSRulerView {
        #[cfg(feature = "objc2-core-foundation")]
        /// *********************** Registering new units ************************
        #[method(registerUnitWithName:abbreviation:unitToPointsConversionFactor:stepUpCycle:stepDownCycle:)]
        pub unsafe fn registerUnitWithName_abbreviation_unitToPointsConversionFactor_stepUpCycle_stepDownCycle(
            unit_name: &NSRulerViewUnitName,
            abbreviation: &NSString,
            conversion_factor: CGFloat,
            step_up_cycle: &NSArray<NSNumber>,
            step_down_cycle: &NSArray<NSNumber>,
            mtm: MainThreadMarker,
        );

        /// ************************** Initialization ***************************
        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[cfg(feature = "NSScrollView")]
        #[unsafe(method_family(init))]
        #[method_id(initWithScrollView:orientation:)]
        pub unsafe fn initWithScrollView_orientation(
            this: Allocated<Self>,
            scroll_view: Option<&NSScrollView>,
            orientation: NSRulerOrientation,
        ) -> Retained<Self>;

        #[cfg(feature = "NSScrollView")]
        /// ************************** Basic setup ***************************
        #[unsafe(method_family(none))]
        #[method_id(scrollView)]
        pub unsafe fn scrollView(&self) -> Option<Retained<NSScrollView>>;

        #[cfg(feature = "NSScrollView")]
        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`scrollView`][Self::scrollView].
        #[method(setScrollView:)]
        pub unsafe fn setScrollView(&self, scroll_view: Option<&NSScrollView>);

        #[method(orientation)]
        pub unsafe fn orientation(&self) -> NSRulerOrientation;

        /// Setter for [`orientation`][Self::orientation].
        #[method(setOrientation:)]
        pub unsafe fn setOrientation(&self, orientation: NSRulerOrientation);

        #[cfg(feature = "objc2-core-foundation")]
        /// ************************** Ruler geometry ***************************
        #[method(baselineLocation)]
        pub unsafe fn baselineLocation(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(requiredThickness)]
        pub unsafe fn requiredThickness(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(ruleThickness)]
        pub unsafe fn ruleThickness(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`ruleThickness`][Self::ruleThickness].
        #[method(setRuleThickness:)]
        pub unsafe fn setRuleThickness(&self, rule_thickness: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(reservedThicknessForMarkers)]
        pub unsafe fn reservedThicknessForMarkers(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`reservedThicknessForMarkers`][Self::reservedThicknessForMarkers].
        #[method(setReservedThicknessForMarkers:)]
        pub unsafe fn setReservedThicknessForMarkers(
            &self,
            reserved_thickness_for_markers: CGFloat,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(reservedThicknessForAccessoryView)]
        pub unsafe fn reservedThicknessForAccessoryView(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`reservedThicknessForAccessoryView`][Self::reservedThicknessForAccessoryView].
        #[method(setReservedThicknessForAccessoryView:)]
        pub unsafe fn setReservedThicknessForAccessoryView(
            &self,
            reserved_thickness_for_accessory_view: CGFloat,
        );

        /// ************************** Rule configuration ***************************
        #[unsafe(method_family(none))]
        #[method_id(measurementUnits)]
        pub unsafe fn measurementUnits(&self) -> Retained<NSRulerViewUnitName>;

        /// Setter for [`measurementUnits`][Self::measurementUnits].
        #[method(setMeasurementUnits:)]
        pub unsafe fn setMeasurementUnits(&self, measurement_units: &NSRulerViewUnitName);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(originOffset)]
        pub unsafe fn originOffset(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`originOffset`][Self::originOffset].
        #[method(setOriginOffset:)]
        pub unsafe fn setOriginOffset(&self, origin_offset: CGFloat);

        /// ************************** Client view setup ***************************
        #[unsafe(method_family(none))]
        #[method_id(clientView)]
        pub unsafe fn clientView(&self) -> Option<Retained<NSView>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`clientView`][Self::clientView].
        #[method(setClientView:)]
        pub unsafe fn setClientView(&self, client_view: Option<&NSView>);

        #[cfg(feature = "NSRulerMarker")]
        #[method(addMarker:)]
        pub unsafe fn addMarker(&self, marker: &NSRulerMarker);

        #[cfg(feature = "NSRulerMarker")]
        #[method(removeMarker:)]
        pub unsafe fn removeMarker(&self, marker: &NSRulerMarker);

        #[cfg(feature = "NSRulerMarker")]
        #[unsafe(method_family(none))]
        #[method_id(markers)]
        pub unsafe fn markers(&self) -> Option<Retained<NSArray<NSRulerMarker>>>;

        #[cfg(feature = "NSRulerMarker")]
        /// Setter for [`markers`][Self::markers].
        #[method(setMarkers:)]
        pub unsafe fn setMarkers(&self, markers: Option<&NSArray<NSRulerMarker>>);

        #[cfg(all(feature = "NSEvent", feature = "NSRulerMarker"))]
        #[method(trackMarker:withMouseEvent:)]
        pub unsafe fn trackMarker_withMouseEvent(
            &self,
            marker: &NSRulerMarker,
            event: &NSEvent,
        ) -> bool;

        #[unsafe(method_family(none))]
        #[method_id(accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Retained<NSView>>;

        /// Setter for [`accessoryView`][Self::accessoryView].
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessory_view: Option<&NSView>);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(moveRulerlineFromLocation:toLocation:)]
        pub unsafe fn moveRulerlineFromLocation_toLocation(
            &self,
            old_location: CGFloat,
            new_location: CGFloat,
        );

        /// ********************* Drawing and hash invalidation **********************
        #[method(invalidateHashMarks)]
        pub unsafe fn invalidateHashMarks(&self);

        #[method(drawHashMarksAndLabelsInRect:)]
        pub unsafe fn drawHashMarksAndLabelsInRect(&self, rect: NSRect);

        #[method(drawMarkersInRect:)]
        pub unsafe fn drawMarkersInRect(&self, rect: NSRect);

        /// ************************** Key overrides ***************************
        #[method(isFlipped)]
        pub unsafe fn isFlipped(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSRulerView {
        #[unsafe(method_family(init))]
        #[method_id(initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSRulerView {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSRulerView {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSRulerMarkerClientViewDelegation
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSView {
        #[cfg(feature = "NSRulerMarker")]
        #[method(rulerView:shouldMoveMarker:)]
        pub unsafe fn rulerView_shouldMoveMarker(
            &self,
            ruler: &NSRulerView,
            marker: &NSRulerMarker,
        ) -> bool;

        #[cfg(all(feature = "NSRulerMarker", feature = "objc2-core-foundation"))]
        #[method(rulerView:willMoveMarker:toLocation:)]
        pub unsafe fn rulerView_willMoveMarker_toLocation(
            &self,
            ruler: &NSRulerView,
            marker: &NSRulerMarker,
            location: CGFloat,
        ) -> CGFloat;

        #[cfg(feature = "NSRulerMarker")]
        #[method(rulerView:didMoveMarker:)]
        pub unsafe fn rulerView_didMoveMarker(&self, ruler: &NSRulerView, marker: &NSRulerMarker);

        #[cfg(feature = "NSRulerMarker")]
        #[method(rulerView:shouldRemoveMarker:)]
        pub unsafe fn rulerView_shouldRemoveMarker(
            &self,
            ruler: &NSRulerView,
            marker: &NSRulerMarker,
        ) -> bool;

        #[cfg(feature = "NSRulerMarker")]
        #[method(rulerView:didRemoveMarker:)]
        pub unsafe fn rulerView_didRemoveMarker(&self, ruler: &NSRulerView, marker: &NSRulerMarker);

        #[cfg(feature = "NSRulerMarker")]
        #[method(rulerView:shouldAddMarker:)]
        pub unsafe fn rulerView_shouldAddMarker(
            &self,
            ruler: &NSRulerView,
            marker: &NSRulerMarker,
        ) -> bool;

        #[cfg(all(feature = "NSRulerMarker", feature = "objc2-core-foundation"))]
        #[method(rulerView:willAddMarker:atLocation:)]
        pub unsafe fn rulerView_willAddMarker_atLocation(
            &self,
            ruler: &NSRulerView,
            marker: &NSRulerMarker,
            location: CGFloat,
        ) -> CGFloat;

        #[cfg(feature = "NSRulerMarker")]
        #[method(rulerView:didAddMarker:)]
        pub unsafe fn rulerView_didAddMarker(&self, ruler: &NSRulerView, marker: &NSRulerMarker);

        #[cfg(feature = "NSEvent")]
        #[method(rulerView:handleMouseDown:)]
        pub unsafe fn rulerView_handleMouseDown(&self, ruler: &NSRulerView, event: &NSEvent);

        #[method(rulerView:willSetClientView:)]
        pub unsafe fn rulerView_willSetClientView(&self, ruler: &NSRulerView, new_client: &NSView);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(rulerView:locationForPoint:)]
        pub unsafe fn rulerView_locationForPoint(
            &self,
            ruler: &NSRulerView,
            point: NSPoint,
        ) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(rulerView:pointForLocation:)]
        pub unsafe fn rulerView_pointForLocation(
            &self,
            ruler: &NSRulerView,
            point: CGFloat,
        ) -> NSPoint;
    }
);
