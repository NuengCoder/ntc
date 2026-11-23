//! Unit tests for the `sys` monitor: key handling, sorting, gauges and disk
//! rows.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Color;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::Duration;
use sysinfo::{DiskKind, Disks, System};

use super::monitor::{compare_pids, BodyLine, SortBy, SysAction, SysMon};
use super::parse_log_flags;
use super::render::{cpu_grid_lines, format_disk_line, paint, render_text_bar};

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

fn ctrl(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::CONTROL)
}

/// Monitor with a fake process list (no OS query) for key tests.
fn bare() -> SysMon {
    let mut sm = SysMon::with_system(System::new(), 100, 40);
    sm.disks = Disks::new();
    sm.flat_pids = (0..5).map(|i| sysinfo::Pid::from_u32(100 + i)).collect();
    sm.sel_pid = sm.flat_pids.first().copied();
    sm
}

/// Monitor backed by the real process table (sorting tests).
fn real() -> SysMon {
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut sm = SysMon::with_system(sys, 100, 40);
    sm.disks = Disks::new();
    sm.refresh_filtered();
    sm
}

#[test]
fn parses_log_switch_and_delay() {
    // Without -l/--log there is nothing to log, whatever the delay says.
    assert_eq!(parse_log_flags("sys").unwrap(), None);
    assert_eq!(parse_log_flags("-d 5").unwrap(), None);

    // `--log` is a switch; -d/--delay carries the seconds and defaults to -1.
    assert_eq!(
        parse_log_flags("--log").unwrap(),
        Some(Duration::from_secs(15))
    );
    assert_eq!(
        parse_log_flags("-l").unwrap(),
        Some(Duration::from_secs(15))
    );
    assert_eq!(
        parse_log_flags("-l -d").unwrap(),
        Some(Duration::from_secs(15))
    );
    assert_eq!(
        parse_log_flags("-l -d -1").unwrap(),
        Some(Duration::from_secs(15))
    );
    assert_eq!(
        parse_log_flags("-l -d 5").unwrap(),
        Some(Duration::from_secs(5))
    );
    assert_eq!(
        parse_log_flags("-d 5 -l").unwrap(),
        Some(Duration::from_secs(5))
    );
    assert_eq!(
        parse_log_flags("--log --delay 5").unwrap(),
        Some(Duration::from_secs(5))
    );
    assert_eq!(
        parse_log_flags("--log --delay=30").unwrap(),
        Some(Duration::from_secs(30))
    );
    assert_eq!(
        parse_log_flags("-d=7 -l").unwrap(),
        Some(Duration::from_secs(7))
    );
    assert_eq!(
        parse_log_flags("-d7 -l").unwrap(),
        Some(Duration::from_secs(7))
    );

    // Combined short flags: `-ld [seconds]` is `-l -d [seconds]`.
    assert_eq!(
        parse_log_flags("-ld").unwrap(),
        Some(Duration::from_secs(15))
    );
    assert_eq!(
        parse_log_flags("-ld 5").unwrap(),
        Some(Duration::from_secs(5))
    );
    assert_eq!(
        parse_log_flags("-dl 5").unwrap(),
        Some(Duration::from_secs(5))
    );

    // `--log` no longer takes the seconds argument.
    assert!(parse_log_flags("--log=5").is_err());
    assert!(parse_log_flags("-l=5").is_err());
    assert!(parse_log_flags("-l -d 0").is_err());
    assert!(parse_log_flags("-l -d nope").is_err());
}

#[test]
fn toggles_hide_and_show_panels() {
    let mut sm = bare();
    assert_eq!(sm.handle_key(key(KeyCode::Char('d'))), SysAction::Render);
    assert!(!sm.show_disks);
    assert_eq!(
        sm.status_msg.as_ref().map(|m| m.text.as_str()),
        Some("disks hidden")
    );
    sm.handle_key(key(KeyCode::Char('d')));
    assert!(sm.show_disks);

    sm.handle_key(key(KeyCode::Char('p')));
    assert!(!sm.show_process);
    sm.handle_key(key(KeyCode::Char('p')));
    assert!(sm.show_process);

    sm.handle_key(key(KeyCode::Char('c')));
    assert!(!sm.show_cpu);
}

#[test]
fn v_toggles_between_tree_and_flat_view() {
    let mut sm = bare();
    assert!(!sm.flat_view);
    assert_eq!(sm.handle_key(key(KeyCode::Char('v'))), SysAction::Render);
    assert!(sm.flat_view);
    assert_eq!(
        sm.status_msg.as_ref().map(|m| m.text.as_str()),
        Some("flat view (all processes)")
    );
    assert!(sm.hint_text().contains("v=tree"), "{}", sm.hint_text());
    assert_eq!(sm.handle_key(key(KeyCode::Char('v'))), SysAction::Render);
    assert!(!sm.flat_view);
    assert_eq!(
        sm.status_msg.as_ref().map(|m| m.text.as_str()),
        Some("tree view")
    );
    assert!(sm.hint_text().contains("v=flat"), "{}", sm.hint_text());
}

#[test]
fn flat_view_lists_every_process_on_one_level() {
    let mut sm = real();
    sm.flat_view = true;
    sm.refresh_filtered();
    assert_eq!(sm.flat_pids.len(), sm.sys.processes().len());
    assert!(sm.depth_map.values().all(|d| *d == 0));
    assert!(
        sm.flat_pids.windows(2).all(|w| compare_pids(
            &sm.sys,
            sm.sort_by,
            sm.sort_asc,
            &w[0],
            &w[1]
        ) != Ordering::Greater),
        "flat view must follow the active sort"
    );
    // Enter has nothing to expand in the flat view.
    assert_eq!(sm.handle_key(key(KeyCode::Enter)), SysAction::None);
}

#[test]
fn jk_moves_selection_when_process_list_is_shown() {
    let mut sm = bare();
    sm.handle_key(key(KeyCode::Char('j')));
    assert_eq!(sm.sel, 1);
    assert_eq!(sm.tui_scroll, 0);
    sm.handle_key(key(KeyCode::Char('j')));
    sm.handle_key(key(KeyCode::Up));
    assert_eq!(sm.sel, 1);
    // Never past the ends.
    for _ in 0..10 {
        sm.handle_key(key(KeyCode::Char('k')));
    }
    assert_eq!(sm.sel, 0);
}

#[test]
fn jk_scrolls_the_view_when_process_list_is_hidden() {
    let mut sm = bare();
    sm.show_process = false;
    sm.handle_key(key(KeyCode::Char('j')));
    assert_eq!(sm.sel, 0);
    assert_eq!(sm.tui_scroll, 1);
    sm.handle_key(key(KeyCode::Down));
    assert_eq!(sm.tui_scroll, 2);
    sm.handle_key(key(KeyCode::Char('k')));
    assert_eq!(sm.tui_scroll, 1);
}

#[test]
fn kill_bindings_do_not_fire_on_plain_k() {
    let mut sm = bare();
    sm.set_sel(2);
    assert_eq!(sm.handle_key(key(KeyCode::Char('x'))), SysAction::Kill(102));
    assert_eq!(
        sm.handle_key(ctrl(KeyCode::Char('k'))),
        SysAction::Kill(102)
    );
    // Plain `k` is navigation, never a kill.
    assert_eq!(sm.handle_key(key(KeyCode::Char('k'))), SysAction::Render);
    assert_eq!(sm.sel, 1);
}

#[test]
fn home_and_end_jump_to_the_edges() {
    let mut sm = bare();
    sm.set_sel(3);
    sm.tui_scroll = 7;
    sm.handle_key(key(KeyCode::Char('g')));
    assert_eq!(sm.sel, 0);
    assert_eq!(sm.tui_scroll, 0);
    sm.handle_key(key(KeyCode::Char('G')));
    assert_eq!(sm.sel, 4);
    // Clamped against the body length during render.
    assert_eq!(sm.tui_scroll, usize::MAX);
}

#[test]
fn page_keys_scroll_without_moving_selection() {
    let mut sm = bare();
    sm.set_sel(2);
    let page = sm.page();
    sm.handle_key(key(KeyCode::PageDown));
    assert_eq!(sm.sel, 2, "PgDn must not move the selection");
    assert_eq!(sm.tui_scroll, page);
    sm.handle_key(key(KeyCode::PageUp));
    assert_eq!(sm.tui_scroll, 0);
}

#[test]
fn esc_clears_the_filter_before_quitting() {
    let mut sm = bare();
    sm.filter = "rust".into();
    assert_eq!(sm.handle_key(key(KeyCode::Esc)), SysAction::Render);
    assert!(sm.filter.is_empty());
    assert_eq!(sm.handle_key(key(KeyCode::Esc)), SysAction::Quit);
    assert_eq!(sm.handle_key(key(KeyCode::Char('q'))), SysAction::Quit);
}

#[test]
fn filter_mode_swallows_navigation_keys() {
    let mut sm = bare();
    sm.handle_key(key(KeyCode::Char('F')));
    assert!(sm.filtering);
    // `q` types instead of quitting, `?` types instead of toggling help.
    sm.handle_key(key(KeyCode::Char('q')));
    sm.handle_key(key(KeyCode::Char('?')));
    assert_eq!(sm.filter, "q?");
    assert!(!sm.help_visible);
    sm.handle_key(key(KeyCode::Backspace));
    assert_eq!(sm.filter, "q");
    sm.handle_key(key(KeyCode::Enter));
    assert!(!sm.filtering);
}

#[test]
fn sort_cycle_resets_the_view_and_reports_direction() {
    let mut sm = bare();
    sm.set_sel(3);
    sm.tui_scroll = 4;
    assert_eq!(sm.handle_key(key(KeyCode::Char('s'))), SysAction::Refresh);
    assert_eq!(sm.sort_by, SortBy::Mem);
    assert_eq!(sm.sel, 0);
    assert_eq!(sm.tui_scroll, 0);
    assert_eq!(
        sm.status_msg.as_ref().map(|m| m.text.as_str()),
        Some("sort: MEM desc")
    );
    sm.handle_key(key(KeyCode::Char('S')));
    assert!(sm.sort_asc);
    assert_eq!(
        sm.status_msg.as_ref().map(|m| m.text.as_str()),
        Some("sort: MEM asc")
    );
}

#[test]
fn hint_bar_reports_the_process_count() {
    let mut sm = real();
    let total = sm.sys.processes().len();
    assert!(
        sm.hint_text().contains(&format!("procs:{total}")),
        "{}",
        sm.hint_text()
    );

    sm.filter = "zzz-no-such-process".into();
    sm.refresh_filtered();
    assert!(sm.flat_pids.is_empty());
    assert!(
        sm.hint_text().contains(&format!("procs:0/{total}")),
        "{}",
        sm.hint_text()
    );
}

#[test]
fn help_overlay_toggles_with_question_mark() {
    let mut sm = bare();
    sm.handle_key(key(KeyCode::Char('?')));
    assert!(sm.help_visible);
    assert!(sm.hint_text().starts_with("?=help"));
    sm.handle_key(key(KeyCode::Char('?')));
    assert!(!sm.help_visible);
}

#[test]
fn comparator_reverses_with_the_direction() {
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut pids: Vec<sysinfo::Pid> = sys.processes().keys().copied().collect();
    assert!(pids.len() >= 2, "need at least two live processes");

    let mut asc = pids.clone();
    asc.sort_by(|a, b| compare_pids(&sys, SortBy::Pid, true, a, b));
    assert!(asc.windows(2).all(|w| w[0] <= w[1]));

    pids.sort_by(|a, b| compare_pids(&sys, SortBy::Pid, false, a, b));
    assert!(pids.windows(2).all(|w| w[0] >= w[1]));
}

#[test]
fn children_follow_the_sort_direction() {
    let mut sm = real();
    // Find a parent with at least two visible children.
    let mut by_parent: HashMap<sysinfo::Pid, Vec<sysinfo::Pid>> = HashMap::new();
    for (pid, proc) in sm.sys.processes() {
        if let Some(ppid) = proc.parent() {
            if ppid != *pid && sm.sys.processes().contains_key(&ppid) {
                by_parent.entry(ppid).or_default().push(*pid);
            }
        }
    }
    sm.sort_by = SortBy::Name;
    sm.sort_asc = false;
    let all: Vec<sysinfo::Pid> = sm.sys.processes().keys().copied().collect();
    // Pick a parent the tree can actually reach (collapsed view first).
    let (roots_only, _) = sm.build_tree(&all);
    let Some(parent) = roots_only
        .iter()
        .copied()
        .find(|p| by_parent.get(p).is_some_and(|kids| kids.len() >= 2))
    else {
        return; // The live process table has no branching tree right now.
    };
    sm.expanded.insert(parent);
    let (flat, depths) = sm.build_tree(&all);
    let pos = flat
        .iter()
        .position(|p| *p == parent)
        .expect("parent listed");
    let Some(&first_child) = flat.get(pos + 1) else {
        return;
    };
    let child_depth = depths[&first_child];
    assert_eq!(child_depth, depths[&parent] + 1);
    let kids: Vec<sysinfo::Pid> = flat[pos + 1..]
        .iter()
        .take_while(|p| depths[p] == child_depth)
        .copied()
        .collect();
    assert!(kids.len() >= 2, "expanded parent must list its children");
    assert!(
        kids.windows(2)
            .all(|w| compare_pids(&sm.sys, SortBy::Name, false, &w[0], &w[1]) != Ordering::Greater),
        "children must follow the root sort order: {kids:?}"
    );
}

#[test]
fn selection_survives_a_refresh() {
    let mut sm = bare();
    sm.set_sel(3);
    let pid = sm.selected_pid();
    // A refresh rebuilds the list from the (empty) fake system.
    sm.refresh_filtered();
    assert!(sm.sel_pid.is_none() || sm.sel_pid == pid);
    assert!(sm.sel < sm.flat_pids.len().max(1));
}

#[test]
fn killed_pid_is_dropped_immediately() {
    let mut sm = bare();
    sm.set_sel(2);
    sm.forget_pid(102);
    assert!(!sm.flat_pids.contains(&sysinfo::Pid::from_u32(102)));
    assert_eq!(sm.sel, 2);
    assert!(sm.killed.contains(&sysinfo::Pid::from_u32(102)));
}

#[test]
fn body_lines_carry_no_control_bytes() {
    let line = BodyLine::new("na\u{1b}[31mme\tx");
    assert!(
        !line.text.chars().any(|c| c.is_control()),
        "{:?}",
        line.text
    );
    assert!(line.text.contains("^["), "{:?}", line.text);
    assert!(line.text.starts_with("na^[[31mme"), "{:?}", line.text);
}

#[test]
fn cpu_grid_numbers_cores_from_one_with_a_gap() {
    let rows = cpu_grid_lines(&[3, 47, 100], 200);
    assert_eq!(rows.len(), 1, "three cores fit on one row: {rows:?}");
    let row = &rows[0];
    assert!(row.starts_with(" C 1 ["), "got: {row}");
    assert!(!row.contains("C 0"), "cores must start at 1: {row}");
    assert!(
        row.contains("]  C 2 ["),
        "cells need a two-space gap: {row}"
    );
    assert!(
        row.contains("]  C 3 ["),
        "cells need a two-space gap: {row}"
    );
    // Fixed cell width keeps the gauges in columns.
    let first = row.find('[').expect("gauge");
    let second = row[first + 1..].find('[').expect("second gauge") + first + 1;
    let third = row[second + 1..].find('[').expect("third gauge") + second + 1;
    assert_eq!(second - first, third - second, "got: {row}");
}

#[test]
fn cpu_grid_wraps_and_keeps_numbering() {
    // 24-cell cells + one leading space: 49 columns fit two cores.
    let usages: Vec<usize> = (0..5).map(|i| i * 10).collect();
    let rows = cpu_grid_lines(&usages, 49);
    assert_eq!(rows.len(), 3, "got: {rows:?}");
    assert!(
        rows[0].contains("C 1 ") && rows[0].contains("C 2 "),
        "{rows:?}"
    );
    assert!(rows[1].starts_with(" C 3 "), "{rows:?}");
    assert!(rows[2].starts_with(" C 5 "), "{rows:?}");
}

#[test]
fn text_bar_width_is_constant() {
    for pct in [0, 3, 9, 10, 55, 99, 100, 250] {
        assert_eq!(
            render_text_bar(pct).chars().count(),
            17,
            "pct {pct}: {:?}",
            render_text_bar(pct)
        );
    }
    assert_eq!(render_text_bar(100), "[---------- 100%]");
    assert_eq!(render_text_bar(0), "[             0%]");
    assert!(render_text_bar(3).ends_with("  3%]"));
}

#[test]
fn paint_honours_setc_off() {
    assert_eq!(paint(true, Color::Red), Color::Red);
    assert_eq!(paint(false, Color::Red), Color::Reset);
}

#[test]
fn disk_line_full_detail() {
    let line = format_disk_line(
        "C:\\",
        "Windows",
        "NTFS",
        DiskKind::SSD,
        false,
        false,
        120 * 1024 * 1024 * 1024,
        500 * 1024 * 1024 * 1024,
    );
    assert!(
        line.starts_with(" C:\\ [Windows] (NTFS SSD) "),
        "got: {}",
        line
    );
    assert!(line.contains("120"), "got: {}", line);
    assert!(!line.contains("USB"), "got: {}", line);
}

#[test]
fn disk_line_tags_removable_and_readonly() {
    let line = format_disk_line("E:\\", "", "FAT32", DiskKind::HDD, true, true, 0, 100);
    assert!(line.contains(" [USB]"), "got: {}", line);
    assert!(line.contains(" [RO]"), "got: {}", line);
    // Empty label omitted, no stray brackets.
    assert!(!line.contains("[]"), "got: {}", line);
}

#[test]
fn disk_line_unknown_kind_and_zero_total() {
    let line = format_disk_line("/", "", "", DiskKind::Unknown(-1), false, false, 0, 0);
    assert!(!line.contains('('), "got: {}", line);
    assert!(line.ends_with("0.0B / 0.0B"), "got: {}", line);
}
