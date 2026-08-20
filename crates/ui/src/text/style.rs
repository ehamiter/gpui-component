use std::path::PathBuf;
use std::sync::Arc;

use gpui::{App, Pixels, Rems, SharedString, StyleRefinement, px, rems};

use crate::ActiveTheme as _;
use crate::highlighter::HighlightTheme;

/// TextViewStyle used to customize the style for [`TextView`].
#[derive(Clone)]
pub struct TextViewStyle {
    /// Gap of each paragraphs, default is 1 rem.
    pub paragraph_gap: Rems,
    /// Base font size for headings, default is 14px.
    pub heading_base_font_size: Pixels,
    /// Function to calculate heading font size based on heading level (1-6).
    ///
    /// The first parameter is the heading level (1-6), the second parameter is the base font size.
    /// The second parameter is the base font size.
    pub heading_font_size: Option<Arc<dyn Fn(u8, Pixels) -> Pixels + Send + Sync + 'static>>,
    /// Highlight theme for code blocks. Default: [`HighlightTheme::default_light()`]
    pub highlight_theme: Arc<HighlightTheme>,
    /// The style refinement for code blocks.
    pub code_block: StyleRefinement,
    /// Font family for inline code spans (`` `code` ``).
    ///
    /// `None` uses the theme's `mono_font_family`, so inline code matches
    /// code blocks.
    pub inline_code_font_family: Option<SharedString>,
    /// Directory that relative image sources (Markdown `![]()` or HTML
    /// `<img src>`) are resolved against.
    ///
    /// `None` leaves a bare `image.png`-style source to fall through to
    /// gpui's own resolution, which treats it as an embedded-asset lookup
    /// rather than a file on disk. Set this to the directory of the
    /// document being rendered so relative image paths load from there.
    pub base_dir: Option<PathBuf>,
    pub is_dark: bool,
}

impl PartialEq for TextViewStyle {
    fn eq(&self, other: &Self) -> bool {
        self.paragraph_gap == other.paragraph_gap
            && self.heading_base_font_size == other.heading_base_font_size
            && self.highlight_theme == other.highlight_theme
            && self.inline_code_font_family == other.inline_code_font_family
            && self.base_dir == other.base_dir
    }
}

impl Default for TextViewStyle {
    fn default() -> Self {
        Self {
            paragraph_gap: rems(1.),
            heading_base_font_size: px(14.),
            heading_font_size: None,
            highlight_theme: HighlightTheme::default_light().clone(),
            code_block: StyleRefinement::default(),
            inline_code_font_family: None,
            base_dir: None,
            is_dark: false,
        }
    }
}

impl TextViewStyle {
    /// Set paragraph gap, default is 1 rem.
    pub fn paragraph_gap(mut self, gap: Rems) -> Self {
        self.paragraph_gap = gap;
        self
    }

    pub fn heading_font_size<F>(mut self, f: F) -> Self
    where
        F: Fn(u8, Pixels) -> Pixels + Send + Sync + 'static,
    {
        self.heading_font_size = Some(Arc::new(f));
        self
    }

    /// Set style for code blocks.
    pub fn code_block(mut self, style: StyleRefinement) -> Self {
        self.code_block = style;
        self
    }

    /// Set the font family used for inline code spans, default is the
    /// theme's `mono_font_family`.
    pub fn inline_code_font_family(mut self, font_family: impl Into<SharedString>) -> Self {
        self.inline_code_font_family = Some(font_family.into());
        self
    }

    /// The font family to render inline code spans with.
    pub(crate) fn inline_code_font(&self, cx: &App) -> SharedString {
        self.inline_code_font_family
            .clone()
            .unwrap_or_else(|| cx.theme().mono_font_family.clone())
    }

    /// Set the directory relative image sources are resolved against.
    pub fn base_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.base_dir = Some(dir.into());
        self
    }
}
