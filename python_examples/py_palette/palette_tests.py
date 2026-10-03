#!/usr/bin/env python3
"""
Unit tests for palette creation workflow including event handlers and palette filtering.
Run with: python palette_tests.py
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

        opacity_btn_pressed(btn_id, 999)

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

        opacity_btn_pressed(btn_id, 999)

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

        opacity_btn_pressed(btn_id, 999)

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

        opacity_btn_pressed(btn_id, 999)

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

        opacity_btn_pressed(btn_id, 999)

        self.assertEqual(pc.opacity_btn_ids[btn_id]["opacity"], 0.0)

    def test_nonexistent_button_returns(self):
        """Test that function handles nonexistent button gracefully."""
        # Should not raise exception
        opacity_btn_pressed(0, 999)


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


def update_palette_part_key(
    pal: dict, status: str, part: str, new_key: str) -> bool:
    """Update a palette's part key for a given status.

    Args:
        pal: Palette dict from get_widget_palette_parameters()
        status: Status name (e.g., "Active", "Hovered", "Pressed", "Disabled")
        part: Part name (e.g., "Background", "Text", "Border")
        new_key: New PaletteKey name (e.g., "Strong", "Base", "Weakest")

    Returns:
        True if updated, False if status/part not found
    """
    for status_entry in pal.get('statuses', []):
        if status_entry.get('status') == status:
            for part_entry in status_entry.get('parts', []):
                if part_entry.get('part') == part:
                    part_entry['key'] = new_key
                    return True
    return False


class TestUpdatePalettePartKey(unittest.TestCase):
    """Test cases for update_palette_part_key function."""

    def setUp(self):
        """Set up test fixtures with a mock palette structure."""
        self.test_palette = {
            'id': 4460,
            'base': {'color': [0.32, 0.2, 0.13, 1.0], 'text': [1.0, 1.0, 1.0, 1.0]},
            'strong': {'color': [0.56, 0.32, 0.17, 1.0], 'text': [1.0, 1.0, 1.0, 1.0]},
            'statuses': [
                {
                    'status': 'Active',
                    'variant': 'NoVariant',
                    'parts': [
                        {'part': 'Background', 'key': 'Base', 'alpha': 1.0},
                        {'part': 'Border', 'key': 'Base', 'alpha': 1.0},
                        {'part': 'Text', 'key': 'BaseText', 'alpha': 1.0},
                    ]
                },
                {
                    'status': 'Hovered',
                    'variant': 'NoVariant',
                    'parts': [
                        {'part': 'Background', 'key': 'Strong', 'alpha': 1.0},
                        {'part': 'Border', 'key': 'Strong', 'alpha': 1.0},
                        {'part': 'Text', 'key': 'StrongText', 'alpha': 1.0},
                    ]
                },
                {
                    'status': 'Disabled',
                    'variant': 'NoVariant',
                    'parts': [
                        {'part': 'Background', 'key': 'Base', 'alpha': 0.5},
                        {'part': 'Border', 'key': 'Strong', 'alpha': 0.5},
                        {'part': 'Text', 'key': 'BaseText', 'alpha': 0.8},
                    ]
                },
            ]
        }

    def test_update_existing_key(self):
        """Test updating an existing part key for a status."""
        result = update_palette_part_key(
            self.test_palette, "Active", "Background", "Strong"
        )

        self.assertTrue(result)
        # Verify the key was updated
        active_status = next(
            s for s in self.test_palette['statuses'] if s['status'] == 'Active'
        )
        bg_part = next(
            p for p in active_status['parts'] if p['part'] == 'Background'
        )
        self.assertEqual(bg_part['key'], 'Strong')

    def test_update_text_part_key(self):
        """Test updating Text part key."""
        result = update_palette_part_key(
            self.test_palette, "Active", "Text", "StrongText"
        )

        self.assertTrue(result)
        active_status = next(
            s for s in self.test_palette['statuses'] if s['status'] == 'Active'
        )
        text_part = next(
            p for p in active_status['parts'] if p['part'] == 'Text'
        )
        self.assertEqual(text_part['key'], 'StrongText')

    def test_update_border_part_key(self):
        """Test updating Border part key."""
        result = update_palette_part_key(
            self.test_palette, "Hovered", "Border", "Weak"
        )

        self.assertTrue(result)
        hovered_status = next(
            s for s in self.test_palette['statuses'] if s['status'] == 'Hovered'
        )
        border_part = next(
            p for p in hovered_status['parts'] if p['part'] == 'Border'
        )
        self.assertEqual(border_part['key'], 'Weak')

    def test_update_nonexistent_status(self):
        """Test updating with a status that doesn't exist."""
        result = update_palette_part_key(
            self.test_palette, "Nonexistent", "Background", "Weakest"
        )

        self.assertFalse(result)

    def test_update_nonexistent_part(self):
        """Test updating with a part that doesn't exist in the status."""
        result = update_palette_part_key(
            self.test_palette, "Active", "Icon", "Base"
        )

        self.assertFalse(result)

    def test_update_disabled_status(self):
        """Test updating multiple parts in Disabled status."""
        result1 = update_palette_part_key(
            self.test_palette, "Disabled", "Background", "Weakest"
        )
        result2 = update_palette_part_key(
            self.test_palette, "Disabled", "Text", "WeakestText"
        )

        self.assertTrue(result1)
        self.assertTrue(result2)

        disabled_status = next(
            s for s in self.test_palette['statuses'] if s['status'] == 'Disabled'
        )
        bg_part = next(
            p for p in disabled_status['parts'] if p['part'] == 'Background'
        )
        text_part = next(
            p for p in disabled_status['parts'] if p['part'] == 'Text'
        )
        self.assertEqual(bg_part['key'], 'Weakest')
        self.assertEqual(text_part['key'], 'WeakestText')

    def test_alpha_unchanged_after_key_update(self):
        """Test that alpha value is not changed when updating key."""
        original_alpha = 0.8

        # Find the original alpha for Disabled/Text
        disabled_status = next(
            s for s in self.test_palette['statuses'] if s['status'] == 'Disabled'
        )
        text_part = next(
            p for p in disabled_status['parts'] if p['part'] == 'Text'
        )
        self.assertEqual(text_part['alpha'], original_alpha)

        # Update the key
        update_palette_part_key(
            self.test_palette, "Disabled", "Text", "NewTextKey"
        )

        # Verify alpha is unchanged
        text_part = next(
            p for p in disabled_status['parts'] if p['part'] == 'Text'
        )
        self.assertEqual(text_part['alpha'], original_alpha)
        self.assertEqual(text_part['key'], 'NewTextKey')


if __name__ == "__main__":
    # Run tests with verbose output
    unittest.main(verbosity=2)
