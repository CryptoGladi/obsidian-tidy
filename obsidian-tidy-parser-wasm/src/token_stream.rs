use alloc::vec::Vec;
use core::range::Range;
use obsidian_tidy_parser::prelude::{
    Token as InnerToken, TokenStreamBuilder as InnerTokenStreamBuilder,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenItem<'input> {
    pub token: InnerToken<'input>,
    pub start: usize,
    pub end: usize,
}

impl<'input> From<(InnerToken<'input>, Range<usize>)> for TokenItem<'input> {
    fn from(value: (InnerToken<'input>, Range<usize>)) -> Self {
        let (token, range) = value;

        Self {
            token,
            start: range.start,
            end: range.end,
        }
    }
}

impl TokenItem<'_> {
    pub fn range(&self) -> Range<usize> {
        Range {
            start: self.start,
            end: self.end,
        }
    }
}

#[derive(Default)]
#[wasm_bindgen]
pub struct TokenStreamBuilder {
    inner: InnerTokenStreamBuilder,
}

#[wasm_bindgen]
impl TokenStreamBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: InnerTokenStreamBuilder::new(),
        }
    }

    #[wasm_bindgen(js_name = default)]
    pub fn wasm_default() -> Self {
        Self::default()
    }

    pub fn build(self, source: &str) -> Result<JsValue, JsValue> {
        let token_stream = self.inner.build(source);
        let tokens: Vec<_> = token_stream.map(TokenItem::from).collect();

        serde_wasm_bindgen::to_value(&tokens)
            .map_err(|e| JsValue::from_str(&alloc::format!("WASM serde Error: {:?}", e)))
    }
}

#[cfg(test)]
mod wasm_tests {
    use super::*;
    use obsidian_tidy_parser::prelude::HeadingLevel;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn builder_serialization_in_js_runtime() {
        let builder = TokenStreamBuilder::new();
        let result = builder.build("# Header Obsidian");

        assert!(result.is_ok());

        let js_value = result.unwrap();

        let tokens: Vec<TokenItem> = serde_wasm_bindgen::from_value(js_value).unwrap();
        assert!(!tokens.is_empty());

        assert!(matches!(
            tokens[0].token,
            InnerToken::Start(ref tag) if tag.as_heading().unwrap().level() == HeadingLevel::H1
        ));

        assert!(matches!(
            tokens[1].token,
            InnerToken::Text(ref text) if text == "Header Obsidian"
        ));

        assert!(matches!(tokens[2].token, InnerToken::End(tag_end) if tag_end.is_heading() ));

        assert_eq!(tokens.len(), 3);
    }

    #[wasm_bindgen_test]
    fn with_default() {
        let result = TokenStreamBuilder::default()
            .build("`Test` **text**")
            .unwrap();

        let tokens: Vec<TokenItem> = serde_wasm_bindgen::from_value(result).unwrap();
        assert!(!tokens.is_empty());
    }
}
