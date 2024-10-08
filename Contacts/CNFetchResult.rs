//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNFetchResult<ValueType: ?Sized = AnyObject> {
        __superclass: NSObject,
        _inner0: PhantomData<*mut ValueType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<ValueType: ?Sized + Message> ClassType for CNFetchResult<ValueType> {
        type Super = NSObject;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }
    }
);

unsafe impl<ValueType: ?Sized> NSObjectProtocol for CNFetchResult<ValueType> {}

extern_methods!(
    unsafe impl<ValueType: Message> CNFetchResult<ValueType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Retained<ValueType>;

        #[method_id(@__retain_semantics Other currentHistoryToken)]
        pub unsafe fn currentHistoryToken(&self) -> Retained<NSData>;
    }
);
