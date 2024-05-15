//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static UIGuidedAccessErrorDomain: &'static NSErrorDomain;
}

// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIGuidedAccessErrorCode(pub NSInteger);
impl UIGuidedAccessErrorCode {
    pub const UIGuidedAccessErrorPermissionDenied: Self = Self(0);
    pub const UIGuidedAccessErrorFailed: Self = Self(NSIntegerMax as _);
}

unsafe impl Encode for UIGuidedAccessErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIGuidedAccessErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIGuidedAccessRestrictionState(pub NSInteger);
impl UIGuidedAccessRestrictionState {
    #[doc(alias = "UIGuidedAccessRestrictionStateAllow")]
    pub const Allow: Self = Self(0);
    #[doc(alias = "UIGuidedAccessRestrictionStateDeny")]
    pub const Deny: Self = Self(1);
}

unsafe impl Encode for UIGuidedAccessRestrictionState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIGuidedAccessRestrictionState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait UIGuidedAccessRestrictionDelegate:
        NSObjectProtocol + IsMainThreadOnly
    {
        #[method_id(@__retain_semantics Other guidedAccessRestrictionIdentifiers)]
        unsafe fn guidedAccessRestrictionIdentifiers(&self) -> Option<Id<NSArray<NSString>>>;

        #[method(guidedAccessRestrictionWithIdentifier:didChangeState:)]
        unsafe fn guidedAccessRestrictionWithIdentifier_didChangeState(
            &self,
            restriction_identifier: &NSString,
            new_restriction_state: UIGuidedAccessRestrictionState,
        );

        #[method_id(@__retain_semantics Other textForGuidedAccessRestrictionWithIdentifier:)]
        unsafe fn textForGuidedAccessRestrictionWithIdentifier(
            &self,
            restriction_identifier: &NSString,
        ) -> Option<Id<NSString>>;

        #[optional]
        #[method_id(@__retain_semantics Other detailTextForGuidedAccessRestrictionWithIdentifier:)]
        unsafe fn detailTextForGuidedAccessRestrictionWithIdentifier(
            &self,
            restriction_identifier: &NSString,
        ) -> Option<Id<NSString>>;
    }

    unsafe impl ProtocolType for dyn UIGuidedAccessRestrictionDelegate {}
);

extern "C" {
    pub fn UIGuidedAccessRestrictionStateForIdentifier(
        restriction_identifier: &NSString,
    ) -> UIGuidedAccessRestrictionState;
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIGuidedAccessAccessibilityFeature(pub NSUInteger);
impl UIGuidedAccessAccessibilityFeature {
    #[doc(alias = "UIGuidedAccessAccessibilityFeatureVoiceOver")]
    pub const VoiceOver: Self = Self(1 << 0);
    #[doc(alias = "UIGuidedAccessAccessibilityFeatureZoom")]
    pub const Zoom: Self = Self(1 << 1);
    #[doc(alias = "UIGuidedAccessAccessibilityFeatureAssistiveTouch")]
    pub const AssistiveTouch: Self = Self(1 << 2);
    #[doc(alias = "UIGuidedAccessAccessibilityFeatureInvertColors")]
    pub const InvertColors: Self = Self(1 << 3);
    #[doc(alias = "UIGuidedAccessAccessibilityFeatureGrayscaleDisplay")]
    pub const GrayscaleDisplay: Self = Self(1 << 4);
}

unsafe impl Encode for UIGuidedAccessAccessibilityFeature {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIGuidedAccessAccessibilityFeature {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    #[cfg(feature = "block2")]
    pub fn UIGuidedAccessConfigureAccessibilityFeatures(
        features: UIGuidedAccessAccessibilityFeature,
        enabled: Bool,
        completion: &block2::Block<dyn Fn(Bool, *mut NSError)>,
    );
}