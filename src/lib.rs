use nu_ansi_term::{Color, Style};
use reedline::{Highlighter, StyledText};
use sqlparser::{dialect, keywords::Keyword, tokenizer::{Token, Tokenizer}};

/// SQL dialect.
#[derive(Debug, Default)]
pub enum Dialect {
    AnsiDialect,
    BigQueryDialect,
    ClickHouseDialect,
    DuckDbDialect,
    GenericDialect,
    HiveDialect,
    MsSqlDialect,
    MySqlDialect,
    #[default]
    PostgreSqlDialect,
    RedshiftSqlDialect,
    SQLiteDialect,
    SnowflakeDialect,
}

impl dialect::Dialect for Dialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        match self {
            Self::AnsiDialect => dialect::AnsiDialect {}.is_identifier_start(ch),
            Self::BigQueryDialect => dialect::BigQueryDialect {}.is_identifier_start(ch),
            Self::ClickHouseDialect => dialect::ClickHouseDialect {}.is_identifier_start(ch),
            Self::DuckDbDialect => dialect::DuckDbDialect {}.is_identifier_start(ch),
            Self::GenericDialect => dialect::GenericDialect {}.is_identifier_start(ch),
            Self::HiveDialect => dialect::HiveDialect {}.is_identifier_start(ch),
            Self::MsSqlDialect => dialect::MsSqlDialect {}.is_identifier_start(ch),
            Self::MySqlDialect => dialect::MySqlDialect {}.is_identifier_start(ch),
            Self::PostgreSqlDialect => dialect::PostgreSqlDialect {}.is_identifier_start(ch),
            Self::RedshiftSqlDialect => dialect::RedshiftSqlDialect {}.is_identifier_start(ch),
            Self::SQLiteDialect => dialect::SQLiteDialect {}.is_identifier_start(ch),
            Self::SnowflakeDialect => dialect::SnowflakeDialect {}.is_identifier_start(ch),
        }
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        match self {
            Self::AnsiDialect => dialect::AnsiDialect {}.is_identifier_part(ch),
            Self::BigQueryDialect => dialect::BigQueryDialect {}.is_identifier_part(ch),
            Self::ClickHouseDialect => dialect::ClickHouseDialect {}.is_identifier_part(ch),
            Self::DuckDbDialect => dialect::DuckDbDialect {}.is_identifier_part(ch),
            Self::GenericDialect => dialect::GenericDialect {}.is_identifier_part(ch),
            Self::HiveDialect => dialect::HiveDialect {}.is_identifier_part(ch),
            Self::MsSqlDialect => dialect::MsSqlDialect {}.is_identifier_part(ch),
            Self::MySqlDialect => dialect::MySqlDialect {}.is_identifier_part(ch),
            Self::PostgreSqlDialect => dialect::PostgreSqlDialect {}.is_identifier_part(ch),
            Self::RedshiftSqlDialect => dialect::RedshiftSqlDialect {}.is_identifier_part(ch),
            Self::SQLiteDialect => dialect::SQLiteDialect {}.is_identifier_part(ch),
            Self::SnowflakeDialect => dialect::SnowflakeDialect {}.is_identifier_part(ch),
        }
    }
}

/// SQL keyword highlighter, for [`reedline`].
pub struct SQLKeywordHighlighter {
    dialect: Dialect,
}

impl Default for SQLKeywordHighlighter {
    fn default() -> Self {
        Self {
            dialect: Dialect::default()
        }
    }
}

impl SQLKeywordHighlighter {
    /// Style for keyword.
    fn keyword_style() -> Style {
        Style {
            foreground: Some(Color::Green),
            background: None,
            is_bold: true,
            is_dimmed: false,
            is_italic: false,
            is_underline: false,
            is_blink: false,
            is_reverse: false,
            is_hidden: false,
            is_strikethrough: false,
            prefix_with_reset: false,
        }
    }

    /// Create a highlighter with the default option.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a highlighter with a specific dialect.
    pub fn new_with_dialect(dialect: Dialect) -> Self {
        Self {
            dialect
        }
    }
}

impl Highlighter for SQLKeywordHighlighter {
    #[allow(unused_variables)]
    fn highlight(&self, line: &str, cursor: usize) -> StyledText {
        let mut ret = StyledText::new();
        let mut tokenizer = Tokenizer::new(&self.dialect, line);

        match tokenizer.tokenize() {
            Ok(tokens) => {
                for token in tokens {
                    match token {
                        Token::Word(word) => match word.keyword {
                            Keyword::NoKeyword => ret.push((
                                Style::default(),
                                word.value.to_string(),
                            )),
                            _ => ret.push((
                                Self::keyword_style(),
                                word.value.to_uppercase(),
                            )),
                        },
                        other => {
                            ret.push((Style::default(), other.to_string()))
                        }
                    }
                }
            }
            Err(e) => ret.push((Style::default(), line.to_string()))
        }

        ret
    }
}
