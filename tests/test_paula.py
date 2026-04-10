import pytest

import paula


class TestMatch:
    def test_repr(self):
        r = paula.Regex(r"\d+")
        m = r.find("abc 42 xyz")
        assert m is not None
        assert "Match" in repr(m)
        assert "42" in repr(m)

    def test_str(self):
        r = paula.Regex(r"\d+")
        m = r.find("abc 42 xyz")
        assert str(m) == "42"


class TestRegex:
    def test_basic_match(self):
        r = paula.Regex(r"\d+")
        assert r.is_match("hello 42 world")

    def test_no_match(self):
        r = paula.Regex(r"\d+")
        assert not r.is_match("hello world")

    def test_invalid_pattern_raises_value_error(self):
        with pytest.raises(ValueError):
            paula.Regex(r"[invalid")

    def test_pattern_property(self):
        pattern = r"\d+"
        r = paula.Regex(pattern)
        assert r.pattern == pattern

    def test_repr(self):
        r = paula.Regex(r"\d+")
        assert "Regex" in repr(r)
        assert r"\\d+" in repr(r)

    def test_find_returns_match(self):
        r = paula.Regex(r"\d+")
        m = r.find("hello 42 world")
        assert m is not None
        assert m.start == 6
        assert m.end == 8
        assert m.as_str() == "42"

    def test_find_returns_none_on_no_match(self):
        r = paula.Regex(r"\d+")
        assert r.find("hello world") is None

    def test_find_iter(self):
        r = paula.Regex(r"\d+")
        matches = r.find_iter("one 1 two 22 three 333")
        assert len(matches) == 3
        assert matches[0].as_str() == "1"
        assert matches[1].as_str() == "22"
        assert matches[2].as_str() == "333"

    def test_find_iter_empty(self):
        r = paula.Regex(r"\d+")
        assert r.find_iter("no digits here") == []

    def test_captures(self):
        r = paula.Regex(r"(\d+)-(\w+)")
        caps = r.captures("42-hello")
        assert caps is not None
        assert caps[0] == "42-hello"
        assert caps[1] == "42"
        assert caps[2] == "hello"

    def test_captures_returns_none_on_no_match(self):
        r = paula.Regex(r"(\d+)-(\w+)")
        assert r.captures("no match") is None

    def test_captures_optional_group(self):
        r = paula.Regex(r"(\d+)(-(\w+))?")
        caps = r.captures("42")
        assert caps is not None
        assert caps[1] == "42"
        assert caps[2] is None
        assert caps[3] is None

    def test_captures_iter(self):
        r = paula.Regex(r"(\d+)")
        results = r.captures_iter("a1b22c333")
        assert len(results) == 3
        assert results[0][1] == "1"
        assert results[1][1] == "22"
        assert results[2][1] == "333"

    def test_replace(self):
        r = paula.Regex(r"\d+")
        assert r.replace("hello 42 world 7", "NUM") == "hello NUM world 7"

    def test_replace_with_capture_group(self):
        r = paula.Regex(r"(\w+)\s+(\w+)")
        assert r.replace("hello world", "$2 $1") == "world hello"

    def test_replace_all(self):
        r = paula.Regex(r"\d+")
        assert r.replace_all("1 2 3", "N") == "N N N"

    def test_split(self):
        r = paula.Regex(r"\s+")
        parts = r.split("hello  world  foo")
        assert parts == ["hello", "world", "foo"]

    def test_split_no_match(self):
        r = paula.Regex(r"\d+")
        parts = r.split("hello world")
        assert parts == ["hello world"]


class TestRegexSet:
    def test_basic_match(self):
        rs = paula.RegexSet([r"\d+", r"\w+", r"hello"])
        assert rs.is_match("hello 42")

    def test_no_match(self):
        rs = paula.RegexSet([r"\d+"])
        assert not rs.is_match("hello")

    def test_matches_returns_indices(self):
        rs = paula.RegexSet([r"\d+", r"\w+", r"hello"])
        m = rs.matches("hello 42")
        assert 0 in m  # \d+ matches "42"
        assert 1 in m  # \w+ matches "hello" or "42"
        assert 2 in m  # "hello" matches

    def test_matches_partial(self):
        rs = paula.RegexSet([r"\d+", r"xyz"])
        m = rs.matches("hello 42")
        assert 0 in m
        assert 1 not in m

    def test_invalid_pattern_raises_value_error(self):
        with pytest.raises(ValueError):
            paula.RegexSet([r"[invalid"])

    def test_patterns_property(self):
        patterns = [r"\d+", r"\w+"]
        rs = paula.RegexSet(patterns)
        assert list(rs.patterns) == patterns

    def test_len(self):
        rs = paula.RegexSet([r"\d+", r"\w+", r"hello"])
        assert len(rs) == 3

    def test_is_empty_true(self):
        rs = paula.RegexSet([])
        assert rs.is_empty()

    def test_is_empty_false(self):
        rs = paula.RegexSet([r"\d+"])
        assert not rs.is_empty()

    def test_repr(self):
        rs = paula.RegexSet([r"\d+"])
        assert "RegexSet" in repr(rs)


class TestBytesMatch:
    def test_repr(self):
        r = paula.BytesRegex(r"\d+")
        m = r.find(b"abc 42 xyz")
        assert m is not None
        assert "BytesMatch" in repr(m)


class TestBytesRegex:
    def test_basic_match(self):
        r = paula.BytesRegex(r"\d+")
        assert r.is_match(b"hello 42 world")

    def test_no_match(self):
        r = paula.BytesRegex(r"\d+")
        assert not r.is_match(b"hello world")

    def test_invalid_pattern_raises_value_error(self):
        with pytest.raises(ValueError):
            paula.BytesRegex(r"[invalid")

    def test_pattern_property(self):
        pattern = r"\d+"
        r = paula.BytesRegex(pattern)
        assert r.pattern == pattern

    def test_repr(self):
        r = paula.BytesRegex(r"\d+")
        assert "BytesRegex" in repr(r)

    def test_find_returns_match(self):
        r = paula.BytesRegex(r"\d+")
        m = r.find(b"hello 42 world")
        assert m is not None
        assert m.start == 6
        assert m.end == 8
        assert m.as_bytes() == b"42"

    def test_find_returns_none_on_no_match(self):
        r = paula.BytesRegex(r"\d+")
        assert r.find(b"hello world") is None

    def test_find_iter(self):
        r = paula.BytesRegex(r"\d+")
        matches = r.find_iter(b"one 1 two 22 three 333")
        assert len(matches) == 3
        assert matches[0].as_bytes() == b"1"
        assert matches[1].as_bytes() == b"22"
        assert matches[2].as_bytes() == b"333"

    def test_captures(self):
        r = paula.BytesRegex(r"(\d+)-(\w+)")
        caps = r.captures(b"42-hello")
        assert caps is not None
        assert caps[0] == b"42-hello"
        assert caps[1] == b"42"
        assert caps[2] == b"hello"

    def test_captures_returns_none_on_no_match(self):
        r = paula.BytesRegex(r"(\d+)-(\w+)")
        assert r.captures(b"no match") is None

    def test_captures_iter(self):
        r = paula.BytesRegex(r"(\d+)")
        results = r.captures_iter(b"a1b22c333")
        assert len(results) == 3
        assert results[0][1] == b"1"
        assert results[1][1] == b"22"
        assert results[2][1] == b"333"

    def test_replace(self):
        r = paula.BytesRegex(r"\d+")
        assert r.replace(b"hello 42 world 7", b"NUM") == b"hello NUM world 7"

    def test_replace_all(self):
        r = paula.BytesRegex(r"\d+")
        assert r.replace_all(b"1 2 3", b"N") == b"N N N"

    def test_split(self):
        r = paula.BytesRegex(r"\s+")
        parts = r.split(b"hello  world  foo")
        assert parts == [b"hello", b"world", b"foo"]

    def test_arbitrary_bytes(self):
        # The bytes::Regex (?-u:.) matches arbitrary bytes with Unicode disabled
        r = paula.BytesRegex(r"(?-u:\xff)")
        assert r.is_match(b"\xff\x00\xff")
        assert not r.is_match(b"\x00\x01\x02")


class TestBytesRegexSet:
    def test_basic_match(self):
        rs = paula.BytesRegexSet([r"\d+", r"\w+"])
        assert rs.is_match(b"hello 42")

    def test_no_match(self):
        rs = paula.BytesRegexSet([r"\d+"])
        assert not rs.is_match(b"hello")

    def test_matches_returns_indices(self):
        rs = paula.BytesRegexSet([r"\d+", r"\w+", r"hello"])
        m = rs.matches(b"hello 42")
        assert 0 in m
        assert 1 in m
        assert 2 in m

    def test_invalid_pattern_raises_value_error(self):
        with pytest.raises(ValueError):
            paula.BytesRegexSet([r"[invalid"])

    def test_patterns_property(self):
        patterns = [r"\d+", r"\w+"]
        rs = paula.BytesRegexSet(patterns)
        assert list(rs.patterns) == patterns

    def test_len(self):
        rs = paula.BytesRegexSet([r"\d+", r"\w+"])
        assert len(rs) == 2

    def test_is_empty_true(self):
        rs = paula.BytesRegexSet([])
        assert rs.is_empty()

    def test_is_empty_false(self):
        rs = paula.BytesRegexSet([r"\d+"])
        assert not rs.is_empty()

    def test_repr(self):
        rs = paula.BytesRegexSet([r"\d+"])
        assert "BytesRegexSet" in repr(rs)
