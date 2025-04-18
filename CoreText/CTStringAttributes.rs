//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
use objc2_core_foundation::*;

use crate::*;

extern "C" {
    /// The font.
    ///
    ///
    /// Value must be a CTFontRef. Default is Helvetica 12.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctfontattributename?language=objc)
    pub static kCTFontAttributeName: &'static CFString;
}

extern "C" {
    /// Never set a foreground color in the CGContext; use what is set as
    /// the context's fill color.
    ///
    ///
    /// Value must be a CFBooleanRef. Default is false. The reason
    /// why this exists is because an NSAttributedString defaults to a
    /// black color if no color attribute is set. This forces CoreText to
    /// set the color in the context. This will allow developers to
    /// sidestep this, making CoreText set nothing but font information
    /// in the CGContext. If set, this attribute also determines the
    /// color used by kCTUnderlineStyleAttributeName, in which case it
    /// overrides the foreground color.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctforegroundcolorfromcontextattributename?language=objc)
    pub static kCTForegroundColorFromContextAttributeName: &'static CFString;
}

extern "C" {
    /// A kerning adjustment.
    ///
    ///
    /// Value must be a CFNumberRef float. Default is standard kerning.
    /// The kerning attribute indicate how many points the following
    /// character should be shifted from its default offset as defined
    /// by the current character's font in points; a positive kern
    /// indicates a shift farther along and a negative kern indicates a
    /// shift closer to the current character. If this attribute is not
    /// present, standard kerning will be used. If this attribute is
    /// set to 0.0, no kerning will be done at all.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctkernattributename?language=objc)
    pub static kCTKernAttributeName: &'static CFString;
}

extern "C" {
    /// Applies tracking (letterspacing).
    ///
    ///
    /// Value must be a CFNumber. Default is zero (no tracking).
    /// The tracking attribute indicates how much additional space, in
    /// points, should be added to each character cluster after layout.
    /// The effect of this attribute is similar to kCTKernAttributeName
    /// but differs in that the added tracking is treated as trailing
    /// whitespace and a non-zero amount disables non-essential ligatures
    /// unless overridden by kCTLigatureAttributeName being present.
    /// If both kCTKernAttributeName and kCTTrackingAttributeName are
    /// present kCTKernAttributeName will be ignored unless zero;
    /// kCTTrackingAttributeName will still be honored.
    ///
    ///
    /// See also: kCTKernAttributeName
    ///
    /// See also: kCTLigatureAttributeName
    ///
    /// See also: CTLineGetTrailingWhitespaceWidth
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kcttrackingattributename?language=objc)
    pub static kCTTrackingAttributeName: &'static CFString;
}

extern "C" {
    /// Controls ligature formation.
    ///
    ///
    /// Value must be a CFNumberRef. Default is int value 1. The ligature
    /// attribute determines what kinds of ligatures should be used when
    /// displaying the string. A value of 0 indicates that only ligatures
    /// essential for proper rendering of text should be used, 1
    /// indicates that standard ligatures should be used, and 2 indicates
    /// that all available ligatures should be used. Which ligatures are
    /// standard depends on the script and possibly the font. Arabic
    /// text, for example, requires ligatures for many character
    /// sequences, but has a rich set of additional ligatures that
    /// combine characters. English text has no essential ligatures, and
    /// typically has only two standard ligatures, those for "fi" and
    /// "fl" -- all others being considered more advanced or fancy.
    ///
    /// On iOS releases prior to 6.0 essential ligatures are applied
    /// if the font contains glyphs for any of U+FB00 through U+FB04 and
    /// the font lacks AAT or OpenType shaping tables, but as of 6.0
    /// shaping tables (or the lack thereof) are treated as definitive.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctligatureattributename?language=objc)
    pub static kCTLigatureAttributeName: &'static CFString;
}

extern "C" {
    /// The foreground color.
    ///
    ///
    /// Value must be a CGColorRef. Default value is black.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctforegroundcolorattributename?language=objc)
    pub static kCTForegroundColorAttributeName: &'static CFString;
}

extern "C" {
    /// The background color.
    ///
    ///
    /// Value must be a CGColorRef. Default is no background color.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctbackgroundcolorattributename?language=objc)
    pub static kCTBackgroundColorAttributeName: &'static CFString;
}

extern "C" {
    /// A CTParagraphStyle object which is used to specify things like
    /// line alignment, tab rulers, writing direction, etc.
    ///
    ///
    /// Value must be a CTParagraphStyleRef. Default is an empty
    /// CTParagraphStyle object: see CTParagraphStyle.h for more
    /// information. The value of this attribute must be uniform over
    /// the range of any paragraphs to which it is applied.
    ///
    ///
    /// See also: CFStringGetParagraphBounds
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctparagraphstyleattributename?language=objc)
    pub static kCTParagraphStyleAttributeName: &'static CFString;
}

extern "C" {
    /// The stroke width.
    ///
    ///
    /// Value must be a CFNumberRef. Default value is 0.0, or no stroke.
    /// This attribute, interpreted as a percentage of font point size,
    /// controls the text drawing mode: positive values effect drawing
    /// with stroke only; negative values are for stroke and fill. A
    /// typical value for outlined text is 3.0.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctstrokewidthattributename?language=objc)
    pub static kCTStrokeWidthAttributeName: &'static CFString;
}

extern "C" {
    /// The stroke color.
    ///
    ///
    /// Value must be a CGColorRef. Default is the foreground color.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctstrokecolorattributename?language=objc)
    pub static kCTStrokeColorAttributeName: &'static CFString;
}

extern "C" {
    /// Allows the setting of an underline to be applied at render
    /// time.
    ///
    ///
    /// Value must be a CFNumberRef. Default is kCTUnderlineStyleNone.
    /// Set a value of something other than kCTUnderlineStyleNone to draw
    /// an underline. In addition, the CTUnderlineStyleModifiers can be
    /// used to modify the look of the underline. The underline color
    /// will be determined by the text's foreground color unless
    /// otherwise specified by kCTUnderlineColorAttributeName.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctunderlinestyleattributename?language=objc)
    pub static kCTUnderlineStyleAttributeName: &'static CFString;
}

extern "C" {
    /// Controls vertical text positioning.
    ///
    ///
    /// Value must be a CFNumberRef. Default is int value 0. If supported
    /// by the specified font, a value of 1 enables superscripting and a
    /// value of -1 enables subscripting.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctsuperscriptattributename?language=objc)
    pub static kCTSuperscriptAttributeName: &'static CFString;
}

extern "C" {
    /// The underline color.
    ///
    ///
    /// Value must be a CGColorRef. Default is the foreground color.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctunderlinecolorattributename?language=objc)
    pub static kCTUnderlineColorAttributeName: &'static CFString;
}

extern "C" {
    /// Controls glyph orientation.
    ///
    ///
    /// Value must be a CFBooleanRef. Default is false. A value of false
    /// indicates that horizontal glyph forms are to be used, true
    /// indicates that vertical glyph forms are to be used.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctverticalformsattributename?language=objc)
    pub static kCTVerticalFormsAttributeName: &'static CFString;
}

extern "C" {
    /// Setting text in tate-chu-yoko form (horizontal numerals in vertical text).
    ///
    ///
    /// Value must be a CFNumberRef. Default is int value 0. A value of 1
    /// to 4 indicates the number of digits or letters to set in horizontal
    /// form. This is to apply the correct feature settings for the text.
    /// This attribute only works when kCTVerticalFormsAttributeName is set
    /// to true.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kcthorizontalinverticalformsattributename?language=objc)
    pub static kCTHorizontalInVerticalFormsAttributeName: &'static CFString;
}

extern "C" {
    /// Allows the use of unencoded glyphs.
    ///
    ///
    /// Value must be a CTGlyphInfoRef. The glyph specified by this
    /// CTGlyphInfo object is assigned to the entire attribute range,
    /// provided that its contents match the specified base string and
    /// that the specified glyph is available in the font specified by
    /// kCTFontAttributeName. See CTGlyphInfo.h for more information.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctglyphinfoattributename?language=objc)
    pub static kCTGlyphInfoAttributeName: &'static CFString;
}

extern "C" {
    /// Controls glyph selection.
    ///
    ///
    /// Value must be a CFNumberRef. Default is value is 0 (disabled).
    /// A non-zero value is interpreted as an SFNT kCharacterShapeType
    /// selector + 1; see SFNTLayoutTypes.h for selectors. For example,
    /// an attribute value of 1 corresponds to kTraditionalCharactersSelector.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctcharactershapeattributename?language=objc)
    pub static kCTCharacterShapeAttributeName: &'static CFString;
}

extern "C" {
    /// Specifies text language.
    ///
    ///
    /// Value must be a CFStringRef containing a locale identifier. Default
    /// is unset. When this attribute is set to a valid identifier, it will
    /// be used to select localized glyphs (if supported by the font) and
    /// locale-specific line breaking rules.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctlanguageattributename?language=objc)
    pub static kCTLanguageAttributeName: &'static CFString;
}

extern "C" {
    /// Allows customization of certain aspects of a range of text's
    /// appearance.
    ///
    ///
    /// Value must be a CTRunDelegateRef. The values returned by the
    /// embedded object for an attribute range apply to each glyph
    /// resulting from the text in that range. Because an embedded object
    /// is only a display-time modification, care should be taken to
    /// avoid applying this attribute to a range of text with complex
    /// behavior, such as a change of writing direction, combining marks,
    /// etc. Consequently, it is recommended that this attribute be
    /// applied to a range containing the single character U+FFFC. See
    /// CTRunDelegate.h for more information.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctrundelegateattributename?language=objc)
    pub static kCTRunDelegateAttributeName: &'static CFString;
}

/// Underline style specifiers.
///
///
/// These underline type specifiers can be applied to the value set
/// with the kCTUnderlineStyleAttributeName attribute to tell
/// CoreText that you want a different underline style.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/ctunderlinestyle?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CTUnderlineStyle(pub i32);
bitflags::bitflags! {
    impl CTUnderlineStyle: i32 {
        #[doc(alias = "kCTUnderlineStyleNone")]
        const None = 0x00;
        #[doc(alias = "kCTUnderlineStyleSingle")]
        const Single = 0x01;
        #[doc(alias = "kCTUnderlineStyleThick")]
        const Thick = 0x02;
        #[doc(alias = "kCTUnderlineStyleDouble")]
        const Double = 0x09;
    }
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CTUnderlineStyle {
    const ENCODING: Encoding = i32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CTUnderlineStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Underline style modifiers.
///
///
/// Set these bits with the CTUnderlineStyle that you set with the
/// kCTUnderlineStyleAttributeName attribute to modify how the
/// underline will be drawn.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/ctunderlinestylemodifiers?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CTUnderlineStyleModifiers(pub i32);
bitflags::bitflags! {
    impl CTUnderlineStyleModifiers: i32 {
        #[doc(alias = "kCTUnderlinePatternSolid")]
        const PatternSolid = 0x0000;
        #[doc(alias = "kCTUnderlinePatternDot")]
        const PatternDot = 0x0100;
        #[doc(alias = "kCTUnderlinePatternDash")]
        const PatternDash = 0x0200;
        #[doc(alias = "kCTUnderlinePatternDashDot")]
        const PatternDashDot = 0x0300;
        #[doc(alias = "kCTUnderlinePatternDashDotDot")]
        const PatternDashDotDot = 0x0400;
    }
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CTUnderlineStyleModifiers {
    const ENCODING: Encoding = i32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CTUnderlineStyleModifiers {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// Key to reference a baseline class override.
    ///
    ///
    /// Value must be one of the kCTBaselineClass constants. Normally,
    /// glyphs on the line will be assigned baseline classes according to
    /// the 'bsln' or 'BASE' table in the font. This attribute may be
    /// used to change this assignment.
    ///
    ///
    /// See also: kCTBaselineClassRoman
    ///
    /// See also: kCTBaselineClassIdeographicCentered
    ///
    /// See also: kCTBaselineClassIdeographicLow
    ///
    /// See also: kCTBaselineClassIdeographicHigh
    ///
    /// See also: kCTBaselineClassHanging
    ///
    /// See also: kCTBaselineClassMath
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctbaselineclassattributename?language=objc)
    pub static kCTBaselineClassAttributeName: &'static CFString;
}

extern "C" {
    /// Key to reference a baseline info dictionary.
    ///
    ///
    /// Value must be a CFDictionaryRef. Normally, baseline offsets will
    /// be assigned based on the 'bsln' or 'BASE' table in the font. This
    /// attribute may be used to assign different offsets. Each key in
    /// the dictionary is one of the kCTBaselineClass constants and the
    /// value is a CFNumberRef of the baseline offset in points. You only
    /// need to specify the offsets you wish to change.
    ///
    ///
    /// See also: kCTBaselineClassRoman
    ///
    /// See also: kCTBaselineClassIdeographicCentered
    ///
    /// See also: kCTBaselineClassIdeographicLow
    ///
    /// See also: kCTBaselineClassIdeographicHigh
    ///
    /// See also: kCTBaselineClassHanging
    ///
    /// See also: kCTBaselineClassMath
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctbaselineinfoattributename?language=objc)
    pub static kCTBaselineInfoAttributeName: &'static CFString;
}

extern "C" {
    /// Key to reference a baseline info dictionary for the reference baseline.
    ///
    ///
    /// Value must be a CFDictionaryRef. All glyphs in a run are assigned
    /// a baseline class and then aligned to the offset for that class in
    /// the reference baseline baseline info. See the discussion of
    /// kCTBaselineInfoAttributeName for information about the contents
    /// of the dictionary. You can also use the kCTBaselineReferenceFont
    /// key to specify that the baseline offsets of a particular
    /// CTFontRef should be used as the reference offsets.
    ///
    ///
    /// See also: kCTBaselineClassRoman
    ///
    /// See also: kCTBaselineClassIdeographicCentered
    ///
    /// See also: kCTBaselineClassIdeographicLow
    ///
    /// See also: kCTBaselineClassIdeographicHigh
    ///
    /// See also: kCTBaselineClassHanging
    ///
    /// See also: kCTBaselineClassMath
    ///
    /// See also: kCTBaselineReferenceFont
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctbaselinereferenceinfoattributename?language=objc)
    pub static kCTBaselineReferenceInfoAttributeName: &'static CFString;
}

extern "C" {
    /// Controls vertical text positioning.
    ///
    ///
    /// Value must be a CFNumberRef float. Default is standard positioning.
    /// The baseline attribute indicates how many points the characters
    /// should be shifted perpendicular to their baseline. A positive
    /// baseline value indicates a shift above (or to the right for vertical
    /// text) the text baseline and a negative baseline value indicates a
    /// shift below (or to the left for vertical text) the text baseline.
    /// If this value is set to 0.0, no baseline shift will be performed.
    ///
    ///
    /// See also: NSBaselineOffsetAttributeName
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctbaselineoffsetattributename?language=objc)
    pub static kCTBaselineOffsetAttributeName: &'static CFString;
}

extern "C" {
    /// Specifies a bidirectional override or embedding.
    ///
    ///
    /// Value must be a CFArray of CFNumberRefs, each of which should
    /// have a value of either kCTWritingDirectionLeftToRight or
    /// kCTWritingDirectionRightToLeft, plus one of
    /// kCTWritingDirectionEmbedding or kCTWritingDirectionOverride.
    /// This array represents a sequence of nested bidirectional
    /// embeddings or overrides, in order from outermost to innermost,
    /// with (kCTWritingDirectionLeftToRight | kCTWritingDirectionEmbedding)
    /// corresponding to a LRE/PDF pair in plain text or
    /// <span dir="ltr">
    /// </span>
    /// in HTML, (kCTWritingDirectionRightToLeft
    /// | kCTWritingDirectionEmbedding) corresponding to a RLE/PDF
    /// pair in plain text or a
    /// <span dir="rtl">
    /// </span>
    /// in HTML,
    /// (kCTWritingDirectionLeftToRight | kCTWritingDirectionOverride)
    /// corresponding to a LRO/PDF pair in plain text or
    /// <bdo
    /// dir="ltr">
    /// </bdo
    /// > in HTML, and (kCTWritingDirectionRightToLeft
    /// | kCTWritingDirectionOverride) corresponding to a RLO/PDF
    /// pair in plain text or
    /// <bdo
    /// dir="rtl">
    /// </bdo
    /// > in HTML.
    ///
    ///
    /// See also: kCTWritingDirectionLeftToRight
    ///
    /// See also: kCTWritingDirectionRightToLeft
    ///
    /// See also: kCTWritingDirectionEmbedding
    ///
    /// See also: kCTWritingDirectionOverride
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctwritingdirectionattributename?language=objc)
    pub static kCTWritingDirectionAttributeName: &'static CFString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctwritingdirectionembedding?language=objc)
pub const kCTWritingDirectionEmbedding: c_uint = 0 << 1;
/// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctwritingdirectionoverride?language=objc)
pub const kCTWritingDirectionOverride: c_uint = 1 << 1;

extern "C" {
    /// Key to reference a CTRubyAnnotation.
    ///
    ///
    /// Value must be a CTRubyAnnotationRef. See CTRubyAnnotation.h for
    /// more information.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctrubyannotationattributename?language=objc)
    pub static kCTRubyAnnotationAttributeName: &'static CFString;
}

extern "C" {
    /// Provide the image for an emoji-like text attachment.
    ///
    ///
    /// The attribute value must be an object conforming to the CTAdaptiveImageProviding protocol.
    /// The range this attribute is applied to should be one or more U+FFFC characters, each of which will be drawn as the provided image,
    /// and the font attribute applied to that range will be used to determine properties such as point size.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctadaptiveimageproviderattributename?language=objc)
    pub static kCTAdaptiveImageProviderAttributeName: &'static CFString;
}
