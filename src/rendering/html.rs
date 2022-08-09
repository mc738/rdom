use crate::core::documents::{
    Block, CodeBlock, HeaderBlock, ImageBlock, InlineContent, ListBlock, ListItem, ParagraphBlock,
    Style,
};

fn render_style(style: Style) -> String {
    match style {
        Style::Ref(c) => format!(" class='{}'", c.join(" ")),
        Style::Custom(m) => {
            format!(
                " style='{}'",
                m.into_iter()
                    .map(|(k, v)| { format!("{}: {}", k, v) })
                    .collect::<Vec<String>>()
                    .join("; ")
            )
        }
        Style::Default => "".to_string(),
    }
}

fn render_inline_content(content: Vec<InlineContent>) -> String {
    content
        .into_iter()
        .map(|c| match c {
            InlineContent::Text(t) => t.content,
            InlineContent::Span(s) => {
                format!("<span{}>{}</span>", render_style(s.style), s.content)
            }
            InlineContent::Link(l) => format!(
                "<a href='{}'{}>{}</span>",
                l.url,
                render_style(l.style),
                l.content
            ),
        })
        .collect::<Vec<String>>()
        .join("")
}

fn render_header(block: HeaderBlock) -> String {
    let tag = match block.level {
        crate::core::documents::HeaderLevel::H1 => "h1",
        crate::core::documents::HeaderLevel::H2 => "h2",
        crate::core::documents::HeaderLevel::H3 => "h3",
        crate::core::documents::HeaderLevel::H4 => "h4",
        crate::core::documents::HeaderLevel::H5 => "h5",
        crate::core::documents::HeaderLevel::H6 => "h6",
    };

    format!(
        "<{}{}>{}</{}>",
        tag,
        render_style(block.style),
        render_inline_content(block.content),
        tag
    )
}

fn render_paragraph(block: ParagraphBlock) -> String {
    format!(
        "<p{}>{}</p>",
        render_style(block.style),
        render_inline_content(block.content)
    )
}

fn render_code_block(block: CodeBlock) -> String {
    let lang = match block.language {
        Some(l) => format!(" class='language-{}'", l),
        None => "".to_string(),
    };

    format!("<pre{}><code>{}</code></pre>", lang, block.content)
}

fn render_list_item(item: ListItem) -> String {
    format!(
        "<li{}>{}</li>",
        render_style(item.style),
        render_inline_content(item.content)
    )
}

fn render_list_block(block: ListBlock) -> String {
    let tag = match block.ordered {
        true => "ol",
        false => "ul",
    };

    format!(
        "<{}{}>{}</{}>",
        tag,
        render_style(block.style),
        block
            .items
            .into_iter()
            .map(|i| render_list_item(i))
            .collect::<Vec<String>>()
            .join(""),
        tag
    )
}

fn render_image_block(block: ImageBlock) -> String {
    let h = match block.height {
        Some(h) => format!(" height='{}'", h),
        None => "".to_string(),
    };

    let w = match block.width {
        Some(w) => format!(" width='{}'", w),
        None => "".to_string(),
    };

    format!(
        "<img src='{}' alt='{}' title='{}'{}{}>",
        block.source, block.alt_text, block.title, h, w
    )
}

pub fn render(blocks: Vec<Block>) -> Vec<String> {
    blocks
        .into_iter()
        .map(|b| match b {
            Block::Header(h) => render_header(h),
            Block::Paragraph(p) => render_paragraph(p),
            Block::Code(c) => render_code_block(c),
            Block::List(l) => render_list_block(l),
            Block::Image(i) => render_image_block(i),
        })
        .collect::<Vec<String>>()
}
