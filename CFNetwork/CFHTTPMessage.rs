//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::ffi::*;
use core::marker::{PhantomData, PhantomPinned};
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
use objc2_core_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/cfnetwork/kcfhttpversion1_0?language=objc)
    pub static kCFHTTPVersion1_0: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/cfnetwork/kcfhttpversion1_1?language=objc)
    pub static kCFHTTPVersion1_1: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/cfnetwork/kcfhttpversion2_0?language=objc)
    pub static kCFHTTPVersion2_0: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/cfnetwork/kcfhttpversion3_0?language=objc)
    pub static kCFHTTPVersion3_0: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/cfnetwork/kcfhttpauthenticationschemebasic?language=objc)
    pub static kCFHTTPAuthenticationSchemeBasic: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/cfnetwork/kcfhttpauthenticationschemedigest?language=objc)
    pub static kCFHTTPAuthenticationSchemeDigest: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/cfnetwork/kcfhttpauthenticationschementlm?language=objc)
    pub static kCFHTTPAuthenticationSchemeNTLM: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/cfnetwork/kcfhttpauthenticationschemekerberos?language=objc)
    pub static kCFHTTPAuthenticationSchemeKerberos: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/cfnetwork/kcfhttpauthenticationschemenegotiate?language=objc)
    pub static kCFHTTPAuthenticationSchemeNegotiate: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/cfnetwork/kcfhttpauthenticationschemenegotiate2?language=objc)
    pub static kCFHTTPAuthenticationSchemeNegotiate2: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/cfnetwork/kcfhttpauthenticationschemexmobilemeauthtoken?language=objc)
    pub static kCFHTTPAuthenticationSchemeXMobileMeAuthToken: &'static CFString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/cfnetwork/cfhttpmessage?language=objc)
#[repr(C)]
pub struct CFHTTPMessage {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    unsafe impl CFHTTPMessage {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl RefEncode<"__CFHTTPMessage"> for CFHTTPMessage {}
);

unsafe impl ConcreteType for CFHTTPMessage {
    #[doc(alias = "CFHTTPMessageGetTypeID")]
    #[inline]
    fn type_id() -> CFTypeID {
        extern "C-unwind" {
            fn CFHTTPMessageGetTypeID() -> CFTypeID;
        }
        unsafe { CFHTTPMessageGetTypeID() }
    }
}

impl CFHTTPMessage {
    #[doc(alias = "CFHTTPMessageCreateRequest")]
    #[inline]
    pub unsafe fn new_request(
        alloc: Option<&CFAllocator>,
        request_method: &CFString,
        url: &CFURL,
        http_version: &CFString,
    ) -> CFRetained<CFHTTPMessage> {
        extern "C-unwind" {
            fn CFHTTPMessageCreateRequest(
                alloc: Option<&CFAllocator>,
                request_method: &CFString,
                url: &CFURL,
                http_version: &CFString,
            ) -> Option<NonNull<CFHTTPMessage>>;
        }
        let ret = unsafe { CFHTTPMessageCreateRequest(alloc, request_method, url, http_version) };
        let ret =
            ret.expect("function was marked as returning non-null, but actually returned NULL");
        unsafe { CFRetained::from_raw(ret) }
    }

    #[doc(alias = "CFHTTPMessageCreateResponse")]
    #[inline]
    pub unsafe fn new_response(
        alloc: Option<&CFAllocator>,
        status_code: CFIndex,
        status_description: Option<&CFString>,
        http_version: &CFString,
    ) -> CFRetained<CFHTTPMessage> {
        extern "C-unwind" {
            fn CFHTTPMessageCreateResponse(
                alloc: Option<&CFAllocator>,
                status_code: CFIndex,
                status_description: Option<&CFString>,
                http_version: &CFString,
            ) -> Option<NonNull<CFHTTPMessage>>;
        }
        let ret = unsafe {
            CFHTTPMessageCreateResponse(alloc, status_code, status_description, http_version)
        };
        let ret =
            ret.expect("function was marked as returning non-null, but actually returned NULL");
        unsafe { CFRetained::from_raw(ret) }
    }

    #[doc(alias = "CFHTTPMessageCreateEmpty")]
    #[inline]
    pub unsafe fn new_empty(
        alloc: Option<&CFAllocator>,
        is_request: bool,
    ) -> CFRetained<CFHTTPMessage> {
        extern "C-unwind" {
            fn CFHTTPMessageCreateEmpty(
                alloc: Option<&CFAllocator>,
                is_request: Boolean,
            ) -> Option<NonNull<CFHTTPMessage>>;
        }
        let ret = unsafe { CFHTTPMessageCreateEmpty(alloc, is_request as _) };
        let ret =
            ret.expect("function was marked as returning non-null, but actually returned NULL");
        unsafe { CFRetained::from_raw(ret) }
    }

    #[doc(alias = "CFHTTPMessageCreateCopy")]
    #[inline]
    pub unsafe fn new_copy(
        alloc: Option<&CFAllocator>,
        message: &CFHTTPMessage,
    ) -> CFRetained<CFHTTPMessage> {
        extern "C-unwind" {
            fn CFHTTPMessageCreateCopy(
                alloc: Option<&CFAllocator>,
                message: &CFHTTPMessage,
            ) -> Option<NonNull<CFHTTPMessage>>;
        }
        let ret = unsafe { CFHTTPMessageCreateCopy(alloc, message) };
        let ret =
            ret.expect("function was marked as returning non-null, but actually returned NULL");
        unsafe { CFRetained::from_raw(ret) }
    }

    #[doc(alias = "CFHTTPMessageIsRequest")]
    #[inline]
    pub unsafe fn is_request(self: &CFHTTPMessage) -> bool {
        extern "C-unwind" {
            fn CFHTTPMessageIsRequest(message: &CFHTTPMessage) -> Boolean;
        }
        let ret = unsafe { CFHTTPMessageIsRequest(self) };
        ret != 0
    }

    #[doc(alias = "CFHTTPMessageCopyVersion")]
    #[inline]
    pub unsafe fn version(self: &CFHTTPMessage) -> CFRetained<CFString> {
        extern "C-unwind" {
            fn CFHTTPMessageCopyVersion(message: &CFHTTPMessage) -> Option<NonNull<CFString>>;
        }
        let ret = unsafe { CFHTTPMessageCopyVersion(self) };
        let ret =
            ret.expect("function was marked as returning non-null, but actually returned NULL");
        unsafe { CFRetained::from_raw(ret) }
    }

    #[doc(alias = "CFHTTPMessageCopyBody")]
    #[inline]
    pub unsafe fn body(self: &CFHTTPMessage) -> Option<CFRetained<CFData>> {
        extern "C-unwind" {
            fn CFHTTPMessageCopyBody(message: &CFHTTPMessage) -> Option<NonNull<CFData>>;
        }
        let ret = unsafe { CFHTTPMessageCopyBody(self) };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }

    #[doc(alias = "CFHTTPMessageSetBody")]
    #[inline]
    pub unsafe fn set_body(self: &CFHTTPMessage, body_data: &CFData) {
        extern "C-unwind" {
            fn CFHTTPMessageSetBody(message: &CFHTTPMessage, body_data: &CFData);
        }
        unsafe { CFHTTPMessageSetBody(self, body_data) }
    }

    #[doc(alias = "CFHTTPMessageCopyHeaderFieldValue")]
    #[inline]
    pub unsafe fn header_field_value(
        self: &CFHTTPMessage,
        header_field: &CFString,
    ) -> Option<CFRetained<CFString>> {
        extern "C-unwind" {
            fn CFHTTPMessageCopyHeaderFieldValue(
                message: &CFHTTPMessage,
                header_field: &CFString,
            ) -> Option<NonNull<CFString>>;
        }
        let ret = unsafe { CFHTTPMessageCopyHeaderFieldValue(self, header_field) };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }

    #[doc(alias = "CFHTTPMessageCopyAllHeaderFields")]
    #[inline]
    pub unsafe fn all_header_fields(self: &CFHTTPMessage) -> Option<CFRetained<CFDictionary>> {
        extern "C-unwind" {
            fn CFHTTPMessageCopyAllHeaderFields(
                message: &CFHTTPMessage,
            ) -> Option<NonNull<CFDictionary>>;
        }
        let ret = unsafe { CFHTTPMessageCopyAllHeaderFields(self) };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }

    #[doc(alias = "CFHTTPMessageSetHeaderFieldValue")]
    #[inline]
    pub unsafe fn set_header_field_value(
        self: &CFHTTPMessage,
        header_field: &CFString,
        value: Option<&CFString>,
    ) {
        extern "C-unwind" {
            fn CFHTTPMessageSetHeaderFieldValue(
                message: &CFHTTPMessage,
                header_field: &CFString,
                value: Option<&CFString>,
            );
        }
        unsafe { CFHTTPMessageSetHeaderFieldValue(self, header_field, value) }
    }

    #[doc(alias = "CFHTTPMessageAppendBytes")]
    #[inline]
    pub unsafe fn append_bytes(
        self: &CFHTTPMessage,
        new_bytes: NonNull<u8>,
        num_bytes: CFIndex,
    ) -> bool {
        extern "C-unwind" {
            fn CFHTTPMessageAppendBytes(
                message: &CFHTTPMessage,
                new_bytes: NonNull<u8>,
                num_bytes: CFIndex,
            ) -> Boolean;
        }
        let ret = unsafe { CFHTTPMessageAppendBytes(self, new_bytes, num_bytes) };
        ret != 0
    }

    #[doc(alias = "CFHTTPMessageIsHeaderComplete")]
    #[inline]
    pub unsafe fn is_header_complete(self: &CFHTTPMessage) -> bool {
        extern "C-unwind" {
            fn CFHTTPMessageIsHeaderComplete(message: &CFHTTPMessage) -> Boolean;
        }
        let ret = unsafe { CFHTTPMessageIsHeaderComplete(self) };
        ret != 0
    }

    #[doc(alias = "CFHTTPMessageCopySerializedMessage")]
    #[inline]
    pub unsafe fn serialized_message(self: &CFHTTPMessage) -> Option<CFRetained<CFData>> {
        extern "C-unwind" {
            fn CFHTTPMessageCopySerializedMessage(
                message: &CFHTTPMessage,
            ) -> Option<NonNull<CFData>>;
        }
        let ret = unsafe { CFHTTPMessageCopySerializedMessage(self) };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }

    /// ******************
    #[doc(alias = "CFHTTPMessageCopyRequestURL")]
    #[inline]
    pub unsafe fn request_url(self: &CFHTTPMessage) -> Option<CFRetained<CFURL>> {
        extern "C-unwind" {
            fn CFHTTPMessageCopyRequestURL(request: &CFHTTPMessage) -> Option<NonNull<CFURL>>;
        }
        let ret = unsafe { CFHTTPMessageCopyRequestURL(self) };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }

    #[doc(alias = "CFHTTPMessageCopyRequestMethod")]
    #[inline]
    pub unsafe fn request_method(self: &CFHTTPMessage) -> Option<CFRetained<CFString>> {
        extern "C-unwind" {
            fn CFHTTPMessageCopyRequestMethod(request: &CFHTTPMessage)
                -> Option<NonNull<CFString>>;
        }
        let ret = unsafe { CFHTTPMessageCopyRequestMethod(self) };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }

    #[doc(alias = "CFHTTPMessageAddAuthentication")]
    #[inline]
    pub unsafe fn add_authentication(
        self: &CFHTTPMessage,
        authentication_failure_response: Option<&CFHTTPMessage>,
        username: &CFString,
        password: &CFString,
        authentication_scheme: Option<&CFString>,
        for_proxy: bool,
    ) -> bool {
        extern "C-unwind" {
            fn CFHTTPMessageAddAuthentication(
                request: &CFHTTPMessage,
                authentication_failure_response: Option<&CFHTTPMessage>,
                username: &CFString,
                password: &CFString,
                authentication_scheme: Option<&CFString>,
                for_proxy: Boolean,
            ) -> Boolean;
        }
        let ret = unsafe {
            CFHTTPMessageAddAuthentication(
                self,
                authentication_failure_response,
                username,
                password,
                authentication_scheme,
                for_proxy as _,
            )
        };
        ret != 0
    }

    /// *******************
    #[doc(alias = "CFHTTPMessageGetResponseStatusCode")]
    #[inline]
    pub unsafe fn response_status_code(self: &CFHTTPMessage) -> CFIndex {
        extern "C-unwind" {
            fn CFHTTPMessageGetResponseStatusCode(response: &CFHTTPMessage) -> CFIndex;
        }
        unsafe { CFHTTPMessageGetResponseStatusCode(self) }
    }

    #[doc(alias = "CFHTTPMessageCopyResponseStatusLine")]
    #[inline]
    pub unsafe fn response_status_line(self: &CFHTTPMessage) -> Option<CFRetained<CFString>> {
        extern "C-unwind" {
            fn CFHTTPMessageCopyResponseStatusLine(
                response: &CFHTTPMessage,
            ) -> Option<NonNull<CFString>>;
        }
        let ret = unsafe { CFHTTPMessageCopyResponseStatusLine(self) };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }
}

#[deprecated = "renamed to `CFHTTPMessage::new_request`"]
#[inline]
pub unsafe extern "C-unwind" fn CFHTTPMessageCreateRequest(
    alloc: Option<&CFAllocator>,
    request_method: &CFString,
    url: &CFURL,
    http_version: &CFString,
) -> CFRetained<CFHTTPMessage> {
    extern "C-unwind" {
        fn CFHTTPMessageCreateRequest(
            alloc: Option<&CFAllocator>,
            request_method: &CFString,
            url: &CFURL,
            http_version: &CFString,
        ) -> Option<NonNull<CFHTTPMessage>>;
    }
    let ret = unsafe { CFHTTPMessageCreateRequest(alloc, request_method, url, http_version) };
    let ret = ret.expect("function was marked as returning non-null, but actually returned NULL");
    unsafe { CFRetained::from_raw(ret) }
}

#[deprecated = "renamed to `CFHTTPMessage::new_response`"]
#[inline]
pub unsafe extern "C-unwind" fn CFHTTPMessageCreateResponse(
    alloc: Option<&CFAllocator>,
    status_code: CFIndex,
    status_description: Option<&CFString>,
    http_version: &CFString,
) -> CFRetained<CFHTTPMessage> {
    extern "C-unwind" {
        fn CFHTTPMessageCreateResponse(
            alloc: Option<&CFAllocator>,
            status_code: CFIndex,
            status_description: Option<&CFString>,
            http_version: &CFString,
        ) -> Option<NonNull<CFHTTPMessage>>;
    }
    let ret = unsafe {
        CFHTTPMessageCreateResponse(alloc, status_code, status_description, http_version)
    };
    let ret = ret.expect("function was marked as returning non-null, but actually returned NULL");
    unsafe { CFRetained::from_raw(ret) }
}

#[deprecated = "renamed to `CFHTTPMessage::new_empty`"]
#[inline]
pub unsafe extern "C-unwind" fn CFHTTPMessageCreateEmpty(
    alloc: Option<&CFAllocator>,
    is_request: bool,
) -> CFRetained<CFHTTPMessage> {
    extern "C-unwind" {
        fn CFHTTPMessageCreateEmpty(
            alloc: Option<&CFAllocator>,
            is_request: Boolean,
        ) -> Option<NonNull<CFHTTPMessage>>;
    }
    let ret = unsafe { CFHTTPMessageCreateEmpty(alloc, is_request as _) };
    let ret = ret.expect("function was marked as returning non-null, but actually returned NULL");
    unsafe { CFRetained::from_raw(ret) }
}

#[deprecated = "renamed to `CFHTTPMessage::new_copy`"]
#[inline]
pub unsafe extern "C-unwind" fn CFHTTPMessageCreateCopy(
    alloc: Option<&CFAllocator>,
    message: &CFHTTPMessage,
) -> CFRetained<CFHTTPMessage> {
    extern "C-unwind" {
        fn CFHTTPMessageCreateCopy(
            alloc: Option<&CFAllocator>,
            message: &CFHTTPMessage,
        ) -> Option<NonNull<CFHTTPMessage>>;
    }
    let ret = unsafe { CFHTTPMessageCreateCopy(alloc, message) };
    let ret = ret.expect("function was marked as returning non-null, but actually returned NULL");
    unsafe { CFRetained::from_raw(ret) }
}

#[deprecated = "renamed to `CFHTTPMessage::is_request`"]
#[inline]
pub unsafe extern "C-unwind" fn CFHTTPMessageIsRequest(message: &CFHTTPMessage) -> bool {
    extern "C-unwind" {
        fn CFHTTPMessageIsRequest(message: &CFHTTPMessage) -> Boolean;
    }
    let ret = unsafe { CFHTTPMessageIsRequest(message) };
    ret != 0
}

#[deprecated = "renamed to `CFHTTPMessage::version`"]
#[inline]
pub unsafe extern "C-unwind" fn CFHTTPMessageCopyVersion(
    message: &CFHTTPMessage,
) -> CFRetained<CFString> {
    extern "C-unwind" {
        fn CFHTTPMessageCopyVersion(message: &CFHTTPMessage) -> Option<NonNull<CFString>>;
    }
    let ret = unsafe { CFHTTPMessageCopyVersion(message) };
    let ret = ret.expect("function was marked as returning non-null, but actually returned NULL");
    unsafe { CFRetained::from_raw(ret) }
}

#[deprecated = "renamed to `CFHTTPMessage::body`"]
#[inline]
pub unsafe extern "C-unwind" fn CFHTTPMessageCopyBody(
    message: &CFHTTPMessage,
) -> Option<CFRetained<CFData>> {
    extern "C-unwind" {
        fn CFHTTPMessageCopyBody(message: &CFHTTPMessage) -> Option<NonNull<CFData>>;
    }
    let ret = unsafe { CFHTTPMessageCopyBody(message) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    #[deprecated = "renamed to `CFHTTPMessage::set_body`"]
    pub fn CFHTTPMessageSetBody(message: &CFHTTPMessage, body_data: &CFData);
}

#[deprecated = "renamed to `CFHTTPMessage::header_field_value`"]
#[inline]
pub unsafe extern "C-unwind" fn CFHTTPMessageCopyHeaderFieldValue(
    message: &CFHTTPMessage,
    header_field: &CFString,
) -> Option<CFRetained<CFString>> {
    extern "C-unwind" {
        fn CFHTTPMessageCopyHeaderFieldValue(
            message: &CFHTTPMessage,
            header_field: &CFString,
        ) -> Option<NonNull<CFString>>;
    }
    let ret = unsafe { CFHTTPMessageCopyHeaderFieldValue(message, header_field) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[deprecated = "renamed to `CFHTTPMessage::all_header_fields`"]
#[inline]
pub unsafe extern "C-unwind" fn CFHTTPMessageCopyAllHeaderFields(
    message: &CFHTTPMessage,
) -> Option<CFRetained<CFDictionary>> {
    extern "C-unwind" {
        fn CFHTTPMessageCopyAllHeaderFields(
            message: &CFHTTPMessage,
        ) -> Option<NonNull<CFDictionary>>;
    }
    let ret = unsafe { CFHTTPMessageCopyAllHeaderFields(message) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    #[deprecated = "renamed to `CFHTTPMessage::set_header_field_value`"]
    pub fn CFHTTPMessageSetHeaderFieldValue(
        message: &CFHTTPMessage,
        header_field: &CFString,
        value: Option<&CFString>,
    );
}

#[deprecated = "renamed to `CFHTTPMessage::append_bytes`"]
#[inline]
pub unsafe extern "C-unwind" fn CFHTTPMessageAppendBytes(
    message: &CFHTTPMessage,
    new_bytes: NonNull<u8>,
    num_bytes: CFIndex,
) -> bool {
    extern "C-unwind" {
        fn CFHTTPMessageAppendBytes(
            message: &CFHTTPMessage,
            new_bytes: NonNull<u8>,
            num_bytes: CFIndex,
        ) -> Boolean;
    }
    let ret = unsafe { CFHTTPMessageAppendBytes(message, new_bytes, num_bytes) };
    ret != 0
}

#[deprecated = "renamed to `CFHTTPMessage::is_header_complete`"]
#[inline]
pub unsafe extern "C-unwind" fn CFHTTPMessageIsHeaderComplete(message: &CFHTTPMessage) -> bool {
    extern "C-unwind" {
        fn CFHTTPMessageIsHeaderComplete(message: &CFHTTPMessage) -> Boolean;
    }
    let ret = unsafe { CFHTTPMessageIsHeaderComplete(message) };
    ret != 0
}

#[deprecated = "renamed to `CFHTTPMessage::serialized_message`"]
#[inline]
pub unsafe extern "C-unwind" fn CFHTTPMessageCopySerializedMessage(
    message: &CFHTTPMessage,
) -> Option<CFRetained<CFData>> {
    extern "C-unwind" {
        fn CFHTTPMessageCopySerializedMessage(message: &CFHTTPMessage) -> Option<NonNull<CFData>>;
    }
    let ret = unsafe { CFHTTPMessageCopySerializedMessage(message) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[deprecated = "renamed to `CFHTTPMessage::request_url`"]
#[inline]
pub unsafe extern "C-unwind" fn CFHTTPMessageCopyRequestURL(
    request: &CFHTTPMessage,
) -> Option<CFRetained<CFURL>> {
    extern "C-unwind" {
        fn CFHTTPMessageCopyRequestURL(request: &CFHTTPMessage) -> Option<NonNull<CFURL>>;
    }
    let ret = unsafe { CFHTTPMessageCopyRequestURL(request) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[deprecated = "renamed to `CFHTTPMessage::request_method`"]
#[inline]
pub unsafe extern "C-unwind" fn CFHTTPMessageCopyRequestMethod(
    request: &CFHTTPMessage,
) -> Option<CFRetained<CFString>> {
    extern "C-unwind" {
        fn CFHTTPMessageCopyRequestMethod(request: &CFHTTPMessage) -> Option<NonNull<CFString>>;
    }
    let ret = unsafe { CFHTTPMessageCopyRequestMethod(request) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[deprecated = "renamed to `CFHTTPMessage::add_authentication`"]
#[inline]
pub unsafe extern "C-unwind" fn CFHTTPMessageAddAuthentication(
    request: &CFHTTPMessage,
    authentication_failure_response: Option<&CFHTTPMessage>,
    username: &CFString,
    password: &CFString,
    authentication_scheme: Option<&CFString>,
    for_proxy: bool,
) -> bool {
    extern "C-unwind" {
        fn CFHTTPMessageAddAuthentication(
            request: &CFHTTPMessage,
            authentication_failure_response: Option<&CFHTTPMessage>,
            username: &CFString,
            password: &CFString,
            authentication_scheme: Option<&CFString>,
            for_proxy: Boolean,
        ) -> Boolean;
    }
    let ret = unsafe {
        CFHTTPMessageAddAuthentication(
            request,
            authentication_failure_response,
            username,
            password,
            authentication_scheme,
            for_proxy as _,
        )
    };
    ret != 0
}

extern "C-unwind" {
    #[deprecated = "renamed to `CFHTTPMessage::response_status_code`"]
    pub fn CFHTTPMessageGetResponseStatusCode(response: &CFHTTPMessage) -> CFIndex;
}

#[deprecated = "renamed to `CFHTTPMessage::response_status_line`"]
#[inline]
pub unsafe extern "C-unwind" fn CFHTTPMessageCopyResponseStatusLine(
    response: &CFHTTPMessage,
) -> Option<CFRetained<CFString>> {
    extern "C-unwind" {
        fn CFHTTPMessageCopyResponseStatusLine(
            response: &CFHTTPMessage,
        ) -> Option<NonNull<CFString>>;
    }
    let ret = unsafe { CFHTTPMessageCopyResponseStatusLine(response) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}
