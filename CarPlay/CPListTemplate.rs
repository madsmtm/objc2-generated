//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/carplay/cpassistantcellactiontype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CPAssistantCellActionType(pub NSInteger);
impl CPAssistantCellActionType {
    #[doc(alias = "CPAssistantCellActionTypePlayMedia")]
    pub const PlayMedia: Self = Self(0);
    #[doc(alias = "CPAssistantCellActionTypeStartCall")]
    pub const StartCall: Self = Self(1);
}

unsafe impl Encode for CPAssistantCellActionType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CPAssistantCellActionType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/carplay/cpassistantcellvisibility?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CPAssistantCellVisibility(pub NSInteger);
impl CPAssistantCellVisibility {
    #[doc(alias = "CPAssistantCellVisibilityOff")]
    pub const Off: Self = Self(0);
    #[doc(alias = "CPAssistantCellVisibilityWhileLimitedUIActive")]
    pub const WhileLimitedUIActive: Self = Self(1);
    #[doc(alias = "CPAssistantCellVisibilityAlways")]
    pub const Always: Self = Self(2);
}

unsafe impl Encode for CPAssistantCellVisibility {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CPAssistantCellVisibility {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/carplay/cpassistantcellposition?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CPAssistantCellPosition(pub NSInteger);
impl CPAssistantCellPosition {
    #[doc(alias = "CPAssistantCellPositionTop")]
    pub const Top: Self = Self(0);
    #[doc(alias = "CPAssistantCellPositionBottom")]
    pub const Bottom: Self = Self(1);
}

unsafe impl Encode for CPAssistantCellPosition {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CPAssistantCellPosition {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// `CPAssistantCellConfiguration`encapsulates the configuration options for your assistant cell.
    ///
    ///
    /// Note: The Assistant Cell is only supported by CarPlay Audio and Communication apps.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/carplay/cpassistantcellconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CPAssistantCellConfiguration;
);

extern_conformance!(
    unsafe impl NSCoding for CPAssistantCellConfiguration {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for CPAssistantCellConfiguration {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for CPAssistantCellConfiguration {}
);

impl CPAssistantCellConfiguration {
    extern_methods!(
        /// Initialize an Assistant Cell Configuration with a position, visibility, and action representing the SiriKit intent that should be invoked when users select the assistant cell.
        #[unsafe(method(initWithPosition:visibility:assistantAction:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithPosition_visibility_assistantAction(
            this: Allocated<Self>,
            position: CPAssistantCellPosition,
            visibility: CPAssistantCellVisibility,
            assistant_action: CPAssistantCellActionType,
        ) -> Retained<Self>;

        /// The position of the Assistant Cell.
        ///
        ///
        /// Note: The default value of this property is
        /// `CPAssistantCellPositionTop.`
        #[unsafe(method(position))]
        #[unsafe(method_family = none)]
        pub unsafe fn position(&self) -> CPAssistantCellPosition;

        /// The visibility of the Assistant Cell.
        ///
        ///
        /// Note: The default value of this property is
        /// `CPAssistantCellVisibilityOff.`
        #[unsafe(method(visibility))]
        #[unsafe(method_family = none)]
        pub unsafe fn visibility(&self) -> CPAssistantCellVisibility;

        /// The action that Siri will perform when users select the assistant cell.
        #[unsafe(method(assistantAction))]
        #[unsafe(method_family = none)]
        pub unsafe fn assistantAction(&self) -> CPAssistantCellActionType;
    );
}

/// Methods declared on superclass `NSObject`.
impl CPAssistantCellConfiguration {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/carplay/cplisttemplate?language=objc)
    #[unsafe(super(CPTemplate, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CPTemplate")]
    pub struct CPListTemplate;
);

#[cfg(all(feature = "CPBarButtonProviding", feature = "CPTemplate"))]
extern_conformance!(
    unsafe impl CPBarButtonProviding for CPListTemplate {}
);

#[cfg(feature = "CPTemplate")]
extern_conformance!(
    unsafe impl NSCoding for CPListTemplate {}
);

#[cfg(feature = "CPTemplate")]
extern_conformance!(
    unsafe impl NSObjectProtocol for CPListTemplate {}
);

#[cfg(feature = "CPTemplate")]
extern_conformance!(
    unsafe impl NSSecureCoding for CPListTemplate {}
);

#[cfg(feature = "CPTemplate")]
impl CPListTemplate {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "CPListSection")]
        /// Initialize a list template with one or more sections of items and an optional title.
        #[unsafe(method(initWithTitle:sections:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithTitle_sections(
            this: Allocated<Self>,
            title: Option<&NSString>,
            sections: &NSArray<CPListSection>,
        ) -> Retained<Self>;

        #[cfg(feature = "CPListSection")]
        /// Initialize a list template with one or more sections of items, an optional title, and configuration for the assistant cell via a
        /// `CPAssistantCellConfiguration`object.
        ///
        ///
        /// Note: The Assistant Cell is only supported by CarPlay Audio and Communication Apps.
        ///
        ///
        /// Unlike
        /// `CPListItem,`your application will not receive a callback when the user selects the cell.
        /// Instead, configure an Intents app extention to receive user requests from SiriKit, in order to turn the requests into an
        /// app-specific actions.
        #[unsafe(method(initWithTitle:sections:assistantCellConfiguration:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithTitle_sections_assistantCellConfiguration(
            this: Allocated<Self>,
            title: Option<&NSString>,
            sections: &NSArray<CPListSection>,
            assistant_cell_configuration: Option<&CPAssistantCellConfiguration>,
        ) -> Retained<Self>;

        /// The list template's delegate is informed of list selection events.
        #[deprecated]
        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn CPListTemplateDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[deprecated]
        #[unsafe(method(setDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn CPListTemplateDelegate>>,
        );

        /// The maximum number of items, across all sections, that may appear in a
        /// `CPListTemplate.`
        /// Note: Your list template will display the first
        /// `maximumItemCount`items, across all sections.
        /// Any items or sections beyond that limit will be trimmed.
        #[unsafe(method(maximumItemCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn maximumItemCount() -> NSUInteger;

        /// The maximum number of sections that may appear in a
        /// `CPListTemplate.`
        /// Note: Your list template will display the first
        /// `maximumSectionCount`sections.
        /// Any sections beyond that limit will be trimmed.
        #[unsafe(method(maximumSectionCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn maximumSectionCount() -> NSUInteger;

        #[cfg(feature = "CPListSection")]
        /// The sections displayed in this list.
        #[unsafe(method(sections))]
        #[unsafe(method_family = none)]
        pub unsafe fn sections(&self) -> Retained<NSArray<CPListSection>>;

        /// Title shown in the navigation bar while this template is visible.
        #[unsafe(method(title))]
        #[unsafe(method_family = none)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "CPListSection")]
        /// Update the list of sections displayed in this list template, reloading
        /// the table view displaying this list.
        #[unsafe(method(updateSections:))]
        #[unsafe(method_family = none)]
        pub unsafe fn updateSections(&self, sections: &NSArray<CPListSection>);

        /// The number of sections currently displayed in this list template.
        #[unsafe(method(sectionCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn sectionCount(&self) -> NSUInteger;

        /// The number of items currently displayed in this list template, across all sections.
        #[unsafe(method(itemCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn itemCount(&self) -> NSUInteger;

        #[cfg(feature = "CPListItemTypes")]
        /// Return an
        /// `NSIndexPath`for the specified item, if it exists in any section
        /// in this list template, or nil if not found.
        #[unsafe(method(indexPathForItem:))]
        #[unsafe(method_family = none)]
        pub unsafe fn indexPathForItem(
            &self,
            item: &ProtocolObject<dyn CPListTemplateItem>,
        ) -> Option<Retained<NSIndexPath>>;

        /// An optional array of strings, ordered from most to least preferred.
        /// The variant strings should be provided as localized, displayable content.
        /// The system will select the first variant that fits the available space.
        ///
        /// If the list template does not contain any items (itemCount == 0), then
        /// the template will display an empty view with a title and subtitle to indicate
        /// that the template has no list items.
        ///
        /// If the list template is updated to contain items, the empty view will be automatically
        /// removed.
        #[unsafe(method(emptyViewTitleVariants))]
        #[unsafe(method_family = none)]
        pub unsafe fn emptyViewTitleVariants(&self) -> Retained<NSArray<NSString>>;

        /// Setter for [`emptyViewTitleVariants`][Self::emptyViewTitleVariants].
        #[unsafe(method(setEmptyViewTitleVariants:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEmptyViewTitleVariants(
            &self,
            empty_view_title_variants: &NSArray<NSString>,
        );

        /// An optional array of strings, ordered from most to least preferred.
        /// The variant strings should be provided as localized, displayable content.
        /// The system will select the first variant that fits the available space.
        ///
        /// If the list template does not contain any items (itemCount == 0), then
        /// the template will display an empty view with a title and subtitle to indicate
        /// that the template has no list items.
        ///
        /// If the list template is updated to contain items, the empty view will be automatically
        /// removed.
        #[unsafe(method(emptyViewSubtitleVariants))]
        #[unsafe(method_family = none)]
        pub unsafe fn emptyViewSubtitleVariants(&self) -> Retained<NSArray<NSString>>;

        /// Setter for [`emptyViewSubtitleVariants`][Self::emptyViewSubtitleVariants].
        #[unsafe(method(setEmptyViewSubtitleVariants:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEmptyViewSubtitleVariants(
            &self,
            empty_view_subtitle_variants: &NSArray<NSString>,
        );

        /// If YES, a spinning activity indicator will be displayed while the list template contains no items.
        /// The activity indicator will be displayed in addition to any
        /// `emptyViewTitleVariants`or
        /// `emptyViewSubtitleVariants.`
        #[unsafe(method(showsSpinnerWhileEmpty))]
        #[unsafe(method_family = none)]
        pub unsafe fn showsSpinnerWhileEmpty(&self) -> bool;

        /// Setter for [`showsSpinnerWhileEmpty`][Self::showsSpinnerWhileEmpty].
        #[unsafe(method(setShowsSpinnerWhileEmpty:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setShowsSpinnerWhileEmpty(&self, shows_spinner_while_empty: bool);

        /// The configuration of the Assistant Cell.
        ///
        /// Assigning to this property will dynamically update the List Template to reflect the visibility, position, and intent identifier of the Assistant Cell.
        ///
        ///
        /// Note: The Assistant Cell is only supported by CarPlay Audio and Communication Apps.
        ///
        ///
        /// Unlike
        /// `CPListItem,`your application will not receive a callback when the user selects the cell.
        /// Instead, configure an Intents app extention to receive user requests from SiriKit, in order to turn the requests into an
        /// app-specific actions.
        #[unsafe(method(assistantCellConfiguration))]
        #[unsafe(method_family = none)]
        pub unsafe fn assistantCellConfiguration(
            &self,
        ) -> Option<Retained<CPAssistantCellConfiguration>>;

        /// Setter for [`assistantCellConfiguration`][Self::assistantCellConfiguration].
        #[unsafe(method(setAssistantCellConfiguration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAssistantCellConfiguration(
            &self,
            assistant_cell_configuration: Option<&CPAssistantCellConfiguration>,
        );
    );
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/carplay/cplisttemplatedelegate?language=objc)
    #[deprecated]
    pub unsafe trait CPListTemplateDelegate: NSObjectProtocol {
        #[cfg(all(feature = "CPListItem", feature = "CPTemplate", feature = "block2"))]
        /// The user has selected an item in the list template.
        ///
        /// Your app has an opportunity to perform any necessary operations to prepare for completing
        /// this item selection. The list template will display a spinner after a short delay.
        ///
        /// You must call the completion block after your app has finished loading and updated its UI.
        ///
        ///
        /// Parameter `listTemplate`: The list template containing this item
        ///
        /// Parameter `item`: The item selected by the user
        ///
        /// Parameter `completionHandler`: A completion block you must call after you have updated your UI.
        #[deprecated]
        #[unsafe(method(listTemplate:didSelectListItem:completionHandler:))]
        #[unsafe(method_family = none)]
        unsafe fn listTemplate_didSelectListItem_completionHandler(
            &self,
            list_template: &CPListTemplate,
            item: &CPListItem,
            completion_handler: &block2::DynBlock<dyn Fn()>,
        );
    }
);
