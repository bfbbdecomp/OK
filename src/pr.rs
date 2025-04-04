use crate::diff::{DifferenceReport, ReportItemDifference};

/***
 *
 *
 *
 *
 * HEY
 *
 *
 *
 * THis whole file is hot garbage and I need to refactor it
 *
 * But we're just trying to get OK bot talking for now
 */

#[derive(Debug)]
pub struct DiffSummary {
    pub unit_name: String,
    pub name: String,
    pub fuzzy_percent: f32,
    pub percent_difference: f32,
    pub size: u64,
    pub size_difference: i64,
}

// test
impl DiffSummary {
    pub fn new(diff: &ReportItemDifference) -> Self {
        let percent_diff = diff.new_fuzzy_match_percent - diff.old_fuzzy_match_percent;
        Self {
            unit_name: diff.unit_name.clone(),
            name: match &diff.demangled_name {
                Some(demangled) => demangled.clone(),
                None => diff.name.clone(),
            },
            size: diff.size,
            fuzzy_percent: diff.new_fuzzy_match_percent,
            percent_difference: percent_diff,
            size_difference: (((diff.size as f32) * (percent_diff / 100.0)) as i64),
        }
    }

    pub fn to_string(&self) -> String {
        let direction = if self.percent_difference > 0.0 {
            "+"
        } else {
            "" // Don't need to add the minus sign, Rust will do it on its own
        };

        //println!("{:?}", self);
        let percent = format!("{:.2}%", self.fuzzy_percent);

        let emoji = match self.fuzzy_percent {
            100.00 => "✅",
            _ => match self.percent_difference > 0.0 {
                true => "📈",
                false => "⚠️",
            },
        };

        format!(
            "{emoji} `{} - {}` {direction}{} bytes -> {percent}",
            self.unit_name, self.name, self.size_difference
        )
    }
}

#[derive(Debug)]
pub struct Regression(DiffSummary);

#[derive(Debug)]
pub struct Progression(DiffSummary);

#[derive(Debug)]
pub struct PullRequestReport {
    pub diffs: DifferenceReport,
}

impl PullRequestReport {
    pub fn new(diffs: DifferenceReport) -> Self {
        Self {
            //
            diffs,
        }
    }

    pub fn get_regressions(&self) -> Vec<Regression> {
        let mut items: Vec<ReportItemDifference> = self.diffs.sections.clone();
        items.extend(self.diffs.functions.clone());

        items
            .iter()
            .filter(|i| i.new_fuzzy_match_percent < i.old_fuzzy_match_percent)
            .map(|i| Regression(DiffSummary::new(i)))
            .filter(|x| x.0.size_difference != 0)
            .collect()
    }

    pub fn get_progressions(&self) -> Vec<Progression> {
        let mut items: Vec<ReportItemDifference> = self.diffs.sections.clone();
        items.sort_by_key(|x| x.size as i32 * -1);
        let mut fns = self.diffs.functions.clone();
        fns.sort_by_key(|x| x.size as i32 * -1);
        items.extend(fns);

        items
            .iter()
            .filter(|i| i.new_fuzzy_match_percent > i.old_fuzzy_match_percent)
            .map(|i| Progression(DiffSummary::new(i)))
            .filter(|x| x.0.size_difference != 0)
            .collect()
    }

    pub fn to_string(&self) -> String {
        let regressions = self.get_regressions();
        let progressions = self.get_progressions();

        //println!("{:?}", progressions);

        let regression_count = regressions.len();
        let regressions_exist = regression_count > 0;

        let header = match regressions_exist {
            false => "🆗 ✅",
            true => "⚠️ 🔥",
        };

        let regressions_header = match regressions_exist {
            false => "No Regressions 🎉".to_owned(),
            true => format!("Regressions: {regression_count}"),
        };

        let regressions_string = match regressions_exist {
            false => "".to_owned(),
            true => {
                let strs: Vec<String> = regressions.iter().map(|x| x.0.to_string()).collect();
                strs.join("\n")
            }
        };

        let progress_count = progressions.len();
        let progress_header = match progress_count {
            0 => "No Progress".to_owned(),
            _ => format!("Progress: {progress_count}"),
        };

        let progress_string = match progress_count {
            0 => "What is this PR doing? 🤔".to_owned(),
            _ => {
                let strs: Vec<String> = progressions.iter().map(|x| x.0.to_string()).collect();
                strs.join("\n")
            }
        };

        let size_diff = progressions
            .iter()
            .map(|x| x.0.size_difference)
            .sum::<i64>();
        let size_direction = if size_diff >= 0 { "+" } else { "" };

        let ok_rating = match size_diff {
            diff if diff >= 5_000 => "You are a decomp GOD, can I have your autograph?",
            diff if diff >= 2_000 => "Amazing contribution, you are the decomp GOAT 🐐",
            diff if diff >= 1_000 => "A Fantastic contribution! ✨🎉",
            diff if diff > 750 => "Ay, dios mio, gracias por la contribución!",
            diff if diff > 500 => "A solid contribution, Спасибо!",
            diff if diff > 250 => "A decent contribution. Thank you!",
            diff if diff < 100 => "A small but commendable contribution",
            diff if diff < 0 => "You're going in the wrong direction..?",
            diff if diff < -1_000 => "You really screwed up 🙉",
            _ => "I don't have an opinion",
        };

        let lines: Vec<String> = vec![
            format!("# {}", header),
            format!("{}{} bytes", size_direction, size_diff),
            format!("🆗 Bot Rating: {}", ok_rating),
            format!("## {}", regressions_header),
            regressions_string,
            format!("## {}", progress_header),
            progress_string,
        ];

        lines.join("\n").trim().to_owned()
    }
}
