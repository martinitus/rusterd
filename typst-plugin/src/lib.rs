use rusterd::ir::DetailLevel;
use rusterd::svg::Notation;
#[cfg(target_arch = "wasm32")]
use wasm_minimal_protocol::wasm_func;

#[cfg(target_arch = "wasm32")]
wasm_minimal_protocol::initiate_protocol!();

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[cfg_attr(target_arch = "wasm32", wasm_func)]
fn render(source: &[u8], view: &[u8], detail: &[u8], notation: &[u8]) -> Result<Vec<u8>, String> {
    let source = std::str::from_utf8(source)
        .map_err(|error| format!("invalid UTF-8 in ERD source: {error}"))?;
    let view = decode_optional(view, "view")?;
    let detail = decode_detail(detail)?;
    let notation = decode_notation(notation)?;

    Ok(rusterd::erd_to_svg(source, view.as_deref(), detail, notation)?.into_bytes())
}

#[allow(dead_code)]
fn decode_optional(value: &[u8], name: &str) -> Result<Option<String>, String> {
    if value.is_empty() {
        return Ok(None);
    }

    std::str::from_utf8(value)
        .map(|value| Some(value.to_owned()))
        .map_err(|error| format!("invalid UTF-8 in {name}: {error}"))
}

#[allow(dead_code)]
fn decode_detail(value: &[u8]) -> Result<DetailLevel, String> {
    if value.is_empty() {
        return Ok(DetailLevel::All);
    }

    let value =
        std::str::from_utf8(value).map_err(|error| format!("invalid UTF-8 in detail: {error}"))?;
    DetailLevel::from_str(value).ok_or_else(|| format!("invalid detail level: {value}"))
}

#[allow(dead_code)]
fn decode_notation(value: &[u8]) -> Result<Notation, String> {
    if value.is_empty() {
        return Ok(Notation::default());
    }

    let value = std::str::from_utf8(value)
        .map_err(|error| format!("invalid UTF-8 in notation: {error}"))?;
    Notation::from_str(value).ok_or_else(|| format!("invalid notation: {value}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_used_for_empty_options() {
        assert_eq!(decode_detail(b""), Ok(DetailLevel::All));
        assert_eq!(decode_notation(b""), Ok(Notation::CrowsFoot));
        assert_eq!(decode_optional(b"", "view"), Ok(None));
    }

    #[test]
    fn invalid_options_are_reported() {
        assert!(decode_detail(b"compact").is_err());
        assert!(decode_notation(b"compact").is_err());
        assert!(decode_optional(&[0xff], "view").is_err());
    }

    #[test]
    fn render_returns_svg_bytes() {
        let svg = render(b"entity User { id int pk }", b"", b"all", b"crowsfoot")
            .expect("valid ERD source should render");

        let svg = String::from_utf8(svg).expect("renderer output should be UTF-8");
        assert!(svg.starts_with("<svg "));
        assert!(svg.contains("User"));
    }
}
