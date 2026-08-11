use crate::image2doc::analyzer::{ElementType, ImageAnalysis, LayoutElement};
use crate::image2doc::classify::Classification;
use crate::image2doc::color;
use crate::image2doc::converter::Metadata;

/// Generate a rich, natural-language description of an image
/// that tells an AI what the image looks like in plain prose.
pub fn generate_description(
    metadata: &Metadata,
    analysis: &ImageAnalysis,
    extracted_text: &Option<String>,
    layout: &[LayoutElement],
    classification: &Classification,
) -> String {
    let mut desc = String::new();

    // ── Opening: image type ──────────────────────────────────────
    desc.push_str(&format!(
        "This image is a{article} **{}**{}",
        classification.image_type.description(),
        confidence_phrase(classification.confidence),
        article = vowel_sound(classification.image_type.description()),
    ));
    desc.push_str(&format!(
        " with dimensions {}×{} pixels, format {}, file size {}.",
        metadata.width,
        metadata.height,
        metadata.format,
        crate::explorer::human_readable_size(metadata.file_size),
    ));
    desc.push('\n');

    // ── Color and visual theme ───────────────────────────────────
    let theme = if analysis.color_analysis.is_dark {
        "dark"
    } else {
        "light"
    };
    let brightness_desc = brightness_phrase(analysis.color_analysis.brightness);
    let gradient_desc = if analysis.color_analysis.gradient_ratio > 0.15 {
        " It features smooth gradient transitions, giving it a polished, modern look."
    } else {
        " The colors are mostly flat with sharp transitions, typical of UI elements or illustrations."
    };
    desc.push_str(&format!(
        "The visual theme is **{}**{} with {} brightness.{}\n",
        theme,
        theme_detail(analysis.color_analysis.is_dark, classification),
        brightness_desc,
        gradient_desc,
    ));

    // Dominant colors
    if !analysis.color_analysis.dominant_colors.is_empty() {
        desc.push_str("The dominant colors are: ");
        let named: Vec<String> = analysis
            .color_analysis
            .dominant_colors
            .iter()
            .map(|(c, pct)| {
                format!(
                    "{} **{}** ({:.1}%)",
                    color_hex_preview(c[0], c[1], c[2]),
                    color::color_name(c[0], c[1], c[2]),
                    pct,
                )
            })
            .collect();
        desc.push_str(&named.join(", "));
        desc.push_str(".\n");
    }

    // ── Layout and structure (reading order) ─────────────────────
    if !layout.is_empty() {
        desc.push_str("\n**Layout structure (top-to-bottom, left-to-right):**\n");
        for (i, elem) in layout.iter().enumerate() {
            let x = elem.bounds.0;
            let y = elem.bounds.1;
            let w = elem.bounds.2;
            let h = elem.bounds.3;
            let pos_phrase = position_phrase(x, y, w, h, metadata.width, metadata.height);

            desc.push_str(&format!("{}. {}", i + 1, element_phrase(elem),));
            desc.push_str(&format!(" — located {} ", pos_phrase));

            if let Some(ref text) = elem.text {
                if !text.is_empty() {
                    desc.push_str(&format!("containing text: \"{}\"", text));
                }
            }

            let confidence_note = confidence_adverb(elem.confidence);
            if !confidence_note.is_empty() {
                desc.push_str(&format!(" ({})", confidence_note));
            }

            desc.push_str(".\n");
        }
    }

    // ── OCR text ─────────────────────────────────────────────────
    if let Some(text) = extracted_text {
        if !text.is_empty() {
            let preview: String = text.chars().take(120).collect();
            desc.push_str(&format!("\n**Text content:** \"{}...\"\n", preview,));
        }
    }

    // ── Summary ──────────────────────────────────────────────────
    desc.push_str(&format!(
        "\n*This image has {edge} edge density, {text} text density, \
         and {sym} horizontal symmetry.{extra}*",
        edge = density_level(analysis.edge_density),
        text = density_level(analysis.text_density),
        sym = sym_level(analysis.symmetry),
        extra = if analysis.color_analysis.contrast > 4.5 {
            " It has strong contrast, making content highly readable."
        } else if analysis.color_analysis.contrast < 2.0 {
            " The contrast is low, giving a softer, more muted appearance."
        } else {
            ""
        },
    ));

    desc
}

// ── helpers ─────────────────────────────────────────────────────

fn vowel_sound(s: &str) -> &'static str {
    let starts_with_vowel = s.starts_with(['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']);
    if starts_with_vowel {
        "n"
    } else {
        ""
    }
}

fn confidence_phrase(conf: f32) -> &'static str {
    if conf > 0.9 {
        " (high confidence)"
    } else if conf > 0.7 {
        " (moderate confidence)"
    } else if conf > 0.5 {
        " (fair confidence)"
    } else {
        " (low confidence)"
    }
}

fn confidence_adverb(conf: f32) -> &'static str {
    if conf > 0.85 {
        "confidently detected"
    } else if conf > 0.6 {
        "likely"
    } else if conf > 0.4 {
        "possibly"
    } else {
        ""
    }
}

fn brightness_phrase(b: f32) -> &'static str {
    if b > 0.8 {
        "very bright"
    } else if b > 0.6 {
        "bright"
    } else if b > 0.4 {
        "moderate"
    } else if b > 0.2 {
        "dim"
    } else {
        "very dark"
    }
}

fn density_level(d: f32) -> &'static str {
    if d > 0.5 {
        "high"
    } else if d > 0.3 {
        "moderate"
    } else {
        "low"
    }
}

fn sym_level(s: f32) -> &'static str {
    if s > 0.8 {
        "strong"
    } else if s > 0.5 {
        "moderate"
    } else {
        "weak"
    }
}

fn theme_detail(is_dark: bool, classification: &Classification) -> &'static str {
    use crate::image2doc::classify::ImageType;
    if is_dark {
        match classification.image_type {
            ImageType::Screenshot => " theme typical of developer tools or night-mode apps",
            ImageType::UiMockup => " theme suggesting a sleek, modern UI design",
            ImageType::Photo => " mood with shadowy or nocturnal tones",
            ImageType::Document => " presentation (dark-mode document viewer)",
            ImageType::Logo => " background, common for light-colored logos",
        }
    } else {
        match classification.image_type {
            ImageType::Screenshot => " theme typical of productivity apps or web pages",
            ImageType::UiMockup => " theme for a clean, professional design",
            ImageType::Photo => " and bright scene",
            ImageType::Document => " document on a white or light background",
            ImageType::Logo => " background making the logo stand out",
        }
    }
}

fn element_phrase(elem: &LayoutElement) -> String {
    let name = match elem.element_type {
        ElementType::Button => "a **button**",
        ElementType::TextField => "a **text input field**",
        ElementType::Heading => "a **heading**",
        ElementType::Paragraph => "a **text block**",
        ElementType::Image => "an **image region**",
        ElementType::Logo => "a **logo**",
        ElementType::Separator => "a **divider or separator**",
    };

    let font_hint = elem
        .font_size_pt
        .map(|f| format!(" (est. {}pt font)", f))
        .unwrap_or_default();

    format!("{}{}", name, font_hint)
}

fn position_phrase(x: u32, y: u32, w: u32, h: u32, img_w: u32, img_h: u32) -> String {
    let cx = x as f32 + w as f32 / 2.0;
    let cy = y as f32 + h as f32 / 2.0;

    let horiz = if cx < img_w as f32 / 3.0 {
        "left side"
    } else if cx > img_w as f32 * 2.0 / 3.0 {
        "right side"
    } else {
        "center"
    };

    let vert = if cy < img_h as f32 / 3.0 {
        "top"
    } else if cy > img_h as f32 * 2.0 / 3.0 {
        "bottom"
    } else {
        "middle"
    };

    format!("at {} of the {}", horiz, vert)
}

fn color_hex_preview(r: u8, g: u8, b: u8) -> String {
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}
