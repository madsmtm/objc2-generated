//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "dispatch2")]
use dispatch2::*;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-ui-kit")]
use objc2_ui_kit::*;

use crate::*;

extern "C" {
    /// The maximum number of images allowed in a
    /// `CPListImageRowItem.`The system may display fewer than this number of images, depending on the available width of the car screen.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/carplay/cpmaximumnumberofgridimages?language=objc)
    pub static CPMaximumNumberOfGridImages: NSUInteger;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/carplay/cplistimagerowitem?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CPListImageRowItem;
);

#[cfg(feature = "CPListItemTypes")]
extern_conformance!(
    unsafe impl CPListTemplateItem for CPListImageRowItem {}
);

#[cfg(feature = "CPListItemTypes")]
extern_conformance!(
    unsafe impl CPSelectableListItem for CPListImageRowItem {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for CPListImageRowItem {}
);

impl CPListImageRowItem {
    extern_methods!(
        #[cfg(feature = "objc2-ui-kit")]
        /// Initialize a list image row item with a text string and an array of
        /// `UIImage`for the grid of images.
        ///
        /// When providing an image, your app should provide a
        /// `UIImage`that is display-ready. If necessary for the image, provide
        /// light and dark styles by using an asset from your asset catalog, prepared with light and dark styles
        /// or by using
        /// `UIImageAsset`to combine two
        /// `UIImage`instances into a single image with
        /// both styles.
        ///
        /// UIImageAsset is used to combine multiple UIImages with different trait collections into a single UIImage.
        ///
        ///
        /// Note: The expected image size is given by +[CPListImageRowItem maximumImageSize]. Images provided
        /// will be resized to this size if necessary.
        ///
        ///
        /// To properly size your images, your app should size them to the display scale of the car screen.
        /// See -[CPInterfaceController carTraitCollection].
        #[unsafe(method(initWithText:images:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithText_images(
            this: Allocated<Self>,
            text: &NSString,
            images: &NSArray<UIImage>,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-ui-kit")]
        /// Initialize a list image row item with a text string, an array of
        /// `UIImage`for the grid of images, and an array of
        /// `NSString`titles, one displayed below each image.
        ///
        /// The number of titles in the
        /// `imageTitles`list should be equal to the number of images in the
        /// `images`list.
        ///
        /// When providing an image, your app should provide a
        /// `UIImage`that is display-ready. If necessary for the image, provide
        /// light and dark styles by using an asset from your asset catalog, prepared with light and dark styles
        /// or by using
        /// `UIImageAsset`to combine two
        /// `UIImage`instances into a single image with
        /// both styles.
        ///
        /// UIImageAsset is used to combine multiple UIImages with different trait collections into a single UIImage.
        ///
        ///
        /// Note: The expected image size is given by +[CPListImageRowItem maximumImageSize]. Images provided
        /// will be resized to this size if necessary.
        ///
        ///
        /// To properly size your images, your app should size them to the display scale of the car screen.
        /// See -[CPInterfaceController carTraitCollection].
        #[unsafe(method(initWithText:images:imageTitles:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithText_images_imageTitles(
            this: Allocated<Self>,
            text: &NSString,
            images: &NSArray<UIImage>,
            image_titles: &NSArray<NSString>,
        ) -> Retained<Self>;

        /// The primary text shown in a cell displaying this list item.
        #[unsafe(method(text))]
        #[unsafe(method_family = none)]
        pub unsafe fn text(&self) -> Option<Retained<NSString>>;

        /// Setter for [`text`][Self::text].
        #[unsafe(method(setText:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setText(&self, text: Option<&NSString>);

        /// Any custom user info related to this item.
        #[unsafe(method(userInfo))]
        #[unsafe(method_family = none)]
        pub unsafe fn userInfo(&self) -> Option<Retained<AnyObject>>;

        /// Setter for [`userInfo`][Self::userInfo].
        #[unsafe(method(setUserInfo:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&AnyObject>);

        #[cfg(all(feature = "CPListItemTypes", feature = "block2", feature = "dispatch2"))]
        /// An optional action block, fired when the user selects this item in a list template.
        #[unsafe(method(handler))]
        #[unsafe(method_family = none)]
        pub unsafe fn handler(
            &self,
        ) -> *mut block2::DynBlock<
            dyn Fn(NonNull<ProtocolObject<dyn CPSelectableListItem>>, dispatch_block_t),
        >;

        #[cfg(all(feature = "CPListItemTypes", feature = "block2", feature = "dispatch2"))]
        /// Setter for [`handler`][Self::handler].
        #[unsafe(method(setHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setHandler(
            &self,
            handler: Option<
                &block2::DynBlock<
                    dyn Fn(NonNull<ProtocolObject<dyn CPSelectableListItem>>, dispatch_block_t),
                >,
            >,
        );

        /// A Boolean value indicating whether the list item is enabled.
        ///
        ///
        /// Set the value of this property to
        /// `YES`to enable the list item or
        /// `NO`to disable it. The default value of this property is
        /// `YES.`
        #[unsafe(method(isEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isEnabled(&self) -> bool;

        /// Setter for [`isEnabled`][Self::isEnabled].
        #[unsafe(method(setEnabled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[cfg(feature = "objc2-ui-kit")]
        /// Read-only access to the grid images shown in a row in the cell.
        ///
        ///
        /// Note: The maximum number of images shown is
        /// `CPMaximumNumberOfGridImages.`If you supply more images, only the first
        /// `CPMaximumNumberOfGridImages`will be used.
        #[unsafe(method(gridImages))]
        #[unsafe(method_family = none)]
        pub unsafe fn gridImages(&self) -> Retained<NSArray<UIImage>>;

        #[cfg(feature = "objc2-ui-kit")]
        /// Update the images displayed in this image row item. If this image row
        /// item is already displayed in a list template, this image row item will be
        /// automatically reloaded.
        ///
        /// When providing an image, your app should provide a
        /// `UIImage`that is display-ready. If necessary for the image, provide
        /// light and dark styles by using an asset from your asset catalog, prepared with light and dark styles
        /// or by using
        /// `UIImageAsset`to combine two
        /// `UIImage`instances into a single image with
        /// both styles.
        ///
        /// UIImageAsset is used to combine multiple UIImages with different trait collections into a single UIImage.
        ///
        ///
        /// See: To update/reload the title of the image row item, assign to
        /// the
        /// `text`property of the image row item.
        ///
        /// See: To update/reload the title labels displayed below each image, assign to
        /// the
        /// `imageTitles`property of the image row item.
        #[unsafe(method(updateImages:))]
        #[unsafe(method_family = none)]
        pub unsafe fn updateImages(&self, grid_images: &NSArray<UIImage>);

        /// Update the titles displayed each image in this image row item. If this image row
        /// item is already displayed in a list template, then it will be automatically
        /// reloaded.
        ///
        ///
        /// See: To update/reload the title of the image row item, assign to
        /// the
        /// `text`property of the image row item.
        #[unsafe(method(imageTitles))]
        #[unsafe(method_family = none)]
        pub unsafe fn imageTitles(&self) -> Retained<NSArray<NSString>>;

        /// Setter for [`imageTitles`][Self::imageTitles].
        #[unsafe(method(setImageTitles:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setImageTitles(&self, image_titles: &NSArray<NSString>);

        #[cfg(all(feature = "block2", feature = "dispatch2"))]
        /// A block that is called when the user selects one of the images in this image row item.
        ///
        /// The user may also select the cell itself - for that event, specify a
        /// `handler.`
        #[unsafe(method(listImageRowHandler))]
        #[unsafe(method_family = none)]
        pub unsafe fn listImageRowHandler(
            &self,
        ) -> *mut block2::DynBlock<dyn Fn(NonNull<CPListImageRowItem>, NSInteger, dispatch_block_t)>;

        #[cfg(all(feature = "block2", feature = "dispatch2"))]
        /// Setter for [`listImageRowHandler`][Self::listImageRowHandler].
        #[unsafe(method(setListImageRowHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setListImageRowHandler(
            &self,
            list_image_row_handler: Option<
                &block2::DynBlock<dyn Fn(NonNull<CPListImageRowItem>, NSInteger, dispatch_block_t)>,
            >,
        );

        #[cfg(feature = "objc2-core-foundation")]
        /// The expected image size for the grid images in your
        /// `CPListImageRowItem.`Images provided
        /// will be resized to this size.
        ///
        ///
        /// To properly size your images, your app should size them to the display scale of the car screen.
        /// See -[CPInterfaceController carTraitCollection].
        #[unsafe(method(maximumImageSize))]
        #[unsafe(method_family = none)]
        pub unsafe fn maximumImageSize() -> CGSize;
    );
}

/// Methods declared on superclass `NSObject`.
impl CPListImageRowItem {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
