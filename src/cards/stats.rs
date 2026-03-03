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
        ("Stars", format_number(data.total_stars), theme.yellow, star_icon()),
        ("Forks", format_number(data.total_forks), theme.accent, fork_icon()),
        ("Repositories", format_number(data.repo_count), theme.green, repo_icon()),
        ("Contributors", format_number(data.member_count), theme.peach, people_icon()),
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
  <text x="25" y="32" fill="{}" font-size="16" font-weight="bold">{org}'s GitHub Stats</text>
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

fn star_icon() -> &'static str {
    r#"<path d="M8 0.25a.75.75 0 0 1 .673.418l1.882 3.815 4.21.612a.75.75 0 0 1 .416 1.279l-3.046 2.97.719 4.192a.75.75 0 0 1-1.088.791L8 12.347l-3.766 1.98a.75.75 0 0 1-1.088-.79l.72-4.194L.818 6.374a.75.75 0 0 1 .416-1.28l4.21-.611L7.327.668A.75.75 0 0 1 8 .25z" transform="scale(0.9)"/>"#
}

fn fork_icon() -> &'static str {
    r#"<path d="M5 3.25a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0zm0 2.122a2.25 2.25 0 1 0-1.5 0v.878A2.25 2.25 0 0 0 5.75 8.5h1.5v2.128a2.251 2.251 0 1 0 1.5 0V8.5h1.5a2.25 2.25 0 0 0 2.25-2.25v-.878a2.25 2.25 0 1 0-1.5 0v.878a.75.75 0 0 1-.75.75h-4.5A.75.75 0 0 1 5 6.25v-.878zm3.75 7.378a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0zm3-8.75a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5z" transform="scale(0.9)"/>"#
}

fn repo_icon() -> &'static str {
    r#"<path d="M2 2.5A2.5 2.5 0 0 1 4.5 0h8.75a.75.75 0 0 1 .75.75v12.5a.75.75 0 0 1-.75.75h-2.5a.75.75 0 0 1 0-1.5h1.75v-2h-8a1 1 0 0 0-.714 1.7.75.75 0 1 1-1.072 1.05A2.495 2.495 0 0 1 2 11.5zm10.5-1h-8a1 1 0 0 0-1 1v6.708A2.486 2.486 0 0 1 4.5 9h8zM5 12.25a.25.25 0 0 1 .25-.25h3.5a.25.25 0 0 1 .25.25v3.25a.25.25 0 0 1-.4.2l-1.45-1.087a.249.249 0 0 0-.3 0L5.4 15.7a.25.25 0 0 1-.4-.2z" transform="scale(0.9)"/>"#
}

fn people_icon() -> &'static str {
    r#"<path d="M2 5.5a3.5 3.5 0 1 1 5.898 2.549 5.508 5.508 0 0 1 3.034 4.084.75.75 0 1 1-1.482.235 4.001 4.001 0 0 0-6.9 0 .75.75 0 0 1-1.482-.236A5.507 5.507 0 0 1 4.6 8.05 3.5 3.5 0 0 1 2 5.5zM5.5 4a2 2 0 1 0 0 4 2 2 0 0 0 0-4zm5.5.5a.75.75 0 0 1 .75-.75 2.5 2.5 0 0 1 0 5 .75.75 0 0 1 0-1.5 1 1 0 0 0 0-2 .75.75 0 0 1-.75-.75zm.75 6.5a.75.75 0 0 1 .75.75 3 3 0 0 1-3 3 .75.75 0 0 1 0-1.5 1.5 1.5 0 0 0 1.5-1.5.75.75 0 0 1 .75-.75z" transform="scale(0.9)"/>"#
}
