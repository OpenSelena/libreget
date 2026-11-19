#!/usr/bin/env python3
"""
Resample LibreGet's master penguin artwork into 1x (32px) and 2x (64px) PNGs
using high-quality Lanczos sinc filtering.
"""
import base64
import glob
import io
import os
import re
from PIL import Image

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
NAV_DIR = os.path.join(ROOT_DIR, "static", "icons", "nav")

SVG_ICONS = ["home", "downloads", "tools", "nami", "marketplace", "settings", "convert", "courses"]
PNG_SOURCES = {
    "about": os.path.join(ROOT_DIR, "static", "icons", "about-64.png"),
    "telegram": os.path.join(ROOT_DIR, "static", "icons", "telegram-64.png"),
}


def resample_and_save(im: Image.Image, name: str) -> None:
    im_rgba = im.convert("RGBA")
    im32 = im_rgba.resize((32, 32), Image.Resampling.LANCZOS)
    im64 = im_rgba.resize((64, 64), Image.Resampling.LANCZOS)

    out32 = os.path.join(NAV_DIR, f"{name}-32.png")
    out64 = os.path.join(NAV_DIR, f"{name}-64.png")

    im32.save(out32, "PNG", optimize=True)
    im64.save(out64, "PNG", optimize=True)
    print(f"Generated {name}-32.png and {name}-64.png")


def main() -> None:
    for name in SVG_ICONS:
        svg_path = os.path.join(NAV_DIR, f"{name}.svg")
        with open(svg_path, "r", encoding="utf-8") as f:
            content = f.read()
        m = re.search(r"data:image/png;base64,([A-Za-z0-9+/=]+)", content)
        if not m:
            raise ValueError(f"No base64 image found in {svg_path}")
        raw = base64.b64decode(m.group(1))
        im = Image.open(io.BytesIO(raw))
        resample_and_save(im, name)

    for name, p in PNG_SOURCES.items():
        im = Image.open(p)
        resample_and_save(im, name)


if __name__ == "__main__":
    main()
