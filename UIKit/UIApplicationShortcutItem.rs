//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIApplicationShortcutIconType(pub NSInteger);
impl UIApplicationShortcutIconType {
    #[doc(alias = "UIApplicationShortcutIconTypeCompose")]
    pub const Compose: Self = Self(0);
    #[doc(alias = "UIApplicationShortcutIconTypePlay")]
    pub const Play: Self = Self(1);
    #[doc(alias = "UIApplicationShortcutIconTypePause")]
    pub const Pause: Self = Self(2);
    #[doc(alias = "UIApplicationShortcutIconTypeAdd")]
    pub const Add: Self = Self(3);
    #[doc(alias = "UIApplicationShortcutIconTypeLocation")]
    pub const Location: Self = Self(4);
    #[doc(alias = "UIApplicationShortcutIconTypeSearch")]
    pub const Search: Self = Self(5);
    #[doc(alias = "UIApplicationShortcutIconTypeShare")]
    pub const Share: Self = Self(6);
    #[doc(alias = "UIApplicationShortcutIconTypeProhibit")]
    pub const Prohibit: Self = Self(7);
    #[doc(alias = "UIApplicationShortcutIconTypeContact")]
    pub const Contact: Self = Self(8);
    #[doc(alias = "UIApplicationShortcutIconTypeHome")]
    pub const Home: Self = Self(9);
    #[doc(alias = "UIApplicationShortcutIconTypeMarkLocation")]
    pub const MarkLocation: Self = Self(10);
    #[doc(alias = "UIApplicationShortcutIconTypeFavorite")]
    pub const Favorite: Self = Self(11);
    #[doc(alias = "UIApplicationShortcutIconTypeLove")]
    pub const Love: Self = Self(12);
    #[doc(alias = "UIApplicationShortcutIconTypeCloud")]
    pub const Cloud: Self = Self(13);
    #[doc(alias = "UIApplicationShortcutIconTypeInvitation")]
    pub const Invitation: Self = Self(14);
    #[doc(alias = "UIApplicationShortcutIconTypeConfirmation")]
    pub const Confirmation: Self = Self(15);
    #[doc(alias = "UIApplicationShortcutIconTypeMail")]
    pub const Mail: Self = Self(16);
    #[doc(alias = "UIApplicationShortcutIconTypeMessage")]
    pub const Message: Self = Self(17);
    #[doc(alias = "UIApplicationShortcutIconTypeDate")]
    pub const Date: Self = Self(18);
    #[doc(alias = "UIApplicationShortcutIconTypeTime")]
    pub const Time: Self = Self(19);
    #[doc(alias = "UIApplicationShortcutIconTypeCapturePhoto")]
    pub const CapturePhoto: Self = Self(20);
    #[doc(alias = "UIApplicationShortcutIconTypeCaptureVideo")]
    pub const CaptureVideo: Self = Self(21);
    #[doc(alias = "UIApplicationShortcutIconTypeTask")]
    pub const Task: Self = Self(22);
    #[doc(alias = "UIApplicationShortcutIconTypeTaskCompleted")]
    pub const TaskCompleted: Self = Self(23);
    #[doc(alias = "UIApplicationShortcutIconTypeAlarm")]
    pub const Alarm: Self = Self(24);
    #[doc(alias = "UIApplicationShortcutIconTypeBookmark")]
    pub const Bookmark: Self = Self(25);
    #[doc(alias = "UIApplicationShortcutIconTypeShuffle")]
    pub const Shuffle: Self = Self(26);
    #[doc(alias = "UIApplicationShortcutIconTypeAudio")]
    pub const Audio: Self = Self(27);
    #[doc(alias = "UIApplicationShortcutIconTypeUpdate")]
    pub const Update: Self = Self(28);
}

unsafe impl Encode for UIApplicationShortcutIconType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIApplicationShortcutIconType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIApplicationShortcutIcon;

    unsafe impl ClassType for UIApplicationShortcutIcon {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for UIApplicationShortcutIcon {}

unsafe impl NSObjectProtocol for UIApplicationShortcutIcon {}

extern_methods!(
    unsafe impl UIApplicationShortcutIcon {
        #[method_id(@__retain_semantics Other iconWithType:)]
        pub unsafe fn iconWithType(r#type: UIApplicationShortcutIconType) -> Id<Self>;

        #[method_id(@__retain_semantics Other iconWithTemplateImageName:)]
        pub unsafe fn iconWithTemplateImageName(template_image_name: &NSString) -> Id<Self>;

        #[method_id(@__retain_semantics Other iconWithSystemImageName:)]
        pub unsafe fn iconWithSystemImageName(system_image_name: &NSString) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIApplicationShortcutIcon {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIApplicationShortcutItem;

    unsafe impl ClassType for UIApplicationShortcutItem {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for UIApplicationShortcutItem {}

unsafe impl NSMutableCopying for UIApplicationShortcutItem {}

unsafe impl NSObjectProtocol for UIApplicationShortcutItem {}

extern_methods!(
    unsafe impl UIApplicationShortcutItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithType:localizedTitle:localizedSubtitle:icon:userInfo:)]
        pub unsafe fn initWithType_localizedTitle_localizedSubtitle_icon_userInfo(
            this: Allocated<Self>,
            r#type: &NSString,
            localized_title: &NSString,
            localized_subtitle: Option<&NSString>,
            icon: Option<&UIApplicationShortcutIcon>,
            user_info: Option<&NSDictionary<NSString, ProtocolObject<dyn NSSecureCoding>>>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithType:localizedTitle:)]
        pub unsafe fn initWithType_localizedTitle(
            this: Allocated<Self>,
            r#type: &NSString,
            localized_title: &NSString,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other localizedTitle)]
        pub unsafe fn localizedTitle(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other localizedSubtitle)]
        pub unsafe fn localizedSubtitle(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other icon)]
        pub unsafe fn icon(&self) -> Option<Id<UIApplicationShortcutIcon>>;

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(
            &self,
        ) -> Option<Id<NSDictionary<NSString, ProtocolObject<dyn NSSecureCoding>>>>;

        #[method_id(@__retain_semantics Other targetContentIdentifier)]
        pub unsafe fn targetContentIdentifier(&self) -> Option<Id<AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIApplicationShortcutItem {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIMutableApplicationShortcutItem;

    unsafe impl ClassType for UIMutableApplicationShortcutItem {
        #[inherits(NSObject)]
        type Super = UIApplicationShortcutItem;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for UIMutableApplicationShortcutItem {}

unsafe impl NSMutableCopying for UIMutableApplicationShortcutItem {}

unsafe impl NSObjectProtocol for UIMutableApplicationShortcutItem {}

extern_methods!(
    unsafe impl UIMutableApplicationShortcutItem {
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Id<NSString>;

        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: &NSString);

        #[method_id(@__retain_semantics Other localizedTitle)]
        pub unsafe fn localizedTitle(&self) -> Id<NSString>;

        #[method(setLocalizedTitle:)]
        pub unsafe fn setLocalizedTitle(&self, localized_title: &NSString);

        #[method_id(@__retain_semantics Other localizedSubtitle)]
        pub unsafe fn localizedSubtitle(&self) -> Option<Id<NSString>>;

        #[method(setLocalizedSubtitle:)]
        pub unsafe fn setLocalizedSubtitle(&self, localized_subtitle: Option<&NSString>);

        #[method_id(@__retain_semantics Other icon)]
        pub unsafe fn icon(&self) -> Option<Id<UIApplicationShortcutIcon>>;

        #[method(setIcon:)]
        pub unsafe fn setIcon(&self, icon: Option<&UIApplicationShortcutIcon>);

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(
            &self,
        ) -> Option<Id<NSDictionary<NSString, ProtocolObject<dyn NSSecureCoding>>>>;

        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(
            &self,
            user_info: Option<&NSDictionary<NSString, ProtocolObject<dyn NSSecureCoding>>>,
        );

        #[method_id(@__retain_semantics Other targetContentIdentifier)]
        pub unsafe fn targetContentIdentifier(&self) -> Option<Id<AnyObject>>;

        #[method(setTargetContentIdentifier:)]
        pub unsafe fn setTargetContentIdentifier(
            &self,
            target_content_identifier: Option<&AnyObject>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `UIApplicationShortcutItem`
    unsafe impl UIMutableApplicationShortcutItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithType:localizedTitle:localizedSubtitle:icon:userInfo:)]
        pub unsafe fn initWithType_localizedTitle_localizedSubtitle_icon_userInfo(
            this: Allocated<Self>,
            r#type: &NSString,
            localized_title: &NSString,
            localized_subtitle: Option<&NSString>,
            icon: Option<&UIApplicationShortcutIcon>,
            user_info: Option<&NSDictionary<NSString, ProtocolObject<dyn NSSecureCoding>>>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithType:localizedTitle:)]
        pub unsafe fn initWithType_localizedTitle(
            this: Allocated<Self>,
            r#type: &NSString,
            localized_title: &NSString,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIMutableApplicationShortcutItem {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);