//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstrackingareaoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTrackingAreaOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSTrackingAreaOptions: NSUInteger {
        #[doc(alias = "NSTrackingMouseEnteredAndExited")]
        const MouseEnteredAndExited = 0x01;
        #[doc(alias = "NSTrackingMouseMoved")]
        const MouseMoved = 0x02;
        #[doc(alias = "NSTrackingCursorUpdate")]
        const CursorUpdate = 0x04;
        #[doc(alias = "NSTrackingActiveWhenFirstResponder")]
        const ActiveWhenFirstResponder = 0x10;
        #[doc(alias = "NSTrackingActiveInKeyWindow")]
        const ActiveInKeyWindow = 0x20;
        #[doc(alias = "NSTrackingActiveInActiveApp")]
        const ActiveInActiveApp = 0x40;
        #[doc(alias = "NSTrackingActiveAlways")]
        const ActiveAlways = 0x80;
        #[doc(alias = "NSTrackingAssumeInside")]
        const AssumeInside = 0x100;
        #[doc(alias = "NSTrackingInVisibleRect")]
        const InVisibleRect = 0x200;
        #[doc(alias = "NSTrackingEnabledDuringMouseDrag")]
        const EnabledDuringMouseDrag = 0x400;
    }
}

unsafe impl Encode for NSTrackingAreaOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTrackingAreaOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstrackingarea?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTrackingArea;
);

extern_conformance!(
    unsafe impl NSCoding for NSTrackingArea {}
);

extern_conformance!(
    unsafe impl NSCopying for NSTrackingArea {}
);

unsafe impl CopyingHelper for NSTrackingArea {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for NSTrackingArea {}
);

impl NSTrackingArea {
    extern_methods!(
        #[unsafe(method(initWithRect:options:owner:userInfo:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithRect_options_owner_userInfo(
            this: Allocated<Self>,
            rect: NSRect,
            options: NSTrackingAreaOptions,
            owner: Option<&AnyObject>,
            user_info: Option<&NSDictionary<AnyObject, AnyObject>>,
        ) -> Retained<Self>;

        #[unsafe(method(rect))]
        #[unsafe(method_family = none)]
        pub unsafe fn rect(&self) -> NSRect;

        #[unsafe(method(options))]
        #[unsafe(method_family = none)]
        pub unsafe fn options(&self) -> NSTrackingAreaOptions;

        #[unsafe(method(owner))]
        #[unsafe(method_family = none)]
        pub unsafe fn owner(&self) -> Option<Retained<AnyObject>>;

        #[unsafe(method(userInfo))]
        #[unsafe(method_family = none)]
        pub unsafe fn userInfo(&self) -> Option<Retained<NSDictionary<AnyObject, AnyObject>>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSTrackingArea {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
