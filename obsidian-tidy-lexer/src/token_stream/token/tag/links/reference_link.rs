use alloc::borrow::Cow;

/// Референсная ссылка: `[text][ref]` или `[text][]`
/// ⚠️ ВАЖНО для линтера: мы сохраняем оригинальный `reference`,
/// даже если он неизвестен (is_known = false), чтобы сообщить пользователю о битой ссылке.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReferenceLink<'input> {
    pub(crate) reference: Cow<'input, str>, // То, что написано в [ref]
    pub(crate) is_known: bool,              // Нашел ли pulldown-cmark определение этой ссылки
    pub(crate) destination: Option<Cow<'input, str>>, // Разрешенный URL (если is_known = true)
    pub(crate) title: Option<Cow<'input, str>>, // Разрешенный title (если is_known = true)
}

impl<'input> ReferenceLink<'input> {
    pub fn reference(&self) -> &str {
        self.reference.as_ref()
    }

    pub fn is_known(&self) -> bool {
        self.is_known
    }

    pub fn destination(&self) -> Option<&str> {
        self.destination.as_ref().map(Cow::as_ref)
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_ref().map(Cow::as_ref)
    }
}

crate::__private::impl_as_target_self!(ReferenceLink<'_>);
