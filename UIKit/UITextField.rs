//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextborderstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITextBorderStyle(pub NSInteger);
impl UITextBorderStyle {
    #[doc(alias = "UITextBorderStyleNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UITextBorderStyleLine")]
    pub const Line: Self = Self(1);
    #[doc(alias = "UITextBorderStyleBezel")]
    pub const Bezel: Self = Self(2);
    #[doc(alias = "UITextBorderStyleRoundedRect")]
    pub const RoundedRect: Self = Self(3);
}

unsafe impl Encode for UITextBorderStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITextBorderStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextfieldviewmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITextFieldViewMode(pub NSInteger);
impl UITextFieldViewMode {
    #[doc(alias = "UITextFieldViewModeNever")]
    pub const Never: Self = Self(0);
    #[doc(alias = "UITextFieldViewModeWhileEditing")]
    pub const WhileEditing: Self = Self(1);
    #[doc(alias = "UITextFieldViewModeUnlessEditing")]
    pub const UnlessEditing: Self = Self(2);
    #[doc(alias = "UITextFieldViewModeAlways")]
    pub const Always: Self = Self(3);
}

unsafe impl Encode for UITextFieldViewMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITextFieldViewMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextfielddidendeditingreason?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITextFieldDidEndEditingReason(pub NSInteger);
impl UITextFieldDidEndEditingReason {
    #[doc(alias = "UITextFieldDidEndEditingReasonCommitted")]
    pub const Committed: Self = Self(0);
    #[doc(alias = "UITextFieldDidEndEditingReasonCancelled")]
    pub const Cancelled: Self = Self(1);
}

unsafe impl Encode for UITextFieldDidEndEditingReason {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITextFieldDidEndEditingReason {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextfield?language=objc)
    #[unsafe(super(UIControl, UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    pub struct UITextField;
);

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
extern_conformance!(
    unsafe impl CALayerDelegate for UITextField {}
);

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl NSCoding for UITextField {}
);

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl NSObjectProtocol for UITextField {}
);

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UIAppearance for UITextField {}
);

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UIAppearanceContainer for UITextField {}
);

#[cfg(all(
    feature = "UIContentSizeCategoryAdjusting",
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UIContentSizeCategoryAdjusting for UITextField {}
);

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UICoordinateSpace for UITextField {}
);

#[cfg(all(
    feature = "UIControl",
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UIDynamicItem for UITextField {}
);

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UIFocusEnvironment for UITextField {}
);

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UIFocusItem for UITextField {}
);

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UIFocusItemContainer for UITextField {}
);

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITextInput",
    feature = "UITextInputTraits",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UIKeyInput for UITextField {}
);

#[cfg(all(
    feature = "UIControl",
    feature = "UILetterformAwareAdjusting",
    feature = "UIResponder",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UILetterformAwareAdjusting for UITextField {}
);

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
extern_conformance!(
    unsafe impl UIResponderStandardEditActions for UITextField {}
);

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITextInput",
    feature = "UITextInputTraits",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UITextInput for UITextField {}
);

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITextInputTraits",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UITextInputTraits for UITextField {}
);

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UITraitEnvironment for UITextField {}
);

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
impl UITextField {
    extern_methods!(
        #[unsafe(method(text))]
        #[unsafe(method_family = none)]
        pub unsafe fn text(&self) -> Option<Retained<NSString>>;

        /// Setter for [`text`][Self::text].
        #[unsafe(method(setText:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setText(&self, text: Option<&NSString>);

        #[unsafe(method(attributedText))]
        #[unsafe(method_family = none)]
        pub unsafe fn attributedText(&self) -> Option<Retained<NSAttributedString>>;

        /// Setter for [`attributedText`][Self::attributedText].
        #[unsafe(method(setAttributedText:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAttributedText(&self, attributed_text: Option<&NSAttributedString>);

        #[cfg(feature = "UIColor")]
        #[unsafe(method(textColor))]
        #[unsafe(method_family = none)]
        pub unsafe fn textColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        /// Setter for [`textColor`][Self::textColor].
        #[unsafe(method(setTextColor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTextColor(&self, text_color: Option<&UIColor>);

        #[cfg(feature = "UIFont")]
        #[unsafe(method(font))]
        #[unsafe(method_family = none)]
        pub unsafe fn font(&self) -> Option<Retained<UIFont>>;

        #[cfg(feature = "UIFont")]
        /// Setter for [`font`][Self::font].
        #[unsafe(method(setFont:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFont(&self, font: Option<&UIFont>);

        #[cfg(feature = "NSText")]
        #[unsafe(method(textAlignment))]
        #[unsafe(method_family = none)]
        pub unsafe fn textAlignment(&self) -> NSTextAlignment;

        #[cfg(feature = "NSText")]
        /// Setter for [`textAlignment`][Self::textAlignment].
        #[unsafe(method(setTextAlignment:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTextAlignment(&self, text_alignment: NSTextAlignment);

        #[unsafe(method(borderStyle))]
        #[unsafe(method_family = none)]
        pub unsafe fn borderStyle(&self) -> UITextBorderStyle;

        /// Setter for [`borderStyle`][Self::borderStyle].
        #[unsafe(method(setBorderStyle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setBorderStyle(&self, border_style: UITextBorderStyle);

        #[unsafe(method(defaultTextAttributes))]
        #[unsafe(method_family = none)]
        pub unsafe fn defaultTextAttributes(
            &self,
        ) -> Retained<NSDictionary<NSAttributedStringKey, AnyObject>>;

        /// Setter for [`defaultTextAttributes`][Self::defaultTextAttributes].
        #[unsafe(method(setDefaultTextAttributes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDefaultTextAttributes(
            &self,
            default_text_attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
        );

        #[unsafe(method(placeholder))]
        #[unsafe(method_family = none)]
        pub unsafe fn placeholder(&self) -> Option<Retained<NSString>>;

        /// Setter for [`placeholder`][Self::placeholder].
        #[unsafe(method(setPlaceholder:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPlaceholder(&self, placeholder: Option<&NSString>);

        #[unsafe(method(attributedPlaceholder))]
        #[unsafe(method_family = none)]
        pub unsafe fn attributedPlaceholder(&self) -> Option<Retained<NSAttributedString>>;

        /// Setter for [`attributedPlaceholder`][Self::attributedPlaceholder].
        #[unsafe(method(setAttributedPlaceholder:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAttributedPlaceholder(
            &self,
            attributed_placeholder: Option<&NSAttributedString>,
        );

        #[unsafe(method(clearsOnBeginEditing))]
        #[unsafe(method_family = none)]
        pub unsafe fn clearsOnBeginEditing(&self) -> bool;

        /// Setter for [`clearsOnBeginEditing`][Self::clearsOnBeginEditing].
        #[unsafe(method(setClearsOnBeginEditing:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setClearsOnBeginEditing(&self, clears_on_begin_editing: bool);

        #[unsafe(method(adjustsFontSizeToFitWidth))]
        #[unsafe(method_family = none)]
        pub unsafe fn adjustsFontSizeToFitWidth(&self) -> bool;

        /// Setter for [`adjustsFontSizeToFitWidth`][Self::adjustsFontSizeToFitWidth].
        #[unsafe(method(setAdjustsFontSizeToFitWidth:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAdjustsFontSizeToFitWidth(&self, adjusts_font_size_to_fit_width: bool);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(minimumFontSize))]
        #[unsafe(method_family = none)]
        pub unsafe fn minimumFontSize(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`minimumFontSize`][Self::minimumFontSize].
        #[unsafe(method(setMinimumFontSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMinimumFontSize(&self, minimum_font_size: CGFloat);

        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn UITextFieldDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[unsafe(method(setDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UITextFieldDelegate>>,
        );

        #[cfg(feature = "UIImage")]
        #[unsafe(method(background))]
        #[unsafe(method_family = none)]
        pub unsafe fn background(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        /// Setter for [`background`][Self::background].
        #[unsafe(method(setBackground:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setBackground(&self, background: Option<&UIImage>);

        #[cfg(feature = "UIImage")]
        #[unsafe(method(disabledBackground))]
        #[unsafe(method_family = none)]
        pub unsafe fn disabledBackground(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        /// Setter for [`disabledBackground`][Self::disabledBackground].
        #[unsafe(method(setDisabledBackground:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDisabledBackground(&self, disabled_background: Option<&UIImage>);

        #[unsafe(method(isEditing))]
        #[unsafe(method_family = none)]
        pub unsafe fn isEditing(&self) -> bool;

        #[unsafe(method(allowsEditingTextAttributes))]
        #[unsafe(method_family = none)]
        pub unsafe fn allowsEditingTextAttributes(&self) -> bool;

        /// Setter for [`allowsEditingTextAttributes`][Self::allowsEditingTextAttributes].
        #[unsafe(method(setAllowsEditingTextAttributes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAllowsEditingTextAttributes(&self, allows_editing_text_attributes: bool);

        #[unsafe(method(typingAttributes))]
        #[unsafe(method_family = none)]
        pub unsafe fn typingAttributes(
            &self,
        ) -> Option<Retained<NSDictionary<NSAttributedStringKey, AnyObject>>>;

        /// Setter for [`typingAttributes`][Self::typingAttributes].
        #[unsafe(method(setTypingAttributes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTypingAttributes(
            &self,
            typing_attributes: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
        );

        #[unsafe(method(clearButtonMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn clearButtonMode(&self) -> UITextFieldViewMode;

        /// Setter for [`clearButtonMode`][Self::clearButtonMode].
        #[unsafe(method(setClearButtonMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setClearButtonMode(&self, clear_button_mode: UITextFieldViewMode);

        #[unsafe(method(leftView))]
        #[unsafe(method_family = none)]
        pub unsafe fn leftView(&self) -> Option<Retained<UIView>>;

        /// Setter for [`leftView`][Self::leftView].
        #[unsafe(method(setLeftView:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLeftView(&self, left_view: Option<&UIView>);

        #[unsafe(method(leftViewMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn leftViewMode(&self) -> UITextFieldViewMode;

        /// Setter for [`leftViewMode`][Self::leftViewMode].
        #[unsafe(method(setLeftViewMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLeftViewMode(&self, left_view_mode: UITextFieldViewMode);

        #[unsafe(method(rightView))]
        #[unsafe(method_family = none)]
        pub unsafe fn rightView(&self) -> Option<Retained<UIView>>;

        /// Setter for [`rightView`][Self::rightView].
        #[unsafe(method(setRightView:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRightView(&self, right_view: Option<&UIView>);

        #[unsafe(method(rightViewMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn rightViewMode(&self) -> UITextFieldViewMode;

        /// Setter for [`rightViewMode`][Self::rightViewMode].
        #[unsafe(method(setRightViewMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRightViewMode(&self, right_view_mode: UITextFieldViewMode);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(borderRectForBounds:))]
        #[unsafe(method_family = none)]
        pub unsafe fn borderRectForBounds(&self, bounds: CGRect) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(textRectForBounds:))]
        #[unsafe(method_family = none)]
        pub unsafe fn textRectForBounds(&self, bounds: CGRect) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(placeholderRectForBounds:))]
        #[unsafe(method_family = none)]
        pub unsafe fn placeholderRectForBounds(&self, bounds: CGRect) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(editingRectForBounds:))]
        #[unsafe(method_family = none)]
        pub unsafe fn editingRectForBounds(&self, bounds: CGRect) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(clearButtonRectForBounds:))]
        #[unsafe(method_family = none)]
        pub unsafe fn clearButtonRectForBounds(&self, bounds: CGRect) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(leftViewRectForBounds:))]
        #[unsafe(method_family = none)]
        pub unsafe fn leftViewRectForBounds(&self, bounds: CGRect) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(rightViewRectForBounds:))]
        #[unsafe(method_family = none)]
        pub unsafe fn rightViewRectForBounds(&self, bounds: CGRect) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[deprecated = "This method is no longer called."]
        #[unsafe(method(drawTextInRect:))]
        #[unsafe(method_family = none)]
        pub unsafe fn drawTextInRect(&self, rect: CGRect);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(drawPlaceholderInRect:))]
        #[unsafe(method_family = none)]
        pub unsafe fn drawPlaceholderInRect(&self, rect: CGRect);

        #[unsafe(method(inputView))]
        #[unsafe(method_family = none)]
        pub unsafe fn inputView(&self) -> Option<Retained<UIView>>;

        /// Setter for [`inputView`][Self::inputView].
        #[unsafe(method(setInputView:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setInputView(&self, input_view: Option<&UIView>);

        #[unsafe(method(inputAccessoryView))]
        #[unsafe(method_family = none)]
        pub unsafe fn inputAccessoryView(&self) -> Option<Retained<UIView>>;

        /// Setter for [`inputAccessoryView`][Self::inputAccessoryView].
        #[unsafe(method(setInputAccessoryView:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setInputAccessoryView(&self, input_accessory_view: Option<&UIView>);

        #[unsafe(method(clearsOnInsertion))]
        #[unsafe(method_family = none)]
        pub unsafe fn clearsOnInsertion(&self) -> bool;

        /// Setter for [`clearsOnInsertion`][Self::clearsOnInsertion].
        #[unsafe(method(setClearsOnInsertion:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setClearsOnInsertion(&self, clears_on_insertion: bool);
    );
}

/// Methods declared on superclass `UIControl`.
#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
impl UITextField {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(initWithFrame:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(all(
            feature = "UIAction",
            feature = "UIMenuElement",
            feature = "objc2-core-foundation"
        ))]
        /// Initializes the control and adds primaryAction for the UIControlEventPrimaryActionTriggered control event. Subclasses of UIControl may alter or add behaviors around the usage of primaryAction, see subclass documentation of this initializer for additional information.
        #[unsafe(method(initWithFrame:primaryAction:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFrame_primaryAction(
            this: Allocated<Self>,
            frame: CGRect,
            primary_action: Option<&UIAction>,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
impl UITextField {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
impl UITextField {
    extern_methods!();
}

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITextDragging",
    feature = "UITextInput",
    feature = "UITextInputTraits",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UITextDraggable for UITextField {}
);

#[cfg(all(
    feature = "UIControl",
    feature = "UIPasteConfigurationSupporting",
    feature = "UIResponder",
    feature = "UITextDropping",
    feature = "UITextInput",
    feature = "UITextInputTraits",
    feature = "UITextPasteConfigurationSupporting",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UITextDroppable for UITextField {}
);

#[cfg(all(
    feature = "UIControl",
    feature = "UIPasteConfigurationSupporting",
    feature = "UIResponder",
    feature = "UITextPasteConfigurationSupporting",
    feature = "UIView"
))]
extern_conformance!(
    unsafe impl UITextPasteConfigurationSupporting for UITextField {}
);

/// UIInteractionStateRestorable.
#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
impl UITextField {
    extern_methods!(
        #[unsafe(method(interactionState))]
        #[unsafe(method_family = none)]
        pub unsafe fn interactionState(&self) -> Retained<AnyObject>;

        /// Setter for [`interactionState`][Self::interactionState].
        #[unsafe(method(setInteractionState:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setInteractionState(&self, interaction_state: &AnyObject);
    );
}

/// UITextField.
#[cfg(all(feature = "UIResponder", feature = "UIView"))]
impl UIView {
    extern_methods!(
        #[unsafe(method(endEditing:))]
        #[unsafe(method_family = none)]
        pub unsafe fn endEditing(&self, force: bool) -> bool;
    );
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextfielddelegate?language=objc)
    pub unsafe trait UITextFieldDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[unsafe(method(textFieldShouldBeginEditing:))]
        #[unsafe(method_family = none)]
        unsafe fn textFieldShouldBeginEditing(&self, text_field: &UITextField) -> bool;

        #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[unsafe(method(textFieldDidBeginEditing:))]
        #[unsafe(method_family = none)]
        unsafe fn textFieldDidBeginEditing(&self, text_field: &UITextField);

        #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[unsafe(method(textFieldShouldEndEditing:))]
        #[unsafe(method_family = none)]
        unsafe fn textFieldShouldEndEditing(&self, text_field: &UITextField) -> bool;

        #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[unsafe(method(textFieldDidEndEditing:))]
        #[unsafe(method_family = none)]
        unsafe fn textFieldDidEndEditing(&self, text_field: &UITextField);

        #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[unsafe(method(textFieldDidEndEditing:reason:))]
        #[unsafe(method_family = none)]
        unsafe fn textFieldDidEndEditing_reason(
            &self,
            text_field: &UITextField,
            reason: UITextFieldDidEndEditingReason,
        );

        #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[unsafe(method(textField:shouldChangeCharactersInRange:replacementString:))]
        #[unsafe(method_family = none)]
        unsafe fn textField_shouldChangeCharactersInRange_replacementString(
            &self,
            text_field: &UITextField,
            range: NSRange,
            string: &NSString,
        ) -> bool;

        #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[unsafe(method(textFieldDidChangeSelection:))]
        #[unsafe(method_family = none)]
        unsafe fn textFieldDidChangeSelection(&self, text_field: &UITextField);

        #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[unsafe(method(textFieldShouldClear:))]
        #[unsafe(method_family = none)]
        unsafe fn textFieldShouldClear(&self, text_field: &UITextField) -> bool;

        #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[unsafe(method(textFieldShouldReturn:))]
        #[unsafe(method_family = none)]
        unsafe fn textFieldShouldReturn(&self, text_field: &UITextField) -> bool;

        #[cfg(all(
            feature = "UIControl",
            feature = "UIMenu",
            feature = "UIMenuElement",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        /// Asks the delegate for the menu to be shown for the specified text range.
        ///
        ///
        /// Parameter `textField`: The text field requesting the menu.
        ///
        /// Parameter `range`: The characters range for which the menu is presented for.
        ///
        /// Parameter `suggestedActions`: The actions and commands that the system suggests.
        ///
        ///
        /// Returns: Return a UIMenu describing the desired menu hierarchy. Return
        /// `nil`to present the default system menu.
        #[optional]
        #[unsafe(method(textField:editMenuForCharactersInRange:suggestedActions:))]
        #[unsafe(method_family = none)]
        unsafe fn textField_editMenuForCharactersInRange_suggestedActions(
            &self,
            text_field: &UITextField,
            range: NSRange,
            suggested_actions: &NSArray<UIMenuElement>,
        ) -> Option<Retained<UIMenu>>;

        #[cfg(all(
            feature = "UIControl",
            feature = "UIEditMenuInteraction",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        /// Called when the text field is about to present the edit menu.
        ///
        ///
        /// Parameter `textField`: The text field displaying the menu.
        ///
        /// Parameter `animator`: Appearance animator. Add animations to this object to run them alongside the appearance transition.
        #[optional]
        #[unsafe(method(textField:willPresentEditMenuWithAnimator:))]
        #[unsafe(method_family = none)]
        unsafe fn textField_willPresentEditMenuWithAnimator(
            &self,
            text_field: &UITextField,
            animator: &ProtocolObject<dyn UIEditMenuInteractionAnimating>,
        );

        #[cfg(all(
            feature = "UIControl",
            feature = "UIEditMenuInteraction",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        /// Called when the text field is about to dismiss the edit menu.
        ///
        ///
        /// Parameter `textField`: The text field displaying the menu.
        ///
        /// Parameter `animator`: Dismissal animator. Add animations to this object to run them alongside the dismissal transition.
        #[optional]
        #[unsafe(method(textField:willDismissEditMenuWithAnimator:))]
        #[unsafe(method_family = none)]
        unsafe fn textField_willDismissEditMenuWithAnimator(
            &self,
            text_field: &UITextField,
            animator: &ProtocolObject<dyn UIEditMenuInteractionAnimating>,
        );

        #[cfg(all(
            feature = "UIControl",
            feature = "UIInputSuggestion",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        /// Tells the delegate when the keyboard delivers an input suggestion.
        ///
        /// - Parameters:
        /// - textField: The text field that is currently the first responder.
        /// - inputSuggestion: The input suggestion that the user or system selected.
        #[optional]
        #[unsafe(method(textField:insertInputSuggestion:))]
        #[unsafe(method_family = none)]
        unsafe fn textField_insertInputSuggestion(
            &self,
            text_field: &UITextField,
            input_suggestion: &UIInputSuggestion,
        );
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextfieldtextdidbegineditingnotification?language=objc)
    pub static UITextFieldTextDidBeginEditingNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextfieldtextdidendeditingnotification?language=objc)
    pub static UITextFieldTextDidEndEditingNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextfieldtextdidchangenotification?language=objc)
    pub static UITextFieldTextDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextfielddidendeditingreasonkey?language=objc)
    pub static UITextFieldDidEndEditingReasonKey: &'static NSString;
}
