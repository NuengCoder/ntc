// src/image2doc/formats/json.rs

use serde_json::json;
use crate::image2doc::converter::Metadata;
use crate::image2doc::analyzer::{ImageAnalysis, LayoutElement};
use crate::image2doc::classify::Classification;
use crate::image2doc::color::ColorAnalysis;

pub fn render(
    metadata: &Metadata,
    analysis: &ImageAnalysis,
    extracted_text: &Option<String>,
    layout: &[LayoutElement],
    ascii_art: &str,
    classification: &Classification,
) -> String {
    let mut json_obj = json!({
        "version": "2.3.0",
        "generator": "ntc-image2doc",
        "metadata": {
            "path": metadata.path,
            "format": metadata.format,
            "width": metadata.width,
            "height": metadata.height,
            "size_bytes": metadata.file_size,
            "size_human": crate::explorer::human_readable_size(metadata.file_size),
            "modified": metadata.modified,
            "aspect_ratio": metadata.aspect_ratio,
        },
        "analysis": {
            "colors": {
                "dominant": render_dominant_colors(&analysis.color_analysis),
                "brightness": analysis.color_analysis.brightness,
                "is_dark": analysis.color_analysis.is_dark,
                "contrast": analysis.color_analysis.contrast,
                "unique_colors": analysis.color_analysis.color_count,
                "gradient_ratio": analysis.color_analysis.gradient_ratio,
            },
            "edge_density": analysis.edge_density,
            "text_density": analysis.text_density,
            "symmetry": analysis.symmetry,
        },
        "classification": {
            "type": classification.image_type.label(),
            "description": classification.image_type.description(),
            "confidence": classification.confidence,
        },
        "layout": render_layout(layout),
        "ascii_preview": ascii_art.lines().take(20).collect::<Vec<&str>>().join("\n"),
    });
    
    if let Some(text) = extracted_text {
        json_obj["extracted_text"] = json!(text);
    }

    serde_json::to_string_pretty(&json_obj).unwrap()
}

fn render_dominant_colors(analysis: &ColorAnalysis) -> Vec<serde_json::Value> {
    analysis.dominant_colors.iter().map(|(color, pct)| {
        json!({
            "rgb": [color[0], color[1], color[2]],
            "hex": format!("#{:02x}{:02x}{:02x}", color[0], color[1], color[2]),
            "name": crate::image2doc::color::color_name(color[0], color[1], color[2]),
            "percentage": pct,
        })
    }).collect()
}

fn render_layout(elements: &[LayoutElement]) -> Vec<serde_json::Value> {
    elements.iter().map(|elem| {
        let type_str = match elem.element_type {
            crate::image2doc::analyzer::ElementType::Button => "button",
            crate::image2doc::analyzer::ElementType::TextField => "text_field",
            crate::image2doc::analyzer::ElementType::Heading => "heading",
            crate::image2doc::analyzer::ElementType::Paragraph => "paragraph",
            crate::image2doc::analyzer::ElementType::Image => "image",
            crate::image2doc::analyzer::ElementType::Logo => "logo",
            crate::image2doc::analyzer::ElementType::Separator => "separator",
        };
        
        let mut obj = json!({
            "type": type_str,
            "bounds": {
                "x": elem.bounds.0,
                "y": elem.bounds.1,
                "width": elem.bounds.2,
                "height": elem.bounds.3,
            },
            "confidence": elem.confidence,
        });
        
        if let Some(ref text) = elem.text {
            obj["text"] = json!(text);
        }
        
        if let Some(fs) = elem.font_size_pt {
            obj["font_size_pt"] = json!(fs);
        }
        
        obj
    }).collect()
}