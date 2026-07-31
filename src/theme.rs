use std::sync::atomic::{ AtomicUsize, Ordering };

use ratatui::style::Color;

struct Palette {
    name: &'static str,
    primary: Color,
    secondary: Color,
    success: Color,
    muted: Color,
    danger: Color,
}

const PALETTES: [Palette; 3] = [
    Palette {
        name: "Nebula",
        primary: Color::Rgb(129, 140, 248),
        secondary: Color::Rgb(45, 212, 191),
        success: Color::Rgb(74, 222, 128),
        muted: Color::Rgb(100, 112, 134),
        danger: Color::Rgb(248, 113, 113),
    },
    Palette {
        name: "Solstice",
        primary: Color::Rgb(251, 146, 60),
        secondary: Color::Rgb(250, 204, 21),
        success: Color::Rgb(163, 230, 53),
        muted: Color::Rgb(120, 113, 108),
        danger: Color::Rgb(244, 63, 94),
    },
    Palette {
        name: "Matrix",
        primary: Color::Rgb(74, 222, 128),
        secondary: Color::Rgb(163, 230, 53),
        success: Color::Rgb(134, 239, 172),
        muted: Color::Rgb(82, 110, 82),
        danger: Color::Rgb(248, 113, 113),
    },
];

static PALETTE_INDEX: AtomicUsize = AtomicUsize::new(0);

fn current() -> &'static Palette {
    &PALETTES[PALETTE_INDEX.load(Ordering::Relaxed) % PALETTES.len()]
}

pub fn cycle() {
    let next = (PALETTE_INDEX.load(Ordering::Relaxed) + 1) % PALETTES.len();

    PALETTE_INDEX.store(next, Ordering::Relaxed);
}

pub fn name() -> &'static str {
    current().name
}

pub fn primary() -> Color {
    current().primary
}

pub fn secondary() -> Color {
    current().secondary
}

pub fn success() -> Color {
    current().success
}

pub fn muted() -> Color {
    current().muted
}

pub fn danger() -> Color {
    current().danger
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycling_visits_every_palette_once_and_wraps_back_to_the_start() {
        let starting_name = name();
        let mut seen = vec![starting_name];

        for _ in 1..PALETTES.len() {
            cycle();
            seen.push(name());
        }

        cycle();
        assert_eq!(name(), starting_name, "cycling exactly `PALETTES.len()` times should return to where it started");
        assert_eq!(seen.len(), PALETTES.len(), "should have visited every palette exactly once");
        assert!(seen.iter().all(|palette_name| seen.iter().filter(|other| other == &palette_name).count() == 1), "no palette should repeat before the full cycle is back");
    }
}
