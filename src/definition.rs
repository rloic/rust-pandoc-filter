use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pandoc {
    #[serde(rename = "pandoc-api-version")]
    pub version: Vec<u32>,
    pub meta: Meta,
    pub blocks: Vec<Block>,
}

pub type Meta = BTreeMap<String, MetaValue>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum MetaValue {
    MetaMap(Meta),
    MetaList(Vec<MetaValue>),
    MetaBool(bool),
    MetaString(String),
    MetaInlines(Vec<Inline>),
    MetaBlocks(Vec<Block>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum Block {
    Plain(Vec<Inline>),
    Para(Vec<Inline>),
    LineBlock(Vec<Vec<Inline>>),
    CodeBlock(Attr, String),
    RawBlock(Format, String),
    BlockQuote(Vec<Block>),
    OrderedList(ListAttributes, Vec<Vec<Block>>),
    BulletList(Vec<Vec<Block>>),
    DefinitionList(Vec<(Vec<Inline>, Vec<Vec<Block>>)>),
    Header(u8, Attr, Vec<Inline>),
    HorizontalRule,
    Table(Attr, Caption, Vec<ColSpec>, TableHead, Vec<TableBody>, TableFoot),
    Div(Attr, Vec<Block>),
    Null,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum Inline {
    Str(String),
    Emph(Vec<Inline>),
    Strong(Vec<Inline>),
    Strikeout(Vec<Inline>),
    Superscript(Vec<Inline>),
    Subscript(Vec<Inline>),
    SmallCaps(Vec<Inline>),
    Quoted(QuoteType, Vec<Inline>),
    Cite(Vec<Citation>, Vec<Inline>),
    Code(Attr, String),
    Space,
    SoftBreak,
    LineBreak,
    Math(MathType, String),
    RawInline(Format, String),
    Link(Attr, Vec<Inline>, Target),
    Image(Attr, Vec<Inline>, Target),
    Note(Vec<Block>),
    Span(Attr, Vec<Inline>),
}

pub type AttrList = Vec<(String, String)>;

pub type AttrJson = (String, Vec<String>, AttrList);

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(from = "AttrJson", into = "AttrJson")]
pub struct Attr {
    pub id: String,
    pub classes: Vec<String>,
    pub attributes: AttrList
}

impl From<AttrJson> for Attr {
    fn from(fields: AttrJson) -> Self {
        Attr { id: fields.0, classes: fields.1, attributes: fields.2 }
    }
}

impl Into<AttrJson> for Attr {
    fn into(self) -> AttrJson {
        (self.id, self.classes, self.attributes)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum ColWidth {
    ColWidth(f64),
    ColWidthDefault
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColSpec(pub Alignment, pub ColWidth);

pub type ShortCaption = Vec<Inline>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Caption(pub Option<ShortCaption>, pub Vec<Block>);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableHead(pub Attr, pub Vec<Row>);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Row(pub Attr, pub Vec<Cell>);

pub type RowSpan = u32;
pub type ColSpan = u32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cell(pub Attr, pub Alignment, pub RowSpan, pub ColSpan, pub Vec<Block>);

pub type RowHeadColumns = u32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableBody(pub Attr, pub RowHeadColumns, pub Vec<Row>, pub Vec<Row>);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableFoot(pub Attr, pub Vec<Row>);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "t")]
pub enum MathType { DisplayMath, InlineMath }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "t")]
pub enum QuoteType { SingleQuote, DoubleQuote }

pub type Target = (String, String);

pub type Format = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum CitationMode {
    AuthorInText,
    SuppressAuthor,
    NormalCitation,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Citation {
    #[serde(rename = "citationId")]
    pub citation_id: String,
    #[serde(rename = "citationPrefix")]
    pub citation_prefix: Vec<Inline>,
    #[serde(rename = "citationSuffix")]
    pub citation_suffix: Vec<Inline>,
    #[serde(rename = "citationMode")]
    pub citation_mode: CitationMode,
    #[serde(rename = "citationNoteNum")]
    pub citation_note_num: u64,
    #[serde(rename = "citationHash")]
    pub citation_hash: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum ListNumberStyle {
    DefaultStyle,
    Example,
    Decimal,
    LowerRoman,
    UpperRoman,
    LowerAlpha,
    UpperAlpha,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum ListNumberDelim {
    DefaultDelim,
    Period,
    OneParen,
    TwoParens,
}

pub type ListAttributes = (u64, ListNumberStyle, ListNumberDelim);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum Alignment {
    AlignLeft,
    AlignRight,
    AlignCenter,
    AlignDefault,
}