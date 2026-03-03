use crate::github::OrgData;
use crate::theme::Theme;

pub fn render(data: &OrgData, org: &str, theme: &Theme, hide_border: bool) -> String {
    let border = if hide_border {
        ""
    } else {
        &format!(
            r#"<rect x="0.5" y="0.5" width="494" height="194" rx="4.5" fill="{}" stroke="{}"/>"#,
            theme.bg, theme.surface1
        )
    };

    let bg = if hide_border {
        format!(
            r#"<rect x="0" y="0" width="495" height="195" rx="4.5" fill="{}"/>"#,
            theme.bg
        )
    } else {
        String::new()
    };

    let items = [
        ("Total Commits", format_number(data.total_commits), theme.accent, commit_icon()),
        ("Last 30 Days", format_number(data.recent_commits), theme.green, calendar_icon()),
        ("Contributors", format_number(data.member_count), theme.peach, people_icon()),
        ("Active Repos", format_number(data.repo_count), theme.yellow, repo_icon()),
    ];

    let mut item_svg = String::new();
    for (i, (label, value, color, icon)) in items.iter().enumerate() {
        let y = 48 + i * 36;
        item_svg.push_str(&format!(
            r#"<g transform="translate(25, {y})">
  <g fill="{color}">{icon}</g>
  <text x="26" y="12" fill="{}" font-size="14">{label}:</text>
  <text x="460" y="12" fill="{color}" font-size="14" text-anchor="end" font-weight="bold">{value}</text>
</g>"#,
            theme.subtext
        ));
    }

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="495" height="195" viewBox="0 0 495 195">
  <style>
    text {{ font-family: 'Segoe UI', Ubuntu, 'Helvetica Neue', sans-serif; }}
  </style>
  {bg}
  {border}
  <text x="25" y="32" fill="{}" font-size="16" font-weight="bold">{org}'s Activity</text>
  {item_svg}
</svg>"#,
        theme.text
    )
}

fn format_number(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}k", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

fn commit_icon() -> &'static str {
    r#"<path d="M1.5 8a6.5 6.5 0 1 0 13 0 6.5 6.5 0 0 0-13 0zM8 0a8 8 0 1 1 0 16A8 8 0 0 1 8 0zm.5 4.002a.5.5 0 0 0-1 0v3.5a.5.5 0 0 0 .146.354l2.5 2.5a.5.5 0 0 0 .708-.708L8.5 7.293z" transform="scale(0.9)"/>"#
}

fn calendar_icon() -> &'static str {
    r#"<path d="M4.75 0a.75.75 0 0 1 .75.75V2h5V.75a.75.75 0 0 1 1.5 0V2h1.25c.966 0 1.75.784 1.75 1.75v10.5A1.75 1.75 0 0 1 13.25 16H2.75A1.75 1.75 0 0 1 1 14.25V3.75C1 2.784 1.784 2 2.75 2H4V.75A.75.75 0 0 1 4.75 0zM2.5 7.5v6.75c0 .138.112.25.25.25h10.5a.25.25 0 0 0 .25-.25V7.5zm0-2h11V3.75a.25.25 0 0 0-.25-.25H2.75a.25.25 0 0 0-.25.25z" transform="scale(0.9)"/>"#
}

fn people_icon() -> &'static str {
    r#"<path d="M2 5.5a3.5 3.5 0 1 1 5.898 2.549 5.508 5.508 0 0 1 3.034 4.084.75.75 0 1 1-1.482.235 4.001 4.001 0 0 0-6.9 0 .75.75 0 0 1-1.482-.236A5.507 5.507 0 0 1 4.6 8.05 3.5 3.5 0 0 1 2 5.5zM5.5 4a2 2 0 1 0 0 4 2 2 0 0 0 0-4zm5.5.5a.75.75 0 0 1 .75-.75 2.5 2.5 0 0 1 0 5 .75.75 0 0 1 0-1.5 1 1 0 0 0 0-2 .75.75 0 0 1-.75-.75zm.75 6.5a.75.75 0 0 1 .75.75 3 3 0 0 1-3 3 .75.75 0 0 1 0-1.5 1.5 1.5 0 0 0 1.5-1.5.75.75 0 0 1 .75-.75z" transform="scale(0.9)"/>"#
}

fn repo_icon() -> &'static str {
    r#"<path d="M2 2.5A2.5 2.5 0 0 1 4.5 0h8.75a.75.75 0 0 1 .75.75v12.5a.75.75 0 0 1-.75.75h-2.5a.75.75 0 0 1 0-1.5h1.75v-2h-8a1 1 0 0 0-.714 1.7.75.75 0 1 1-1.072 1.05A2.495 2.495 0 0 1 2 11.5zm10.5-1h-8a1 1 0 0 0-1 1v6.708A2.486 2.486 0 0 1 4.5 9h8zM5 12.25a.25.25 0 0 1 .25-.25h3.5a.25.25 0 0 1 .25.25v3.25a.25.25 0 0 1-.4.2l-1.45-1.087a.249.249 0 0 0-.3 0L5.4 15.7a.25.25 0 0 1-.4-.2z" transform="scale(0.9)"/>"#
}
