use crate::github::OrgData;
use crate::theme::Theme;

pub fn render(data: &OrgData, org: &str, theme: &Theme, hide_border: bool, langs_count: usize) -> String {
    let count = langs_count.min(data.languages.len()).max(1);
    let langs = &data.languages[..count];
    let total_bytes: u64 = langs.iter().map(|l| l.bytes).sum();

    if total_bytes == 0 {
        return empty_card(org, theme, hide_border);
    }

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

    // Build the language bar segments
    let bar_width = 445.0f64;
    let bar_y = 44;
    let mut bar_svg = String::new();
    let mut x_offset = 25.0f64;

    for lang in langs {
        let pct = lang.bytes as f64 / total_bytes as f64;
        let w = (pct * bar_width).max(1.0);
        let rx = if (x_offset - 25.0).abs() < 0.01 { "3" } else { "0" };
        bar_svg.push_str(&format!(
            r#"<rect x="{x_offset:.1}" y="{bar_y}" width="{w:.1}" height="8" rx="{rx}" fill="{}"/>"#,
            lang.color
        ));
        x_offset += w;
    }

    // Build language legend (2 columns)
    let mut legend_svg = String::new();
    for (i, lang) in langs.iter().enumerate() {
        let pct = lang.bytes as f64 / total_bytes as f64 * 100.0;
        let col = i % 2;
        let row = i / 2;
        let x = 25 + col * 225;
        let y = 72 + row * 25;
        legend_svg.push_str(&format!(
            r#"<g transform="translate({x}, {y})">
  <circle cx="5" cy="6" r="5" fill="{}"/>
  <text x="16" y="10" fill="{}" font-size="12">{} <tspan fill="{}">{:.1}%</tspan></text>
</g>"#,
            lang.color, theme.text, lang.name, theme.subtext, pct
        ));
    }

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="495" height="195" viewBox="0 0 495 195">
  <style>
    text {{ font-family: 'Segoe UI', Ubuntu, 'Helvetica Neue', sans-serif; }}
  </style>
  {bg}
  {border}
  <text x="25" y="32" fill="{}" font-size="16" font-weight="bold">{org}'s Top Languages</text>
  {bar_svg}
  {legend_svg}
</svg>"#,
        theme.text
    )
}

fn empty_card(org: &str, theme: &Theme, hide_border: bool) -> String {
    let border = if hide_border {
        ""
    } else {
        &format!(
            r#"<rect x="0.5" y="0.5" width="494" height="194" rx="4.5" fill="{}" stroke="{}"/>"#,
            theme.bg, theme.surface1
        )
    };

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="495" height="195" viewBox="0 0 495 195">
  <style>
    text {{ font-family: 'Segoe UI', Ubuntu, 'Helvetica Neue', sans-serif; }}
  </style>
  {border}
  <text x="25" y="32" fill="{}" font-size="16" font-weight="bold">{org}'s Top Languages</text>
  <text x="247" y="110" fill="{}" font-size="14" text-anchor="middle">No language data available</text>
</svg>"#,
        theme.text, theme.subtext
    )
}
