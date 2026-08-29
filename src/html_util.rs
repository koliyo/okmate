pub fn wrap_article_tables(html: &str) -> String {
    const OPEN: &str = "<table";
    const CLOSE: &str = "</table>";
    const WRAP_OPEN: &str = r#"<div class="okmate-md-table">"#;
    const WRAP_CLOSE: &str = "</div>";
    let mut out = String::with_capacity(html.len() + 64);
    let mut rest = html;
    while let Some(start) = rest.find(OPEN) {
        let before = &rest[..start];
        out.push_str(before);
        let Some(end_rel) = rest[start..].find(CLOSE) else {
            out.push_str(&rest[start..]);
            return out;
        };
        let end = start + end_rel + CLOSE.len();
        if before.trim_end().ends_with(WRAP_OPEN) {
            out.push_str(&rest[start..end]);
        } else {
            out.push_str(WRAP_OPEN);
            out.push_str(&rest[start..end]);
            out.push_str(WRAP_CLOSE);
        }
        rest = &rest[end..];
    }
    out.push_str(rest);
    out
}

pub fn strip_leading_h1(html: &str) -> String {
    let trimmed = html.trim_start();
    let Some(after_open) = trimmed.strip_prefix("<h1") else {
        return html.to_string();
    };
    let Some(end) = after_open.find("</h1>") else {
        return html.to_string();
    };
    after_open[end + 5..].trim_start().to_string()
}

pub fn first_prose_paragraph(article_html: &str) -> String {
    let mut rest = article_html;
    loop {
        let trimmed = rest.trim_start();
        let Some(after_open) = trimmed.strip_prefix("<h") else {
            rest = trimmed;
            break;
        };
        let Some(end) = after_open.find("</h") else {
            rest = trimmed;
            break;
        };
        let after_end = &after_open[end..];
        let Some(close) = after_end.find('>') else {
            rest = trimmed;
            break;
        };
        rest = &after_end[close + 1..];
    }
    let rest = rest.trim_start();
    let Some(after_p) = rest.strip_prefix("<p") else {
        return String::new();
    };
    let Some(gt) = after_p.find('>') else {
        return String::new();
    };
    let inner = &after_p[gt + 1..];
    let Some(end) = inner.find("</p>") else {
        return String::new();
    };
    plaintext(&inner[..end])
}

fn plaintext(html: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::wrap_article_tables;

    #[test]
    fn wrap_article_tables_wraps_each_table_once() {
        let html = "<p>Lead</p>\n<table><tr><td>A</td></tr></table>\n<p>Mid</p>\n<table><thead><tr><th>H</th></tr></thead></table>";
        let wrapped = wrap_article_tables(html);
        assert_eq!(
            wrapped,
            "<p>Lead</p>\n<div class=\"okmate-md-table\"><table><tr><td>A</td></tr></table></div>\n<p>Mid</p>\n<div class=\"okmate-md-table\"><table><thead><tr><th>H</th></tr></thead></table></div>"
        );
        assert_eq!(wrap_article_tables(&wrapped), wrapped);
    }
}
