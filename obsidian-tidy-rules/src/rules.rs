macro_rules! rules {
    ($($rule:expr),* $(,)?) => {
        LazyLock::new(|| {
            vec![
               $(obsidian_tidy_core::rule::SharedErrorRule::new($rule)),*
            ]
        })
    };
}

pub(crate) use rules;
