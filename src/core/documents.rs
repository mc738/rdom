use std::iter::Map;

#[derive(Debug)]
pub enum Block {
    Header(HeaderBlock),
    Paragraph(ParagraphBlock),
    Code(CodeBlock),
    List(ListBlock),
    Image(ImageBlock),
}

#[derive(Debug)]
pub enum HeaderLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Debug)]
pub struct HeaderBlock {
    pub(crate) style: Style,
    pub(crate) level: HeaderLevel,
    pub(crate) content: Vec<InlineContent>,
    pub(crate) indexed: bool,
}

#[derive(Debug)]
pub struct ParagraphBlock {
    pub(crate) style: Style,
    pub(crate) content: Vec<InlineContent>,
}

#[derive(Debug)]
pub struct CodeBlock {
    pub(crate) style: Style,
    pub(crate) content: String,
    pub(crate) language: Option<String>,
}

#[derive(Debug)]
pub struct ListBlock {
    pub(crate) ordered: bool,
    pub(crate) style: Style,
    pub(crate) items: Vec<ListItem>,
}

#[derive(Debug)]
pub struct ListItem {
    pub(crate) style: Style,
    pub(crate) content: Vec<InlineContent>,
}

#[derive(Debug)]
pub struct ImageBlock {
    pub(crate) style: Style,
    pub(crate) source: String,
    pub(crate) title: String,
    pub(crate) alt_text: String,
    pub(crate) height: Option<String>,
    pub(crate) width: Option<String>,
}

#[derive(Debug)]
pub enum Style {
    Ref(Vec<String>),
    Custom(Map<String, String>),
    Default,
}

#[derive(Debug)]
pub enum InlineContent {
    Text(InlineText),
    Span(InlineSpan),
    Link(InlineLink),
}

#[derive(Debug)]
pub struct InlineText {
    pub(crate) content: String,
}

#[derive(Debug)]
pub struct InlineSpan {
    pub(crate) content: String,
    pub(crate) style: Style,
}

#[derive(Debug)]
pub struct InlineLink {
    pub(crate) content: String,
    pub(crate) url: String,
    pub(crate) style: Style,
}

impl Block {
    pub fn header(block: HeaderBlock) -> Block {
        Block::Header(block)
    }

    pub fn paragraph(block: ParagraphBlock) -> Block {
        Block::Paragraph(block)
    }

    pub fn code(block: CodeBlock) -> Block {
        Block::Code(block)
    }

    pub fn list(block: ListBlock) -> Block {
        Block::List(block)
    }

    pub fn image(block: ImageBlock) -> Block {
        Block::Image(block)
    }
}

impl HeaderBlock {
    pub fn new(
        style: Style,
        level: HeaderLevel,
        content: Vec<InlineContent>,
        indexed: bool,
    ) -> HeaderBlock {
        HeaderBlock {
            style,
            level,
            content,
            indexed,
        }
    }

    pub fn h1(style: Style, content: Vec<InlineContent>, indexed: bool) -> HeaderBlock {
        HeaderBlock::new(style, HeaderLevel::H1, content, indexed)
    }

    pub fn h2(style: Style, content: Vec<InlineContent>, indexed: bool) -> HeaderBlock {
        HeaderBlock::new(style, HeaderLevel::H2, content, indexed)
    }

    pub fn h3(style: Style, content: Vec<InlineContent>, indexed: bool) -> HeaderBlock {
        HeaderBlock::new(style, HeaderLevel::H3, content, indexed)
    }

    pub fn h4(style: Style, content: Vec<InlineContent>, indexed: bool) -> HeaderBlock {
        HeaderBlock::new(style, HeaderLevel::H4, content, indexed)
    }

    pub fn h5(style: Style, content: Vec<InlineContent>, indexed: bool) -> HeaderBlock {
        HeaderBlock::new(style, HeaderLevel::H5, content, indexed)
    }

    pub fn h6(style: Style, content: Vec<InlineContent>, indexed: bool) -> HeaderBlock {
        HeaderBlock::new(style, HeaderLevel::H6, content, indexed)
    }
}

impl ParagraphBlock {
    pub fn new(style: Style, content: Vec<InlineContent>) -> ParagraphBlock {
        ParagraphBlock { style, content }
    }
}

impl CodeBlock {
    pub fn new(style: Style, content: String, language: Option<String>) -> CodeBlock {
        CodeBlock {
            style,
            content,
            language,
        }
    }
}

impl ListBlock {
    pub fn new(style: Style, ordered: bool, items: Vec<ListItem>) -> ListBlock {
        ListBlock {
            ordered,
            style,
            items,
        }
    }

    pub fn new_ordered(style: Style, items: Vec<ListItem>) -> ListBlock {
        ListBlock::new(style, true, items)
    }

    pub fn new_unordered(style: Style, items: Vec<ListItem>) -> ListBlock {
        ListBlock::new(style, false, items)
    }
}

impl ListItem {
    pub fn new(style: Style, content: Vec<InlineContent>) -> ListItem {
        ListItem { style, content }
    }
}

impl ImageBlock {
    pub fn new(
        style: Style,
        source: String,
        title: String,
        alt_text: String,
        height: Option<String>,
        width: Option<String>,
    ) -> ImageBlock {
        ImageBlock {
            style,
            source,
            title,
            alt_text,
            height,
            width,
        }
    }
}

impl InlineText {
    pub fn new(content: String) -> InlineText {
        InlineText { content }
    }
}

impl InlineSpan {
    pub fn new(style: Style, content: String) -> InlineSpan {
        InlineSpan { content, style }
    }
}

impl InlineLink {
    pub fn new(style: Style, url: String, content: String) -> InlineLink {
        InlineLink {
            content,
            url,
            style,
        }
    }
}

impl Style {
    pub fn create_ref(classes: Vec<String>) -> Style {
        Style::Ref(classes)
    }

    pub fn create_custom(values: Map<String, String>) -> Style {
        Style::Custom(values)
    }

    pub fn default() -> Style {
        Style::Default
    }
}
