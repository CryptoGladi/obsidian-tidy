macro_rules! lint_collection {
    ($($lint:expr),* $(,)?) => {
        LazyLock::new(|| {
            vec![
               $(obsidian_tidy_core::lint::SharedErrorLint::new($lint)),*
            ]
        })
    };
}

pub(crate) use lint_collection;
