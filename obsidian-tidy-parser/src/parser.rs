use pulldown_cmark::{Options as MarkOptions, Parser as MarkParser};

#[derive(Debug)]
pub struct Parser<'input> {
    inner: MarkParser<'input>,
}

impl<'input> Parser<'input> {
    pub fn new(text: &'input str) -> Self {
        // TODO Тут явно не всё нужно учитывать!
        let options = MarkOptions::all();

        Self {
            inner: MarkParser::new_ext(text, options),
        }
    }
}
