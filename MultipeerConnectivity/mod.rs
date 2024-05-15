// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[link(name = "MultipeerConnectivity", kind = "framework")]
extern "C" {}

#[cfg(feature = "MCAdvertiserAssistant")]
#[path = "MCAdvertiserAssistant.rs"]
mod __MCAdvertiserAssistant;
#[cfg(feature = "MCBrowserViewController")]
#[path = "MCBrowserViewController.rs"]
mod __MCBrowserViewController;
#[cfg(feature = "MCError")]
#[path = "MCError.rs"]
mod __MCError;
#[cfg(feature = "MCNearbyServiceAdvertiser")]
#[path = "MCNearbyServiceAdvertiser.rs"]
mod __MCNearbyServiceAdvertiser;
#[cfg(feature = "MCNearbyServiceBrowser")]
#[path = "MCNearbyServiceBrowser.rs"]
mod __MCNearbyServiceBrowser;
#[cfg(feature = "MCPeerID")]
#[path = "MCPeerID.rs"]
mod __MCPeerID;
#[cfg(feature = "MCSession")]
#[path = "MCSession.rs"]
mod __MCSession;

#[cfg(feature = "MCAdvertiserAssistant")]
pub use self::__MCAdvertiserAssistant::MCAdvertiserAssistant;
#[cfg(feature = "MCAdvertiserAssistant")]
pub use self::__MCAdvertiserAssistant::MCAdvertiserAssistantDelegate;
#[cfg(all(feature = "MCBrowserViewController", feature = "objc2-app-kit"))]
pub use self::__MCBrowserViewController::MCBrowserViewController;
#[cfg(feature = "MCBrowserViewController")]
pub use self::__MCBrowserViewController::MCBrowserViewControllerDelegate;
#[cfg(feature = "MCError")]
pub use self::__MCError::MCErrorCode;
#[cfg(feature = "MCError")]
pub use self::__MCError::MCErrorDomain;
#[cfg(feature = "MCNearbyServiceAdvertiser")]
pub use self::__MCNearbyServiceAdvertiser::MCNearbyServiceAdvertiser;
#[cfg(feature = "MCNearbyServiceAdvertiser")]
pub use self::__MCNearbyServiceAdvertiser::MCNearbyServiceAdvertiserDelegate;
#[cfg(feature = "MCNearbyServiceBrowser")]
pub use self::__MCNearbyServiceBrowser::MCNearbyServiceBrowser;
#[cfg(feature = "MCNearbyServiceBrowser")]
pub use self::__MCNearbyServiceBrowser::MCNearbyServiceBrowserDelegate;
#[cfg(feature = "MCPeerID")]
pub use self::__MCPeerID::MCPeerID;
#[cfg(feature = "MCSession")]
pub use self::__MCSession::kMCSessionMaximumNumberOfPeers;
#[cfg(feature = "MCSession")]
pub use self::__MCSession::kMCSessionMinimumNumberOfPeers;
#[cfg(feature = "MCSession")]
pub use self::__MCSession::MCEncryptionPreference;
#[cfg(feature = "MCSession")]
pub use self::__MCSession::MCSession;
#[cfg(feature = "MCSession")]
pub use self::__MCSession::MCSessionDelegate;
#[cfg(feature = "MCSession")]
pub use self::__MCSession::MCSessionSendDataMode;
#[cfg(feature = "MCSession")]
pub use self::__MCSession::MCSessionState;