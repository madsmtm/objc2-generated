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

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/carplay/cpnowplayingbuttonmaximumimagesize?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static CPNowPlayingButtonMaximumImageSize: CGSize;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/carplay/cpnowplayingbutton?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CPNowPlayingButton;
);

extern_conformance!(
    unsafe impl NSCoding for CPNowPlayingButton {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for CPNowPlayingButton {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for CPNowPlayingButton {}
);

impl CPNowPlayingButton {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// Initialize a now playing button with a handler. The handler will be called when the user
        /// selects this button.
        ///
        ///
        /// Note: Your app should use this method in one of the six concrete subclasses of
        /// `CPNowPlayingButton.`Do not initialize this class directly.
        #[unsafe(method(initWithHandler:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithHandler(
            this: Allocated<Self>,
            handler: Option<&block2::DynBlock<dyn Fn(NonNull<CPNowPlayingButton>)>>,
        ) -> Retained<Self>;

        /// A Boolean value indicating whether the button is enabled.
        ///
        ///
        /// Set the value of this property to
        /// `YES`to enable the button or
        /// `NO`to disable it. The default value of this property is
        /// `YES.`
        #[unsafe(method(isEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isEnabled(&self) -> bool;

        /// Setter for [`isEnabled`][Self::isEnabled].
        #[unsafe(method(setEnabled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        /// A Boolean value indicating whether the button is selected. When selected, the button draws with a selected appearance to
        /// indicate its selected state.
        ///
        ///
        /// Only custom image buttons may display a custom selected state. Other system-provided buttons, like repeat
        /// and shuffle, change their selected states depending on what your app reports for current repeat and shuffle states.
        #[unsafe(method(isSelected))]
        #[unsafe(method_family = none)]
        pub unsafe fn isSelected(&self) -> bool;

        /// Setter for [`isSelected`][Self::isSelected].
        #[unsafe(method(setSelected:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSelected(&self, selected: bool);
    );
}

extern_class!(
    /// A now playing button that indicates the current shuffle mode for your app.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/carplay/cpnowplayingshufflebutton?language=objc)
    #[unsafe(super(CPNowPlayingButton, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CPNowPlayingShuffleButton;
);

extern_conformance!(
    unsafe impl NSCoding for CPNowPlayingShuffleButton {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for CPNowPlayingShuffleButton {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for CPNowPlayingShuffleButton {}
);

impl CPNowPlayingShuffleButton {
    extern_methods!();
}

/// Methods declared on superclass `CPNowPlayingButton`.
impl CPNowPlayingShuffleButton {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// Initialize a now playing button with a handler. The handler will be called when the user
        /// selects this button.
        ///
        ///
        /// Note: Your app should use this method in one of the six concrete subclasses of
        /// `CPNowPlayingButton.`Do not initialize this class directly.
        #[unsafe(method(initWithHandler:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithHandler(
            this: Allocated<Self>,
            handler: Option<&block2::DynBlock<dyn Fn(NonNull<CPNowPlayingButton>)>>,
        ) -> Retained<Self>;
    );
}

extern_class!(
    /// A now playing button that can be used to allow the user to add the current
    /// playing item to a collection, like their library.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/carplay/cpnowplayingaddtolibrarybutton?language=objc)
    #[unsafe(super(CPNowPlayingButton, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CPNowPlayingAddToLibraryButton;
);

extern_conformance!(
    unsafe impl NSCoding for CPNowPlayingAddToLibraryButton {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for CPNowPlayingAddToLibraryButton {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for CPNowPlayingAddToLibraryButton {}
);

impl CPNowPlayingAddToLibraryButton {
    extern_methods!();
}

/// Methods declared on superclass `CPNowPlayingButton`.
impl CPNowPlayingAddToLibraryButton {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// Initialize a now playing button with a handler. The handler will be called when the user
        /// selects this button.
        ///
        ///
        /// Note: Your app should use this method in one of the six concrete subclasses of
        /// `CPNowPlayingButton.`Do not initialize this class directly.
        #[unsafe(method(initWithHandler:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithHandler(
            this: Allocated<Self>,
            handler: Option<&block2::DynBlock<dyn Fn(NonNull<CPNowPlayingButton>)>>,
        ) -> Retained<Self>;
    );
}

extern_class!(
    /// A now playing button that shows a callout-style action. For example,
    /// your app could present a
    /// `CPActionSheetTemplate`to show more actions
    /// when the user taps this button.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/carplay/cpnowplayingmorebutton?language=objc)
    #[unsafe(super(CPNowPlayingButton, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CPNowPlayingMoreButton;
);

extern_conformance!(
    unsafe impl NSCoding for CPNowPlayingMoreButton {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for CPNowPlayingMoreButton {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for CPNowPlayingMoreButton {}
);

impl CPNowPlayingMoreButton {
    extern_methods!();
}

/// Methods declared on superclass `CPNowPlayingButton`.
impl CPNowPlayingMoreButton {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// Initialize a now playing button with a handler. The handler will be called when the user
        /// selects this button.
        ///
        ///
        /// Note: Your app should use this method in one of the six concrete subclasses of
        /// `CPNowPlayingButton.`Do not initialize this class directly.
        #[unsafe(method(initWithHandler:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithHandler(
            this: Allocated<Self>,
            handler: Option<&block2::DynBlock<dyn Fn(NonNull<CPNowPlayingButton>)>>,
        ) -> Retained<Self>;
    );
}

extern_class!(
    /// A now playing button that shows the current playback rate and allows
    /// the user to cycle between different playback rates provided by your app.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/carplay/cpnowplayingplaybackratebutton?language=objc)
    #[unsafe(super(CPNowPlayingButton, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CPNowPlayingPlaybackRateButton;
);

extern_conformance!(
    unsafe impl NSCoding for CPNowPlayingPlaybackRateButton {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for CPNowPlayingPlaybackRateButton {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for CPNowPlayingPlaybackRateButton {}
);

impl CPNowPlayingPlaybackRateButton {
    extern_methods!();
}

/// Methods declared on superclass `CPNowPlayingButton`.
impl CPNowPlayingPlaybackRateButton {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// Initialize a now playing button with a handler. The handler will be called when the user
        /// selects this button.
        ///
        ///
        /// Note: Your app should use this method in one of the six concrete subclasses of
        /// `CPNowPlayingButton.`Do not initialize this class directly.
        #[unsafe(method(initWithHandler:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithHandler(
            this: Allocated<Self>,
            handler: Option<&block2::DynBlock<dyn Fn(NonNull<CPNowPlayingButton>)>>,
        ) -> Retained<Self>;
    );
}

extern_class!(
    /// A now playing button that shows the current repeat state, like "once"
    /// or "all".
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/carplay/cpnowplayingrepeatbutton?language=objc)
    #[unsafe(super(CPNowPlayingButton, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CPNowPlayingRepeatButton;
);

extern_conformance!(
    unsafe impl NSCoding for CPNowPlayingRepeatButton {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for CPNowPlayingRepeatButton {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for CPNowPlayingRepeatButton {}
);

impl CPNowPlayingRepeatButton {
    extern_methods!();
}

/// Methods declared on superclass `CPNowPlayingButton`.
impl CPNowPlayingRepeatButton {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// Initialize a now playing button with a handler. The handler will be called when the user
        /// selects this button.
        ///
        ///
        /// Note: Your app should use this method in one of the six concrete subclasses of
        /// `CPNowPlayingButton.`Do not initialize this class directly.
        #[unsafe(method(initWithHandler:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithHandler(
            this: Allocated<Self>,
            handler: Option<&block2::DynBlock<dyn Fn(NonNull<CPNowPlayingButton>)>>,
        ) -> Retained<Self>;
    );
}

extern_class!(
    /// A now playing button that shows a custom image provided by your app
    /// for any other custom actions on the now playing screen.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/carplay/cpnowplayingimagebutton?language=objc)
    #[unsafe(super(CPNowPlayingButton, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CPNowPlayingImageButton;
);

extern_conformance!(
    unsafe impl NSCoding for CPNowPlayingImageButton {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for CPNowPlayingImageButton {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for CPNowPlayingImageButton {}
);

impl CPNowPlayingImageButton {
    extern_methods!(
        #[cfg(all(feature = "block2", feature = "objc2-ui-kit"))]
        /// Initialize a button with a custom image.
        ///
        ///
        /// Parameter `image`: A custom image for this button. The maximum image size is CPNowPlayingButtonMaximumImageSize; larger images will be scaled down.
        ///
        /// Parameter `handler`: A block to execute when the user selects the button. The block has no return value and takes the selected button as its only parameter.
        ///
        ///
        /// Provided image should be dark variant and will be recolored.
        #[unsafe(method(initWithImage:handler:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithImage_handler(
            this: Allocated<Self>,
            image: &UIImage,
            handler: Option<&block2::DynBlock<dyn Fn(NonNull<CPNowPlayingButton>)>>,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-ui-kit")]
        /// The custom image, if any, displayed on the button.
        ///
        ///
        /// Animated images are not supported. If an animated image is assigned, only the first image will be used. If an empty or zero-size image is provided, a system placeholder glyph will be displayed instead.
        #[unsafe(method(image))]
        #[unsafe(method_family = none)]
        pub unsafe fn image(&self) -> Option<Retained<UIImage>>;
    );
}

/// Methods declared on superclass `CPNowPlayingButton`.
impl CPNowPlayingImageButton {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// Initialize a now playing button with a handler. The handler will be called when the user
        /// selects this button.
        ///
        ///
        /// Note: Your app should use this method in one of the six concrete subclasses of
        /// `CPNowPlayingButton.`Do not initialize this class directly.
        #[unsafe(method(initWithHandler:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithHandler(
            this: Allocated<Self>,
            handler: Option<&block2::DynBlock<dyn Fn(NonNull<CPNowPlayingButton>)>>,
        ) -> Retained<Self>;
    );
}
