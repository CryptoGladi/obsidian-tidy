macro_rules! lint_collection {
    ($($lint:expr),* $(,)?) => {
        LazyLock::new(|| {
            vec![
               $(obsidian_tidy_core::lint::BoxedErrorLint::new($lint)),*
            ]
        })
    };
}

pub(crate) use lint_collection;
