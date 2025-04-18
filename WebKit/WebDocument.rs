//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// Protocol implemented by the document view of WebFrameView
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/webkit/webdocumentview?language=objc)
    #[deprecated]
    pub unsafe trait WebDocumentView: NSObjectProtocol {
        #[cfg(feature = "WebDataSource")]
        /// Called when the corresponding data source has been created.
        ///
        /// Parameter `dataSource`: The corresponding data source.
        #[deprecated]
        #[unsafe(method(setDataSource:))]
        #[unsafe(method_family = none)]
        unsafe fn setDataSource(&self, data_source: Option<&WebDataSource>);

        #[cfg(feature = "WebDataSource")]
        /// Called when the corresponding data source has received data.
        ///
        /// Parameter `dataSource`: The corresponding data source.
        #[deprecated]
        #[unsafe(method(dataSourceUpdated:))]
        #[unsafe(method_family = none)]
        unsafe fn dataSourceUpdated(&self, data_source: Option<&WebDataSource>);

        /// Called when WebKit has determined that the document view needs to layout.
        /// This method should simply set a flag and call layout from drawRect if the flag is YES.
        ///
        /// Parameter `flag`: YES to cause a layout, no to not cause a layout.
        #[deprecated]
        #[unsafe(method(setNeedsLayout:))]
        #[unsafe(method_family = none)]
        unsafe fn setNeedsLayout(&self, flag: bool);

        /// Called when the document view must immediately layout. For simple views,
        /// setting the frame is a sufficient implementation of this method.
        #[deprecated]
        #[unsafe(method(layout))]
        #[unsafe(method_family = none)]
        unsafe fn layout(&self);

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        /// Parameter `hostWindow`: The host window for the document view.
        ///
        /// Called before the host window is set on the parent web view.
        #[deprecated]
        #[unsafe(method(viewWillMoveToHostWindow:))]
        #[unsafe(method_family = none)]
        unsafe fn viewWillMoveToHostWindow(&self, host_window: Option<&NSWindow>);

        /// Called after the host window is set on the parent web view.
        #[deprecated]
        #[unsafe(method(viewDidMoveToHostWindow))]
        #[unsafe(method_family = none)]
        unsafe fn viewDidMoveToHostWindow(&self);
    }
);

extern_protocol!(
    /// Optional protocol for searching document view of WebFrameView.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/webkit/webdocumentsearching?language=objc)
    #[deprecated]
    pub unsafe trait WebDocumentSearching: NSObjectProtocol {
        /// Searches a document view for a string and highlights the string if it is found.
        ///
        /// Parameter `string`: The string to search for.
        ///
        /// Parameter `forward`: YES to search forward, NO to seach backwards.
        ///
        /// Parameter `caseFlag`: YES to for case-sensitive search, NO for case-insensitive search.
        ///
        /// Parameter `wrapFlag`: YES to wrap around, NO to avoid wrapping.
        ///
        /// Returns: YES if found, NO if not found.
        #[deprecated]
        #[unsafe(method(searchFor:direction:caseSensitive:wrap:))]
        #[unsafe(method_family = none)]
        unsafe fn searchFor_direction_caseSensitive_wrap(
            &self,
            string: Option<&NSString>,
            forward: bool,
            case_flag: bool,
            wrap_flag: bool,
        ) -> bool;
    }
);

extern_protocol!(
    /// Optional protocol for supporting text operations.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/webkit/webdocumenttext?language=objc)
    #[deprecated]
    pub unsafe trait WebDocumentText: NSObjectProtocol {
        /// Returns: YES if the document view support text encoding, NO if it doesn't.
        #[deprecated]
        #[unsafe(method(supportsTextEncoding))]
        #[unsafe(method_family = none)]
        unsafe fn supportsTextEncoding(&self) -> bool;

        /// Returns: String that represents the entire document.
        #[deprecated]
        #[unsafe(method(string))]
        #[unsafe(method_family = none)]
        unsafe fn string(&self) -> Option<Retained<NSString>>;

        /// Returns: Attributed string that represents the entire document.
        #[deprecated]
        #[unsafe(method(attributedString))]
        #[unsafe(method_family = none)]
        unsafe fn attributedString(&self) -> Option<Retained<NSAttributedString>>;

        /// Returns: String that represents the current selection.
        #[deprecated]
        #[unsafe(method(selectedString))]
        #[unsafe(method_family = none)]
        unsafe fn selectedString(&self) -> Option<Retained<NSString>>;

        /// Returns: Attributed string that represents the current selection.
        #[deprecated]
        #[unsafe(method(selectedAttributedString))]
        #[unsafe(method_family = none)]
        unsafe fn selectedAttributedString(&self) -> Option<Retained<NSAttributedString>>;

        /// Selects all the text in the document.
        #[deprecated]
        #[unsafe(method(selectAll))]
        #[unsafe(method_family = none)]
        unsafe fn selectAll(&self);

        /// Causes a text selection to lose its selection.
        #[deprecated]
        #[unsafe(method(deselectAll))]
        #[unsafe(method_family = none)]
        unsafe fn deselectAll(&self);
    }
);

extern_protocol!(
    /// Protocol implemented by the document representation of a data source.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/webkit/webdocumentrepresentation?language=objc)
    #[deprecated]
    pub unsafe trait WebDocumentRepresentation: NSObjectProtocol {
        #[cfg(feature = "WebDataSource")]
        /// Called soon after the document representation is created.
        ///
        /// Parameter `dataSource`: The data source that is set.
        #[deprecated]
        #[unsafe(method(setDataSource:))]
        #[unsafe(method_family = none)]
        unsafe fn setDataSource(&self, data_source: Option<&WebDataSource>);

        #[cfg(feature = "WebDataSource")]
        /// Called when the data source has received data.
        ///
        /// Parameter `data`: The data that the data source has received.
        ///
        /// Parameter `dataSource`: The data source that has received data.
        #[deprecated]
        #[unsafe(method(receivedData:withDataSource:))]
        #[unsafe(method_family = none)]
        unsafe fn receivedData_withDataSource(
            &self,
            data: Option<&NSData>,
            data_source: Option<&WebDataSource>,
        );

        #[cfg(feature = "WebDataSource")]
        /// Called when the data source has received an error.
        ///
        /// Parameter `error`: The error that the data source has received.
        ///
        /// Parameter `dataSource`: The data source that has received the error.
        #[deprecated]
        #[unsafe(method(receivedError:withDataSource:))]
        #[unsafe(method_family = none)]
        unsafe fn receivedError_withDataSource(
            &self,
            error: Option<&NSError>,
            data_source: Option<&WebDataSource>,
        );

        #[cfg(feature = "WebDataSource")]
        /// Called when the data source has finished loading.
        ///
        /// Parameter `dataSource`: The datasource that has finished loading.
        #[deprecated]
        #[unsafe(method(finishedLoadingWithDataSource:))]
        #[unsafe(method_family = none)]
        unsafe fn finishedLoadingWithDataSource(&self, data_source: Option<&WebDataSource>);

        /// Returns: Returns true if the representation can provide document source.
        #[deprecated]
        #[unsafe(method(canProvideDocumentSource))]
        #[unsafe(method_family = none)]
        unsafe fn canProvideDocumentSource(&self) -> bool;

        /// Returns: Returns the textual source representation of the document.  For HTML documents
        /// this is the original HTML source.
        #[deprecated]
        #[unsafe(method(documentSource))]
        #[unsafe(method_family = none)]
        unsafe fn documentSource(&self) -> Option<Retained<NSString>>;

        /// Returns: Return the title for the document.
        #[deprecated]
        #[unsafe(method(title))]
        #[unsafe(method_family = none)]
        unsafe fn title(&self) -> Option<Retained<NSString>>;
    }
);
