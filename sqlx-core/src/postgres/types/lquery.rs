use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::postgres::{PgArgumentBuffer, PgTypeInfo, PgValueFormat, PgValueRef, Postgres};
use crate::types::Type;
use bitflags::bitflags;
use std::fmt::{self, Display, Formatter};
use std::io::Write;
use std::ops::Deref;
use std::str::FromStr;

use crate::postgres::types::ltree::{PgLTreeLabel, PgLTreeParseError};

/// Represents lquery specific errors
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum PgLQueryParseError {
    #[error("lquery cannot be empty")]
    EmptyString,
    #[error("unexpected character in lquery")]
    UnexpectedCharacter,
    #[error("error parsing integer: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("error parsing integer: {0}")]
    LTreeParrseError(#[from] PgLTreeParseError),
    /// LQuery version not supported
    #[error("lquery version not supported")]
    InvalidLqueryVersion,
}

/// Container for a Label Tree Query (`lquery`) in Postgres.
///
/// See https://www.postgresql.org/docs/current/ltree.html
///
/// ### Note: Requires Postgres 13+
///
/// This integration requires that the `lquery` type support the binary format in the Postgres
/// wire protocol, which only became available in Postgres 13.
/// ([Postgres 13.0 Release Notes, Additional Modules][https://www.postgresql.org/docs/13/release-13.html#id-1.11.6.11.5.14])
///
/// Ideally, SQLx's Postgres driver should support falling back to text format for types
/// which don't have `typsend` and `typrecv` entries in `pg_type`, but that work still needs
/// to be done.
///
/// ### Note: Extension Required
/// The `ltree` extension is not enabled by default in Postgres. You will need to do so explicitly:
///
/// ```ignore
/// CREATE EXTENSION IF NOT EXISTS "ltree";
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PgLQuery {
    levels: Vec<PgLQueryLevel>,
}

// TODO: maybe a QueryBuilder pattern would be nice here
impl PgLQuery {
    /// creates default/empty lquery
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(levels: Vec<PgLQueryLevel>) -> Self {
        Self { levels }
    }

    /// push a query level
    pub fn push(&mut self, level: PgLQueryLevel) {
        self.levels.push(level);
    }

    /// pop a query level
    pub fn pop(&mut self) -> Option<PgLQueryLevel> {
        self.levels.pop()
    }

    /// creates lquery from an iterator with checking labels
    pub fn from_iter<I, S>(levels: I) -> Result<Self, PgLQueryParseError>
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        let mut lquery = Self::default();
        for level in levels {
            lquery.push(PgLQueryLevel::from_str(&level.into())?);
        }
        Ok(lquery)
    }
}

impl IntoIterator for PgLQuery {
    type Item = PgLQueryLevel;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.levels.into_iter()
    }
}

impl FromStr for PgLQuery {
    type Err = PgLQueryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            levels: s
                .split('.')
                .map(|s| PgLQueryLevel::from_str(s))
                .collect::<Result<_, Self::Err>>()?,
        })
    }
}

impl Display for PgLQuery {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut iter = self.levels.iter();
        if let Some(label) = iter.next() {
            write!(f, "{}", label)?;
            for label in iter {
                write!(f, ".{}", label)?;
            }
        }
        Ok(())
    }
}

impl Deref for PgLQuery {
    type Target = [PgLQueryLevel];

    fn deref(&self) -> &Self::Target {
        &self.levels
    }
}

impl Type<Postgres> for PgLQuery {
    fn type_info() -> PgTypeInfo {
        // Since `ltree` is enabled by an extension, it does not have a stable OID.
        PgTypeInfo::with_name("lquery")
    }
}

impl Encode<'_, Postgres> for PgLQuery {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        buf.extend(1i8.to_le_bytes());
        write!(buf, "{}", self)
            .expect("Display implementation panicked while writing to PgArgumentBuffer");

        IsNull::No
    }
}

impl<'r> Decode<'r, Postgres> for PgLQuery {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.format() {
            PgValueFormat::Binary => {
                let bytes = value.as_bytes()?;
                let version = i8::from_le_bytes([bytes[0]; 1]);
                if version != 1 {
                    return Err(Box::new(PgLQueryParseError::InvalidLqueryVersion));
                }
                Ok(Self::from_str(std::str::from_utf8(&bytes[1..])?)?)
            }
            PgValueFormat::Text => Ok(Self::from_str(value.as_str()?)?),
        }
    }
}

bitflags! {
    /// Modifiers that can be set to non-star labels
    pub struct PgLQueryVariantFlag: u16 {
        /// * - Match any label with this prefix, for example foo* matches foobar
        const ANY_END = 0x01;
        /// @ - Match case-insensitively, for example a@ matches A
        const IN_CASE = 0x02;
        /// % - Match initial underscore-separated words
        const SUBLEXEME = 0x04;
    }
}

impl Display for PgLQueryVariantFlag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.contains(PgLQueryVariantFlag::ANY_END) {
            write!(f, "*")?;
        }
        if self.contains(PgLQueryVariantFlag::IN_CASE) {
            write!(f, "@")?;
        }
        if self.contains(PgLQueryVariantFlag::SUBLEXEME) {
            write!(f, "%")?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PgLQueryVariant {
    label: PgLTreeLabel,
    modifiers: PgLQueryVariantFlag,
}

impl Display for PgLQueryVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.label, self.modifiers)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PgLQueryLevel {
    /// match any label (*) with optional at least / at most numbers
    Star(Option<u16>, Option<u16>),
    /// match any of specified labels with optional flags
    NonStar(Vec<PgLQueryVariant>),
    /// match none of specified labels with optional flags
    NotNonStar(Vec<PgLQueryVariant>),
}

impl FromStr for PgLQueryLevel {
    type Err = PgLQueryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.is_empty() {
            Err(PgLQueryParseError::EmptyString)
        } else {
            match bytes[0] {
                b'*' => {
                    if bytes.len() > 1 {
                        let parts = s[2..s.len() - 1].split(',').collect::<Vec<_>>();
                        match parts.len() {
                            1 => {
                                let number = parts[0].parse()?;
                                Ok(PgLQueryLevel::Star(Some(number), Some(number)))
                            }
                            2 => Ok(PgLQueryLevel::Star(
                                Some(parts[0].parse()?),
                                Some(parts[1].parse()?),
                            )),
                            _ => Err(PgLQueryParseError::UnexpectedCharacter),
                        }
                    } else {
                        Ok(PgLQueryLevel::Star(None, None))
                    }
                }
                b'!' => Ok(PgLQueryLevel::NotNonStar(
                    s[1..]
                        .split('|')
                        .map(|s| PgLQueryVariant::from_str(s))
                        .collect::<Result<Vec<_>, PgLQueryParseError>>()?,
                )),
                _ => Ok(PgLQueryLevel::NonStar(
                    s.split('|')
                        .map(|s| PgLQueryVariant::from_str(s))
                        .collect::<Result<Vec<_>, PgLQueryParseError>>()?,
                )),
            }
        }
    }
}

impl FromStr for PgLQueryVariant {
    type Err = PgLQueryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut label_length = s.len();
        let mut rev_iter = s.bytes().rev();
        let mut modifiers = PgLQueryVariantFlag { bits: 0 };

        while let Some(b) = rev_iter.next() {
            match b {
                b'@' => modifiers.insert(PgLQueryVariantFlag::IN_CASE),
                b'*' => modifiers.insert(PgLQueryVariantFlag::ANY_END),
                b'%' => modifiers.insert(PgLQueryVariantFlag::SUBLEXEME),
                _ => break,
            }
            label_length -= 1;
        }

        Ok(PgLQueryVariant {
            label: PgLTreeLabel::new(&s[0..label_length])?,
            modifiers,
        })
    }
}

fn write_variants(f: &mut Formatter<'_>, variants: &[PgLQueryVariant], not: bool) -> fmt::Result {
    let mut iter = variants.iter();
    if let Some(variant) = iter.next() {
        write!(f, "{}{}", if not { "!" } else { "" }, variant)?;
        for variant in iter {
            write!(f, ".{}", variant)?;
        }
    }
    Ok(())
}

impl Display for PgLQueryLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PgLQueryLevel::Star(Some(at_least), Some(at_most)) => {
                if at_least == at_most {
                    write!(f, "*{{{}}}", at_least)
                } else {
                    write!(f, "*{{{},{}}}", at_least, at_most)
                }
            }
            PgLQueryLevel::Star(Some(at_least), _) => write!(f, "*{{{},}}", at_least),
            PgLQueryLevel::Star(_, Some(at_most)) => write!(f, "*{{,{}}}", at_most),
            PgLQueryLevel::Star(_, _) => write!(f, "*"),
            PgLQueryLevel::NonStar(variants) => write_variants(f, &variants, false),
            PgLQueryLevel::NotNonStar(variants) => write_variants(f, &variants, true),
        }
    }
}
