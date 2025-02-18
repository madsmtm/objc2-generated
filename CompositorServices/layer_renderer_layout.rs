//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// Constants that specify the organization of the textures you use for
/// drawing.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/compositorservices/cp_layer_renderer_layout?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct cp_layer_renderer_layout(pub u32);
impl cp_layer_renderer_layout {
    /// A layout that assigns a separate texture to each rendered view.
    ///
    /// When the layout contains multiple views, each view receives its
    /// own dedicated texture. The type of each texture is MTLTextureType2D.
    #[doc(alias = "cp_layer_renderer_layout_dedicated")]
    pub const dedicated: Self = Self(0);
    /// A layout that uses a single texture to store the content for all
    /// rendered views.
    ///
    /// When a layer contains multiple views, the texture stores the images
    /// for those views side-by-side. The texture map for each view contains
    /// a viewport that defines the boundaries of the view’s content. The
    /// type of each texture is MTLTextureType2D.
    #[doc(alias = "cp_layer_renderer_layout_shared")]
    pub const shared: Self = Self(1);
    /// A layout that specifies each view’s content as a slice of a single
    /// texture.
    ///
    /// The layout uses a single texture to store the content for all rendered
    /// views. The type of the texture is MTLTextureType2DArray. The texture
    /// map’s slice index indicates which array slot contains the view’s
    /// content.
    #[doc(alias = "cp_layer_renderer_layout_layered")]
    pub const layered: Self = Self(2);
}

unsafe impl Encode for cp_layer_renderer_layout {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for cp_layer_renderer_layout {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
