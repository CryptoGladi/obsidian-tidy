use super::{Cli, Runner};
use itertools::Itertools;
use obsidian_tidy_config::template::Template;
use owo_colors::OwoColorize;
use tracing::{debug, instrument};

#[derive(Debug)]
pub struct RunnerListRules<'a> {
    from_template: &'a Template,
}

impl<'a> RunnerListRules<'a> {
    pub fn new(from_template: &'a Template) -> Self {
        Self { from_template }
    }
}

impl Runner for RunnerListRules<'_> {
    #[instrument]
    fn run(&self, args: &Cli) -> anyhow::Result<()> {
        debug!("Run command `list-rules`");

        let rules_by_category = self
            .from_template
            .iter()
            .sorted_by_key(|rule| rule.category())
            .chunk_by(|rule| rule.category());

        println!("{}", "📋 Available Rules".bold().green());
        println!("{}", "══════════════════".green());

        let mut total_rules = 0;

        for (category, rules) in &rules_by_category {
            let rules_vec: Vec<_> = rules.collect();
            println!(
                "\n{} {} ({} rule{})",
                "📁".cyan(),
                format!("{}", category).bold().blue(),
                rules_vec.len(),
                if rules_vec.len() == 1 { "" } else { "s" }
            );

            for rule in &rules_vec {
                let status = match rule.is_enabled() {
                    true => "✓".green().to_string(),
                    false => "✘".red().to_string(),
                };

                println!(
                    "  {} {}",
                    status,
                    format!("{}: {}", rule.name().bold(), rule.description())
                );
            }

            total_rules += rules_vec.len();
        }

        println!(
            "\n{} {} {}",
            "📊".cyan(),
            "Total enabled rules:".bold(),
            total_rules.to_string().bold().yellow()
        );

        Ok(())
    }
}
