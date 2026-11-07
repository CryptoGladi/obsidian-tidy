use pulldown_cmark::Options as MarkOptions;

/// A utility struct that applies Obsidian-specific patches to `pulldown-cmark` options.
///
/// Obsidian uses a slightly customized flavor of Markdown. This struct encapsulates
/// the necessary deviations from the standard `pulldown-cmark` defaults to ensure
/// correct parsing and optimal performance for Obsidian documents.
pub struct ObsidianMarkdownPatches;

impl ObsidianMarkdownPatches {
    /// Applies all Obsidian-specific patches to the given `MarkOptions`.
    ///
    /// This is the recommended entry point for configuring the lexer for Obsidian files.
    /// It disables features that conflict with custom Obsidian syntax (like callouts)
    /// or introduce unnecessary performance overhead.
    ///
    /// Disabling this is required for [`crate::CalloutInterceptor`] to function correctly
    ///
    /// # Arguments
    ///
    /// * `options` - The base `pulldown_cmark::Options` to be patched.
    ///
    /// # Returns
    ///
    /// The patched `MarkOptions` instance, ready for use in the lexer.
    #[must_use]
    pub fn apply_all(mut options: MarkOptions) -> MarkOptions {
        options = Self::disable_gfm_callout_conflict(options);
        options = Self::disable_smart_punctuation(options);
        options
    }

    /// Disables GitHub Flavored Markdown (GFM) to prevent conflicts with custom callouts.
    ///
    /// Obsidian implements its own callout syntax (e.g., `> [!note]`). The standard GFM
    /// blockquote parsing can interfere with our custom `CalloutInterceptor`. Disabling
    /// GFM ensures that blockquotes are parsed in a raw format that our interceptor can
    /// reliably transform into callout AST nodes.
    ///
    /// # Arguments
    ///
    /// * `options` - The `MarkOptions` to modify.
    ///
    /// # Returns
    ///
    /// The modified `MarkOptions` with `ENABLE_GFM` set to `false`.
    #[must_use]
    pub fn disable_gfm_callout_conflict(mut options: MarkOptions) -> MarkOptions {
        options.set(MarkOptions::ENABLE_GFM, false);
        options
    }

    /// Disables smart punctuation to improve parsing performance and predictability.
    ///
    /// Smart punctuation (e.g., converting `--` to `—` or `"` to `“`) requires additional
    /// lookahead and state tracking in `pulldown-cmark`. For a linter/AST builder, this
    /// introduces unnecessary performance overhead and can occasionally cause edge-case
    /// parsing bugs. Disabling it ensures faster, more deterministic tokenization.
    ///
    /// # Arguments
    ///
    /// * `options` - The `MarkOptions` to modify.
    ///
    /// # Returns
    ///
    /// The modified `MarkOptions` with `ENABLE_SMART_PUNCTUATION` set to `false`.
    #[must_use]
    pub fn disable_smart_punctuation(mut options: MarkOptions) -> MarkOptions {
        options.set(MarkOptions::ENABLE_SMART_PUNCTUATION, false);
        options
    }
}
