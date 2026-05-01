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

    let mut output = String::new();
    output.push_str(&render_branch_header(&view.header));
    output.push('\n');

    let visible_sections: Vec<_> = view
        .sections
        .iter()
        .filter(|section| !section.entries.is_empty())
        .collect();

    if visible_sections.is_empty() {
        output.push_str("✓ working tree clean\n");
        return output;
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

    for (index, section) in visible_sections.iter().enumerate() {
        if index > 0 {
            output.push('\n');
        }

        let header = format!("{} ({})", section.kind.title(), section.entries.len());
        output.push_str(&colorize_section(&header, section.kind, use_color));
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

    output
}

fn render_branch_header(header: &BranchHeader) -> String {
    match header {
        BranchHeader::Branch {
            name,
            ahead,
            behind,
        } => {
            let mut rendered = name.clone();
            if *ahead > 0 {
                rendered.push_str(&format!(" ↑{ahead}"));
            }
            if *behind > 0 {
                rendered.push_str(&format!(" ↓{behind}"));
            }
            rendered
        }
        BranchHeader::Detached { short_sha } => format!("detached @ {short_sha}"),
    }
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
            "main\n✓ working tree clean\n"
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
            "feature ↑2 ↓1\nTracked (2)\n  M a.txt                 +1/-0\n  D nested/long-name.txt  +0/-3\n"
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
            "main\nUntracked (2)\n  ? large.txt  +1153/-0\n  ? small.txt     +3/-0\n"
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
            "main\n\x1b[38;5;2mStaged (1)\x1b[0m\n  \x1b[38;5;2mA staged.txt\x1b[0m     \x1b[38;5;2m+2\x1b[0m\x1b[38;5;244m/\x1b[0m\x1b[38;5;1m-0\x1b[0m\n\n\x1b[38;5;180mTracked (1)\x1b[0m\n  \x1b[38;5;180mM tracked.txt\x1b[0m    \x1b[38;5;2m+?\x1b[0m\x1b[38;5;244m/\x1b[0m\x1b[38;5;1m-?\x1b[0m\n\n\x1b[38;5;245mUntracked (1)\x1b[0m\n  \x1b[38;5;245m? untracked.txt\x1b[0m  \x1b[38;5;2m+1\x1b[0m\x1b[38;5;244m/\x1b[0m\x1b[38;5;1m-0\x1b[0m\n"
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
