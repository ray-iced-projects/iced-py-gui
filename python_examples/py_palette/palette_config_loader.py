#!/usr/bin/env python3
"""
Palette Config Loader - Loads palette definitions from YAML config files.

Converts YAML palette configurations into Python palette definitions
that can be used with custom_palette().
"""
from pathlib import Path
import yaml


def print_config_info(config_file: str | Path, base_dir: str | Path | None = None):
    """
    Print human-readable info about a palette config.

    Parameters
    ----------
    config_file : str or Path
        Path to the .yml configuration file
    base_dir : str or Path, optional
        Base directory for resolving relative paths
    """
    config_file = Path(config_file)

    # Resolve relative paths using base_dir
    if not config_file.is_absolute() and base_dir:
        config_file = Path(base_dir) / config_file

    with open(config_file, "r", encoding="utf-8") as f:
        config = yaml.safe_load(f)

    print(f"\n{'='*70}")
    print(f"Palette Config: {config_file.name}")
    print(f"{'='*70}")
    print(f"\nWidget: {config.get('widget', 'N/A')}")
    print(f"Palette Enabled: {config.get('metadata', {}).get('palette_enabled', 'N/A')}")

    print("\nStatuses:")
    for status in config.get("statuses", []):
        print(f"  - {status}")

    print("\nState Variants:")
    for variant in config.get("state_variants", []):
        print(f"  - {variant}")

    print("\nStyle Parts:")
    for part in config.get("parts", []):
        print(f"  - {part}")

    print("\nPalette Mappings:")
    for mapping in config.get("palette_mappings", []):
        status = mapping.get("status")
        variant = mapping.get("variant", "NoVariant")
        print(f"\n  {status} ({variant}):")
        for part in mapping.get("parts", []):
            part_name = part.get("part")
            key = part.get("key")
            alpha = part.get("alpha", 1.0)
            desc = part.get("description", "")
            print(f"    - {part_name:12} → {key:15} (alpha: {alpha}) {desc}")

    print(f"\n{'='*70}\n")


def parse_mappings(mappings: list) -> list:
    """Convert YAML palette mappings to Python tuple structure."""
    result = []

    for mapping in mappings:
        status_str = mapping.get("status")
        variant_str = mapping.get("variant", "NoVariant")
        parts_list = mapping.get("parts", [])

        # Convert status and variant strings to enums
        status = cls.STATUS_MAP.get(status_str)
        variant = cls.VARIANT_MAP.get(variant_str)

        if not status:
            raise ValueError(f"Unknown status: {status_str}")
        if not variant:
            raise ValueError(f"Unknown variant: {variant_str}")

        # Convert parts to tuples
        parts_tuples = []
        for part_def in parts_list:
            part_str = part_def.get("part")
            key_str = part_def.get("key")
            alpha = float(part_def.get("alpha", 1.0))

            part = cls.PART_MAP.get(part_str)
            key = cls.PALETTE_KEY_MAP.get(key_str)

            if not part:
                raise ValueError(f"Unknown part: {part_str}")
            if not key:
                raise ValueError(f"Unknown palette key: {key_str}")

            parts_tuples.append((part, key, alpha))

        # Create the status-variant mapping
        result.append(((status, variant), tuple(parts_tuples)))

    return result
