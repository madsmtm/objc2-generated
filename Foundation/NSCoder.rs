//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// Describes the action an NSCoder should take when it encounters decode failures (e.g. corrupt data) for non-TopLevel decodes.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdecodingfailurepolicy?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDecodingFailurePolicy(pub NSInteger);
impl NSDecodingFailurePolicy {
    #[doc(alias = "NSDecodingFailurePolicyRaiseException")]
    pub const RaiseException: Self = Self(0);
    #[doc(alias = "NSDecodingFailurePolicySetErrorAndReturn")]
    pub const SetErrorAndReturn: Self = Self(1);
}

unsafe impl Encode for NSDecodingFailurePolicy {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSDecodingFailurePolicy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscoder?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCoder;
);

unsafe impl NSObjectProtocol for NSCoder {}

extern_methods!(
    unsafe impl NSCoder {
        #[method(encodeValueOfObjCType:at:)]
        pub unsafe fn encodeValueOfObjCType_at(
            &self,
            r#type: NonNull<c_char>,
            addr: NonNull<c_void>,
        );

        #[cfg(feature = "NSData")]
        #[method(encodeDataObject:)]
        pub unsafe fn encodeDataObject(&self, data: &NSData);

        #[cfg(feature = "NSData")]
        #[unsafe(method_family(none))]
        #[method_id(decodeDataObject)]
        pub unsafe fn decodeDataObject(&self) -> Option<Retained<NSData>>;

        #[method(decodeValueOfObjCType:at:size:)]
        pub unsafe fn decodeValueOfObjCType_at_size(
            &self,
            r#type: NonNull<c_char>,
            data: NonNull<c_void>,
            size: NSUInteger,
        );

        #[cfg(feature = "NSString")]
        #[method(versionForClassName:)]
        pub unsafe fn versionForClassName(&self, class_name: &NSString) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSCoder {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSExtendedCoder
    unsafe impl NSCoder {
        #[method(encodeObject:)]
        pub unsafe fn encodeObject(&self, object: Option<&AnyObject>);

        #[method(encodeRootObject:)]
        pub unsafe fn encodeRootObject(&self, root_object: &AnyObject);

        #[method(encodeBycopyObject:)]
        pub unsafe fn encodeBycopyObject(&self, an_object: Option<&AnyObject>);

        #[method(encodeByrefObject:)]
        pub unsafe fn encodeByrefObject(&self, an_object: Option<&AnyObject>);

        #[method(encodeConditionalObject:)]
        pub unsafe fn encodeConditionalObject(&self, object: Option<&AnyObject>);

        #[method(encodeArrayOfObjCType:count:at:)]
        pub unsafe fn encodeArrayOfObjCType_count_at(
            &self,
            r#type: NonNull<c_char>,
            count: NSUInteger,
            array: NonNull<c_void>,
        );

        #[method(encodeBytes:length:)]
        pub unsafe fn encodeBytes_length(&self, byteaddr: *const c_void, length: NSUInteger);

        #[unsafe(method_family(none))]
        #[method_id(decodeObject)]
        pub unsafe fn decodeObject(&self) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSError")]
        #[unsafe(method_family(none))]
        #[method_id(decodeTopLevelObjectAndReturnError:_)]
        pub unsafe fn decodeTopLevelObjectAndReturnError(
            &self,
        ) -> Result<Retained<AnyObject>, Retained<NSError>>;

        #[method(decodeArrayOfObjCType:count:at:)]
        pub unsafe fn decodeArrayOfObjCType_count_at(
            &self,
            item_type: NonNull<c_char>,
            count: NSUInteger,
            array: NonNull<c_void>,
        );

        #[method(decodeBytesWithReturnedLength:)]
        pub unsafe fn decodeBytesWithReturnedLength(
            &self,
            lengthp: NonNull<NSUInteger>,
        ) -> *mut c_void;

        #[method(encodePropertyList:)]
        pub unsafe fn encodePropertyList(&self, a_property_list: &AnyObject);

        #[unsafe(method_family(none))]
        #[method_id(decodePropertyList)]
        pub unsafe fn decodePropertyList(&self) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSZone")]
        #[method(setObjectZone:)]
        pub unsafe fn setObjectZone(&self, zone: *mut NSZone);

        #[cfg(feature = "NSZone")]
        #[method(objectZone)]
        pub unsafe fn objectZone(&self) -> *mut NSZone;

        #[method(systemVersion)]
        pub unsafe fn systemVersion(&self) -> c_uint;

        #[method(allowsKeyedCoding)]
        pub unsafe fn allowsKeyedCoding(&self) -> bool;

        #[cfg(feature = "NSString")]
        #[method(encodeObject:forKey:)]
        pub unsafe fn encodeObject_forKey(&self, object: Option<&AnyObject>, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(encodeConditionalObject:forKey:)]
        pub unsafe fn encodeConditionalObject_forKey(
            &self,
            object: Option<&AnyObject>,
            key: &NSString,
        );

        #[cfg(feature = "NSString")]
        #[method(encodeBool:forKey:)]
        pub unsafe fn encodeBool_forKey(&self, value: bool, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(encodeInt:forKey:)]
        pub unsafe fn encodeInt_forKey(&self, value: c_int, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(encodeInt32:forKey:)]
        pub unsafe fn encodeInt32_forKey(&self, value: i32, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(encodeInt64:forKey:)]
        pub unsafe fn encodeInt64_forKey(&self, value: i64, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(encodeFloat:forKey:)]
        pub unsafe fn encodeFloat_forKey(&self, value: c_float, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(encodeDouble:forKey:)]
        pub unsafe fn encodeDouble_forKey(&self, value: c_double, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(encodeBytes:length:forKey:)]
        pub unsafe fn encodeBytes_length_forKey(
            &self,
            bytes: *const u8,
            length: NSUInteger,
            key: &NSString,
        );

        #[cfg(feature = "NSString")]
        #[method(containsValueForKey:)]
        pub unsafe fn containsValueForKey(&self, key: &NSString) -> bool;

        #[cfg(feature = "NSString")]
        #[unsafe(method_family(none))]
        #[method_id(decodeObjectForKey:)]
        pub unsafe fn decodeObjectForKey(&self, key: &NSString) -> Option<Retained<AnyObject>>;

        #[cfg(all(feature = "NSError", feature = "NSString"))]
        #[unsafe(method_family(none))]
        #[method_id(decodeTopLevelObjectForKey:error:_)]
        pub unsafe fn decodeTopLevelObjectForKey_error(
            &self,
            key: &NSString,
        ) -> Result<Retained<AnyObject>, Retained<NSError>>;

        #[cfg(feature = "NSString")]
        #[method(decodeBoolForKey:)]
        pub unsafe fn decodeBoolForKey(&self, key: &NSString) -> bool;

        #[cfg(feature = "NSString")]
        #[method(decodeIntForKey:)]
        pub unsafe fn decodeIntForKey(&self, key: &NSString) -> c_int;

        #[cfg(feature = "NSString")]
        #[method(decodeInt32ForKey:)]
        pub unsafe fn decodeInt32ForKey(&self, key: &NSString) -> i32;

        #[cfg(feature = "NSString")]
        #[method(decodeInt64ForKey:)]
        pub unsafe fn decodeInt64ForKey(&self, key: &NSString) -> i64;

        #[cfg(feature = "NSString")]
        #[method(decodeFloatForKey:)]
        pub unsafe fn decodeFloatForKey(&self, key: &NSString) -> c_float;

        #[cfg(feature = "NSString")]
        #[method(decodeDoubleForKey:)]
        pub unsafe fn decodeDoubleForKey(&self, key: &NSString) -> c_double;

        #[cfg(feature = "NSString")]
        #[method(decodeBytesForKey:returnedLength:)]
        pub unsafe fn decodeBytesForKey_returnedLength(
            &self,
            key: &NSString,
            lengthp: *mut NSUInteger,
        ) -> *const u8;

        #[cfg(feature = "NSString")]
        #[method(encodeInteger:forKey:)]
        pub unsafe fn encodeInteger_forKey(&self, value: NSInteger, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(decodeIntegerForKey:)]
        pub unsafe fn decodeIntegerForKey(&self, key: &NSString) -> NSInteger;

        #[method(requiresSecureCoding)]
        pub unsafe fn requiresSecureCoding(&self) -> bool;

        #[cfg(feature = "NSString")]
        #[unsafe(method_family(none))]
        #[method_id(decodeObjectOfClass:forKey:)]
        pub unsafe fn decodeObjectOfClass_forKey(
            &self,
            a_class: &AnyClass,
            key: &NSString,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(all(feature = "NSError", feature = "NSString"))]
        #[unsafe(method_family(none))]
        #[method_id(decodeTopLevelObjectOfClass:forKey:error:_)]
        pub unsafe fn decodeTopLevelObjectOfClass_forKey_error(
            &self,
            a_class: &AnyClass,
            key: &NSString,
        ) -> Result<Retained<AnyObject>, Retained<NSError>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        /// Decodes the
        /// `NSArray`object for the given
        /// `key,`which should be an
        /// `NSArray<cls>,`containing the given non-collection class (no nested arrays or arrays of dictionaries, etc) from the coder.
        ///
        /// Requires
        /// `NSSecureCoding`otherwise an exception is thrown and sets the
        /// `decodingFailurePolicy`to
        /// `NSDecodingFailurePolicySetErrorAndReturn.`
        /// Returns
        /// `nil`if the object for
        /// `key`is not of the expected types, or cannot be decoded, and sets the
        /// `error`on the decoder.
        #[unsafe(method_family(none))]
        #[method_id(decodeArrayOfObjectsOfClass:forKey:)]
        pub unsafe fn decodeArrayOfObjectsOfClass_forKey(
            &self,
            cls: &AnyClass,
            key: &NSString,
        ) -> Option<Retained<NSArray>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        /// Decodes the
        /// `NSDictionary`object for the given
        /// `key,`which should be an
        /// `NSDictionary<keyCls,objectCls>`, with keys of type given in
        /// `keyCls`and objects of the given non-collection class
        /// `objectCls`(no nested dictionaries or other dictionaries contained in the dictionary, etc) from the coder.
        ///
        /// Requires
        /// `NSSecureCoding`otherwise an exception is thrown and sets the
        /// `decodingFailurePolicy`to
        /// `NSDecodingFailurePolicySetErrorAndReturn.`
        /// Returns
        /// `nil`if the object for
        /// `key`is not of the expected types, or cannot be decoded, and sets the
        /// `error`on the decoder.
        #[unsafe(method_family(none))]
        #[method_id(decodeDictionaryWithKeysOfClass:objectsOfClass:forKey:)]
        pub unsafe fn decodeDictionaryWithKeysOfClass_objectsOfClass_forKey(
            &self,
            key_cls: &AnyClass,
            object_cls: &AnyClass,
            key: &NSString,
        ) -> Option<Retained<NSDictionary>>;

        #[cfg(all(feature = "NSSet", feature = "NSString"))]
        #[unsafe(method_family(none))]
        #[method_id(decodeObjectOfClasses:forKey:)]
        pub unsafe fn decodeObjectOfClasses_forKey(
            &self,
            classes: Option<&NSSet<AnyClass>>,
            key: &NSString,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(all(feature = "NSError", feature = "NSSet", feature = "NSString"))]
        #[unsafe(method_family(none))]
        #[method_id(decodeTopLevelObjectOfClasses:forKey:error:_)]
        pub unsafe fn decodeTopLevelObjectOfClasses_forKey_error(
            &self,
            classes: Option<&NSSet<AnyClass>>,
            key: &NSString,
        ) -> Result<Retained<AnyObject>, Retained<NSError>>;

        #[cfg(all(feature = "NSArray", feature = "NSSet", feature = "NSString"))]
        /// Decodes the
        /// `NSArray`object for the given
        /// `key,`which should be an
        /// `NSArray,`containing the given non-collection classes (no nested arrays or arrays of dictionaries, etc) from the coder.
        ///
        /// Requires
        /// `NSSecureCoding`otherwise an exception is thrown and sets the
        /// `decodingFailurePolicy`to
        /// `NSDecodingFailurePolicySetErrorAndReturn.`
        /// Returns
        /// `nil`if the object for
        /// `key`is not of the expected types, or cannot be decoded, and sets the
        /// `error`on the decoder.
        #[unsafe(method_family(none))]
        #[method_id(decodeArrayOfObjectsOfClasses:forKey:)]
        pub unsafe fn decodeArrayOfObjectsOfClasses_forKey(
            &self,
            classes: &NSSet<AnyClass>,
            key: &NSString,
        ) -> Option<Retained<NSArray>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSSet", feature = "NSString"))]
        /// Decodes the
        /// `NSDictionary`object for the given
        /// `key,`which should be an
        /// `NSDictionary,`with keys of the types given in
        /// `keyClasses`and objects of the given non-collection classes in
        /// `objectClasses`(no nested dictionaries or other dictionaries contained in the dictionary, etc) from the given coder.
        ///
        /// Requires
        /// `NSSecureCoding`otherwise an exception is thrown and sets the
        /// `decodingFailurePolicy`to
        /// `NSDecodingFailurePolicySetErrorAndReturn.`
        /// Returns
        /// `nil`if the object for
        /// `key`is not of the expected types, or cannot be decoded, and sets the
        /// `error`on the decoder.
        #[unsafe(method_family(none))]
        #[method_id(decodeDictionaryWithKeysOfClasses:objectsOfClasses:forKey:)]
        pub unsafe fn decodeDictionaryWithKeysOfClasses_objectsOfClasses_forKey(
            &self,
            key_classes: &NSSet<AnyClass>,
            object_classes: &NSSet<AnyClass>,
            key: &NSString,
        ) -> Option<Retained<NSDictionary>>;

        #[cfg(feature = "NSString")]
        #[unsafe(method_family(none))]
        #[method_id(decodePropertyListForKey:)]
        pub unsafe fn decodePropertyListForKey(
            &self,
            key: &NSString,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSSet")]
        #[unsafe(method_family(none))]
        #[method_id(allowedClasses)]
        pub unsafe fn allowedClasses(&self) -> Option<Retained<NSSet<AnyClass>>>;

        #[cfg(feature = "NSError")]
        /// Signals to this coder that the decode has failed.
        /// Parameter non-nil error that describes the reason why the decode failed
        ///
        /// Sets an error on this NSCoder once per TopLevel decode; calling it repeatedly will have no effect until the call stack unwinds to one of the TopLevel decode entry-points.
        ///
        /// This method is only meaningful to call for decodes.
        ///
        /// Typically, you would want to call this method in your -initWithCoder: implementation when you detect situations like:
        /// - lack of secure coding
        /// - corruption of your data
        /// - domain validation failures
        ///
        /// After calling -failWithError: within your -initWithCoder: implementation, you should clean up and return nil as early as possible.
        ///
        /// Once an error has been signaled to a decoder, it remains set until it has handed off to the first TopLevel decode invocation above it.  For example, consider the following call graph:
        /// A    -decodeTopLevelObjectForKey:error:
        /// B        -initWithCoder:
        /// C            -decodeObjectForKey:
        /// D                -initWithCoder:
        /// E                    -decodeObjectForKey:
        /// F                        -failWithError:
        ///
        /// In this case the error provided in stack-frame F will be returned via the outError in stack-frame A. Furthermore the result object from decodeTopLevelObjectForKey:error: will be nil, regardless of the result of stack-frame B.
        ///
        /// NSCoder implementations support two mechanisms for the stack-unwinding from F to A:
        /// - forced (NSException based)
        /// - particpatory (error based)
        ///
        /// The kind of unwinding you get is determined by the decodingFailurePolicy property of this NSCoder (which defaults to NSDecodingFailurePolicyRaiseException to match historical behavior).
        #[method(failWithError:)]
        pub unsafe fn failWithError(&self, error: &NSError);

        /// Defines the behavior this NSCoder should take on decode failure (i.e. corrupt archive, invalid data, etc.).
        ///
        /// The default result of this property is NSDecodingFailurePolicyRaiseException, subclasses can change this to an alternative policy.
        #[method(decodingFailurePolicy)]
        pub unsafe fn decodingFailurePolicy(&self) -> NSDecodingFailurePolicy;

        #[cfg(feature = "NSError")]
        /// The current error (if there is one) for the current TopLevel decode.
        ///
        /// The meaning of this property changes based on the result of the decodingFailurePolicy property:
        /// For NSDecodingFailurePolicyRaiseException, this property will always be nil.
        /// For NSDecodingFailurePolicySetErrorAndReturn, this property can be non-nil, and if so, indicates that there was a failure while decoding the archive (specifically its the very first error encountered).
        ///
        /// While .error is non-nil, all attempts to decode data from this coder will return a nil/zero-equivalent value.
        ///
        /// This error is consumed by a TopLevel decode API (which resets this coder back to a being able to potentially decode data).
        #[unsafe(method_family(none))]
        #[method_id(error)]
        pub unsafe fn error(&self) -> Option<Retained<NSError>>;
    }
);

#[deprecated = "Not supported"]
#[inline]
pub unsafe extern "C-unwind" fn NXReadNSObjectFromCoder(
    decoder: &NSCoder,
) -> Option<Retained<NSObject>> {
    extern "C-unwind" {
        fn NXReadNSObjectFromCoder(decoder: &NSCoder) -> *mut NSObject;
    }
    let ret = unsafe { NXReadNSObjectFromCoder(decoder) };
    unsafe { Retained::retain_autoreleased(ret) }
}

extern_methods!(
    /// NSTypedstreamCompatibility
    unsafe impl NSCoder {
        #[deprecated = "Not supported"]
        #[method(encodeNXObject:)]
        pub unsafe fn encodeNXObject(&self, object: &AnyObject);

        #[deprecated = "Not supported"]
        #[unsafe(method_family(none))]
        #[method_id(decodeNXObject)]
        pub unsafe fn decodeNXObject(&self) -> Option<Retained<AnyObject>>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSCoder {
        #[deprecated]
        #[method(decodeValueOfObjCType:at:)]
        pub unsafe fn decodeValueOfObjCType_at(
            &self,
            r#type: NonNull<c_char>,
            data: NonNull<c_void>,
        );
    }
);
