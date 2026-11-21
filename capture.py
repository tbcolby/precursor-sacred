"""Renode capture sequence for Sacred Geometry (not yet run).

Run from xous-dev-toolkit:
    python3 scripts/renode_capture.py --app sacred --script ../precursor-sacred/capture.py

Contract: run(ctl, outdir) -> number of failed steps. The app is already launched.
"""
import os


def run(ctl, outdir) -> int:
    failures = 0

    def shot(name):
        nonlocal failures
        if not ctl.try_screenshot(os.path.join(outdir, name)):
            failures += 1

    shot("01_selector.png")
    ctl.inject_line("")                 # Enter: Seed of Life
    ctl.sleep(2.0)
    shot("02_seed_of_life.png")
    ctl.timed_key("Right", after=3.0)   # Flower of Life
    shot("03_flower_of_life.png")
    ctl.timed_key("Right", after=3.0)   # Metatron's Cube
    shot("04_metatron.png")
    ctl.timed_key("Up", after=3.0)      # bigger: clipped at the edge
    shot("05_metatron_big.png")
    return failures
