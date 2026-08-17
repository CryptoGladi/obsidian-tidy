use super::TokenStream;
use crate::token_stream::token::Callout;
use pulldown_cmark::{Event as MarkEvent, Tag as MarkTag};
use std::range::Range;

impl<'input> TokenStream<'input> {
    pub(crate) fn try_read_callout(&mut self) -> Option<(Callout<'input>, Range<usize>)> {
        /*
        // event=Start(BlockQuote(None)) offset=0..30
        / event=Start(Paragraph) offset=1..30
        event=Text(Borrowed("[")) offset=1..2
        event=Text(Borrowed("!example")) offset=2..10
        event=Text(Borrowed("]")) offset=10..11
        event=Text(Borrowed("+ Text")) offset=11..17
        */

        let guard = self.lexer.peek_many::<4>()?;
        let [paragraph, text_start, label, text_stop] = guard.data();

        // Нужно сделать более красивые проверки
        if !matches!(paragraph.0, MarkEvent::Start(MarkTag::Paragraph)) {
            return None;
        }

        guard.commit();

        None
    }
}
