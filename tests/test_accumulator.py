"""Tests for the example Accumulator class implemented in Rust."""

import pyrust_template as m


def test_accumulator_default_start():
    acc = m.Accumulator()
    assert acc.total == 0.0
    assert acc.add(5.0) == 5.0
    assert acc.add(2.5) == 7.5
    assert acc.total == 7.5


def test_accumulator_custom_start():
    acc = m.Accumulator(10.0)
    assert acc.total == 10.0


def test_accumulator_reset():
    acc = m.Accumulator(3.0)
    acc.add(4.0)
    acc.reset()
    assert acc.total == 0.0
    acc.reset(100.0)
    assert acc.total == 100.0


def test_accumulator_repr():
    acc = m.Accumulator(1.0)
    assert "Accumulator" in repr(acc)
