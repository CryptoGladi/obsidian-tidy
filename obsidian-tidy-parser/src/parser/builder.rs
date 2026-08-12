use super::Parser;
use pulldown_cmark::{Options as MarkOptions, Parser as MarkParser};

pub struct ParserBuilder {
    enable_tables: bool,
    enable_strikethrough: bool,
    enable_tasklists: bool,
    enable_smart_punctuation: bool,
    enable_yaml_style_metadata_blocks: bool,
    enable_old_footnotes: bool,
    enable_math: bool,
    enable_gfm: bool,
    enable_definition_list: bool,
    enable_superscript: bool,
    enable_subscript: bool,
    enable_wikilinks: bool,
}

impl Default for ParserBuilder {
    fn default() -> Self {
        Self::all()
    }
}

impl ParserBuilder {
    pub fn new() -> Self {
        Self {
            enable_tables: false,
            enable_strikethrough: false,
            enable_tasklists: false,
            enable_smart_punctuation: false,
            enable_yaml_style_metadata_blocks: false,
            enable_old_footnotes: false,
            enable_math: false,
            enable_gfm: false,
            enable_definition_list: false,
            enable_superscript: false,
            enable_subscript: false,
            enable_wikilinks: false,
        }
    }

    pub fn all() -> Self {
        Self {
            enable_tables: true,
            enable_strikethrough: true,
            enable_tasklists: true,
            enable_smart_punctuation: true,
            enable_yaml_style_metadata_blocks: true,
            enable_old_footnotes: true,
            enable_math: true,
            enable_gfm: true,
            enable_definition_list: true,
            enable_superscript: true,
            enable_subscript: true,
            enable_wikilinks: true,
        }
    }

    pub fn tables(mut self, enable: bool) -> Self {
        self.enable_tables = enable;
        self
    }

    pub fn strikethrough(mut self, enable: bool) -> Self {
        self.enable_strikethrough = enable;
        self
    }

    pub fn tasklists(mut self, enable: bool) -> Self {
        self.enable_tasklists = enable;
        self
    }

    pub fn smart_punctuation(mut self, enable: bool) -> Self {
        self.enable_smart_punctuation = enable;
        self
    }

    pub fn frontmatter(mut self, enable: bool) -> Self {
        self.enable_yaml_style_metadata_blocks = enable;
        self
    }

    pub fn old_footnotes(mut self, enable: bool) -> Self {
        self.enable_old_footnotes = enable;
        self
    }

    pub fn math(mut self, enable: bool) -> Self {
        self.enable_math = enable;
        self
    }

    pub fn gfm(mut self, enable: bool) -> Self {
        self.enable_gfm = enable;
        self
    }

    pub fn definition_list(mut self, enable: bool) -> Self {
        self.enable_definition_list = enable;
        self
    }

    pub fn superscript(mut self, enable: bool) -> Self {
        self.enable_superscript = enable;
        self
    }

    pub fn subscript(mut self, enable: bool) -> Self {
        self.enable_subscript = enable;
        self
    }

    pub fn wikilinks(mut self, enable: bool) -> Self {
        self.enable_wikilinks = enable;
        self
    }

    pub fn strict_markdown() -> Self {
        Self::new().tables(true).tasklists(true).old_footnotes(true)
    }

    // Построение парсера
    pub fn build<'input>(self, text: &'input str) -> Parser<'input> {
        let options = MarkOptions::from(self);

        Parser {
            inner: MarkParser::new_ext(text, options).into_offset_iter(),
        }
    }
}

impl From<ParserBuilder> for MarkOptions {
    fn from(builder: ParserBuilder) -> Self {
        let mut options = MarkOptions::empty();

        if builder.enable_tables {
            options.insert(MarkOptions::ENABLE_TABLES);
        }
        if builder.enable_strikethrough {
            options.insert(MarkOptions::ENABLE_STRIKETHROUGH);
        }
        if builder.enable_tasklists {
            options.insert(MarkOptions::ENABLE_TASKLISTS);
        }
        if builder.enable_smart_punctuation {
            options.insert(MarkOptions::ENABLE_SMART_PUNCTUATION);
        }
        if builder.enable_yaml_style_metadata_blocks {
            options.insert(MarkOptions::ENABLE_YAML_STYLE_METADATA_BLOCKS);
        }
        if builder.enable_old_footnotes {
            options.insert(MarkOptions::ENABLE_OLD_FOOTNOTES);
        }
        if builder.enable_math {
            options.insert(MarkOptions::ENABLE_MATH);
        }
        if builder.enable_gfm {
            options.insert(MarkOptions::ENABLE_GFM);
        }
        if builder.enable_definition_list {
            options.insert(MarkOptions::ENABLE_DEFINITION_LIST);
        }
        if builder.enable_superscript {
            options.insert(MarkOptions::ENABLE_SUPERSCRIPT);
        }
        if builder.enable_subscript {
            options.insert(MarkOptions::ENABLE_SUBSCRIPT);
        }
        if builder.enable_wikilinks {
            options.insert(MarkOptions::ENABLE_WIKILINKS);
        }

        options
    }
}
