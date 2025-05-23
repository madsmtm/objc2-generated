//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsfilehandle?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFileHandle;
);

unsafe impl Send for NSFileHandle {}

unsafe impl Sync for NSFileHandle {}

#[cfg(feature = "NSObject")]
extern_conformance!(
    unsafe impl NSCoding for NSFileHandle {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSFileHandle {}
);

#[cfg(feature = "NSObject")]
extern_conformance!(
    unsafe impl NSSecureCoding for NSFileHandle {}
);

impl NSFileHandle {
    extern_methods!(
        #[cfg(feature = "NSData")]
        #[unsafe(method(availableData))]
        #[unsafe(method_family = none)]
        pub unsafe fn availableData(&self) -> Retained<NSData>;

        #[unsafe(method(initWithFileDescriptor:closeOnDealloc:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFileDescriptor_closeOnDealloc(
            this: Allocated<Self>,
            fd: c_int,
            closeopt: bool,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "NSData", feature = "NSError"))]
        #[unsafe(method(readDataToEndOfFileAndReturnError:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn readDataToEndOfFileAndReturnError(
            &self,
        ) -> Result<Retained<NSData>, Retained<NSError>>;

        #[cfg(all(feature = "NSData", feature = "NSError"))]
        #[unsafe(method(readDataUpToLength:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn readDataUpToLength_error(
            &self,
            length: NSUInteger,
        ) -> Result<Retained<NSData>, Retained<NSError>>;

        #[cfg(all(feature = "NSData", feature = "NSError"))]
        #[unsafe(method(writeData:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn writeData_error(&self, data: &NSData) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "NSError")]
        #[unsafe(method(getOffset:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn getOffset_error(
            &self,
            offset_in_file: NonNull<c_ulonglong>,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "NSError")]
        #[unsafe(method(seekToEndReturningOffset:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn seekToEndReturningOffset_error(
            &self,
            offset_in_file: *mut c_ulonglong,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "NSError")]
        #[unsafe(method(seekToOffset:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn seekToOffset_error(
            &self,
            offset: c_ulonglong,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "NSError")]
        #[unsafe(method(truncateAtOffset:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn truncateAtOffset_error(
            &self,
            offset: c_ulonglong,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "NSError")]
        #[unsafe(method(synchronizeAndReturnError:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn synchronizeAndReturnError(&self) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "NSError")]
        #[unsafe(method(closeAndReturnError:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn closeAndReturnError(&self) -> Result<(), Retained<NSError>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSFileHandle {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// NSFileHandleCreation.
impl NSFileHandle {
    extern_methods!(
        #[unsafe(method(fileHandleWithStandardInput))]
        #[unsafe(method_family = none)]
        pub unsafe fn fileHandleWithStandardInput() -> Retained<NSFileHandle>;

        #[unsafe(method(fileHandleWithStandardOutput))]
        #[unsafe(method_family = none)]
        pub unsafe fn fileHandleWithStandardOutput() -> Retained<NSFileHandle>;

        #[unsafe(method(fileHandleWithStandardError))]
        #[unsafe(method_family = none)]
        pub unsafe fn fileHandleWithStandardError() -> Retained<NSFileHandle>;

        #[unsafe(method(fileHandleWithNullDevice))]
        #[unsafe(method_family = none)]
        pub unsafe fn fileHandleWithNullDevice() -> Retained<NSFileHandle>;

        #[cfg(feature = "NSString")]
        #[unsafe(method(fileHandleForReadingAtPath:))]
        #[unsafe(method_family = none)]
        pub unsafe fn fileHandleForReadingAtPath(path: &NSString) -> Option<Retained<Self>>;

        #[cfg(feature = "NSString")]
        #[unsafe(method(fileHandleForWritingAtPath:))]
        #[unsafe(method_family = none)]
        pub unsafe fn fileHandleForWritingAtPath(path: &NSString) -> Option<Retained<Self>>;

        #[cfg(feature = "NSString")]
        #[unsafe(method(fileHandleForUpdatingAtPath:))]
        #[unsafe(method_family = none)]
        pub unsafe fn fileHandleForUpdatingAtPath(path: &NSString) -> Option<Retained<Self>>;

        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[unsafe(method(fileHandleForReadingFromURL:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn fileHandleForReadingFromURL_error(
            url: &NSURL,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[unsafe(method(fileHandleForWritingToURL:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn fileHandleForWritingToURL_error(
            url: &NSURL,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[unsafe(method(fileHandleForUpdatingURL:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn fileHandleForUpdatingURL_error(
            url: &NSURL,
        ) -> Result<Retained<Self>, Retained<NSError>>;
    );
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsfilehandleoperationexception?language=objc)
    #[cfg(all(feature = "NSObjCRuntime", feature = "NSString"))]
    pub static NSFileHandleOperationException: &'static NSExceptionName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsfilehandlereadcompletionnotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSFileHandleReadCompletionNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsfilehandlereadtoendoffilecompletionnotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSFileHandleReadToEndOfFileCompletionNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsfilehandleconnectionacceptednotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSFileHandleConnectionAcceptedNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsfilehandledataavailablenotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSFileHandleDataAvailableNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsfilehandlenotificationdataitem?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSFileHandleNotificationDataItem: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsfilehandlenotificationfilehandleitem?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSFileHandleNotificationFileHandleItem: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsfilehandlenotificationmonitormodes?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSFileHandleNotificationMonitorModes: &'static NSString;
}

/// NSFileHandleAsynchronousAccess.
impl NSFileHandle {
    extern_methods!(
        #[cfg(all(feature = "NSArray", feature = "NSObjCRuntime", feature = "NSString"))]
        #[unsafe(method(readInBackgroundAndNotifyForModes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn readInBackgroundAndNotifyForModes(
            &self,
            modes: Option<&NSArray<NSRunLoopMode>>,
        );

        #[unsafe(method(readInBackgroundAndNotify))]
        #[unsafe(method_family = none)]
        pub unsafe fn readInBackgroundAndNotify(&self);

        #[cfg(all(feature = "NSArray", feature = "NSObjCRuntime", feature = "NSString"))]
        #[unsafe(method(readToEndOfFileInBackgroundAndNotifyForModes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn readToEndOfFileInBackgroundAndNotifyForModes(
            &self,
            modes: Option<&NSArray<NSRunLoopMode>>,
        );

        #[unsafe(method(readToEndOfFileInBackgroundAndNotify))]
        #[unsafe(method_family = none)]
        pub unsafe fn readToEndOfFileInBackgroundAndNotify(&self);

        #[cfg(all(feature = "NSArray", feature = "NSObjCRuntime", feature = "NSString"))]
        #[unsafe(method(acceptConnectionInBackgroundAndNotifyForModes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn acceptConnectionInBackgroundAndNotifyForModes(
            &self,
            modes: Option<&NSArray<NSRunLoopMode>>,
        );

        #[unsafe(method(acceptConnectionInBackgroundAndNotify))]
        #[unsafe(method_family = none)]
        pub unsafe fn acceptConnectionInBackgroundAndNotify(&self);

        #[cfg(all(feature = "NSArray", feature = "NSObjCRuntime", feature = "NSString"))]
        #[unsafe(method(waitForDataInBackgroundAndNotifyForModes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn waitForDataInBackgroundAndNotifyForModes(
            &self,
            modes: Option<&NSArray<NSRunLoopMode>>,
        );

        #[unsafe(method(waitForDataInBackgroundAndNotify))]
        #[unsafe(method_family = none)]
        pub unsafe fn waitForDataInBackgroundAndNotify(&self);

        #[cfg(feature = "block2")]
        #[unsafe(method(readabilityHandler))]
        #[unsafe(method_family = none)]
        pub unsafe fn readabilityHandler(
            &self,
        ) -> *mut block2::DynBlock<dyn Fn(NonNull<NSFileHandle>)>;

        #[cfg(feature = "block2")]
        /// Setter for [`readabilityHandler`][Self::readabilityHandler].
        #[unsafe(method(setReadabilityHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setReadabilityHandler(
            &self,
            readability_handler: Option<&block2::DynBlock<dyn Fn(NonNull<NSFileHandle>)>>,
        );

        #[cfg(feature = "block2")]
        #[unsafe(method(writeabilityHandler))]
        #[unsafe(method_family = none)]
        pub unsafe fn writeabilityHandler(
            &self,
        ) -> *mut block2::DynBlock<dyn Fn(NonNull<NSFileHandle>)>;

        #[cfg(feature = "block2")]
        /// Setter for [`writeabilityHandler`][Self::writeabilityHandler].
        #[unsafe(method(setWriteabilityHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setWriteabilityHandler(
            &self,
            writeability_handler: Option<&block2::DynBlock<dyn Fn(NonNull<NSFileHandle>)>>,
        );
    );
}

/// NSFileHandlePlatformSpecific.
impl NSFileHandle {
    extern_methods!(
        #[unsafe(method(initWithFileDescriptor:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFileDescriptor(this: Allocated<Self>, fd: c_int) -> Retained<Self>;

        #[unsafe(method(fileDescriptor))]
        #[unsafe(method_family = none)]
        pub unsafe fn fileDescriptor(&self) -> c_int;
    );
}

impl NSFileHandle {
    extern_methods!(
        #[cfg(feature = "NSData")]
        #[deprecated]
        #[unsafe(method(readDataToEndOfFile))]
        #[unsafe(method_family = none)]
        pub unsafe fn readDataToEndOfFile(&self) -> Retained<NSData>;

        #[cfg(feature = "NSData")]
        #[deprecated]
        #[unsafe(method(readDataOfLength:))]
        #[unsafe(method_family = none)]
        pub unsafe fn readDataOfLength(&self, length: NSUInteger) -> Retained<NSData>;

        #[cfg(feature = "NSData")]
        #[deprecated]
        #[unsafe(method(writeData:))]
        #[unsafe(method_family = none)]
        pub unsafe fn writeData(&self, data: &NSData);

        #[deprecated]
        #[unsafe(method(offsetInFile))]
        #[unsafe(method_family = none)]
        pub unsafe fn offsetInFile(&self) -> c_ulonglong;

        #[deprecated]
        #[unsafe(method(seekToEndOfFile))]
        #[unsafe(method_family = none)]
        pub unsafe fn seekToEndOfFile(&self) -> c_ulonglong;

        #[deprecated]
        #[unsafe(method(seekToFileOffset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn seekToFileOffset(&self, offset: c_ulonglong);

        #[deprecated]
        #[unsafe(method(truncateFileAtOffset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn truncateFileAtOffset(&self, offset: c_ulonglong);

        #[deprecated]
        #[unsafe(method(synchronizeFile))]
        #[unsafe(method_family = none)]
        pub unsafe fn synchronizeFile(&self);

        #[deprecated]
        #[unsafe(method(closeFile))]
        #[unsafe(method_family = none)]
        pub unsafe fn closeFile(&self);
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nspipe?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPipe;
);

unsafe impl Send for NSPipe {}

unsafe impl Sync for NSPipe {}

extern_conformance!(
    unsafe impl NSObjectProtocol for NSPipe {}
);

impl NSPipe {
    extern_methods!(
        #[unsafe(method(fileHandleForReading))]
        #[unsafe(method_family = none)]
        pub unsafe fn fileHandleForReading(&self) -> Retained<NSFileHandle>;

        #[unsafe(method(fileHandleForWriting))]
        #[unsafe(method_family = none)]
        pub unsafe fn fileHandleForWriting(&self) -> Retained<NSFileHandle>;

        #[unsafe(method(pipe))]
        #[unsafe(method_family = none)]
        pub unsafe fn pipe() -> Retained<NSPipe>;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSPipe {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
