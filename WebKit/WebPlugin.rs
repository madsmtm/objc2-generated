//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_category!(
    /// Category "WebPlugIn" on [`NSObject`].
    #[doc(alias = "WebPlugIn")]
    pub unsafe trait NSObjectWebPlugIn {
        #[method(webPlugInInitialize)]
        unsafe fn webPlugInInitialize(&self);

        #[method(webPlugInStart)]
        unsafe fn webPlugInStart(&self);

        #[method(webPlugInStop)]
        unsafe fn webPlugInStop(&self);

        #[method(webPlugInDestroy)]
        unsafe fn webPlugInDestroy(&self);

        #[method(webPlugInSetIsSelected:)]
        unsafe fn webPlugInSetIsSelected(&self, is_selected: bool);

        #[method_id(@__retain_semantics Other objectForWebScript)]
        unsafe fn objectForWebScript(&self) -> Option<Retained<AnyObject>>;

        #[method(webPlugInMainResourceDidReceiveResponse:)]
        unsafe fn webPlugInMainResourceDidReceiveResponse(&self, response: Option<&NSURLResponse>);

        #[method(webPlugInMainResourceDidReceiveData:)]
        unsafe fn webPlugInMainResourceDidReceiveData(&self, data: Option<&NSData>);

        #[method(webPlugInMainResourceDidFailWithError:)]
        unsafe fn webPlugInMainResourceDidFailWithError(&self, error: Option<&NSError>);

        #[method(webPlugInMainResourceDidFinishLoading)]
        unsafe fn webPlugInMainResourceDidFinishLoading(&self);
    }

    unsafe impl NSObjectWebPlugIn for NSObject {}
);
