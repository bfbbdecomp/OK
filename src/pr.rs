use objdiff_core::bindings::report::Changes;
use tabled::{builder::Builder, settings::Style};

trait Pretty {
    fn pretty_percent(self) -> String;
}

impl Pretty for f32 {
    fn pretty_percent(self) -> String {
        format!("{:.2}%", self)
    }
}

fn byte_diff(from: f32, to: f32, size: u64) -> (String, String) {
    // println!("to: {} from: {} size: {}", to, from, size);
    let diff = (((to - from) / 100.0) * size as f32) as i64;
    let sign = match diff {
        d if d > 0 => "+",
        d if d < 0 => "-",
        _ => "+/-",
    };
    let emoji = match to {
        100.0 => "✅",
        _ => match sign {
            "+" => "📈",
            "-" => "️⚠️",
            _ => "❔",
        },
    };
    (emoji.into(), format!("{sign}{diff}"))
}

pub fn generate_pr_report(changes: &Changes) -> String {
    let mut comment = String::new();

    for unit in &changes.units {
        comment.push_str(format!("## `{}`\n", unit.name).as_str());

        // Add section tables
        let mut builder = Builder::new();
        builder.push_record(["", "Section", "From", "To", "Bytes"]);
        // report on sections changed
        for section in &unit.sections {
            if let Some(from) = section.from
                && let Some(to) = section.to
            {
                // let diff = to.fuzzy_match_percent - from.fuzzy_match_percent;
                // let byte_diff = (to.size as f32 * diff) as u64;
                // println!("{} {} {}", from.size, diff, byte_diff);
                let (emoji, diff) =
                    &byte_diff(from.fuzzy_match_percent, to.fuzzy_match_percent, to.size);

                builder.push_record([
                    emoji,
                    &format!("`{}`", &section.name),
                    &from.fuzzy_match_percent.pretty_percent(),
                    &to.fuzzy_match_percent.pretty_percent(),
                    diff,
                ]);
            }
        }
        let mut table = builder.build();
        table.with(Style::markdown());

        comment.push_str(table.to_string().as_str());
        comment.push_str("\n\n");

        // Add function tables
        let mut builder = Builder::new();
        builder.push_record(["", "Function", "From", "To", "Bytes"]);
        // report on sections changed
        for function in &unit.functions {
            if let Some(from) = function.from
                && let Some(to) = function.to
            {
                let name: String = match &function.metadata {
                    Some(meta) => match &meta.demangled_name {
                        Some(demangled) => demangled.clone(),
                        None => function.name.clone(),
                    },
                    None => function.name.clone(),
                };

                let (emoji, diff) =
                    &byte_diff(from.fuzzy_match_percent, to.fuzzy_match_percent, to.size);

                builder.push_record([
                    emoji,
                    &format!("`{}`", &name),
                    &from.fuzzy_match_percent.pretty_percent(),
                    &to.fuzzy_match_percent.pretty_percent(),
                    diff,
                ]);
            }
        }
        let mut table = builder.build();
        table.with(Style::markdown());

        comment.push_str(table.to_string().as_str());
        comment.push_str("\n\n");
    }

    comment
}
