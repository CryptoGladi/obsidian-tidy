use super::MarkdownLexer;
use pulldown_cmark::Options as MarkOptions;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownLexerBuilder {
    options: MarkOptions,
}

impl Default for MarkdownLexerBuilder {
    fn default() -> Self {
        Self::all()
    }
}

impl MarkdownLexerBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            options: MarkOptions::empty(),
        }
    }

    #[must_use]
    pub fn all() -> Self {
        let mut options = MarkOptions::all();

        // Мы сделали свою реализацию callout
        options.set(MarkOptions::ENABLE_GFM, false);

        // Эта ломает парсинг и замедляет работу
        options.set(MarkOptions::ENABLE_SMART_PUNCTUATION, false);

        Self { options }
    }

    #[must_use]
    pub fn tables(mut self, enable: bool) -> Self {
        self.options.set(MarkOptions::ENABLE_TABLES, enable);
        self
    }

    #[must_use]
    pub fn strikethrough(mut self, enable: bool) -> Self {
        self.options.set(MarkOptions::ENABLE_STRIKETHROUGH, enable);
        self
    }

    #[must_use]
    pub fn tasklists(mut self, enable: bool) -> Self {
        self.options.set(MarkOptions::ENABLE_TASKLISTS, enable);
        self
    }

    #[must_use]
    pub fn frontmatter(mut self, enable: bool) -> Self {
        self.options
            .set(MarkOptions::ENABLE_YAML_STYLE_METADATA_BLOCKS, enable);

        self
    }

    #[must_use]
    pub fn old_footnotes(mut self, enable: bool) -> Self {
        self.options.set(MarkOptions::ENABLE_OLD_FOOTNOTES, enable);
        self
    }

    #[must_use]
    pub fn math(mut self, enable: bool) -> Self {
        self.options.set(MarkOptions::ENABLE_MATH, enable);
        self
    }

    #[must_use]
    pub fn definition_list(mut self, enable: bool) -> Self {
        self.options
            .set(MarkOptions::ENABLE_DEFINITION_LIST, enable);

        self
    }

    #[must_use]
    pub fn superscript(mut self, enable: bool) -> Self {
        self.options.set(MarkOptions::ENABLE_SUPERSCRIPT, enable);
        self
    }

    #[must_use]
    pub fn subscript(mut self, enable: bool) -> Self {
        self.options.set(MarkOptions::ENABLE_SUBSCRIPT, enable);
        self
    }

    #[must_use]
    pub fn wikilinks(mut self, enable: bool) -> Self {
        self.options.set(MarkOptions::ENABLE_WIKILINKS, enable);
        self
    }

    #[must_use]
    pub fn strict_markdown() -> Self {
        Self::new().tables(true).tasklists(true).old_footnotes(true)
    }

    #[must_use]
    #[expect(clippy::elidable_lifetime_names, reason = "чтобы было более понятно")]
    pub fn build<'input>(self, text: &'input str) -> MarkdownLexer<'input> {
        MarkdownLexer::new(text, self.options)
    }
}
