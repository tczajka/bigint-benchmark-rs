#!/usr/bin/env python3
import argparse
import csv
import datetime
from pathlib import Path

LINKS = {
    "rug": "https://crates.io/crates/rug",
    "malachite": "https://crates.io/crates/malachite",
    "dashu": "https://crates.io/crates/dashu",
    "ibig": "https://crates.io/crates/ibig",
    "num-bigint": "https://crates.io/crates/num-bigint",
}
BEGIN = "<!-- GITHUB_RUNNER_RESULTS_BEGIN -->"
END = "<!-- GITHUB_RUNNER_RESULTS_END -->"


def render_table(path: str) -> str:
    with Path(path).open(encoding="utf-8", newline="") as source:
        rows = list(csv.DictReader(source))

    if not rows:
        msg = "Benchmark produced no results"
        raise SystemExit(msg)

    columns = list(rows[0])[1:]
    minimums = {column: min(float(row[column]) for row in rows) for column in columns}
    output = [
        "| Library | "
        + " | ".join(column.replace("_", " ") for column in columns)
        + " |",
        "|---|" + "---:|" * len(columns),
    ]
    for row in rows:
        cells = [f"[{row['library']}]({LINKS[row['library']]})"]
        for column in columns:
            value = float(row[column])
            cells.append(f"{value:.3f} (x{value / minimums[column]:.2f})")
        output.append("| " + " | ".join(cells) + " |")
    return "\n".join(output)


def update_readme(path: str, table: str) -> None:
    readme = Path(path)
    text = readme.read_text(encoding="utf-8")
    body = "\n".join([
        "<details>",
        "<summary>Results from GitHub runners</summary>",
        "",
        f"Updated {datetime.datetime.now(datetime.UTC):%Y-%m-%d %H:%M} UTC.",
        "",
        table,
        "",
        "</details>",
    ])
    if BEGIN not in text or END not in text:
        msg = "README markers not found"
        raise SystemExit(msg)
    prefix, remainder = text.split(BEGIN, 1)
    _, suffix = remainder.split(END, 1)
    readme.write_text(
        prefix + BEGIN + "\n" + body + "\n" + END + suffix,
        encoding="utf-8",
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("csv")
    parser.add_argument("--readme")
    args = parser.parse_args()
    table = render_table(args.csv)
    if args.readme:
        update_readme(args.readme, table)
    else:
        print(table)


if __name__ == "__main__":
    main()
