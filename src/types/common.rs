use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, SerializeAs};
use std::fmt;
use std::str::FromStr;

// ─── Label ───────────────────────────────────────────────────────────────────

/// A key-value label used for filtering and classification across many resources.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Label {
    pub key: String,
    pub value: String,
}

/// Wrapper for labels on older-style endpoints: `{"label": [{key, value}, ...]}`.
///
/// Newer endpoints (LB, K8s, MOS, Gateway, File Storage) return labels as a plain
/// `HashMap<String, String>` instead.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Labels {
    #[serde(default)]
    pub label: Vec<Label>,
}

// ─── YesNo ───────────────────────────────────────────────────────────────────

/// Serde helper for fields that UpCloud encodes as the strings `"yes"` or `"no"`.
///
/// Use with `#[serde(with = "yes_no")]` or `#[serde_as(as = "YesNo")]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YesNo;

impl SerializeAs<bool> for YesNo {
    fn serialize_as<S: Serializer>(val: &bool, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(if *val { "yes" } else { "no" })
    }
}

impl<'de> DeserializeAs<'de, bool> for YesNo {
    fn deserialize_as<D: Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
        let s: &str = Deserialize::deserialize(d)?;
        match s {
            "yes" => Ok(true),
            "no" => Ok(false),
            other => Err(serde::de::Error::unknown_variant(other, &["yes", "no"])),
        }
    }
}

/// Serde helper for `Option<bool>` encoded as `"yes"` / `"no"` / absent.
pub struct YesNoOpt;

impl SerializeAs<Option<bool>> for YesNoOpt {
    fn serialize_as<S: Serializer>(val: &Option<bool>, s: S) -> Result<S::Ok, S::Error> {
        match val {
            Some(b) => s.serialize_str(if *b { "yes" } else { "no" }),
            None => s.serialize_none(),
        }
    }
}

impl<'de> DeserializeAs<'de, Option<bool>> for YesNoOpt {
    fn deserialize_as<D: Deserializer<'de>>(d: D) -> Result<Option<bool>, D::Error> {
        let opt: Option<&str> = Deserialize::deserialize(d)?;
        match opt {
            None => Ok(None),
            Some("yes") => Ok(Some(true)),
            Some("no") => Ok(Some(false)),
            Some(other) => Err(serde::de::Error::unknown_variant(other, &["yes", "no"])),
        }
    }
}

// ─── StringNumber ─────────────────────────────────────────────────────────────

/// Serde helper for numeric fields that UpCloud encodes as JSON strings (e.g. `"core_number": "2"`).
///
/// Use as `#[serde_as(as = "StringNumber")]`.
pub struct StringNumber;

impl<T> SerializeAs<T> for StringNumber
where
    T: fmt::Display,
{
    fn serialize_as<S: Serializer>(val: &T, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&val.to_string())
    }
}

impl<'de, T> DeserializeAs<'de, T> for StringNumber
where
    T: FromStr,
    T::Err: fmt::Display,
{
    fn deserialize_as<D: Deserializer<'de>>(d: D) -> Result<T, D::Error> {
        let s: &str = Deserialize::deserialize(d)?;
        s.parse::<T>().map_err(serde::de::Error::custom)
    }
}

/// Like [`StringNumber`] but for `Option<T>`.
pub struct StringNumberOpt;

impl<T: fmt::Display> SerializeAs<Option<T>> for StringNumberOpt {
    fn serialize_as<S: Serializer>(val: &Option<T>, s: S) -> Result<S::Ok, S::Error> {
        match val {
            Some(v) => s.serialize_str(&v.to_string()),
            None => s.serialize_none(),
        }
    }
}

impl<'de, T> DeserializeAs<'de, Option<T>> for StringNumberOpt
where
    T: FromStr,
    T::Err: fmt::Display,
{
    fn deserialize_as<D: Deserializer<'de>>(d: D) -> Result<Option<T>, D::Error> {
        let opt: Option<&str> = Deserialize::deserialize(d)?;
        match opt {
            None => Ok(None),
            Some(s) => s.parse::<T>().map(Some).map_err(serde::de::Error::custom),
        }
    }
}

// ─── Sort order ───────────────────────────────────────────────────────────────

/// Sort direction for list endpoints that support ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Asc,
    Desc,
}

impl Serialize for SortOrder {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
        })
    }
}

impl<'de> Deserialize<'de> for SortOrder {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s: &str = Deserialize::deserialize(d)?;
        match s {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            other => Err(serde::de::Error::unknown_variant(other, &["asc", "desc"])),
        }
    }
}

impl fmt::Display for SortOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

// ─── Pagination ───────────────────────────────────────────────────────────────

/// Query parameters for paginated list endpoints.
#[derive(Debug, Default, Clone, Serialize)]
pub struct PageParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

impl PageParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }
}

// ─── Wrapper-object helpers ───────────────────────────────────────────────────

/// Deserializes the UpCloud "wrapper object" list pattern:
/// `{ "servers": { "server": [...] } }` → `Vec<T>`.
///
/// Use with `#[serde(deserialize_with = "deserialize_wrapped_list")]` after defining
/// the specific outer/inner key names via a custom Deserialize impl.
///
/// For a simpler one-shot approach, prefer the `wrapped_list!` macro.
pub fn deserialize_vec_from_null<'de, D, T>(d: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Ok(Option::<Vec<T>>::deserialize(d)?.unwrap_or_default())
}

/// Deserializes a value as `Option<T>`, treating JSON empty strings (`""`) as `None`.
///
/// Use with `#[serde(default, deserialize_with = "deserialize_empty_string_as_none")]`.
pub fn deserialize_empty_string_as_none<'de, D, T>(d: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: serde::de::DeserializeOwned,
{
    let v: Option<serde_json::Value> = Deserialize::deserialize(d)?;
    match v {
        None => Ok(None),
        Some(serde_json::Value::String(ref s)) if s.is_empty() => Ok(None),
        Some(v) => serde_json::from_value(v).map(Some).map_err(serde::de::Error::custom),
    }
}
