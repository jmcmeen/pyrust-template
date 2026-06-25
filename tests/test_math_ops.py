"""Tests for the Rust functions exposed through pyrust_template."""

import pytest

import pyrust_template as m


def test_add():
    assert m.add(2, 3) == 5
    assert m.add(-1, 1) == 0


def test_dot():
    assert m.dot([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]) == pytest.approx(32.0)


def test_dot_length_mismatch_raises_value_error():
    # The Err returned by math_ops::dot becomes a Python ValueError.
    with pytest.raises(ValueError):
        m.dot([1.0, 2.0], [1.0])


def test_cumulative_sum():
    assert m.cumulative_sum([1.0, 2.0, 3.0]) == pytest.approx([1.0, 3.0, 6.0])
    assert m.cumulative_sum([]) == []


def test_version_is_exposed():
    assert isinstance(m.__version__, str)
    assert m.__version__ != ""
