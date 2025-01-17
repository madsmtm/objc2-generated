//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-ml")]
use objc2_core_ml::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/naturallanguage/nlmodeltype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NLModelType(pub NSInteger);
impl NLModelType {
    #[doc(alias = "NLModelTypeClassifier")]
    pub const Classifier: Self = Self(0);
    #[doc(alias = "NLModelTypeSequence")]
    pub const Sequence: Self = Self(1);
}

unsafe impl Encode for NLModelType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NLModelType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/naturallanguage/nlmodel?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NLModel;
);

unsafe impl NSObjectProtocol for NLModel {}

extern_methods!(
    unsafe impl NLModel {
        #[unsafe(method_family(none))]
        #[method_id(modelWithContentsOfURL:error:_)]
        pub unsafe fn modelWithContentsOfURL_error(
            url: &NSURL,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "objc2-core-ml")]
        #[unsafe(method_family(none))]
        #[method_id(modelWithMLModel:error:_)]
        pub unsafe fn modelWithMLModel_error(
            ml_model: &MLModel,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[unsafe(method_family(none))]
        #[method_id(configuration)]
        pub unsafe fn configuration(&self) -> Retained<NLModelConfiguration>;

        #[unsafe(method_family(none))]
        #[method_id(predictedLabelForString:)]
        pub unsafe fn predictedLabelForString(
            &self,
            string: &NSString,
        ) -> Option<Retained<NSString>>;

        #[unsafe(method_family(none))]
        #[method_id(predictedLabelsForTokens:)]
        pub unsafe fn predictedLabelsForTokens(
            &self,
            tokens: &NSArray<NSString>,
        ) -> Retained<NSArray<NSString>>;

        #[unsafe(method_family(none))]
        #[method_id(predictedLabelHypothesesForString:maximumCount:)]
        pub unsafe fn predictedLabelHypothesesForString_maximumCount(
            &self,
            string: &NSString,
            maximum_count: NSUInteger,
        ) -> Retained<NSDictionary<NSString, NSNumber>>;

        #[unsafe(method_family(none))]
        #[method_id(predictedLabelHypothesesForTokens:maximumCount:)]
        pub unsafe fn predictedLabelHypothesesForTokens_maximumCount(
            &self,
            tokens: &NSArray<NSString>,
            maximum_count: NSUInteger,
        ) -> Retained<NSArray<NSDictionary<NSString, NSNumber>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NLModel {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/naturallanguage/nlmodelconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NLModelConfiguration;
);

unsafe impl NSCoding for NLModelConfiguration {}

unsafe impl NSCopying for NLModelConfiguration {}

unsafe impl CopyingHelper for NLModelConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NLModelConfiguration {}

unsafe impl NSSecureCoding for NLModelConfiguration {}

extern_methods!(
    unsafe impl NLModelConfiguration {
        #[method(type)]
        pub unsafe fn r#type(&self) -> NLModelType;

        #[cfg(feature = "NLLanguage")]
        #[unsafe(method_family(none))]
        #[method_id(language)]
        pub unsafe fn language(&self) -> Option<Retained<NLLanguage>>;

        #[method(revision)]
        pub unsafe fn revision(&self) -> NSUInteger;

        #[unsafe(method_family(none))]
        #[method_id(supportedRevisionsForType:)]
        pub unsafe fn supportedRevisionsForType(r#type: NLModelType) -> Retained<NSIndexSet>;

        #[method(currentRevisionForType:)]
        pub unsafe fn currentRevisionForType(r#type: NLModelType) -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NLModelConfiguration {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
