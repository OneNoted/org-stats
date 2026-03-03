pub struct Theme {
    pub bg: &'static str,
    pub text: &'static str,
    pub subtext: &'static str,
    pub accent: &'static str,
    pub green: &'static str,
    pub yellow: &'static str,
    pub peach: &'static str,
    pub red: &'static str,
    pub surface0: &'static str,
    pub surface1: &'static str,
    pub overlay0: &'static str,
}

pub const CATPPUCCIN_MOCHA: Theme = Theme {
    bg: "#1e1e2e",
    text: "#cdd6f4",
    subtext: "#a6adc8",
    accent: "#89b4fa",
    green: "#a6e3a1",
    yellow: "#f9e2af",
    peach: "#fab387",
    red: "#f38ba8",
    surface0: "#313244",
    surface1: "#45475a",
    overlay0: "#6c7086",
};

pub const CATPPUCCIN_LATTE: Theme = Theme {
    bg: "#eff1f5",
    text: "#4c4f69",
    subtext: "#6c6f85",
    accent: "#1e66f5",
    green: "#40a02b",
    yellow: "#df8e1d",
    peach: "#fe640b",
    red: "#d20f39",
    surface0: "#ccd0da",
    surface1: "#bcc0cc",
    overlay0: "#9ca0b0",
};

pub const DARK: Theme = Theme {
    bg: "#0d1117",
    text: "#c9d1d9",
    subtext: "#8b949e",
    accent: "#58a6ff",
    green: "#3fb950",
    yellow: "#d29922",
    peach: "#f0883e",
    red: "#f85149",
    surface0: "#161b22",
    surface1: "#21262d",
    overlay0: "#484f58",
};

pub fn get_theme(name: Option<&str>) -> &'static Theme {
    match name {
        Some("catppuccin_mocha" | "catppuccin-mocha") => &CATPPUCCIN_MOCHA,
        Some("catppuccin_latte" | "catppuccin-latte") => &CATPPUCCIN_LATTE,
        Some("dark") => &DARK,
        _ => &CATPPUCCIN_MOCHA,
    }
}
