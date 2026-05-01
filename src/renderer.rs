use crate::model::{BranchHeader, ColorMode, EntryStats, SectionKind, StatusView};

const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[38;5;2m";
const RED: &str = "\x1b[38;5;1m";
const TRACKED_TAN: &str = "\x1b[38;5;180m";
const MUTED_GRAY: &str = "\x1b[38;5;245m";
const STATS_SEPARATOR: &str = "\x1b[38;5;244m";

pub fn render(view: &StatusView, color_mode: ColorMode, stdout_is_tty: bool) -> String {
    let use_color = match color_mode {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => stdout_is_tty,
    };

    let visible_sections: Vec<_> = view
        .sections
        .iter()
        .filter(|section| !section.entries.is_empty())
        .collect();

    let mut output = String::new();

    if visible_sections.is_empty() {
        output.push_str(&render_branch_header(&view.header));
        output.push('\n');
        output.push_str("✓ working tree clean\n");
        return add_left_buffer(output);
    }

    let max_path_width = visible_sections
        .iter()
        .flat_map(|section| section.entries.iter())
        .map(|entry| entry.display_path.chars().count())
        .max()
        .unwrap_or(0);
    let max_addition_width = visible_sections
        .iter()
        .flat_map(|section| section.entries.iter())
        .map(|entry| addition_text(entry.stats).chars().count())
        .max()
        .unwrap_or(0);
    let border_width = border_width(view, &visible_sections, max_path_width, max_addition_width);
    let border = render_border(border_width, use_color);
    output.push_str(&border);
    output.push('\n');
    output.push_str(&render_branch_line(
        &view.header,
        max_path_width,
        max_addition_width,
    ));
    output.push('\n');
    output.push_str(&border);
    output.push('\n');

    for (index, section) in visible_sections.iter().enumerate() {
        if index > 0 {
            output.push('\n');
        }

        let header = format!("{} ({})", section.kind.title(), section.entries.len());
        output.push_str(&header);
        output.push('\n');

        for entry in &section.entries {
            let prefix = format!("{} {}", entry.symbol.letter(), entry.display_path);
            let padding =
                " ".repeat(max_path_width.saturating_sub(entry.display_path.chars().count()) + 2);
            let colored_prefix = colorize_section(&prefix, section.kind, use_color);
            let stats = render_stats(entry.stats, use_color, max_addition_width);
            output.push_str("  ");
            output.push_str(&colored_prefix);
            output.push_str(&padding);
            output.push_str(&stats);
            output.push('\n');
        }
    }

    add_left_buffer(output)
}

fn add_left_buffer(output: String) -> String {
    let mut buffered = String::with_capacity(output.len() + output.lines().count());
    for line in output.split_inclusive('\n') {
        if line == "\n" {
            buffered.push('\n');
        } else {
            buffered.push(' ');
            buffered.push_str(line);
        }
    }
    buffered
}

fn border_width(
    view: &StatusView,
    visible_sections: &[&crate::model::Section],
    max_path_width: usize,
    max_addition_width: usize,
) -> usize {
    let header_width = render_branch_line(&view.header, max_path_width, max_addition_width)
        .chars()
        .count();
    let section_width = visible_sections
        .iter()
        .map(|section| {
            format!("{} ({})", section.kind.title(), section.entries.len())
                .chars()
                .count()
        })
        .max()
        .unwrap_or(0);
    let entry_width = visible_sections
        .iter()
        .flat_map(|section| section.entries.iter())
        .map(|entry| {
            let prefix_width = 2 + entry.display_path.chars().count();
            let padding_width =
                max_path_width.saturating_sub(entry.display_path.chars().count()) + 2;
            let addition_padding_width =
                max_addition_width.saturating_sub(addition_text(entry.stats).chars().count());
            let stats_width = addition_padding_width
                + addition_text(entry.stats).chars().count()
                + 1
                + deletion_text(entry.stats).chars().count();
            2 + prefix_width + padding_width + stats_width
        })
        .max()
        .unwrap_or(0);

    header_width.max(section_width).max(entry_width).max(1)
}

fn render_border(width: usize, use_color: bool) -> String {
    let border = "─".repeat(width);
    if use_color {
        format!("{STATS_SEPARATOR}{border}{RESET}")
    } else {
        border
    }
}

fn render_branch_header(header: &BranchHeader) -> String {
    match header {
        BranchHeader::Branch {
            name,
            ahead,
            behind,
        } => format!("Branch: {name} {}", render_branch_stats(*ahead, *behind, 0)),
        BranchHeader::Detached { short_sha } => format!("detached @ {short_sha}"),
    }
}

fn render_branch_line(
    header: &BranchHeader,
    max_path_width: usize,
    max_addition_width: usize,
) -> String {
    match header {
        BranchHeader::Branch {
            name,
            ahead,
            behind,
        } => {
            let label = format!("Branch: {name}");
            let stats_start = 2 + 2 + max_path_width + 2;
            let stats = render_branch_stats(*ahead, *behind, max_addition_width);
            let padding = " ".repeat(stats_start.saturating_sub(label.chars().count()).max(2));
            format!("{label}{padding}{stats}")
        }
        BranchHeader::Detached { .. } => render_branch_header(header),
    }
}

fn render_branch_stats(ahead: usize, behind: usize, max_addition_width: usize) -> String {
    let ahead_text = format!("↑{ahead}");
    let behind_text = format!("↓{behind}");
    let padding = " ".repeat(max_addition_width.saturating_sub(ahead_text.chars().count()));
    format!("{padding}{ahead_text} {behind_text}")
}

fn render_stats(stats: EntryStats, use_color: bool, max_addition_width: usize) -> String {
    let addition = addition_text(stats);
    let deletion = deletion_text(stats);
    let addition_padding = " ".repeat(max_addition_width.saturating_sub(addition.chars().count()));

    if use_color {
        format!(
            "{addition_padding}{GREEN}{addition}{RESET}{STATS_SEPARATOR}/{RESET}{RED}{deletion}{RESET}"
        )
    } else {
        format!("{addition_padding}{addition}/{deletion}")
    }
}

fn addition_text(stats: EntryStats) -> String {
    match stats {
        EntryStats::Known { additions, .. } => format!("+{additions}"),
        EntryStats::Unknown => "+?".to_string(),
    }
}

fn deletion_text(stats: EntryStats) -> String {
    match stats {
        EntryStats::Known { deletions, .. } => format!("-{deletions}"),
        EntryStats::Unknown => "-?".to_string(),
    }
}

fn colorize_section(text: &str, kind: SectionKind, use_color: bool) -> String {
    if !use_color {
        return text.to_string();
    }

    let color = match kind {
        SectionKind::Staged => GREEN,
        SectionKind::Tracked => TRACKED_TAN,
        SectionKind::Untracked => MUTED_GRAY,
    };

    format!("{color}{text}{RESET}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{BranchHeader, Entry, EntryStats, Section, SectionKind, StatusSymbol};

    #[test]
    fn renders_clean_plain_output() {
        let view = StatusView {
            header: BranchHeader::Branch {
                name: "main".to_string(),
                ahead: 0,
                behind: 0,
            },
            sections: vec![],
        };

        assert_eq!(
            render(&view, ColorMode::Never, false),
            " Branch: main ↑0 ↓0\n ✓ working tree clean\n"
        );
    }

    #[test]
    fn renders_plain_sections_with_counts_hidden_empty_sections_and_aligned_stats() {
        let view = StatusView {
            header: BranchHeader::Branch {
                name: "feature".to_string(),
                ahead: 2,
                behind: 1,
            },
            sections: vec![
                Section::new(SectionKind::Staged, vec![]),
                Section::new(
                    SectionKind::Tracked,
                    vec![
                        Entry::new(
                            StatusSymbol::Modified,
                            "a.txt",
                            "a.txt",
                            EntryStats::Known {
                                additions: 1,
                                deletions: 0,
                            },
                        ),
                        Entry::new(
                            StatusSymbol::Deleted,
                            "nested/long-name.txt",
                            "nested/long-name.txt",
                            EntryStats::Known {
                                additions: 0,
                                deletions: 3,
                            },
                        ),
                    ],
                ),
            ],
        };

        assert_eq!(
            render(&view, ColorMode::Never, false),
            " ───────────────────────────────\n Branch: feature           ↑2 ↓1\n ───────────────────────────────\n Tracked (2)\n   M a.txt                 +1/-0\n   D nested/long-name.txt  +0/-3\n"
        );
    }

    #[test]
    fn vertically_aligns_stat_slashes() {
        let view = StatusView {
            header: BranchHeader::Branch {
                name: "main".to_string(),
                ahead: 0,
                behind: 0,
            },
            sections: vec![Section::new(
                SectionKind::Untracked,
                vec![
                    Entry::new(
                        StatusSymbol::Untracked,
                        "large.txt",
                        "large.txt",
                        EntryStats::Known {
                            additions: 1153,
                            deletions: 0,
                        },
                    ),
                    Entry::new(
                        StatusSymbol::Untracked,
                        "small.txt",
                        "small.txt",
                        EntryStats::Known {
                            additions: 3,
                            deletions: 0,
                        },
                    ),
                ],
            )],
        };

        assert_eq!(
            render(&view, ColorMode::Never, false),
            " ───────────────────────\n Branch: main      ↑0 ↓0\n ───────────────────────\n Untracked (2)\n   ? large.txt  +1153/-0\n   ? small.txt     +3/-0\n"
        );
    }

    #[test]
    fn renders_forced_deterministic_ansi_256_color_output() {
        let view = StatusView {
            header: BranchHeader::Branch {
                name: "main".to_string(),
                ahead: 0,
                behind: 0,
            },
            sections: vec![
                Section::new(
                    SectionKind::Staged,
                    vec![Entry::new(
                        StatusSymbol::Added,
                        "staged.txt",
                        "staged.txt",
                        EntryStats::Known {
                            additions: 2,
                            deletions: 0,
                        },
                    )],
                ),
                Section::new(
                    SectionKind::Tracked,
                    vec![Entry::new(
                        StatusSymbol::Modified,
                        "tracked.txt",
                        "tracked.txt",
                        EntryStats::Unknown,
                    )],
                ),
                Section::new(
                    SectionKind::Untracked,
                    vec![Entry::new(
                        StatusSymbol::Untracked,
                        "untracked.txt",
                        "untracked.txt",
                        EntryStats::Known {
                            additions: 1,
                            deletions: 0,
                        },
                    )],
                ),
            ],
        };

        assert_eq!(
            render(&view, ColorMode::Always, false),
            " \x1b[38;5;244m────────────────────────\x1b[0m\n Branch: main       ↑0 ↓0\n \x1b[38;5;244m────────────────────────\x1b[0m\n Staged (1)\n   \x1b[38;5;2mA staged.txt\x1b[0m     \x1b[38;5;2m+2\x1b[0m\x1b[38;5;244m/\x1b[0m\x1b[38;5;1m-0\x1b[0m\n\n Tracked (1)\n   \x1b[38;5;180mM tracked.txt\x1b[0m    \x1b[38;5;2m+?\x1b[0m\x1b[38;5;244m/\x1b[0m\x1b[38;5;1m-?\x1b[0m\n\n Untracked (1)\n   \x1b[38;5;245m? untracked.txt\x1b[0m  \x1b[38;5;2m+1\x1b[0m\x1b[38;5;244m/\x1b[0m\x1b[38;5;1m-0\x1b[0m\n"
        );
    }

    #[test]
    fn auto_color_follows_tty_state() {
        let view = StatusView {
            header: BranchHeader::Branch {
                name: "main".to_string(),
                ahead: 0,
                behind: 0,
            },
            sections: vec![Section::new(
                SectionKind::Untracked,
                vec![Entry::new(
                    StatusSymbol::Untracked,
                    "a.txt",
                    "a.txt",
                    EntryStats::Unknown,
                )],
            )],
        };

        assert!(!render(&view, ColorMode::Auto, false).contains("\x1b["));
        assert!(render(&view, ColorMode::Auto, true).contains("\x1b[38;5;245m"));
    }
}
