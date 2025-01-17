//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/storekit/skproductstorepromotionvisibility?language=objc)
// NS_CLOSED_ENUM
#[deprecated = "Use Product.PromotionInfo.Visibility"]
#[repr(isize)] // NSInteger
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SKProductStorePromotionVisibility {
    #[deprecated = "Use Product.PromotionInfo.Visibility"]
    #[doc(alias = "SKProductStorePromotionVisibilityDefault")]
    Default = 0,
    #[deprecated = "Use Product.PromotionInfo.Visibility"]
    #[doc(alias = "SKProductStorePromotionVisibilityShow")]
    Show = 1,
    #[deprecated = "Use Product.PromotionInfo.Visibility"]
    #[doc(alias = "SKProductStorePromotionVisibilityHide")]
    Hide = 2,
}

unsafe impl Encode for SKProductStorePromotionVisibility {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SKProductStorePromotionVisibility {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/storekit/skproductstorepromotioncontroller?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use Product.PromotionInfo"]
    pub struct SKProductStorePromotionController;
);

unsafe impl NSObjectProtocol for SKProductStorePromotionController {}

extern_methods!(
    unsafe impl SKProductStorePromotionController {
        #[deprecated = "Use Product.PromotionInfo"]
        #[unsafe(method_family(none))]
        #[method_id(defaultController)]
        pub unsafe fn defaultController() -> Retained<Self>;

        #[cfg(all(feature = "SKProduct", feature = "block2"))]
        #[deprecated = "Get visibility from Product.PromotionInfo.currentOrder"]
        #[method(fetchStorePromotionVisibilityForProduct:completionHandler:)]
        pub unsafe fn fetchStorePromotionVisibilityForProduct_completionHandler(
            &self,
            product: &SKProduct,
            completion_handler: Option<
                &block2::Block<dyn Fn(SKProductStorePromotionVisibility, *mut NSError)>,
            >,
        );

        #[cfg(all(feature = "SKProduct", feature = "block2"))]
        #[deprecated = "Use Product.PromotionInfo.updateProductVisibility(_:for:)"]
        #[method(updateStorePromotionVisibility:forProduct:completionHandler:)]
        pub unsafe fn updateStorePromotionVisibility_forProduct_completionHandler(
            &self,
            promotion_visibility: SKProductStorePromotionVisibility,
            product: &SKProduct,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(all(feature = "SKProduct", feature = "block2"))]
        #[deprecated = "Use Product.PromotionInfo.currentOrder"]
        #[method(fetchStorePromotionOrderWithCompletionHandler:)]
        pub unsafe fn fetchStorePromotionOrderWithCompletionHandler(
            &self,
            completion_handler: Option<
                &block2::Block<dyn Fn(NonNull<NSArray<SKProduct>>, *mut NSError)>,
            >,
        );

        #[cfg(all(feature = "SKProduct", feature = "block2"))]
        #[deprecated = "Use Product.PromotionInfo.updateProductOrder(byID:)"]
        #[method(updateStorePromotionOrder:completionHandler:)]
        pub unsafe fn updateStorePromotionOrder_completionHandler(
            &self,
            promotion_order: &NSArray<SKProduct>,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SKProductStorePromotionController {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
