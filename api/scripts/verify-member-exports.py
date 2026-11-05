#!/usr/bin/env python3
"""Independently inspect retained member-export proof files without changing them.

Run with a Python environment containing openpyxl, pypdf and pdfplumber:
  python verify-member-exports.py /private/tmp/revolution-member-export-proof-UUID
Poppler's pdftoppm renders selected pages into a separate QA directory for inspection.
"""
import argparse
from collections import Counter
from datetime import datetime
import csv
import hashlib
import json
from pathlib import Path
import re
import subprocess
import unicodedata
import xml.etree.ElementTree as ET
import zipfile

import openpyxl
import pdfplumber
from pypdf import PdfReader


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def csv_text(value):
    value = value or ""
    significant = value
    while significant and (significant[0].isspace() or significant[0] == "\ufeff"):
        significant = significant[1:]
    return "'" + value if value.startswith(("\t", "\r", "\n")) or significant.startswith(("=", "+", "-", "@")) else value


def unicode_counts(text):
    return Counter(char for char in unicodedata.normalize("NFC", text) if ord(char) > 127 and char.isprintable() and not char.isspace())


def normalized_field(text):
    # PDF line wrapping and text extractors do not retain original whitespace bytes.
    return " ".join(unicodedata.normalize("NFC", text).split())


def poppler_words(path):
    """Use an independent reader that honors PDF ActualText for shaped clusters."""
    result = subprocess.run(["pdftotext", "-bbox-layout", "-enc", "UTF-8", str(path), "-"], check=True, capture_output=True)
    root = ET.fromstring(result.stdout)
    return [[{
        "text": word.text or "", "x0": float(word.attrib["xMin"]),
        "x1": float(word.attrib["xMax"]), "top": float(word.attrib["yMin"]),
        "bottom": float(word.attrib["yMax"]),
    } for word in page.iter() if word.tag.endswith("}word")]
        for page in root.iter() if page.tag.endswith("}page")]


def verify_pdf_rows(pages, expected):
    """Read actual Poppler table cells by column and printed row identity."""
    sources = {row["id"]: row for row in expected}
    rows = {}
    sequence = []
    boundaries = [28, 138, 328, 558, 658, 814]
    for page_number, words in enumerate(pages, 1):
        anchors = sorted(
            (word for word in words
             if 28 <= word["x0"] < 138 and 100 <= word["top"] < 555),
            key=lambda word: word["top"],
        )
        require(bool(anchors), f"PDF page{page_number} has no member identities")
        for index, anchor in enumerate(anchors):
            member_id = anchor["text"]
            require(member_id in sources, f"Unknown or altered PDF member ID {member_id!r} on page{page_number}")
            if not sequence or sequence[-1] != member_id:
                require(member_id not in sequence, f"PDF member {member_id} appears again outside its continuation")
                sequence.append(member_id)
            target = rows.setdefault(member_id, {"cells": [[], [], [], []], "pages": []})
            target["pages"].append(page_number)
            top = max(100, anchor["top"] - 6)
            bottom = anchors[index + 1]["top"] - 6 if index + 1 < len(anchors) else 555
            require(bottom > top, f"Overlapping PDF member identity rows on page{page_number}")
            for column in range(1, 5):
                selected = sorted((word for word in words
                    if boundaries[column] <= (word["x0"] + word["x1"]) / 2 < boundaries[column + 1]
                    and top <= (word["top"] + word["bottom"]) / 2 < bottom),
                    key=lambda word: (word["top"], word["x0"]))
                target["cells"][column - 1].append(" ".join(word["text"] for word in selected))
    require(sequence == [row["id"] for row in expected], "PDF member IDs/order differ from the selected source rows")
    exact_names = 0
    complex_name_observations = []
    for source in expected:
        member_id = source["id"]
        name, email, role, created_at = ["\n".join(parts) for parts in rows[member_id]["cells"]]
        require("".join(email.split()) == source["email"], f"PDF email cell mismatch for member {member_id}")
        require(normalized_field(role) == normalized_field(source["role"] or ""), f"PDF role cell mismatch for member {member_id}")
        require("".join(created_at.split()) == "".join(source["created_at"].split()), f"PDF recorded timestamp mismatch for member {member_id}")
        expected_name = normalized_field(source["name"] or "")
        actual_name = normalized_field(name)
        if actual_name == expected_name:
            exact_names += 1
        else:
            complex_script = any(
                unicodedata.bidirectional(char) in {"R", "AL", "AN"}
                or unicodedata.category(char) in {"Mn", "Mc", "Me"}
                for char in expected_name
            )
            require(complex_script, f"PDF name text mismatch for member {member_id}: {actual_name!r} != {expected_name!r}")
            require(Counter("".join(actual_name.split())) == Counter("".join(expected_name.split())), f"PDF complex-script name lost or duplicated characters for member {member_id}")
            complex_name_observations.append({
                "member_id": member_id, "source_name": source["name"], "extracted_name": name,
                "pages": rows[member_id]["pages"],
                "limitation": "Every per-row source character is retained; this reader emits a different bidi/ligature order. This is not an exact logical-name extraction pass. Source-range tests and rendered source-aware review remain necessary.",
            })
    long_source = max(expected, key=lambda row: len(row["name"] or ""))
    require(len(long_source["name"] or "") == 255, "Long-name fixture does not exercise the actual255-character database bound")
    long_name = "\n".join(rows[long_source["id"]]["cells"][0])
    require(normalized_field(long_name) == normalized_field(long_source["name"]), "PDF lost or changed text from the actual255-character long-name fixture")
    return {
        "semantic_reader": "Poppler pdftotext -bbox-layout, with ActualText support",
        "member_ids_and_order_exact": len(sequence), "email_role_recorded_timestamp_cells_exact": len(rows),
        "name_cells_exact_after_whitespace_normalization": exact_names,
        "complex_script_extraction_observations": complex_name_observations,
        "long_name": {"member_id": long_source["id"], "source_characters": 255, "all_text_preserved_after_whitespace_normalization": True, "pages": rows[long_source["id"]]["pages"]},
        "extraction_limitations": "Whitespace is normalized because PDF wraps fields and extractors discard original spaces/newlines. Text extraction does not prove visual shaping, reading order or absence of clipping; inspect the retained renders.",
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=Path)
    parser.add_argument("--render-dir", type=Path)
    args = parser.parse_args()
    directory = args.directory.resolve(strict=True)
    expected = json.loads((directory / "expected.json").read_text())
    require(len(expected) == 1105, "The independent fixture must contain exactly1105 selected members")
    require(len({row["id"] for row in expected}) == len(expected), "Duplicate fixture IDs")
    headers = ["ID", "Name", "Email", "Role", "Created at (recorded)"]
    with (directory / "members.csv").open(newline="", encoding="utf-8") as stream:
        csv_rows = list(csv.reader(stream))
    require(csv_rows[0] == headers, "Incorrect CSV headers")
    require(len(csv_rows) == len(expected) + 1, "CSV omitted or duplicated members")
    for index, (actual, source) in enumerate(zip(csv_rows[1:], expected), 2):
        wanted = [source["id"], source["name"] or "", source["email"], source["role"] or "", source["created_at"]]
        require(actual == [csv_text(value) for value in wanted], f"CSV source mismatch on row{index}")

    workbook_path = directory / "members.xlsx"
    require(zipfile.is_zipfile(workbook_path), "XLSX is not an OOXML zip package")
    with zipfile.ZipFile(workbook_path) as archive:
        require("xl/workbook.xml" in archive.namelist(), "Missing OOXML workbook")
        require(archive.testzip() is None, "Corrupt OOXML member")
    workbook = openpyxl.load_workbook(workbook_path, read_only=False, data_only=False)
    require(workbook.sheetnames == ["Members"], "Unexpected workbook sheets")
    sheet = workbook["Members"]
    require(sheet.max_row == len(expected) + 1 and sheet.max_column == 5, "Incorrect XLSX dimensions")
    require([cell.value for cell in sheet[1]] == headers, "Incorrect XLSX headers")
    require(sheet.freeze_panes == "A2", "XLSX header is not frozen")
    require(sheet.auto_filter.ref == f"A1:E{len(expected) + 1}", "XLSX autofilter does not cover every member")
    require(sheet.print_title_rows == "$1:$1", "XLSX print header does not repeat")
    layout_observations = []
    for index, source in enumerate(expected, 2):
        cells = list(sheet[index])
        require(all(cell.data_type != "f" for cell in cells), f"Formula found in member data row{index}")
        require([cell.value or "" for cell in cells[:4]] == [source["id"], source["name"] or "", source["email"], source["role"] or ""], f"XLSX literal data mismatch on row{index}")
        require(all(cell.data_type == "s" for cell in cells[:4]), f"XLSX identifier/text types changed on row{index}")
        require(isinstance(cells[4].value, datetime), f"XLSX timestamp is not typed on row{index}")
        # Excel date serials round to milliseconds in openpyxl; source CSV remains exact.
        require(abs((cells[4].value - datetime.fromisoformat(source["created_at"])).total_seconds()) <= .001, f"XLSX timestamp differs by more than1ms on row{index}")
        require(cells[4].number_format == "yyyy-mm-dd hh:mm:ss", f"Missing explicit XLSX timestamp format on row{index}")
        if len(source["name"] or "") >= 200:
            layout_observations.append({"row": index, "member_id": source["id"], "characters": len(source["name"] or ""), "row_height_points": sheet.row_dimensions[index].height, "name_column_width": sheet.column_dimensions["B"].width, "review": "Full cell value verified; inspect the rendered row for clipping at Excel's maximum row height"})

    pdf_path = directory / "members.pdf"
    pdf = PdfReader(pdf_path, strict=True)
    require(len(pdf.pages) > 1, "PDF fixture did not span multiple pages")
    page_text = [page.extract_text() or "" for page in pdf.pages]
    text = "\n".join(page_text)
    semantic_pages = poppler_words(pdf_path)
    semantic_text = "\n".join(word["text"] for page in semantic_pages for word in page)
    actual_emails = Counter(re.findall(r"member-export-\d+@example\.test", text))
    require(actual_emails == Counter(row["email"] for row in expected), "PDF omitted, duplicated or altered member emails")
    expected_unicode = unicode_counts("".join(row["name"] or "" for row in expected))
    require(unicode_counts(semantic_text) == expected_unicode, "PDF lost or altered Unicode source characters in ActualText-aware extraction")
    row_verification = verify_pdf_rows(semantic_pages, expected)
    secondary_reader = {
        "reader": "pypdf (does not resolve all many-to-many cluster ActualText)",
        "unicode_counts_match": unicode_counts(text) == expected_unicode,
        "unexpected_control_codepoints": sorted({f"U+{ord(char):04X}" for char in text if unicodedata.category(char) == "Cc" and char not in "\n\r\t\f"}),
        "limitation": "Retained diagnostic only; plain CMap extraction may expose unmapped glyph IDs even when the ActualText-aware semantic check succeeds.",
    }
    selected_pages = {0, len(pdf.pages) // 2, len(pdf.pages) - 1}
    long_source = max(expected, key=lambda row: len(row["name"] or ""))
    for row in expected:
        if unicode_counts(row["name"] or ""):
            selected_pages.update(index for index, value in enumerate(page_text) if row["email"] in value)
    long_pages = []
    with pdfplumber.open(pdf_path) as document:
        for index, page in enumerate(document.pages):
            require(all(-1 <= char["x0"] <= char["x1"] <= page.width + 1 and -1 <= char["top"] <= char["bottom"] <= page.height + 1 for char in page.chars if char["text"].strip()), f"PDF text extends outside page{index + 1}")
            words = page.extract_words()
            if any(word["text"] == long_source["id"] and word["x0"] < 130 and 100 <= word["top"] <= 555 for word in words):
                long_pages.append(index)
            require(re.search(rf"Page\s+{index + 1}\s+of\s+{len(pdf.pages)}", page_text[index]), f"Incorrect PDF footer on page{index + 1}")
    if long_pages:
        selected_pages.update([long_pages[0], long_pages[len(long_pages) // 2], long_pages[-1]])
    empty = PdfReader(directory / "empty.pdf", strict=True)
    require(len(empty.pages) == 1 and "No members match the selected filters." in (empty.pages[0].extract_text() or ""), "Empty PDF is not an explicit empty result")
    wrapped_name = (directory / "wrapped-name.txt").read_text()
    require(len(wrapped_name) <= 255, "Wrapped-name fixture exceeds the real database field bound")
    wrapped_pdf = PdfReader(directory / "wrapped.pdf", strict=True)
    require(len(wrapped_pdf.pages) > 1, "Wrapped record did not exercise PDF continuation")
    wrapped_text = ""
    with pdfplumber.open(directory / "wrapped.pdf") as document:
        for index, page in enumerate(document.pages):
            wrapped_text += page.crop((138, 100, 328, 555)).extract_text() or ""
            identity = page.crop((28, 100, 138, 555)).extract_text() or ""
            require(bool(re.search(r"\d+", identity)), f"Wrapped PDF continuation lost member identity on page{index + 1}")
    require("".join(wrapped_text.split()) == "".join(wrapped_name.split()), "Wrapped PDF lost, duplicated or altered source name text across pages")
    require(sum((page.extract_text() or "").count("member-export-wrapped@example.test") for page in wrapped_pdf.pages) == 1, "Wrapped PDF repeated or dropped the member email")
    render_directory = (args.render_dir or directory.with_name(directory.name + "-qa")).resolve()
    require(render_directory != directory, "Render into a separate QA directory")
    render_directory.mkdir(parents=True, exist_ok=True)
    images = []
    for index in sorted(selected_pages):
        destination = render_directory / f"members-page-{index + 1}"
        subprocess.run(["pdftoppm", "-f", str(index + 1), "-l", str(index + 1), "-r", "120", "-png", "-singlefile", str(pdf_path), str(destination)], check=True, capture_output=True)
        images.append(str(destination.with_suffix(".png")))
    for filename, pages in [("wrapped.pdf", range(len(wrapped_pdf.pages))), ("empty.pdf", range(1))]:
        for index in pages:
            destination = render_directory / f"{Path(filename).stem}-page-{index + 1}"
            subprocess.run(["pdftoppm", "-f", str(index + 1), "-l", str(index + 1), "-r", "120", "-png", "-singlefile", str(directory / filename), str(destination)], check=True, capture_output=True)
            images.append(str(destination.with_suffix(".png")))
    report = {"members": len(expected), "csv": "all source fields exact; dangerous text escaped", "xlsx": "all source text exact; no formulas; typed dates within1ms reader precision; full filter/frozen/print headers", "pdf": {"pages": len(pdf.pages), "all_member_emails_exactly_once": True, "unicode_source_character_counts_exact": True, "row_verification": row_verification, "secondary_reader_diagnostic": secondary_reader, "long_row_pages": [page + 1 for page in long_pages], "wrapped_record_pages": len(wrapped_pdf.pages), "wrapped_source_name_exact": True}, "sha256": {name: hashlib.sha256((directory / name).read_bytes()).hexdigest() for name in ["members.csv", "members.xlsx", "members.pdf", "wrapped.pdf"]}, "xlsx_layout_observations": layout_observations, "rendered_images_require_visual_review": images}
    (render_directory / "report.json").write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps(report, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
