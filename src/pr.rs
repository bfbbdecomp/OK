use crate::diff::{DifferenceReport, ReportItemDifference};

#[derive(Debug)]
pub struct DiffSummary {
    pub unit_name: String,
    pub name: String,
    pub percent_difference: f32,
    pub size_difference: u64,
}

impl DiffSummary {
    pub fn new(diff: &ReportItemDifference) -> Self {
        let percent_diff = diff.new_fuzzy_match_percent - diff.old_fuzzy_match_percent;
        Self {
            unit_name: diff.unit_name.clone(),
            name: match &diff.demangled_name {
                Some(demangled) => demangled.clone(),
                None => diff.name.clone(),
            },
            percent_difference: percent_diff,
            size_difference: (((diff.size as f32) * percent_diff) as u64 / 4),
        }
    }

    pub fn to_string(&self) -> String {
        let direction = if self.percent_difference > 0.0 {
            "+"
        } else {
            "foo"
        };

        format!(
            "{}: {} ({direction}{})",
            self.name, self.percent_difference, self.size_difference
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
            .collect()
    }

    pub fn get_progressions(&self) -> Vec<Progression> {
        let mut items: Vec<ReportItemDifference> = self.diffs.sections.clone();
        items.extend(self.diffs.functions.clone());

        items
            .iter()
            .filter(|i| i.new_fuzzy_match_percent > i.old_fuzzy_match_percent)
            .map(|i| Progression(DiffSummary::new(i)))
            .collect()
    }

    pub fn to_string(&self) -> String {
        let regressions = self.get_regressions();
        let progressions = self.get_progressions();

        println!("{:?}", progressions);

        let regressions_exist = regressions.len() > 0;

        let header = match regressions_exist {
            false => "🆗 ✅",
            true => "⚠️ 🔥",
        };

        let regressions_header = match regressions_exist {
            false => "No Regressions 🎉".to_owned(),
            true => "foo".to_owned(),
        };
        let regressions_string = match regressions_exist {
            false => "".to_owned(),
            true => {
                let strs: Vec<String> = regressions.iter().map(|x| x.0.to_string()).collect();
                strs.join("\n")
            }
        };

        let progress_header = match progressions.len() {
            0 => "No Progress 🤔".to_owned(),
            _ => "foo".to_owned(),
        };

        let progress_string = match progressions.len() {
            0 => "".to_owned(),
            _ => {
                let strs: Vec<String> = progressions.iter().map(|x| x.0.to_string()).collect();
                strs.join("\n")
            }
        };

        let lines: Vec<String> = vec![
            // h
            format!("# {}", header),
            format!("## {}", regressions_header),
            regressions_string,
            format!("## {}", progress_header),
            progress_string,
        ];

        lines.join("\n").trim().to_owned()
    }
}
