use egui::{Color32, Stroke, Visuals};

/// A named color set, picked from `catalog()` and applied app-wide via
/// `apply()`. Adding a new theme is just adding an entry to `catalog()` —
/// nothing else in the app has to change.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub name: &'static str,
    pub dark: bool,
    pub background: Color32,
    pub panel: Color32,
    pub accent: Color32,
    pub text: Color32,
}

/// `Color32::from_rgb` is a `const fn`, so this whole catalog can live as a
/// `'static` array instead of being rebuilt on every call.
const CATALOG: &[Theme] = &[
    Theme {
        name: "Krémová",
        dark: false,
        background: Color32::from_rgb(0xFB, 0xF3, 0xE7),
        panel: Color32::from_rgb(0xF3, 0xE9, 0xD8),
        accent: Color32::from_rgb(0xF2, 0x85, 0x22),
        text: Color32::from_rgb(0x3A, 0x2E, 0x26),
    },
    Theme {
        name: "Oceán",
        dark: false,
        background: Color32::from_rgb(0xE8, 0xF6, 0xF3),
        panel: Color32::from_rgb(0xD3, 0xEC, 0xE6),
        accent: Color32::from_rgb(0x2E, 0xC4, 0xB6),
        text: Color32::from_rgb(0x16, 0x3B, 0x38),
    },
    Theme {
        name: "Tmavá",
        dark: true,
        background: Color32::from_rgb(0x20, 0x1E, 0x1B),
        panel: Color32::from_rgb(0x2B, 0x28, 0x24),
        accent: Color32::from_rgb(0xFF, 0xD2, 0x3F),
        text: Color32::from_rgb(0xF0, 0xE8, 0xDA),
    },
];

pub fn catalog() -> &'static [Theme] {
    CATALOG
}

/// Applies a theme's colors app-wide via `egui::Visuals`. Every panel
/// (including the header, since `Panel::top` inherits `panel_fill`) picks
/// this up automatically — nothing panel/widget-specific needed. Call this
/// once when the theme changes, not every frame; the context remembers the
/// visuals until told otherwise.
pub fn apply(ctx: &egui::Context, theme: &Theme) {
    let mut visuals = if theme.dark {
        Visuals::dark()
    } else {
        Visuals::light()
    };

    visuals.panel_fill = theme.background;
    visuals.window_fill = theme.background;
    visuals.faint_bg_color = theme.panel;
    visuals.extreme_bg_color = theme.panel;

    // `bg_fill` is for widgets that must have a background (checkbox,
    // slider, radio button) — those still get a normal fill/highlight.
    visuals.widgets.inactive.bg_fill = theme.panel;
    visuals.widgets.hovered.bg_fill = theme.panel;
    visuals.widgets.active.bg_fill = theme.accent;

    // `weak_bg_fill` is what buttons/selectable labels actually paint
    // (see egui's `button_style()`). Keeping it identical across
    // inactive/hovered means hovering a button never paints a highlight
    // box behind it — only `fg_stroke` (below) changes, so hover reads as
    // a text-color change instead.
    visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    visuals.widgets.hovered.weak_bg_fill = Color32::TRANSPARENT;
    visuals.widgets.active.weak_bg_fill = theme.accent.linear_multiply(0.25);

    // `fg_stroke.color` is the actual text/icon color per widget state.
    // No `override_text_color` here on purpose — that field pins every
    // widget's text to one color regardless of state, which would silently
    // undo the hover-color change below.
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, theme.text);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, theme.text);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, theme.accent);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, theme.accent);

    visuals.selection.bg_fill = theme.accent.linear_multiply(0.55);
    visuals.selection.stroke = Stroke::new(1.0, theme.accent);

    ctx.set_visuals(visuals);
}
