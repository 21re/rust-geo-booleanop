#!/usr/bin/env python

from __future__ import print_function

import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
from matplotlib.patches import PathPatch
from matplotlib.path import Path

import argparse
import json
import os


def extract_multi_polygon(feature):
    """
    Extracts the coordinates of a GeoJSON Feature of either type Polygon or Multipolygon,
    implicitly converting the coordinates to the form of a Multipolygon.
    """
    kind = feature["geometry"]["type"]
    if kind == "Polygon":
        return [feature["geometry"]["coordinates"]]
    elif kind == "MultiPolygon":
        return feature["geometry"]["coordinates"]
    else:
        raise ValueError("Feature has wrong type: {}".format(kind))


def check_winding_order_clockwise(points):
    """
    Implements winding order check as per: https://stackoverflow.com/a/1180256/1804173
    """
    points = points[:-1]    # no need for repeated endpoints
    if len(points) == 0:
        return True

    min_y = None
    min_x = None
    min_i = None
    for i, p in enumerate(points):
        if (min_y is None) or (min_x is None) or (p[1] < min_y) or (p[1] == min_y and p[0] < min_x):
            min_x = p[0]
            min_y = p[1]
            min_i = i

    a = min_i
    b = a - 1
    c = a + 1
    if b < 0:
        b += len(points)
    if c >= len(points):
        c -= len(points)
    ab = [points[b][0] - points[a][0], points[b][1] - points[a][1]]
    ac = [points[c][0] - points[a][0], points[c][1] - points[a][1]]
    prod = ab[0] * ac[1] - ab[1] * ac[0]

    return prod > 0


def fix_winding_order(points, should_be_clockwise):
    """
    Ensures a ring is in either clockwise or counter-clockwise order.
    """
    is_clockwise = check_winding_order_clockwise(points)
    if is_clockwise == should_be_clockwise:
        return points[:]
    else:
        return points[::-1]


def validate_ring(ring):
    assert isinstance(ring, list), "Ring must be of type list, but is {}.".format(type(ring))
    for p in ring:
        assert isinstance(p, list), "Points must be of type list, but is {}".format(type(p))
        assert len(p) == 2, "Points must be list of length 2, but is {}".format(len(p))
        assert isinstance(p[0], float) or isinstance(p[0], int), \
            "Values must be float or int, got: {}".format(p[0])
        assert isinstance(p[1], float) or isinstance(p[1], int), \
            "Values must be float or int, got: {}".format(p[1])


def plot(ax, multi_polygon, label, shade_color=None):
    for j, polygon in enumerate(multi_polygon):
        path_points = []
        path_commands = []
        for k, ring in enumerate(polygon):
            try:
                validate_ring(ring)
                xs = [p[0] for p in ring]
                ys = [p[1] for p in ring]
                ax.plot(xs, ys, "o-", label="{} (poly = {}, ring = {})".format(label, j + 1, k + 1), ms=2)
            except (IndexError, AssertionError) as e:
                plt.figtext(
                    0.5, 0.5,
                    "Plot failed because of invalid GeoJSON",
                    ha='center', fontsize=12, color="#bf1932",
                )

            points_ordered = fix_winding_order(ring, should_be_clockwise=(k == 0))
            path_points += points_ordered
            path_commands += [Path.MOVETO] + [Path.LINETO] * (len(points_ordered) - 2) + [Path.CLOSEPOLY]

        if shade_color is not None and len(path_points) > 0:
            path = Path(path_points, path_commands)
            patch = PathPatch(path, ec=None, fc=shade_color, alpha=0.15)
            ax.add_patch(patch)


def parse_args():
    parser = argparse.ArgumentParser("Tool to plot content test case files.")
    parser.add_argument(
        "-i", "--interactive",
        action="store_true",
        help="whether to show interactive plot windows"
    )
    parser.add_argument(
        "--pngs",
        action="store_true",
        help="whether to generate individual PNGs for each plot"
    )
    parser.add_argument(
        "-o", "--output",
        help="PDF output file name (default: test_cases.pdf)",
        default="test_cases.pdf",
    )
    parser.add_argument(
        "files",
        metavar="<TEST-CASE-FILE>",
        nargs="+",
        help="test case GeoJSON file(s) to plot",
    )
    args = parser.parse_args()
    return args


def main():
    args = parse_args()
    files = args.files
    interactive = args.interactive
    pngs = args.pngs

    with PdfPages(args.output) as pp:

        for f in sorted(files):
            print("Plotting test case: {}".format(f))
            data = json.load(open(f))
            assert data["type"] == "FeatureCollection"

            features = data["features"]
            assert len(features) >= 2

            p1 = extract_multi_polygon(features[0])
            p2 = extract_multi_polygon(features[1])

            for feature in features[2:]:
                properties = feature["properties"]
                op = properties["operation"]
                p_res = extract_multi_polygon(feature)

                fig, axes = plt.subplots(1, 3, figsize=(15, 7), sharex=True, sharey=True)

                plot(axes[0], p1, "A", shade_color="#0000FF")
                plot(axes[0], p2, "B", shade_color="#FF0000")

                plot(axes[1], p_res, "Result", shade_color="#00FF00")

                plot(axes[2], p1, "A")
                plot(axes[2], p2, "B")
                plot(axes[2], p_res, "Result")

                if len(axes[0].get_legend_handles_labels()[0]) < 10:
                    axes[0].legend(loc="best", prop={"size": 9})
                if len(axes[1].get_legend_handles_labels()[0]) < 10:
                    axes[1].legend(loc="best", prop={"size": 9})
                if len(axes[2].get_legend_handles_labels()[0]) < 10:
                    axes[2].legend(loc="best", prop={"size": 9})

                fig.suptitle("{} / {}".format(os.path.basename(f), op))
                if "comment" in properties and properties["comment"] is not None:
                    plt.figtext(
                        0.5, 0.93,
                        properties["comment"],
                        ha='center', fontsize=9, color="#f2760d",
                    )
                if "swap_ab_is_broken" in properties and properties["swap_ab_is_broken"]:
                    plt.figtext(
                        0.99, 0.96,
                        "Swap A/B broken",
                        ha='right', fontsize=9, color="#111111",
                        bbox=dict(facecolor='#FF3860', edgecolor='#111111', boxstyle='round,pad=0.5')
                    )

                plt.tight_layout()
                plt.subplots_adjust(top=0.90)

                pp.savefig(fig)
                if pngs:
                    plt.savefig("{}_{}.png".format(f, op))
                if interactive:
                    plt.show()
                plt.close(fig)


if __name__ == "__main__":
    main()
