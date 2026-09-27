#!/usr/bin/env python3
"""
Unit tests for event_kp keyboard event handler and button press functions.
Run with: python test_event_kp.py
"""

import unittest
import sys
from pathlib import Path

# Import the real functions and pc object from py_palette_create
from python_examples.py_palette.py_palette_create import (
    event_kp, opacity_btn_pressed, border_btn_pressed, pc)

# Add parent directories to path for imports
sys.path.insert(0, str(Path(__file__).parent.parent.parent))


class TestEventKp(unittest.TestCase):
    """Test cases for event_kp function using the real implementation."""

    def setUp(self):
        """Set up test fixtures by clearing pc state."""
        pc.opacity_btn_hovered = 0
        pc.opacity_btn_ids = {}
        pc.border_btn_hovered = 0
        pc.border_btn_ids = {}

    def test_no_button_hovered(self):
        """Test that early return happens when no button is hovered."""
        pc.opacity_btn_hovered = 0
        pc.border_btn_hovered = 0
        # Should not raise exception
        event_kp(0, {"key": "ArrowUp"})
        self.assertEqual(len(pc.opacity_btn_ids), 0)
        self.assertEqual(len(pc.border_btn_ids), 0)


    def test_escape_clears_opacity_modifier(self):
        """Test that Escape key clears opacity button modifier."""
        btn_id = 101
        pc.opacity_btn_hovered = btn_id
        pc.border_btn_hovered = 0
        pc.opacity_btn_ids[btn_id] = {
            "modifier": "Control",
            "key": "ArrowUp",
            "opacity": 0.5
        }

        event_kp(0, {"key": "Escape"})

        self.assertEqual(pc.opacity_btn_ids[btn_id]["modifier"], "None")

    def test_escape_clears_border_modifier(self):
        """Test that Escape key clears border button modifier."""
        btn_id = 102
        pc.opacity_btn_hovered = 0
        pc.border_btn_hovered = btn_id
        pc.border_btn_ids[btn_id] = {
            "modifier": "Shift",
            "key": "ArrowDown",
            "width": 3.0
        }

        event_kp(0, {"key": "Escape"})

        self.assertEqual(pc.border_btn_ids[btn_id]["modifier"], "None")

    def test_control_key_sets_opacity_modifier(self):
        """Test that Control key sets opacity button modifier."""
        btn_id = 103
        pc.opacity_btn_hovered = btn_id
        pc.border_btn_hovered = 0
        pc.opacity_btn_ids[btn_id] = {
            "modifier": "None",
            "key": "ArrowUp",
            "opacity": 0.8
        }

        event_kp(0, {"key": "Control"})

        self.assertEqual(pc.opacity_btn_ids[btn_id]["modifier"], "Control")

    def test_arrow_key_sets_opacity_key(self):
        """Test that ArrowUp key sets opacity button key."""
        btn_id = 104
        pc.opacity_btn_hovered = btn_id
        pc.border_btn_hovered = 0
        pc.opacity_btn_ids[btn_id] = {
            "modifier": "None",
            "key": "ArrowUp",
            "opacity": 0.6
        }

        event_kp(0, {"key": "ArrowDown"})

        self.assertEqual(pc.opacity_btn_ids[btn_id]["key"], "ArrowDown")

    def test_control_key_sets_border_modifier(self):
        """Test that Control key sets border button modifier."""
        btn_id = 105
        pc.opacity_btn_hovered = 0
        pc.border_btn_hovered = btn_id
        pc.border_btn_ids[btn_id] = {
            "modifier": "None",
            "key": "ArrowDown",
            "width": 2.0
        }

        event_kp(0, {"key": "Control"})

        self.assertEqual(pc.border_btn_ids[btn_id]["modifier"], "Control")

    def test_shift_key_sets_border_modifier(self):
        """Test that Shift key sets border button modifier."""
        btn_id = 106
        pc.opacity_btn_hovered = 0
        pc.border_btn_hovered = btn_id
        pc.border_btn_ids[btn_id] = {
            "modifier": "None",
            "key": "ArrowUp",
            "width": 1.5
        }

        event_kp(0, {"key": "Shift"})

        self.assertEqual(pc.border_btn_ids[btn_id]["modifier"], "Shift")

    def test_multiple_sequential_keypress(self):
        """Test handling of multiple sequential key presses."""
        btn_id = 107
        pc.opacity_btn_hovered = btn_id
        pc.border_btn_hovered = 0
        pc.opacity_btn_ids[btn_id] = {
            "modifier": "None",
            "key": "ArrowUp",
            "opacity": 0.5
        }

        # First: set Control modifier
        event_kp(0, {"key": "Control"})
        self.assertEqual(pc.opacity_btn_ids[btn_id]["modifier"], "Control")

        # Second: set ArrowUp key
        event_kp(0, {"key": "ArrowUp"})
        self.assertEqual(pc.opacity_btn_ids[btn_id]["key"], "ArrowUp")

        # Third: clear with Escape
        event_kp(0, {"key": "Escape"})
        self.assertEqual(pc.opacity_btn_ids[btn_id]["modifier"], "None")


class TestOpacityBtnPressed(unittest.TestCase):
    """Test cases for refactored opacity_btn_pressed function."""

    def setUp(self):
        """Set up test fixtures."""
        pc.opacity_btn_ids = {}
        pc.opacity_step_size = 0.1

    def test_arrow_up_increments_opacity(self):
        """Test that ArrowUp increases opacity."""
        btn_id = 201
        pc.opacity_btn_ids[btn_id] = {
            "modifier": "None",
            "key": "ArrowUp",
            "opacity": 0.5,
            "input_id": 999
        }

        opacity_btn_pressed(btn_id)

        self.assertAlmostEqual(pc.opacity_btn_ids[btn_id]["opacity"], 0.6, places=2)

    def test_arrow_down_decrements_opacity(self):
        """Test that ArrowDown decreases opacity."""
        btn_id = 202
        pc.opacity_btn_ids[btn_id] = {
            "modifier": "None",
            "key": "ArrowDown",
            "opacity": 0.5,
            "input_id": 999
        }

        opacity_btn_pressed(btn_id)

        self.assertAlmostEqual(pc.opacity_btn_ids[btn_id]["opacity"], 0.4, places=2)

    def test_control_arrow_up_fine_increment(self):
        """Test that Control-ArrowUp increments by 0.01."""
        btn_id = 203
        pc.opacity_btn_ids[btn_id] = {
            "modifier": "Control",
            "key": "ArrowUp",
            "opacity": 0.5,
            "input_id": 999
        }

        opacity_btn_pressed(btn_id)

        self.assertAlmostEqual(pc.opacity_btn_ids[btn_id]["opacity"], 0.51, places=2)

    def test_opacity_clamped_at_max(self):
        """Test that opacity is clamped to 1.0."""
        btn_id = 204
        pc.opacity_btn_ids[btn_id] = {
            "modifier": "None",
            "key": "ArrowUp",
            "opacity": 0.99,
            "input_id": 999
        }

        opacity_btn_pressed(btn_id)

        self.assertEqual(pc.opacity_btn_ids[btn_id]["opacity"], 1.0)

    def test_opacity_clamped_at_min(self):
        """Test that opacity is clamped to 0.0."""
        btn_id = 205
        pc.opacity_btn_ids[btn_id] = {
            "modifier": "None",
            "key": "ArrowDown",
            "opacity": 0.05,
            "input_id": 999
        }

        opacity_btn_pressed(btn_id)

        self.assertEqual(pc.opacity_btn_ids[btn_id]["opacity"], 0.0)

    def test_nonexistent_button_returns(self):
        """Test that function handles nonexistent button gracefully."""
        # Should not raise exception
        opacity_btn_pressed(999)


class TestBorderBtnPressed(unittest.TestCase):
    """Test cases for refactored border_btn_pressed function."""

    def setUp(self):
        """Set up test fixtures."""
        pc.border_btn_ids = {}

    def test_arrow_up_increments_border(self):
        """Test that ArrowUp increases border width with None modifier."""
        btn_id = 301
        pc.border_btn_ids[btn_id] = {
            "modifier": "None",
            "key": "ArrowUp",
            "width": 2.0
        }

        border_btn_pressed(btn_id, 999)

        self.assertEqual(pc.border_btn_ids[btn_id]["width"], 3.0)

    def test_arrow_down_decrements_border(self):
        """Test that ArrowDown decreases border width with None modifier."""
        btn_id = 302
        pc.border_btn_ids[btn_id] = {
            "modifier": "None",
            "key": "ArrowDown",
            "width": 2.0
        }

        border_btn_pressed(btn_id, 999)

        self.assertEqual(pc.border_btn_ids[btn_id]["width"], 1.0)

    def test_control_modifier_step(self):
        """Test that Control modifier uses 0.5 step."""
        btn_id = 303
        pc.border_btn_ids[btn_id] = {
            "modifier": "Control",
            "key": "ArrowUp",
            "width": 2.0
        }

        border_btn_pressed(btn_id, 999)

        self.assertAlmostEqual(pc.border_btn_ids[btn_id]["width"], 2.5, places=2)

    def test_shift_modifier_step(self):
        """Test that Shift modifier uses 0.1 step."""
        btn_id = 304
        pc.border_btn_ids[btn_id] = {
            "modifier": "Shift",
            "key": "ArrowUp",
            "width": 2.0
        }

        border_btn_pressed(btn_id, 999)

        self.assertAlmostEqual(pc.border_btn_ids[btn_id]["width"], 2.1, places=2)

    def test_border_clamped_at_min(self):
        """Test that border width is clamped to 0.0."""
        btn_id = 305
        pc.border_btn_ids[btn_id] = {
            "modifier": "None",
            "key": "ArrowDown",
            "width": 0.5
        }

        border_btn_pressed(btn_id, 999)

        self.assertEqual(pc.border_btn_ids[btn_id]["width"], 0.0)

    def test_nonexistent_button_returns(self):
        """Test that function handles nonexistent button gracefully."""
        # Should not raise exception
        border_btn_pressed(999, 999)


if __name__ == "__main__":
    # Run tests with verbose output
    unittest.main(verbosity=2)
