//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundoclosegroupingrunloopordering?language=objc)
pub static NSUndoCloseGroupingRunLoopOrdering: NSUInteger = 350000;

/// A key used to set and get user info for undo and redo actions
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundomanageruserinfokey?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "NSString")]
pub type NSUndoManagerUserInfoKey = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundomanagergroupisdiscardablekey?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSUndoManagerGroupIsDiscardableKey: &'static NSString;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundomanager?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSUndoManager;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSUndoManager {}
);

impl NSUndoManager {
    extern_methods!(
        /// Marks the beginning of an undo group.
        ///
        /// All individual undo operations before a subsequent ``endUndoGrouping`` message are grouped together and reversed by a later ``undo`` message. By default undo groups are begun automatically at the start of the event loop, but you can begin your own undo groups with this method, and nest them within other groups.
        ///
        /// This method posts an ``NSUndoManagerCheckpointNotification`` unless a top-level undo is in progress. It posts an ``NSUndoManagerDidOpenUndoGroupNotification`` if a new group was successfully created.
        #[unsafe(method(beginUndoGrouping))]
        #[unsafe(method_family = none)]
        pub unsafe fn beginUndoGrouping(&self);

        /// Marks the end of an undo group.
        ///
        /// All individual undo operations back to the matching ``beginUndoGrouping`` message are grouped together and reversed by a later ``undo`` or ``undoNestedGroup`` message. Undo groups can be nested, thus providing functionality similar to nested transactions. Raises an ``NSInternalInconsistencyException`` if there’s no ``beginUndoGrouping`` message in effect.
        ///
        /// This method posts an ``NSUndoManagerCheckpointNotification`` and an ``NSUndoManagerDidCloseUndoGroupNotification`` just before the group is closed.
        #[unsafe(method(endUndoGrouping))]
        #[unsafe(method_family = none)]
        pub unsafe fn endUndoGrouping(&self);

        /// The number of nested undo groups (or redo groups, if Redo was invoked last) in the current event loop.
        ///
        /// An integer indicating the number of nested groups. If `0` is returned, there is no open undo or redo group.
        #[unsafe(method(groupingLevel))]
        #[unsafe(method_family = none)]
        pub unsafe fn groupingLevel(&self) -> NSInteger;

        /// Disables the recording of undo operations, whether by ``registerUndoWithTarget:selector:object:`` or by invocation-based undo.
        ///
        /// This method can be invoked multiple times by multiple clients. The ``enableUndoRegistration`` method must be invoked an equal number of times to re-enable undo registration.
        #[unsafe(method(disableUndoRegistration))]
        #[unsafe(method_family = none)]
        pub unsafe fn disableUndoRegistration(&self);

        /// Enables the recording of undo operations.
        ///
        /// Because undo registration is enabled by default, this is used to balance a prior ``disableUndoRegistration``. Undo registration isn’t actually re-enabled until an enable message balances the last disable message in effect.
        /// Raises an NSInternalInconsistencyException if invoked while no disableUndoRegistration() message is in effect.
        #[unsafe(method(enableUndoRegistration))]
        #[unsafe(method_family = none)]
        pub unsafe fn enableUndoRegistration(&self);

        /// Whether the recording of undo operations is enabled.
        #[unsafe(method(isUndoRegistrationEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isUndoRegistrationEnabled(&self) -> bool;

        /// A Boolean value that indicates whether the receiver automatically creates undo groups around each pass of the run loop.
        ///
        /// If `true`, the receiver automatically creates undo groups around each pass of the run loop.
        /// The default is `true`. If you turn automatic grouping off, you must close groups explicitly before invoking either ``undo`` or ``undoNestedGroup``.
        #[unsafe(method(groupsByEvent))]
        #[unsafe(method_family = none)]
        pub unsafe fn groupsByEvent(&self) -> bool;

        /// Setter for [`groupsByEvent`][Self::groupsByEvent].
        #[unsafe(method(setGroupsByEvent:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setGroupsByEvent(&self, groups_by_event: bool);

        /// The maximum number of top-level undo groups the receiver holds.
        ///
        /// An integer specifying the number of undo groups. A limit of 0 indicates no limit, so old undo groups are never dropped.
        /// When ending an undo group results in the number of groups exceeding this limit, the oldest groups are dropped from the stack. The default is 0.
        /// If you change the limit to a level below the prior limit, old undo groups are immediately dropped.
        #[unsafe(method(levelsOfUndo))]
        #[unsafe(method_family = none)]
        pub unsafe fn levelsOfUndo(&self) -> NSUInteger;

        /// Setter for [`levelsOfUndo`][Self::levelsOfUndo].
        #[unsafe(method(setLevelsOfUndo:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLevelsOfUndo(&self, levels_of_undo: NSUInteger);

        #[cfg(all(feature = "NSArray", feature = "NSObjCRuntime", feature = "NSString"))]
        /// The modes governing the types of input handled during a cycle of the run loop.
        ///
        /// An array of string constants specifying the current run-loop modes.
        /// By default, the sole run-loop mode is ``NSDefaultRunLoopMode`` (which excludes data from ``NSConnection`` objects). Some examples of other uses are to limit the input to data received during a mouse-tracking session by setting the mode to ``NSEventTrackingRunLoopMode``, or limit it to data received from a modal panel with ``NSModalPanelRunLoopMode``.
        #[unsafe(method(runLoopModes))]
        #[unsafe(method_family = none)]
        pub unsafe fn runLoopModes(&self) -> Retained<NSArray<NSRunLoopMode>>;

        #[cfg(all(feature = "NSArray", feature = "NSObjCRuntime", feature = "NSString"))]
        /// Setter for [`runLoopModes`][Self::runLoopModes].
        #[unsafe(method(setRunLoopModes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRunLoopModes(&self, run_loop_modes: &NSArray<NSRunLoopMode>);

        /// Closes the top-level undo group if necessary and invokes ``undoNestedGroup``.
        ///
        /// This method also invokes ``endUndoGrouping`` if the nesting level is 1. Raises an ``NSInternalInconsistencyException`` if more than one undo group is open (that is, if the last group isn’t at the top level).
        /// This method posts an ``NSUndoManagerCheckpointNotification``.
        #[unsafe(method(undo))]
        #[unsafe(method_family = none)]
        pub unsafe fn undo(&self);

        /// Performs the operations in the last group on the redo stack, if there are any, recording them on the undo stack as a single group.
        ///
        /// Raises an ``NSInternalInconsistencyException`` if the method is invoked during an undo operation.
        /// This method posts an ``NSUndoManagerCheckpointNotification`` and ``NSUndoManagerWillRedoChangeNotification`` before it performs the redo operation, and it posts the ``NSUndoManagerDidRedoChangeNotification`` after it performs the redo operation.
        #[unsafe(method(redo))]
        #[unsafe(method_family = none)]
        pub unsafe fn redo(&self);

        /// Performs the undo operations in the last undo group (whether top-level or nested), recording the operations on the redo stack as a single group.
        ///
        /// Raises an ``NSInternalInconsistencyException`` if any undo operations have been registered since the last ``enableUndoRegistration`` message.
        /// This method posts an ``NSUndoManagerCheckpointNotification`` and ``NSUndoManagerWillUndoChangeNotification`` before it performs the undo operation, and it posts an ``NSUndoManagerDidUndoChangeNotification`` after it performs the undo operation.
        #[unsafe(method(undoNestedGroup))]
        #[unsafe(method_family = none)]
        pub unsafe fn undoNestedGroup(&self);

        /// Whether the receiver has any actions to undo.
        ///
        /// The return value does not mean you can safely invoke ``undo`` or ``undoNestedGroup`` — you may have to close open undo groups first.
        #[unsafe(method(canUndo))]
        #[unsafe(method_family = none)]
        pub unsafe fn canUndo(&self) -> bool;

        /// Whether the receiver has any actions to redo.
        ///
        /// Because any undo operation registered clears the redo stack, this method posts an NSUndoManagerCheckpointNotification to allow clients to apply their pending operations before testing the redo stack.
        #[unsafe(method(canRedo))]
        #[unsafe(method_family = none)]
        pub unsafe fn canRedo(&self) -> bool;

        /// How many times `undo` can be invoked before there are no more actions left to
        /// be undone
        #[unsafe(method(undoCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn undoCount(&self) -> NSUInteger;

        /// How many times `redo` can be invoked before there are no more actions left to
        /// be redone
        #[unsafe(method(redoCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn redoCount(&self) -> NSUInteger;

        /// Whether the receiver is in the process of performing its ``undo`` or ``undoNestedGroup`` method.
        #[unsafe(method(isUndoing))]
        #[unsafe(method_family = none)]
        pub unsafe fn isUndoing(&self) -> bool;

        /// Whether the receiver is in the process of performing its ``redo`` method.
        #[unsafe(method(isRedoing))]
        #[unsafe(method_family = none)]
        pub unsafe fn isRedoing(&self) -> bool;

        /// Clears the undo and redo stacks and re-enables the receiver.
        #[unsafe(method(removeAllActions))]
        #[unsafe(method_family = none)]
        pub unsafe fn removeAllActions(&self);

        /// Clears the undo and redo stacks of all operations involving the specified target as the recipient of the undo message.
        ///
        /// Doesn't re-enable the receiver if it's disabled.
        ///
        /// - Parameter target: The recepient of the undo mesages to be removed.
        #[unsafe(method(removeAllActionsWithTarget:))]
        #[unsafe(method_family = none)]
        pub unsafe fn removeAllActionsWithTarget(&self, target: &AnyObject);

        /// Registers the selector of the specified target to implement a single undo operation that the target receives.
        ///
        /// - Parameter target: The target of the undo operation. The undo manager maintains an unowned reference to `target` to prevent retain cycles.
        /// - Parameter selector: The selector for the undo operation.
        /// - Parameter object: The argument sent with the selector. The undo manager maintains a strong reference to `object`
        #[unsafe(method(registerUndoWithTarget:selector:object:))]
        #[unsafe(method_family = none)]
        pub unsafe fn registerUndoWithTarget_selector_object(
            &self,
            target: &AnyObject,
            selector: Sel,
            object: Option<&AnyObject>,
        );

        /// Prepares the undo manager for invocation-based undo with the given target as the subject of the next undo operation.
        ///
        /// For example, when called as:
        ///
        /// [[undoManager prepareWithInvocationTarget:target] setFont:oldFont color:oldColor]
        ///
        /// When undo is called, the specified target will be called with
        ///
        /// [target setFont:oldFont color:oldColor]
        ///
        /// - Parameter target: The target of the undo operation. The undo manager maintains a weak reference to `target`.
        /// - Returns:  A proxy object that forwards messages to the undo manager for recording as undo actions.
        #[unsafe(method(prepareWithInvocationTarget:))]
        #[unsafe(method_family = none)]
        pub unsafe fn prepareWithInvocationTarget(&self, target: &AnyObject)
            -> Retained<AnyObject>;

        #[cfg(feature = "block2")]
        /// Records a single undo operation for a given target so that when an undo is performed, it executes the specified block.
        ///
        /// As with other undo operations, this does not strongly retain target. Care should be taken to avoid introducing retain cycles by other references captured by the block.
        ///
        /// - Parameter target: The target of the undo operation.
        /// - Parameter undoHandler: The block to be executed when an operation is undone. The block takes a single argument, the target of the undo operation.
        #[unsafe(method(registerUndoWithTarget:handler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn registerUndoWithTarget_handler(
            &self,
            target: &AnyObject,
            undo_handler: &block2::DynBlock<dyn Fn(NonNull<AnyObject>)>,
        );

        /// Sets whether the next undo or redo action is discardable.
        ///
        /// Specifies that the latest undo action may be safely discarded when a document can not be saved for any reason.
        /// An example might be an undo action that changes the viewable area of a document.
        /// To find out if an undo group contains only discardable actions, look for the ``NSUndoManagerGroupIsDiscardableKey`` in the `userInfo` dictionary of the ``NSUndoManagerWillCloseUndoGroupNotification``.
        ///
        /// - Parameter discardable: Specifies if the action is discardable. YES if the next undo or redo action can be discarded; NO otherwise.
        #[unsafe(method(setActionIsDiscardable:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setActionIsDiscardable(&self, discardable: bool);

        /// Whether the next undo action is discardable.
        ///
        /// Specifies that the latest undo action may be safely discarded when a document can not be saved for any reason. These are typically actions that don’t affect persistent state.
        /// An example might be an undo action that changes the viewable area of a document.
        #[unsafe(method(undoActionIsDiscardable))]
        #[unsafe(method_family = none)]
        pub unsafe fn undoActionIsDiscardable(&self) -> bool;

        /// Whether the next redo action is discardable.
        ///
        /// Specifies that the latest redo action may be safely discarded when a document can not be saved for any reason. These are typically actions that don’t affect persistent state.
        /// An example might be an redo action that changes the viewable area of a document.
        #[unsafe(method(redoActionIsDiscardable))]
        #[unsafe(method_family = none)]
        pub unsafe fn redoActionIsDiscardable(&self) -> bool;

        #[cfg(feature = "NSString")]
        /// The name identifying the undo action.
        ///
        /// The undo action name. Returns an empty string if no action name has been assigned or if there is nothing to undo.
        /// For example, if the menu title is “Undo Delete,” the string returned is “Delete.”
        #[unsafe(method(undoActionName))]
        #[unsafe(method_family = none)]
        pub unsafe fn undoActionName(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// The name identifying the redo action.
        ///
        /// The redo action name. Returns an empty string if no action name has been assigned or if there is nothing to redo.
        /// For example, if the menu title is “Redo Delete,” the string returned is “Delete.”
        #[unsafe(method(redoActionName))]
        #[unsafe(method_family = none)]
        pub unsafe fn redoActionName(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Sets the name of the action associated with the Undo or Redo command.
        ///
        /// If actionName is an empty string, the action name currently associated with the menu command is removed. There is no effect if actionName is nil.
        ///
        /// - Parameter actionName: The name of the action.
        #[unsafe(method(setActionName:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setActionName(&self, action_name: &NSString);

        #[cfg(feature = "NSString")]
        /// Get a value from the undo action's user info
        ///
        /// - Parameter key: Which value should be retrieved
        #[unsafe(method(undoActionUserInfoValueForKey:))]
        #[unsafe(method_family = none)]
        pub unsafe fn undoActionUserInfoValueForKey(
            &self,
            key: &NSUndoManagerUserInfoKey,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSString")]
        /// Get a value from the redo action's user info
        ///
        /// - Parameter key: Which value should be retrieved
        #[unsafe(method(redoActionUserInfoValueForKey:))]
        #[unsafe(method_family = none)]
        pub unsafe fn redoActionUserInfoValueForKey(
            &self,
            key: &NSUndoManagerUserInfoKey,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSString")]
        /// Set user info for the Undo or Redo command.
        /// - Parameter info: Value to be saved in the user info
        /// - Parameter key: Key at which the object should be saved
        #[unsafe(method(setActionUserInfoValue:forKey:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setActionUserInfoValue_forKey(
            &self,
            info: Option<&AnyObject>,
            key: &NSUndoManagerUserInfoKey,
        );

        #[cfg(feature = "NSString")]
        /// The complete title of the Undo menu command, for example, “Undo Paste.”
        ///
        /// Returns “Undo” if no action name has been assigned or nil if there is nothing to undo.
        #[unsafe(method(undoMenuItemTitle))]
        #[unsafe(method_family = none)]
        pub unsafe fn undoMenuItemTitle(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// The complete title of the Redo menu command, for example, “Redo Paste.”
        ///
        /// Returns “Redo” if no action name has been assigned or nil if there is nothing to redo.
        #[unsafe(method(redoMenuItemTitle))]
        #[unsafe(method_family = none)]
        pub unsafe fn redoMenuItemTitle(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Returns the complete, localized title of the Undo menu command for the action identified by the given name.
        ///
        /// Override this method if you want to customize the localization behaviour. This method is invoked by ``undoMenuItemTitle``.
        ///
        /// - Parameter actionName: The name of the undo action.
        /// - Returns: The localized title of the undo menu item.
        #[unsafe(method(undoMenuTitleForUndoActionName:))]
        #[unsafe(method_family = none)]
        pub unsafe fn undoMenuTitleForUndoActionName(
            &self,
            action_name: &NSString,
        ) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Returns the complete, localized title of the Redo menu command for the action identified by the given name.
        ///
        /// Override this method if you want to customize the localization behaviour. This method is invoked by ``redoMenuItemTitle``.
        ///
        /// - Parameter actionName: The name of the redo action.
        /// - Returns: The localized title of the redo menu item.
        #[unsafe(method(redoMenuTitleForUndoActionName:))]
        #[unsafe(method_family = none)]
        pub unsafe fn redoMenuTitleForUndoActionName(
            &self,
            action_name: &NSString,
        ) -> Retained<NSString>;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSUndoManager {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundomanagercheckpointnotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSUndoManagerCheckpointNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundomanagerwillundochangenotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSUndoManagerWillUndoChangeNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundomanagerwillredochangenotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSUndoManagerWillRedoChangeNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundomanagerdidundochangenotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSUndoManagerDidUndoChangeNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundomanagerdidredochangenotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSUndoManagerDidRedoChangeNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundomanagerdidopenundogroupnotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSUndoManagerDidOpenUndoGroupNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundomanagerwillcloseundogroupnotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSUndoManagerWillCloseUndoGroupNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundomanagerdidcloseundogroupnotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSUndoManagerDidCloseUndoGroupNotification: &'static NSNotificationName;
}
