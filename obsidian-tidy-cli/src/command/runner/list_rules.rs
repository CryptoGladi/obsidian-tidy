//! List all available rules with beautiful, user-friendly formatting
//!
//! Outputs a colorized table with emojis, summary statistics, and legend.

use super::Runnable;
use crate::Cli;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use obsidian_tidy_config::template::Template;
use obsidian_tidy_core::rule::{Category, Rules};

fn status_cell(enabled: bool) -> Cell {
    if enabled {
        Cell::new("🟢")
            .fg(Color::Green)
            .add_attribute(Attribute::Bold)
    } else {
        Cell::new("🔴")
            .fg(Color::Red)
            .add_attribute(Attribute::Bold)
    }
}

fn category_with_emoji(category: Category) -> String {
    let emoji = match category {
        Category::Yaml => "📚",
        Category::Heading => "🔤",
        Category::Content => "📝",
        Category::Spacing => "🎨",
        Category::Other => "📋",
    };

    format!("{emoji} {category}")
}

fn name_cell(name: &str, enabled: bool) -> Cell {
    let mut cell = Cell::new(name).add_attribute(Attribute::Bold);

    if enabled {
        cell = cell.fg(Color::White);
    } else {
        cell = cell.fg(Color::Grey);
    }

    cell
}

#[derive(Debug)]
pub struct ListRules {
    template: Template,
}

impl ListRules {
    #[must_use]
    pub const fn new(template: Template) -> Self {
        Self { template }
    }
}

impl Runnable for ListRules {
    fn run(self, _cli: &Cli) -> miette::Result<()> {
        tracing::info!("📚 Loading rules from `{}` template", self.template);

        let rules = Rules::from(self.template);
        let rules_vec: Vec<_> = rules.rules().collect();

        let total = rules_vec.len();
        let enabled_count = rules_vec.iter().filter(|r| r.is_enabled()).count();
        let disabled_count = total - enabled_count;

        let mut table = Table::new();
        table
            .load_preset(comfy_table::presets::UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("Status").add_attribute(Attribute::Bold),
                Cell::new("Name").add_attribute(Attribute::Bold),
                Cell::new("Category").add_attribute(Attribute::Bold),
                Cell::new("Description").add_attribute(Attribute::Bold),
            ]);

        for rule in &rules_vec {
            table.add_row(vec![
                status_cell(rule.is_enabled()),
                name_cell(rule.name(), rule.is_enabled()),
                Cell::new(category_with_emoji(rule.category())).fg(Color::Yellow),
                Cell::new(rule.description()).fg(Color::White),
            ]);
        }

        println!("{table}");

        tracing::info!(
            "📊 Summary: {} total | {} 🟢 enabled | {} 🔴 disabled",
            total,
            enabled_count,
            disabled_count
        );

        Ok(())
    }
}
