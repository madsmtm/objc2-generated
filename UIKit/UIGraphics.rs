//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C-unwind" {
    pub fn UIGraphicsPopContext();
}

extern "C-unwind" {
    pub fn UIRectFill(rect: CGRect);
}

extern "C-unwind" {
    pub fn UIRectFrame(rect: CGRect);
}

extern "C-unwind" {
    pub fn UIRectClip(rect: CGRect);
}

extern "C-unwind" {
    #[deprecated = "Replace usage of UIGraphicsBeginImageContext with UIGraphicsImageRenderer."]
    pub fn UIGraphicsBeginImageContext(size: CGSize);
}

extern "C-unwind" {
    #[deprecated = "Replace usage of UIGraphicsBeginImageContextWithOptions with UIGraphicsImageRenderer."]
    pub fn UIGraphicsBeginImageContextWithOptions(size: CGSize, opaque: Bool, scale: CGFloat);
}

extern "C-unwind" {
    #[cfg(feature = "UIImage")]
    #[deprecated = "Replace usage of UIGraphicsGetImageFromCurrentImageContext with UIGraphicsImageRendererContext.currentImage."]
    pub fn UIGraphicsGetImageFromCurrentImageContext() -> *mut UIImage;
}

extern "C-unwind" {
    #[deprecated = "UIGraphicsEndImageContext should only be used alongside UIGraphicsBeginImageContext[WithOptions]."]
    pub fn UIGraphicsEndImageContext();
}

extern "C-unwind" {
    pub fn UIGraphicsBeginPDFContextToFile(
        path: &NSString,
        bounds: CGRect,
        document_info: Option<&NSDictionary>,
    ) -> Bool;
}

extern "C-unwind" {
    pub fn UIGraphicsBeginPDFContextToData(
        data: &NSMutableData,
        bounds: CGRect,
        document_info: Option<&NSDictionary>,
    );
}

extern "C-unwind" {
    pub fn UIGraphicsEndPDFContext();
}

extern "C-unwind" {
    pub fn UIGraphicsBeginPDFPage();
}

extern "C-unwind" {
    pub fn UIGraphicsBeginPDFPageWithInfo(bounds: CGRect, page_info: Option<&NSDictionary>);
}

extern "C-unwind" {
    pub fn UIGraphicsGetPDFContextBounds() -> CGRect;
}

extern "C-unwind" {
    pub fn UIGraphicsSetPDFContextURLForRect(url: &NSURL, rect: CGRect);
}

extern "C-unwind" {
    pub fn UIGraphicsAddPDFContextDestinationAtPoint(name: &NSString, point: CGPoint);
}

extern "C-unwind" {
    pub fn UIGraphicsSetPDFContextDestinationForRect(name: &NSString, rect: CGRect);
}
