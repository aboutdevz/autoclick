#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsedHotkey {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub vk_code: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyEvent {
    Pressed,
    Released,
}

pub fn parse_hotkey(input: &str) -> Option<ParsedHotkey> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    let mut ctrl = false;
    let mut alt = false;
    let mut shift = false;
    let mut main_key: Option<&str> = None;

    for part in trimmed.split('+') {
        let token = part.trim();
        if token.is_empty() {
            continue;
        }
        match token.to_ascii_lowercase().as_str() {
            "ctrl" | "control" => ctrl = true,
            "alt" => alt = true,
            "shift" => shift = true,
            _ => main_key = Some(token),
        }
    }

    let vk_code = key_name_to_vk(main_key?)?;
    Some(ParsedHotkey {
        ctrl,
        alt,
        shift,
        vk_code,
    })
}

fn key_name_to_vk(name: &str) -> Option<u32> {
    let upper = name.to_ascii_uppercase();

    // Single ASCII letter A-Z (0x41..=0x5A) or digit 0-9 (0x30..=0x39)
    if upper.len() == 1 {
        let b = upper.as_bytes()[0];
        if b.is_ascii_uppercase() || b.is_ascii_digit() {
            return Some(b as u32);
        }
    }

    // Function keys F1-F24 (0x70..=0x87)
    if let Some(rest) = upper.strip_prefix('F') {
        if let Ok(n) = rest.parse::<u32>() {
            if (1..=24).contains(&n) {
                return Some(0x70 + (n - 1));
            }
        }
    }

    match upper.as_str() {
        "SPACE" => Some(0x20),
        "ENTER" | "RETURN" => Some(0x0D),
        "TAB" => Some(0x09),
        "BACKSPACE" => Some(0x08),
        "ESCAPE" | "ESC" => Some(0x1B),
        "INSERT" => Some(0x2D),
        "DELETE" | "DEL" => Some(0x2E),
        "HOME" => Some(0x24),
        "END" => Some(0x23),
        "PAGEUP" => Some(0x21),
        "PAGEDOWN" => Some(0x22),
        "ARROWUP" | "UP" => Some(0x26),
        "ARROWDOWN" | "DOWN" => Some(0x28),
        "ARROWLEFT" | "LEFT" => Some(0x25),
        "ARROWRIGHT" | "RIGHT" => Some(0x27),
        "CAPSLOCK" => Some(0x14),
        "PAUSE" => Some(0x13),
        "SCROLLLOCK" => Some(0x91),
        "NUMPAD0" => Some(0x60),
        "NUMPAD1" => Some(0x61),
        "NUMPAD2" => Some(0x62),
        "NUMPAD3" => Some(0x63),
        "NUMPAD4" => Some(0x64),
        "NUMPAD5" => Some(0x65),
        "NUMPAD6" => Some(0x66),
        "NUMPAD7" => Some(0x67),
        "NUMPAD8" => Some(0x68),
        "NUMPAD9" => Some(0x69),
        "MULTIPLY" | "*" => Some(0x6A),
        "ADD" | "+" => Some(0x6B),
        "SUBTRACT" | "-" => Some(0x6D),
        "DECIMAL" | "." => Some(0x6E),
        "DIVIDE" | "/" => Some(0x6F),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_function_keys_and_modifiers() {
        assert_eq!(
            parse_hotkey("F6"),
            Some(ParsedHotkey {
                ctrl: false,
                alt: false,
                shift: false,
                vk_code: 0x75,
            })
        );
        assert_eq!(
            parse_hotkey("Ctrl+Shift+A"),
            Some(ParsedHotkey {
                ctrl: true,
                alt: false,
                shift: true,
                vk_code: 0x41,
            })
        );
        assert_eq!(
            parse_hotkey("Alt+Space"),
            Some(ParsedHotkey {
                ctrl: false,
                alt: true,
                shift: false,
                vk_code: 0x20,
            })
        );
        assert_eq!(parse_hotkey(""), None);
    }
}
