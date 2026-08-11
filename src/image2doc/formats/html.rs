use crate::image2doc::analyzer::{ImageAnalysis, LayoutElement};
use crate::image2doc::classify::Classification;
use crate::image2doc::converter::Metadata;

pub fn render(
    metadata: &Metadata,
    analysis: &ImageAnalysis,
    extracted_text: &Option<String>,
    layout: &[LayoutElement],
    ascii_art: &str,
    classification: &Classification,
) -> String {
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str("<title>Image Analysis</title>\n");
    html.push_str("<style>\n");
    html.push_str("body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; max-width: 960px; margin: 0 auto; padding: 20px; background: #f5f5f5; color: #333; }\n");
    html.push_str(
        "h1 { color: #2c3e50; border-bottom: 2px solid #3498db; padding-bottom: 10px; }\n",
    );
    html.push_str("h2 { color: #2980b9; margin-top: 30px; }\n");
    html.push_str("h3 { color: #555; margin: 10px 0 5px; }\n");
    html.push_str("table { border-collapse: collapse; width: 100%; background: white; box-shadow: 0 1px 3px rgba(0,0,0,0.1); }\n");
    html.push_str(
        "th, td { padding: 8px 12px; text-align: left; border-bottom: 1px solid #ddd; }\n",
    );
    html.push_str("th { background: #3498db; color: white; }\n");
    html.push_str("tr:hover { background: #f1f1f1; }\n");
    html.push_str(".card { background: white; border-radius: 8px; padding: 20px; margin: 20px 0; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }\n");
    html.push_str(".badge { display: inline-block; padding: 4px 8px; border-radius: 4px; font-size: 12px; font-weight: bold; }\n");
    html.push_str(".badge-dark { background: #2c3e50; color: white; }\n");
    html.push_str(".badge-light { background: #f39c12; color: white; }\n");
    html.push_str(".color-swatch { display: inline-block; width: 16px; height: 16px; border-radius: 3px; margin-right: 6px; vertical-align: middle; border: 1px solid #ccc; }\n");
    html.push_str("pre { background: #2c3e50; color: #ecf0f1; padding: 15px; border-radius: 6px; overflow-x: auto; font-size: 11px; line-height: 1.4; }\n");
    html.push_str(".layout-item { display: inline-block; margin: 4px; padding: 6px 12px; border-radius: 16px; font-size: 13px; background: #e8f4f8; border: 1px solid #b6d4e6; }\n");
    html.push_str(".layout-row { display: flex; align-items: center; gap: 8px; padding: 6px 0; border-bottom: 1px solid #eee; font-size: 13px; }\n");
    html.push_str(".layout-type { font-weight: bold; min-width: 90px; }\n");
    html.push_str(".layout-pos { color: #666; min-width: 80px; font-family: monospace; }\n");
    html.push_str(".layout-size { color: #888; min-width: 70px; font-family: monospace; }\n");
    html.push_str(".layout-conf { color: #27ae60; min-width: 50px; }\n");
    html.push_str(".layout-font { color: #e67e22; min-width: 40px; font-family: monospace; }\n");
    html.push_str(".layout-text { color: #2c3e50; font-style: italic; flex: 1; overflow: hidden; text-overflow: ellipsis; }\n");
    html.push_str(".footer { margin-top: 40px; padding-top: 15px; border-top: 1px solid #ddd; font-size: 12px; color: #888; text-align: center; }\n");
    html.push_str("</style>\n</head>\n<body>\n");

    // Header
    html.push_str(&format!(
        "<h1>Image Analysis: {}</h1>\n",
        escape_html(&metadata.path)
    ));

    // Classification card
    html.push_str("<div class=\"card\">\n<h2>Image Classification</h2>\n");
    html.push_str(&format!(
        "<p><span class=\"badge badge-dark\">{}</span>",
        classification.image_type.label()
    ));
    html.push_str(&format!(
        " <strong>Confidence:</strong> {:.0}%</p>\n",
        classification.confidence * 100.0
    ));
    html.push_str(&format!(
        "<p><em>{}</em></p>\n",
        classification.image_type.description()
    ));
    html.push_str("</div>\n");

    // Metadata card
    html.push_str("<div class=\"card\">\n<h2>Metadata</h2>\n");
    html.push_str("<table>\n");
    html.push_str("<tr><th>Property</th><th>Value</th></tr>\n");
    html.push_str(&format!(
        "<tr><td>Format</td><td>{}</td></tr>\n",
        escape_html(&metadata.format)
    ));
    html.push_str(&format!(
        "<tr><td>Dimensions</td><td>{} x {} px</td></tr>\n",
        metadata.width, metadata.height
    ));
    html.push_str(&format!(
        "<tr><td>File size</td><td>{}</td></tr>\n",
        crate::explorer::human_readable_size(metadata.file_size)
    ));
    html.push_str(&format!(
        "<tr><td>Aspect ratio</td><td>{:.2}</td></tr>\n",
        metadata.aspect_ratio
    ));
    html.push_str("</table>\n</div>\n");

    // Color analysis
    html.push_str("<div class=\"card\">\n<h2>Color Analysis</h2>\n");
    let theme_class = if analysis.color_analysis.is_dark {
        "badge-dark"
    } else {
        "badge-light"
    };
    let theme_label = if analysis.color_analysis.is_dark {
        "Dark"
    } else {
        "Light"
    };
    html.push_str(&format!(
        "<p><span class=\"badge {}\">{}</span>",
        theme_class, theme_label
    ));
    html.push_str(&format!(
        " <strong>Brightness:</strong> {:.1}%",
        analysis.color_analysis.brightness * 100.0
    ));
    html.push_str(&format!(
        " <strong>Contrast ratio:</strong> {:.2}:1",
        analysis.color_analysis.contrast
    ));
    html.push_str(&format!(
        " <strong>Unique colors:</strong> {}",
        analysis.color_analysis.color_count
    ));
    html.push_str(&format!(
        " <strong>Gradient ratio:</strong> {:.1}%</p>\n",
        analysis.color_analysis.gradient_ratio * 100.0
    ));
    html.push_str("<h3>Dominant colors</h3>\n<div>\n");
    for (color, pct) in &analysis.color_analysis.dominant_colors {
        let hex = format!("#{:02x}{:02x}{:02x}", color[0], color[1], color[2]);
        let name = crate::image2doc::color::color_name(color[0], color[1], color[2]);
        html.push_str(&format!(
            "<span class=\"layout-item\"><span class=\"color-swatch\" style=\"background:{0}\"></span>{0} <strong>{2}</strong> - {1:.1}%</span>\n",
            hex, pct, name
        ));
    }
    html.push_str("</div>\n</div>\n");

    // Image analysis metrics
    html.push_str("<div class=\"card\">\n<h2>Image Analysis Metrics</h2>\n");
    html.push_str("<table>\n");
    html.push_str("<tr><th>Metric</th><th>Value</th></tr>\n");
    html.push_str(&format!(
        "<tr><td>Edge density</td><td>{:.1}%</td></tr>\n",
        analysis.edge_density * 100.0
    ));
    html.push_str(&format!(
        "<tr><td>Text density</td><td>{:.1}%</td></tr>\n",
        analysis.text_density * 100.0
    ));
    html.push_str(&format!(
        "<tr><td>Horizontal symmetry</td><td>{:.1}%</td></tr>\n",
        analysis.symmetry * 100.0
    ));
    html.push_str(&format!(
        "<tr><td>Gradient ratio</td><td>{:.1}%</td></tr>\n",
        analysis.color_analysis.gradient_ratio * 100.0
    ));
    html.push_str("</table>\n</div>\n");

    // ASCII preview
    html.push_str("<div class=\"card\">\n<h2>ASCII Preview</h2>\n");
    html.push_str("<pre>\n");
    html.push_str(&escape_html(ascii_art));
    html.push_str("</pre>\n</div>\n");

    // Extracted text
    if let Some(text) = extracted_text {
        html.push_str("<div class=\"card\">\n<h2>Extracted Text (OCR)</h2>\n");
        html.push_str("<pre>\n");
        html.push_str(&escape_html(text));
        html.push_str("</pre>\n</div>\n");
    }

    // Layout detection
    if !layout.is_empty() {
        html.push_str("<div class=\"card\">\n<h2>Detected Layout</h2>\n");
        for elem in layout.iter() {
            let type_str = match elem.element_type {
                crate::image2doc::analyzer::ElementType::Button => "Button",
                crate::image2doc::analyzer::ElementType::TextField => "Text Field",
                crate::image2doc::analyzer::ElementType::Heading => "Heading",
                crate::image2doc::analyzer::ElementType::Paragraph => "Paragraph",
                crate::image2doc::analyzer::ElementType::Image => "Image",
                crate::image2doc::analyzer::ElementType::Logo => "Logo",
                crate::image2doc::analyzer::ElementType::Separator => "Separator",
            };
            let x = elem.bounds.0;
            let y = elem.bounds.1;
            let w = elem.bounds.2;
            let h = elem.bounds.3;
            let conf = elem.confidence * 100.0;
            let font = elem
                .font_size_pt
                .map(|f| format!("{}pt", f))
                .unwrap_or_default();
            html.push_str(&format!(
                "<div class=\"layout-row\"><span class=\"layout-type\">{}</span><span class=\"layout-pos\">({},{})</span><span class=\"layout-size\">{}×{}</span><span class=\"layout-font\">{}</span><span class=\"layout-conf\">{:.0}%</span>",
                type_str, x, y, w, h, font, conf
            ));
            if let Some(ref text) = elem.text {
                html.push_str(&format!(
                    "<span class=\"layout-text\">“{}”</span>",
                    escape_html(text)
                ));
            }
            html.push_str("</div>\n");
        }
        html.push_str("</div>\n");
    }

    // Footer
    html.push_str("<div class=\"footer\">\n");
    html.push_str(&format!(
        "Generated by ntc v{} - Image to Document Converter\n",
        env!("CARGO_PKG_VERSION")
    ));
    html.push_str("</div>\n</body>\n</html>\n");

    html
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
