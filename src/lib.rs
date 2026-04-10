use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use regex::bytes::NoExpand;

/// Represents a match from a regex search on a Unicode string.
#[pyclass(frozen)]
pub struct Match {
    #[pyo3(get)]
    pub start: usize,
    #[pyo3(get)]
    pub end: usize,
    value: String,
}

#[pymethods]
impl Match {
    /// Return the matched string.
    pub fn as_str(&self) -> &str {
        &self.value
    }

    fn __repr__(&self) -> String {
        format!(
            "Match(start={}, end={}, value={:?})",
            self.start, self.end, self.value
        )
    }

    fn __str__(&self) -> &str {
        &self.value
    }
}

/// Represents a match from a regex search on a byte string.
#[pyclass(frozen)]
pub struct BytesMatch {
    #[pyo3(get)]
    pub start: usize,
    #[pyo3(get)]
    pub end: usize,
    value: Vec<u8>,
}

#[pymethods]
impl BytesMatch {
    /// Return the matched bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.value
    }

    fn __repr__(&self) -> String {
        format!(
            "BytesMatch(start={}, end={}, value={:?})",
            self.start, self.end, self.value
        )
    }
}

/// A compiled regular expression for matching Unicode strings.
///
/// Raises ``ValueError`` on construction if the pattern is invalid.
#[pyclass(frozen)]
pub struct Regex {
    inner: regex::Regex,
}

#[pymethods]
impl Regex {
    /// Compile a regular expression from *pattern*.
    #[new]
    pub fn new(pattern: &str) -> PyResult<Self> {
        regex::Regex::new(pattern)
            .map(|inner| Regex { inner })
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// The original pattern string.
    #[getter]
    pub fn pattern(&self) -> &str {
        self.inner.as_str()
    }

    /// Return ``True`` if the pattern matches anywhere in *text*.
    pub fn is_match(&self, text: &str) -> bool {
        self.inner.is_match(text)
    }

    /// Return the leftmost :class:`Match`, or ``None`` if there is no match.
    pub fn find(&self, text: &str) -> Option<Match> {
        self.inner.find(text).map(|m| Match {
            start: m.start(),
            end: m.end(),
            value: m.as_str().to_string(),
        })
    }

    /// Return a list of all non-overlapping :class:`Match` objects.
    pub fn find_iter(&self, text: &str) -> Vec<Match> {
        self.inner
            .find_iter(text)
            .map(|m| Match {
                start: m.start(),
                end: m.end(),
                value: m.as_str().to_string(),
            })
            .collect()
    }

    /// Return the capture groups of the first match as a list of strings.
    ///
    /// The first element is the entire match; subsequent elements are the
    /// named or numbered capture groups.  Unmatched optional groups are
    /// ``None``.  Returns ``None`` when there is no match.
    pub fn captures(&self, text: &str) -> Option<Vec<Option<String>>> {
        self.inner.captures(text).map(|c| {
            c.iter()
                .map(|m| m.map(|m| m.as_str().to_string()))
                .collect()
        })
    }

    /// Return captures for every non-overlapping match as a list of lists.
    pub fn captures_iter(&self, text: &str) -> Vec<Vec<Option<String>>> {
        self.inner
            .captures_iter(text)
            .map(|c| {
                c.iter()
                    .map(|m| m.map(|m| m.as_str().to_string()))
                    .collect()
            })
            .collect()
    }

    /// Replace the leftmost match with *replacement*.
    ///
    /// The replacement may reference capture groups with ``$1`` or
    /// ``$name`` syntax.
    pub fn replace(&self, text: &str, replacement: &str) -> String {
        self.inner.replace(text, replacement).into_owned()
    }

    /// Replace every non-overlapping match with *replacement*.
    pub fn replace_all(&self, text: &str, replacement: &str) -> String {
        self.inner.replace_all(text, replacement).into_owned()
    }

    /// Split *text* at each match and return the list of substrings.
    pub fn split(&self, text: &str) -> Vec<String> {
        self.inner.split(text).map(|s| s.to_string()).collect()
    }

    fn __repr__(&self) -> String {
        format!("Regex({:?})", self.inner.as_str())
    }
}

/// A set of compiled regular expressions for matching Unicode strings.
///
/// Raises ``ValueError`` on construction if any pattern is invalid.
#[pyclass(frozen)]
pub struct RegexSet {
    inner: regex::RegexSet,
}

#[pymethods]
impl RegexSet {
    /// Compile a set of regular expressions from *patterns*.
    #[new]
    pub fn new(patterns: Vec<String>) -> PyResult<Self> {
        regex::RegexSet::new(&patterns)
            .map(|inner| RegexSet { inner })
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// The list of patterns in this set.
    #[getter]
    pub fn patterns(&self) -> Vec<String> {
        self.inner
            .patterns()
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    /// Return the number of patterns in the set.
    pub fn __len__(&self) -> usize {
        self.inner.len()
    }

    /// Return ``True`` if the set contains no patterns.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Return ``True`` if any pattern matches anywhere in *text*.
    pub fn is_match(&self, text: &str) -> bool {
        self.inner.is_match(text)
    }

    /// Return the indices of every pattern that matches somewhere in *text*.
    pub fn matches(&self, text: &str) -> Vec<usize> {
        self.inner.matches(text).into_iter().collect()
    }

    fn __repr__(&self) -> String {
        format!("RegexSet({:?})", self.inner.patterns())
    }
}

/// A compiled regular expression for matching byte strings.
///
/// Raises ``ValueError`` on construction if the pattern is invalid.
#[pyclass(frozen)]
pub struct BytesRegex {
    inner: regex::bytes::Regex,
}

#[pymethods]
impl BytesRegex {
    /// Compile a regular expression from *pattern*.
    #[new]
    pub fn new(pattern: &str) -> PyResult<Self> {
        regex::bytes::Regex::new(pattern)
            .map(|inner| BytesRegex { inner })
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// The original pattern string.
    #[getter]
    pub fn pattern(&self) -> &str {
        self.inner.as_str()
    }

    /// Return ``True`` if the pattern matches anywhere in *data*.
    pub fn is_match(&self, data: &[u8]) -> bool {
        self.inner.is_match(data)
    }

    /// Return the leftmost :class:`BytesMatch`, or ``None`` if there is no match.
    pub fn find(&self, data: &[u8]) -> Option<BytesMatch> {
        self.inner.find(data).map(|m| BytesMatch {
            start: m.start(),
            end: m.end(),
            value: m.as_bytes().to_vec(),
        })
    }

    /// Return a list of all non-overlapping :class:`BytesMatch` objects.
    pub fn find_iter(&self, data: &[u8]) -> Vec<BytesMatch> {
        self.inner
            .find_iter(data)
            .map(|m| BytesMatch {
                start: m.start(),
                end: m.end(),
                value: m.as_bytes().to_vec(),
            })
            .collect()
    }

    /// Return the capture groups of the first match as a list of byte strings.
    ///
    /// The first element is the entire match.  Unmatched optional groups are
    /// ``None``.  Returns ``None`` when there is no match.
    pub fn captures(&self, data: &[u8]) -> Option<Vec<Option<Vec<u8>>>> {
        self.inner
            .captures(data)
            .map(|c| c.iter().map(|m| m.map(|m| m.as_bytes().to_vec())).collect())
    }

    /// Return captures for every non-overlapping match as a list of lists.
    pub fn captures_iter(&self, data: &[u8]) -> Vec<Vec<Option<Vec<u8>>>> {
        self.inner
            .captures_iter(data)
            .map(|c| c.iter().map(|m| m.map(|m| m.as_bytes().to_vec())).collect())
            .collect()
    }

    /// Replace the leftmost match with the literal bytes *replacement*.
    pub fn replace(&self, data: &[u8], replacement: &[u8]) -> Vec<u8> {
        self.inner.replace(data, NoExpand(replacement)).into_owned()
    }

    /// Replace every non-overlapping match with the literal bytes *replacement*.
    pub fn replace_all(&self, data: &[u8], replacement: &[u8]) -> Vec<u8> {
        self.inner
            .replace_all(data, NoExpand(replacement))
            .into_owned()
    }

    /// Split *data* at each match and return the list of byte strings.
    pub fn split(&self, data: &[u8]) -> Vec<Vec<u8>> {
        self.inner.split(data).map(|s| s.to_vec()).collect()
    }

    fn __repr__(&self) -> String {
        format!("BytesRegex({:?})", self.inner.as_str())
    }
}

/// A set of compiled regular expressions for matching byte strings.
///
/// Raises ``ValueError`` on construction if any pattern is invalid.
#[pyclass(frozen)]
pub struct BytesRegexSet {
    inner: regex::bytes::RegexSet,
}

#[pymethods]
impl BytesRegexSet {
    /// Compile a set of regular expressions from *patterns*.
    #[new]
    pub fn new(patterns: Vec<String>) -> PyResult<Self> {
        regex::bytes::RegexSet::new(&patterns)
            .map(|inner| BytesRegexSet { inner })
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// The list of patterns in this set.
    #[getter]
    pub fn patterns(&self) -> Vec<String> {
        self.inner
            .patterns()
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    /// Return the number of patterns in the set.
    pub fn __len__(&self) -> usize {
        self.inner.len()
    }

    /// Return ``True`` if the set contains no patterns.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Return ``True`` if any pattern matches anywhere in *data*.
    pub fn is_match(&self, data: &[u8]) -> bool {
        self.inner.is_match(data)
    }

    /// Return the indices of every pattern that matches somewhere in *data*.
    pub fn matches(&self, data: &[u8]) -> Vec<usize> {
        self.inner.matches(data).into_iter().collect()
    }

    fn __repr__(&self) -> String {
        format!("BytesRegexSet({:?})", self.inner.patterns())
    }
}

/// Python bindings for the Rust ``regex`` crate.
///
/// Exposes :class:`Regex`, :class:`RegexSet`, :class:`BytesRegex`, and
/// :class:`BytesRegexSet` with GIL-free support.
#[pymodule(gil_used = false)]
fn paula(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Match>()?;
    m.add_class::<BytesMatch>()?;
    m.add_class::<Regex>()?;
    m.add_class::<RegexSet>()?;
    m.add_class::<BytesRegex>()?;
    m.add_class::<BytesRegexSet>()?;
    Ok(())
}
