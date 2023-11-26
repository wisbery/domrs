use crate::SvgAttribute;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CssProperty {
  AccentColor,
  AlignContent,
  AlignItems,
  AlignSelf,
  All,
  Animation,
  AnimationDelay,
  AnimationDirection,
  AnimationDuration,
  AnimationFillMode,
  AnimationIterationCount,
  AnimationName,
  AnimationPlayState,
  AnimationTimingFunction,
  AspectRatio,
  BackdropFilter,
  BackfaceVisibility,
  Background,
  BackgroundAttachment,
  BackgroundBlendMode,
  BackgroundClip,
  BackgroundColor,
  BackgroundImage,
  BackgroundOrigin,
  BackgroundPosition,
  BackgroundPositionX,
  BackgroundPositionY,
  BackgroundRepeat,
  BackgroundSize,
  BlockSize,
  Border,
  BorderBlock,
  BorderBlockColor,
  BorderBlockEnd,
  BorderBlockEndColor,
  BorderBlockEndStyle,
  BorderBlockEndWidth,
  BorderBlockStart,
  BorderBlockStartColor,
  BorderBlockStartStyle,
  BorderBlockStartWidth,
  BorderBlockStyle,
  BorderBlockWidth,
  BorderBottom,
  BorderBottomColor,
  BorderBottomLeftRadius,
  BorderBottomRightRadius,
  BorderBottomStyle,
  BorderBottomWidth,
  BorderCollapse,
  BorderColor,
  BorderEndEndRadius,
  BorderEndStartRadius,
  BorderImage,
  BorderImageOutset,
  BorderImageRepeat,
  BorderImageSlice,
  BorderImageSource,
  BorderImageWidth,
  BorderInline,
  BorderInlineColor,
  BorderInlineEnd,
  BorderInlineEndColor,
  BorderInlineEndStyle,
  BorderInlineEndWidth,
  BorderInlineStart,
  BorderInlineStartColor,
  BorderInlineStartStyle,
  BorderInlineStartWidth,
  BorderInlineStyle,
  BorderInlineWidth,
  BorderLeft,
  BorderLeftColor,
  BorderLeftStyle,
  BorderLeftWidth,
  BorderRadius,
  BorderRight,
  BorderRightColor,
  BorderRightStyle,
  BorderRightWidth,
  BorderSpacing,
  BorderStartEndRadius,
  BorderStartStartRadius,
  BorderStyle,
  BorderTop,
  BorderTopColor,
  BorderTopLeftRadius,
  BorderTopRightRadius,
  BorderTopStyle,
  BorderTopWidth,
  BorderWidth,
  Bottom,
  BoxDecorationBreak,
  BoxReflect,
  BoxShadow,
  BoxSizing,
  BreakAfter,
  BreakBefore,
  BreakInside,
  CaptionSide,
  CaretColor,
  AtCharset,
  Clear,
  Clip,
  ClipPath,
  Color,
  ColumnCount,
  ColumnFill,
  ColumnGap,
  ColumnRule,
  ColumnRuleColor,
  ColumnRuleStyle,
  ColumnRuleWidth,
  ColumnSpan,
  ColumnWidth,
  Columns,
  Content,
  CounterIncrement,
  CounterReset,
  CounterSet,
  Cursor,
  Direction,
  Display,
  EmptyCells,
  Filter,
  Flex,
  FlexBasis,
  FlexDirection,
  FlexFlow,
  FlexGrow,
  FlexShrink,
  FlexWrap,
  Float,
  Font,
  AtFontFace,
  FontFamily,
  FontFeatureSettings,
  AtFontFeatureValues,
  FontKerning,
  FontLanguageOverride,
  FontSize,
  FontSizeAdjust,
  FontStretch,
  FontStyle,
  FontSynthesis,
  FontVariant,
  FontVariantAlternates,
  FontVariantCaps,
  FontVariantEastAsian,
  FontVariantLigatures,
  FontVariantNumeric,
  FontVariantPosition,
  FontWeight,
  Gap,
  Grid,
  GridArea,
  GridAutoColumns,
  GridAutoFlow,
  GridAutoRows,
  GridColumn,
  GridColumnEnd,
  GridColumnGap,
  GridColumnStart,
  GridGap,
  GridRow,
  GridRowEnd,
  GridRowGap,
  GridRowStart,
  GridTemplate,
  GridTemplateAreas,
  GridTemplateColumns,
  GridTemplateRows,
  HangingPunctuation,
  Height,
  Hyphens,
  HypenateCharacter,
  ImageRendering,
  AtImport,
  InlineSize,
  Inset,
  InsetBlock,
  InsetBlockEnd,
  InsetBlockStart,
  InsetInline,
  InsetInlineEnd,
  InsetInlineStart,
  Isolation,
  JustifyContent,
  JustifyItems,
  JustifySelf,
  AtKeyframes,
  Left,
  LetterSpacing,
  LineBreak,
  LineHeight,
  ListStyle,
  ListStyleImage,
  ListStylePosition,
  ListStyleType,
  Margin,
  MarginBlock,
  MarginBlockEnd,
  MarginBlockStart,
  MarginBottom,
  MarginInline,
  MarginInlineEnd,
  MarginInlineStart,
  MarginLeft,
  MarginRight,
  MarginTop,
  Mask,
  MaskClip,
  MaskComposite,
  MaskImage,
  MaskMode,
  MaskOrigin,
  MaskPosition,
  MaskRepeat,
  MaskSize,
  MaskType,
  MaxHeight,
  MaxWidth,
  AtMedia,
  MaxBlockSize,
  MaxInlineSize,
  MinBlockSize,
  MinInlineSize,
  MinHeight,
  MinWidth,
  MixBlendMode,
  ObjectFit,
  ObjectPosition,
  Offset,
  OffsetAnchor,
  OffsetDistance,
  OffsetPath,
  OffsetRotate,
  Opacity,
  Order,
  Orphans,
  Outline,
  OutlineColor,
  OutlineOffset,
  OutlineStyle,
  OutlineWidth,
  Overflow,
  OverflowAnchor,
  OverflowWrap,
  OverflowX,
  OverflowY,
  OverscrollBehavior,
  OverscrollBehaviorBlock,
  OverscrollBehaviorInline,
  OverscrollBehaviorX,
  OverscrollBehaviorY,
  Padding,
  PaddingBlock,
  PaddingBlockEnd,
  PaddingBlockStart,
  PaddingBottom,
  PaddingInline,
  PaddingInlineEnd,
  PaddingInlineStart,
  PaddingLeft,
  PaddingRight,
  PaddingTop,
  PageBreakAfter,
  PageBreakBefore,
  PageBreakInside,
  PaintOrder,
  Perspective,
  PerspectiveOrigin,
  PlaceContent,
  PlaceItems,
  PlaceSelf,
  PointerEvents,
  Position,
  Quotes,
  Resize,
  Right,
  Rotate,
  RowGap,
  Scale,
  ScrollBehavior,
  ScrollMargin,
  ScrollMarginBlock,
  ScrollMarginBlockEnd,
  ScrollMarginBlockStart,
  ScrollMarginBottom,
  ScrollMarginInline,
  ScrollMarginInlineEnd,
  ScrollMarginInlineStart,
  ScrollMarginLeft,
  ScrollMarginRight,
  ScrollMarginTop,
  ScrollPadding,
  ScrollPaddingBlock,
  ScrollPaddingBlockEnd,
  ScrollPaddingBlockStart,
  ScrollPaddingBottom,
  ScrollPaddingInline,
  ScrollPaddingInlineEnd,
  ScrollPaddingInlineStart,
  ScrollPaddingLeft,
  ScrollPaddingRight,
  ScrollPaddingTop,
  ScrollSnapAlign,
  ScrollSnapStop,
  ScrollSnapType,
  ScrollbarColor,
  TabSize,
  TableLayout,
  TextAlign,
  TextAlignLast,
  TextCombineUpright,
  TextDecoration,
  TextDecorationColor,
  TextDecorationLine,
  TextDecorationStyle,
  TextDecorationThickness,
  TextEmphasis,
  TextEmphasisColor,
  TextEmphasisPosition,
  TextEmphasisStyle,
  TextIndent,
  TextJustify,
  TextOrientation,
  TextOverflow,
  TextShadow,
  TextTransform,
  TextUnderlineOffset,
  TextUnderlinePosition,
  Top,
  Transform,
  TransformOrigin,
  TransformStyle,
  Transition,
  TransitionDelay,
  TransitionDuration,
  TransitionProperty,
  TransitionTimingFunction,
  Translate,
  UnicodeBidi,
  UserSelect,
  VerticalAlign,
  Visibility,
  WhiteSpace,
  Widows,
  Width,
  WordBreak,
  WordSpacing,
  WordWrap,
  WritingMode,
  ZIndex,
  SvgAttribute(SvgAttribute),
}

impl Display for CssProperty {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        CssProperty::AccentColor => "accent-color".to_string(),
        CssProperty::AlignContent => "align-content".to_string(),
        CssProperty::AlignItems => "align-items".to_string(),
        CssProperty::AlignSelf => "align-self".to_string(),
        CssProperty::All => "all".to_string(),
        CssProperty::Animation => "animation".to_string(),
        CssProperty::AnimationDelay => "animation-delay".to_string(),
        CssProperty::AnimationDirection => "animation-direction".to_string(),
        CssProperty::AnimationDuration => "animation-duration".to_string(),
        CssProperty::AnimationFillMode => "animation-fill-mode".to_string(),
        CssProperty::AnimationIterationCount => "animation-iteration-count".to_string(),
        CssProperty::AnimationName => "animation-name".to_string(),
        CssProperty::AnimationPlayState => "animation-play-state".to_string(),
        CssProperty::AnimationTimingFunction => "animation-timing-function".to_string(),
        CssProperty::AspectRatio => "aspect-ratio".to_string(),
        CssProperty::BackdropFilter => "backdrop-filter".to_string(),
        CssProperty::BackfaceVisibility => "backface-visibility".to_string(),
        CssProperty::Background => "background".to_string(),
        CssProperty::BackgroundAttachment => "background-attachment".to_string(),
        CssProperty::BackgroundBlendMode => "background-blend-mode".to_string(),
        CssProperty::BackgroundClip => "background-clip".to_string(),
        CssProperty::BackgroundColor => "background-color".to_string(),
        CssProperty::BackgroundImage => "background-image".to_string(),
        CssProperty::BackgroundOrigin => "background-origin".to_string(),
        CssProperty::BackgroundPosition => "background-position".to_string(),
        CssProperty::BackgroundPositionX => "background-position-x".to_string(),
        CssProperty::BackgroundPositionY => "background-position-y".to_string(),
        CssProperty::BackgroundRepeat => "background-repeat".to_string(),
        CssProperty::BackgroundSize => "background-size".to_string(),
        CssProperty::BlockSize => "block-size".to_string(),
        CssProperty::Border => "border".to_string(),
        CssProperty::BorderBlock => "border-block".to_string(),
        CssProperty::BorderBlockColor => "border-block-color".to_string(),
        CssProperty::BorderBlockEnd => "border-block-end".to_string(),
        CssProperty::BorderBlockEndColor => "border-block-end-color".to_string(),
        CssProperty::BorderBlockEndStyle => "border-block-end-style".to_string(),
        CssProperty::BorderBlockEndWidth => "border-block-end-width".to_string(),
        CssProperty::BorderBlockStart => "border-block-start".to_string(),
        CssProperty::BorderBlockStartColor => "border-block-start-color".to_string(),
        CssProperty::BorderBlockStartStyle => "border-block-start-style".to_string(),
        CssProperty::BorderBlockStartWidth => "border-block-start-width".to_string(),
        CssProperty::BorderBlockStyle => "border-block-style".to_string(),
        CssProperty::BorderBlockWidth => "border-block-width".to_string(),
        CssProperty::BorderBottom => "border-bottom".to_string(),
        CssProperty::BorderBottomColor => "border-bottom-color".to_string(),
        CssProperty::BorderBottomLeftRadius => "border-bottom-left-radius".to_string(),
        CssProperty::BorderBottomRightRadius => "border-bottom-right-radius".to_string(),
        CssProperty::BorderBottomStyle => "border-bottom-style".to_string(),
        CssProperty::BorderBottomWidth => "border-bottom-width".to_string(),
        CssProperty::BorderCollapse => "border-collapse".to_string(),
        CssProperty::BorderColor => "border-color".to_string(),
        CssProperty::BorderEndEndRadius => "border-end-end-radius".to_string(),
        CssProperty::BorderEndStartRadius => "border-end-start-radius".to_string(),
        CssProperty::BorderImage => "border-image".to_string(),
        CssProperty::BorderImageOutset => "border-image-outset".to_string(),
        CssProperty::BorderImageRepeat => "border-image-repeat".to_string(),
        CssProperty::BorderImageSlice => "border-image-slice".to_string(),
        CssProperty::BorderImageSource => "border-image-source".to_string(),
        CssProperty::BorderImageWidth => "border-image-width".to_string(),
        CssProperty::BorderInline => "border-inline".to_string(),
        CssProperty::BorderInlineColor => "border-inline-color".to_string(),
        CssProperty::BorderInlineEnd => "border-inline-end".to_string(),
        CssProperty::BorderInlineEndColor => "border-inline-end-color".to_string(),
        CssProperty::BorderInlineEndStyle => "border-inline-end-style".to_string(),
        CssProperty::BorderInlineEndWidth => "border-inline-end-width".to_string(),
        CssProperty::BorderInlineStart => "border-inline-start".to_string(),
        CssProperty::BorderInlineStartColor => "border-inline-start-color".to_string(),
        CssProperty::BorderInlineStartStyle => "border-inline-start-style".to_string(),
        CssProperty::BorderInlineStartWidth => "border-inline-start-width".to_string(),
        CssProperty::BorderInlineStyle => "border-inline-style".to_string(),
        CssProperty::BorderInlineWidth => "border-inline-width".to_string(),
        CssProperty::BorderLeft => "border-left".to_string(),
        CssProperty::BorderLeftColor => "border-left-color".to_string(),
        CssProperty::BorderLeftStyle => "border-left-style".to_string(),
        CssProperty::BorderLeftWidth => "border-left-width".to_string(),
        CssProperty::BorderRadius => "border-radius".to_string(),
        CssProperty::BorderRight => "border-right".to_string(),
        CssProperty::BorderRightColor => "border-right-color".to_string(),
        CssProperty::BorderRightStyle => "border-right-style".to_string(),
        CssProperty::BorderRightWidth => "border-right-width".to_string(),
        CssProperty::BorderSpacing => "border-spacing".to_string(),
        CssProperty::BorderStartEndRadius => "border-start-end-radius".to_string(),
        CssProperty::BorderStartStartRadius => "border-start-start-radius".to_string(),
        CssProperty::BorderStyle => "border-style".to_string(),
        CssProperty::BorderTop => "border-top".to_string(),
        CssProperty::BorderTopColor => "border-top-color".to_string(),
        CssProperty::BorderTopLeftRadius => "border-top-left-radius".to_string(),
        CssProperty::BorderTopRightRadius => "border-top-right-radius".to_string(),
        CssProperty::BorderTopStyle => "border-top-style".to_string(),
        CssProperty::BorderTopWidth => "border-top-width".to_string(),
        CssProperty::BorderWidth => "border-width".to_string(),
        CssProperty::Bottom => "bottom".to_string(),
        CssProperty::BoxDecorationBreak => "box-decoration-break".to_string(),
        CssProperty::BoxReflect => "box-reflect".to_string(),
        CssProperty::BoxShadow => "box-shadow".to_string(),
        CssProperty::BoxSizing => "box-sizing".to_string(),
        CssProperty::BreakAfter => "break-after".to_string(),
        CssProperty::BreakBefore => "break-before".to_string(),
        CssProperty::BreakInside => "break-inside".to_string(),
        CssProperty::CaptionSide => "caption-side".to_string(),
        CssProperty::CaretColor => "caret-color".to_string(),
        CssProperty::AtCharset => "@charset".to_string(),
        CssProperty::Clear => "clear".to_string(),
        CssProperty::Clip => "clip".to_string(),
        CssProperty::ClipPath => "clip-path".to_string(),
        CssProperty::Color => "color".to_string(),
        CssProperty::ColumnCount => "column-count".to_string(),
        CssProperty::ColumnFill => "column-fill".to_string(),
        CssProperty::ColumnGap => "column-gap".to_string(),
        CssProperty::ColumnRule => "column-rule".to_string(),
        CssProperty::ColumnRuleColor => "column-rule-color".to_string(),
        CssProperty::ColumnRuleStyle => "column-rule-style".to_string(),
        CssProperty::ColumnRuleWidth => "column-rule-width".to_string(),
        CssProperty::ColumnSpan => "column-span".to_string(),
        CssProperty::ColumnWidth => "column-width".to_string(),
        CssProperty::Columns => "columns".to_string(),
        CssProperty::Content => "content".to_string(),
        CssProperty::CounterIncrement => "counter-increment".to_string(),
        CssProperty::CounterReset => "counter-reset".to_string(),
        CssProperty::CounterSet => "counter-set".to_string(),
        CssProperty::Cursor => "cursor".to_string(),
        CssProperty::Direction => "direction".to_string(),
        CssProperty::Display => "display".to_string(),
        CssProperty::EmptyCells => "empty-cells".to_string(),
        CssProperty::Filter => "filter".to_string(),
        CssProperty::Flex => "flex".to_string(),
        CssProperty::FlexBasis => "flex-basis".to_string(),
        CssProperty::FlexDirection => "flex-direction".to_string(),
        CssProperty::FlexFlow => "flex-flow".to_string(),
        CssProperty::FlexGrow => "flex-grow".to_string(),
        CssProperty::FlexShrink => "flex-shrink".to_string(),
        CssProperty::FlexWrap => "flex-wrap".to_string(),
        CssProperty::Float => "float".to_string(),
        CssProperty::Font => "font".to_string(),
        CssProperty::AtFontFace => "@font-face".to_string(),
        CssProperty::FontFamily => "font-family".to_string(),
        CssProperty::FontFeatureSettings => "font-feature-settings".to_string(),
        CssProperty::AtFontFeatureValues => "@font-feature-values".to_string(),
        CssProperty::FontKerning => "font-kerning".to_string(),
        CssProperty::FontLanguageOverride => "font-language-override".to_string(),
        CssProperty::FontSize => "font-size".to_string(),
        CssProperty::FontSizeAdjust => "font-size-adjust".to_string(),
        CssProperty::FontStretch => "font-stretch".to_string(),
        CssProperty::FontStyle => "font-style".to_string(),
        CssProperty::FontSynthesis => "font-synthesis".to_string(),
        CssProperty::FontVariant => "font-variant".to_string(),
        CssProperty::FontVariantAlternates => "font-variant-alternates".to_string(),
        CssProperty::FontVariantCaps => "font-variant-caps".to_string(),
        CssProperty::FontVariantEastAsian => "font-variant-east-asian".to_string(),
        CssProperty::FontVariantLigatures => "font-variant-ligatures".to_string(),
        CssProperty::FontVariantNumeric => "font-variant-numeric".to_string(),
        CssProperty::FontVariantPosition => "font-variant-position".to_string(),
        CssProperty::FontWeight => "font-weight".to_string(),
        CssProperty::Gap => "gap".to_string(),
        CssProperty::Grid => "grid".to_string(),
        CssProperty::GridArea => "grid-area".to_string(),
        CssProperty::GridAutoColumns => "grid-auto-columns".to_string(),
        CssProperty::GridAutoFlow => "grid-auto-flow".to_string(),
        CssProperty::GridAutoRows => "grid-auto-rows".to_string(),
        CssProperty::GridColumn => "grid-column".to_string(),
        CssProperty::GridColumnEnd => "grid-column-end".to_string(),
        CssProperty::GridColumnGap => "grid-column-gap".to_string(),
        CssProperty::GridColumnStart => "grid-column-start".to_string(),
        CssProperty::GridGap => "grid-gap".to_string(),
        CssProperty::GridRow => "grid-row".to_string(),
        CssProperty::GridRowEnd => "grid-row-end".to_string(),
        CssProperty::GridRowGap => "grid-row-gap".to_string(),
        CssProperty::GridRowStart => "grid-row-start".to_string(),
        CssProperty::GridTemplate => "grid-template".to_string(),
        CssProperty::GridTemplateAreas => "grid-template-areas".to_string(),
        CssProperty::GridTemplateColumns => "grid-template-columns".to_string(),
        CssProperty::GridTemplateRows => "grid-template-rows".to_string(),
        CssProperty::HangingPunctuation => "hanging-punctuation".to_string(),
        CssProperty::Height => "height".to_string(),
        CssProperty::Hyphens => "hyphens".to_string(),
        CssProperty::HypenateCharacter => "hypenate-character".to_string(),
        CssProperty::ImageRendering => "image-rendering".to_string(),
        CssProperty::AtImport => "@import".to_string(),
        CssProperty::InlineSize => "inline-size".to_string(),
        CssProperty::Inset => "inset".to_string(),
        CssProperty::InsetBlock => "inset-block".to_string(),
        CssProperty::InsetBlockEnd => "inset-block-end".to_string(),
        CssProperty::InsetBlockStart => "inset-block-start".to_string(),
        CssProperty::InsetInline => "inset-inline".to_string(),
        CssProperty::InsetInlineEnd => "inset-inline-end".to_string(),
        CssProperty::InsetInlineStart => "inset-inline-start".to_string(),
        CssProperty::Isolation => "isolation".to_string(),
        CssProperty::JustifyContent => "justify-content".to_string(),
        CssProperty::JustifyItems => "justify-items".to_string(),
        CssProperty::JustifySelf => "justify-self".to_string(),
        CssProperty::AtKeyframes => "@keyframes".to_string(),
        CssProperty::Left => "left".to_string(),
        CssProperty::LetterSpacing => "letter-spacing".to_string(),
        CssProperty::LineBreak => "line-break".to_string(),
        CssProperty::LineHeight => "line-height".to_string(),
        CssProperty::ListStyle => "list-style".to_string(),
        CssProperty::ListStyleImage => "list-style-image".to_string(),
        CssProperty::ListStylePosition => "list-style-position".to_string(),
        CssProperty::ListStyleType => "list-style-type".to_string(),
        CssProperty::Margin => "margin".to_string(),
        CssProperty::MarginBlock => "margin-block".to_string(),
        CssProperty::MarginBlockEnd => "margin-block-end".to_string(),
        CssProperty::MarginBlockStart => "margin-block-start".to_string(),
        CssProperty::MarginBottom => "margin-bottom".to_string(),
        CssProperty::MarginInline => "margin-inline".to_string(),
        CssProperty::MarginInlineEnd => "margin-inline-end".to_string(),
        CssProperty::MarginInlineStart => "margin-inline-start".to_string(),
        CssProperty::MarginLeft => "margin-left".to_string(),
        CssProperty::MarginRight => "margin-right".to_string(),
        CssProperty::MarginTop => "margin-top".to_string(),
        CssProperty::Mask => "mask".to_string(),
        CssProperty::MaskClip => "mask-clip".to_string(),
        CssProperty::MaskComposite => "mask-composite".to_string(),
        CssProperty::MaskImage => "mask-image".to_string(),
        CssProperty::MaskMode => "mask-mode".to_string(),
        CssProperty::MaskOrigin => "mask-origin".to_string(),
        CssProperty::MaskPosition => "mask-position".to_string(),
        CssProperty::MaskRepeat => "mask-repeat".to_string(),
        CssProperty::MaskSize => "mask-size".to_string(),
        CssProperty::MaskType => "mask-type".to_string(),
        CssProperty::MaxHeight => "max-height".to_string(),
        CssProperty::MaxWidth => "max-width".to_string(),
        CssProperty::AtMedia => "@media".to_string(),
        CssProperty::MaxBlockSize => "max-block-size".to_string(),
        CssProperty::MaxInlineSize => "max-inline-size".to_string(),
        CssProperty::MinBlockSize => "min-block-size".to_string(),
        CssProperty::MinInlineSize => "min-inline-size".to_string(),
        CssProperty::MinHeight => "min-height".to_string(),
        CssProperty::MinWidth => "min-width".to_string(),
        CssProperty::MixBlendMode => "mix-blend-mode".to_string(),
        CssProperty::ObjectFit => "object-fit".to_string(),
        CssProperty::ObjectPosition => "object-position".to_string(),
        CssProperty::Offset => "offset".to_string(),
        CssProperty::OffsetAnchor => "offset-anchor".to_string(),
        CssProperty::OffsetDistance => "offset-distance".to_string(),
        CssProperty::OffsetPath => "offset-path".to_string(),
        CssProperty::OffsetRotate => "offset-rotate".to_string(),
        CssProperty::Opacity => "opacity".to_string(),
        CssProperty::Order => "order".to_string(),
        CssProperty::Orphans => "orphans".to_string(),
        CssProperty::Outline => "outline".to_string(),
        CssProperty::OutlineColor => "outline-color".to_string(),
        CssProperty::OutlineOffset => "outline-offset".to_string(),
        CssProperty::OutlineStyle => "outline-style".to_string(),
        CssProperty::OutlineWidth => "outline-width".to_string(),
        CssProperty::Overflow => "overflow".to_string(),
        CssProperty::OverflowAnchor => "overflow-anchor".to_string(),
        CssProperty::OverflowWrap => "overflow-wrap".to_string(),
        CssProperty::OverflowX => "overflow-x".to_string(),
        CssProperty::OverflowY => "overflow-y".to_string(),
        CssProperty::OverscrollBehavior => "overscroll-behavior".to_string(),
        CssProperty::OverscrollBehaviorBlock => "overscroll-behavior-block".to_string(),
        CssProperty::OverscrollBehaviorInline => "overscroll-behavior-inline".to_string(),
        CssProperty::OverscrollBehaviorX => "overscroll-behavior-x".to_string(),
        CssProperty::OverscrollBehaviorY => "overscroll-behavior-y".to_string(),
        CssProperty::Padding => "padding".to_string(),
        CssProperty::PaddingBlock => "padding-block".to_string(),
        CssProperty::PaddingBlockEnd => "padding-block-end".to_string(),
        CssProperty::PaddingBlockStart => "padding-block-start".to_string(),
        CssProperty::PaddingBottom => "padding-bottom".to_string(),
        CssProperty::PaddingInline => "padding-inline".to_string(),
        CssProperty::PaddingInlineEnd => "padding-inline-end".to_string(),
        CssProperty::PaddingInlineStart => "padding-inline-start".to_string(),
        CssProperty::PaddingLeft => "padding-left".to_string(),
        CssProperty::PaddingRight => "padding-right".to_string(),
        CssProperty::PaddingTop => "padding-top".to_string(),
        CssProperty::PageBreakAfter => "page-break-after".to_string(),
        CssProperty::PageBreakBefore => "page-break-before".to_string(),
        CssProperty::PageBreakInside => "page-break-inside".to_string(),
        CssProperty::PaintOrder => "paint-order".to_string(),
        CssProperty::Perspective => "perspective".to_string(),
        CssProperty::PerspectiveOrigin => "perspective-origin".to_string(),
        CssProperty::PlaceContent => "place-content".to_string(),
        CssProperty::PlaceItems => "place-items".to_string(),
        CssProperty::PlaceSelf => "place-self".to_string(),
        CssProperty::PointerEvents => "pointer-events".to_string(),
        CssProperty::Position => "position".to_string(),
        CssProperty::Quotes => "quotes".to_string(),
        CssProperty::Resize => "resize".to_string(),
        CssProperty::Right => "right".to_string(),
        CssProperty::Rotate => "rotate".to_string(),
        CssProperty::RowGap => "row-gap".to_string(),
        CssProperty::Scale => "scale".to_string(),
        CssProperty::ScrollBehavior => "scroll-behavior".to_string(),
        CssProperty::ScrollMargin => "scroll-margin".to_string(),
        CssProperty::ScrollMarginBlock => "scroll-margin-block".to_string(),
        CssProperty::ScrollMarginBlockEnd => "scroll-margin-block-end".to_string(),
        CssProperty::ScrollMarginBlockStart => "scroll-margin-block-start".to_string(),
        CssProperty::ScrollMarginBottom => "scroll-margin-bottom".to_string(),
        CssProperty::ScrollMarginInline => "scroll-margin-inline".to_string(),
        CssProperty::ScrollMarginInlineEnd => "scroll-margin-inline-end".to_string(),
        CssProperty::ScrollMarginInlineStart => "scroll-margin-inline-start".to_string(),
        CssProperty::ScrollMarginLeft => "scroll-margin-left".to_string(),
        CssProperty::ScrollMarginRight => "scroll-margin-right".to_string(),
        CssProperty::ScrollMarginTop => "scroll-margin-top".to_string(),
        CssProperty::ScrollPadding => "scroll-padding".to_string(),
        CssProperty::ScrollPaddingBlock => "scroll-padding-block".to_string(),
        CssProperty::ScrollPaddingBlockEnd => "scroll-padding-block-end".to_string(),
        CssProperty::ScrollPaddingBlockStart => "scroll-padding-block-start".to_string(),
        CssProperty::ScrollPaddingBottom => "scroll-padding-bottom".to_string(),
        CssProperty::ScrollPaddingInline => "scroll-padding-inline".to_string(),
        CssProperty::ScrollPaddingInlineEnd => "scroll-padding-inline-end".to_string(),
        CssProperty::ScrollPaddingInlineStart => "scroll-padding-inline-start".to_string(),
        CssProperty::ScrollPaddingLeft => "scroll-padding-left".to_string(),
        CssProperty::ScrollPaddingRight => "scroll-padding-right".to_string(),
        CssProperty::ScrollPaddingTop => "scroll-padding-top".to_string(),
        CssProperty::ScrollSnapAlign => "scroll-snap-align".to_string(),
        CssProperty::ScrollSnapStop => "scroll-snap-stop".to_string(),
        CssProperty::ScrollSnapType => "scroll-snap-type".to_string(),
        CssProperty::ScrollbarColor => "scrollbar-color".to_string(),
        CssProperty::TabSize => "tab-size".to_string(),
        CssProperty::TableLayout => "table-layout".to_string(),
        CssProperty::TextAlign => "text-align".to_string(),
        CssProperty::TextAlignLast => "text-align-last".to_string(),
        CssProperty::TextCombineUpright => "text-combine-upright".to_string(),
        CssProperty::TextDecoration => "text-decoration".to_string(),
        CssProperty::TextDecorationColor => "text-decoration-color".to_string(),
        CssProperty::TextDecorationLine => "text-decoration-line".to_string(),
        CssProperty::TextDecorationStyle => "text-decoration-style".to_string(),
        CssProperty::TextDecorationThickness => "text-decoration-thickness".to_string(),
        CssProperty::TextEmphasis => "text-emphasis".to_string(),
        CssProperty::TextEmphasisColor => "text-emphasis-color".to_string(),
        CssProperty::TextEmphasisPosition => "text-emphasis-position".to_string(),
        CssProperty::TextEmphasisStyle => "text-emphasis-style".to_string(),
        CssProperty::TextIndent => "text-indent".to_string(),
        CssProperty::TextJustify => "text-justify".to_string(),
        CssProperty::TextOrientation => "text-orientation".to_string(),
        CssProperty::TextOverflow => "text-overflow".to_string(),
        CssProperty::TextShadow => "text-shadow".to_string(),
        CssProperty::TextTransform => "text-transform".to_string(),
        CssProperty::TextUnderlineOffset => "text-underline-offset".to_string(),
        CssProperty::TextUnderlinePosition => "text-underline-position".to_string(),
        CssProperty::Top => "top".to_string(),
        CssProperty::Transform => "transform".to_string(),
        CssProperty::TransformOrigin => "transform-origin".to_string(),
        CssProperty::TransformStyle => "transform-style".to_string(),
        CssProperty::Transition => "transition".to_string(),
        CssProperty::TransitionDelay => "transition-delay".to_string(),
        CssProperty::TransitionDuration => "transition-duration".to_string(),
        CssProperty::TransitionProperty => "transition-property".to_string(),
        CssProperty::TransitionTimingFunction => "transition-timing-function".to_string(),
        CssProperty::Translate => "translate".to_string(),
        CssProperty::UnicodeBidi => "unicode-bidi".to_string(),
        CssProperty::UserSelect => "user-select".to_string(),
        CssProperty::VerticalAlign => "vertical-align".to_string(),
        CssProperty::Visibility => "visibility".to_string(),
        CssProperty::WhiteSpace => "white-space".to_string(),
        CssProperty::Widows => "widows".to_string(),
        CssProperty::Width => "width".to_string(),
        CssProperty::WordBreak => "word-break".to_string(),
        CssProperty::WordSpacing => "word-spacing".to_string(),
        CssProperty::WordWrap => "word-wrap".to_string(),
        CssProperty::WritingMode => "writing-mode".to_string(),
        CssProperty::ZIndex => "z-index".to_string(),
        CssProperty::SvgAttribute(attr) => attr.to_string(),
      }
    )
  }
}

impl From<SvgAttribute> for CssProperty {
  fn from(value: SvgAttribute) -> Self {
    CssProperty::SvgAttribute(value)
  }
}
