//! Actual downloadable formats. Every selected row is emitted or generation fails.
use super::{Member, invalid};
use crate::utils::errors::ApiError;
use chrono::Utc;
use krilla::{
    Document,
    color::rgb,
    geom::{PathBuilder, Point},
    page::PageSettings,
    paint::Fill,
    surface::Surface,
    text::{Font, GlyphId, KrillaGlyph},
};
use parley::{
    Alignment, FontContext as FontSystem, Layout, LayoutContext, LineHeight, OverflowWrap,
    StyleProperty,
};
use rust_xlsxwriter::{Color, Format, FormatAlign, Workbook};
use std::{
    borrow::Cow,
    collections::HashMap,
    ops::Range,
    sync::{Arc, LazyLock, Mutex},
};

const HEADERS: [&str; 5] = ["ID", "Name", "Email", "Role", "Created at (recorded)"];
const FONT_FILES: [&[u8]; 5] = [
    include_bytes!("../../../assets/fonts/NotoSans-Regular.ttf"),
    include_bytes!("../../../assets/fonts/NotoSansArabic-Regular.ttf"),
    include_bytes!("../../../assets/fonts/NotoSansHebrew-Regular.ttf"),
    include_bytes!("../../../assets/fonts/NotoSansDevanagari-Regular.ttf"),
    include_bytes!("../../../assets/fonts/NotoSansCJKsc-Regular.otf"),
];
static PDF_FONTS: LazyLock<Mutex<FontSystem>> = LazyLock::new(|| {
    let mut context = FontSystem {
        collection: parley::fontique::Collection::new(parley::fontique::CollectionOptions {
            system_fonts: false,
            ..Default::default()
        }),
        ..Default::default()
    };
    for bytes in FONT_FILES {
        context
            .collection
            .register_fonts(bytes.to_vec().into(), None);
    }
    // The system-font feature is disabled and the collection is explicitly local.
    Mutex::new(context)
});

pub(super) fn generate(format: &str, members: &[Member]) -> Result<Vec<u8>, ApiError> {
    let bytes = match format {
        "xlsx" => xlsx(members)?,
        "pdf" => pdf(members)?,
        "csv_members" => csv_members(members)?,
        _ => csv(members)?,
    };
    if bytes.len() > 32 * 1024 * 1024 {
        return Err(invalid(
            "Generated export exceeds 32 MiB; narrow the date range",
        ));
    }
    Ok(bytes)
}
fn generation(error: impl std::fmt::Display) -> ApiError {
    tracing::error!(%error, "Member export formatting failed");
    ApiError::new(
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        "Cannot generate member export",
    )
}
// Excel/LibreOffice must not interpret untrusted member values as formulas.
// The prefix is reversible and preserves the original punctuation/newlines.
fn csv_cell(value: String) -> String {
    let significant = value.trim_start_matches(|c: char| c.is_whitespace() || c == '\u{feff}');
    if value.starts_with(['\t', '\r', '\n']) || significant.starts_with(['=', '+', '-', '@']) {
        format!("'{value}")
    } else {
        value
    }
}
fn csv(members: &[Member]) -> Result<Vec<u8>, ApiError> {
    let mut writer = csv::WriterBuilder::new()
        .terminator(csv::Terminator::CRLF)
        .from_writer(Vec::new());
    writer.write_record(HEADERS).map_err(generation)?;
    for member in members {
        writer
            .write_record(member.cells().map(csv_cell))
            .map_err(generation)?;
    }
    writer.into_inner().map_err(generation)
}
fn csv_members(members: &[Member]) -> Result<Vec<u8>, ApiError> {
    let mut writer = csv::WriterBuilder::new()
        .terminator(csv::Terminator::CRLF)
        .from_writer(Vec::new());
    writer
        .write_record(["id", "name", "email", "created_at", "updated_at"])
        .map_err(generation)?;
    for member in members {
        writer
            .write_record(
                [
                    member.id.to_string(),
                    member.name.clone().unwrap_or_default(),
                    member.email.clone(),
                    member.created_at.format("%Y-%m-%d %H:%M:%S%.f").to_string(),
                    member.updated_at.format("%Y-%m-%d %H:%M:%S%.f").to_string(),
                ]
                .map(csv_cell),
            )
            .map_err(generation)?;
    }
    writer.into_inner().map_err(generation)
}
fn xlsx(members: &[Member]) -> Result<Vec<u8>, ApiError> {
    let mut book = Workbook::new();
    let sheet = book.add_worksheet();
    sheet.set_name("Members").map_err(generation)?;
    sheet.set_freeze_panes(1, 0).map_err(generation)?;
    sheet
        .set_landscape()
        .set_paper_size(9)
        .set_repeat_rows(0, 0)
        .map_err(generation)?;
    sheet.set_footer("&LMember export&RPage &P of &N");
    let heading = Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0x17263B))
        .set_font_color(Color::White)
        .set_text_wrap()
        .set_align(FormatAlign::VerticalCenter);
    let text = Format::new().set_text_wrap().set_align(FormatAlign::Top);
    let date = text.clone().set_num_format("yyyy-mm-dd hh:mm:ss");
    for (column, label) in HEADERS.iter().enumerate() {
        sheet
            .write_string_with_format(0, column as u16, *label, &heading)
            .map_err(generation)?;
    }
    sheet.set_row_height(0, 30).map_err(generation)?;
    for (column, width) in [18., 38., 52., 20., 25.].into_iter().enumerate() {
        sheet
            .set_column_width(column as u16, width)
            .map_err(generation)?;
    }
    for (index, member) in members.iter().enumerate() {
        let row = index as u32 + 1;
        for (column, value) in member.cells().iter().take(4).enumerate() {
            if value.encode_utf16().count() > 32_767 {
                return Err(invalid(format!(
                    "Member {} exceeds the Excel cell text limit; use CSV",
                    member.id
                )));
            }
            // Explicit string writes preserve long IDs and never create formulas/URLs.
            sheet
                .write_string_with_format(row, column as u16, value, &text)
                .map_err(generation)?;
        }
        sheet
            .write_datetime_with_format(row, 4, member.created_at, &date)
            .map_err(generation)?;
        let lines = [
            member.name.as_deref().unwrap_or(""),
            &member.email,
            member.role.as_deref().unwrap_or(""),
        ]
        .iter()
        .zip([36, 50, 18])
        .map(|(text, width)| {
            text.split('\n')
                .map(|line| {
                    line.chars()
                        .map(|c| if c.is_ascii() { 1 } else { 2 })
                        .sum::<usize>()
                        .div_ceil(width)
                        .max(1)
                })
                .sum::<usize>()
        })
        .max()
        .unwrap_or(1)
        .max(1);
        let height = 15. * lines as f64 + 6.;
        if height > 409. {
            return Err(invalid(format!(
                "Member {} has more wrapped text than Excel can display in one row; use CSV or PDF",
                member.id
            )));
        }
        sheet.set_row_height(row, height).map_err(generation)?;
    }
    sheet
        .autofilter(0, 0, members.len() as u32, 4)
        .map_err(generation)?;
    book.save_to_buffer().map_err(generation)
}

struct DrawRun {
    font: parley::FontData,
    x: f32,
    size: f32,
    glyphs: Vec<KrillaGlyph>,
}
struct Line {
    text: Arc<str>,
    runs: Vec<DrawRun>,
    baseline: f32,
}
struct TextAtom {
    range: Range<usize>,
    glyphs: Vec<KrillaGlyph>,
}
fn layout(
    fonts: &mut FontSystem,
    text: &str,
    width: f32,
    size: f32,
) -> Result<Vec<Line>, ApiError> {
    let mut context = LayoutContext::new();
    let mut builder = context.ranged_builder(fonts, text, 1., false);
    builder.push_default(StyleProperty::FontSize(size));
    builder.push_default(StyleProperty::FontFamily(parley::FontFamily::Source(Cow::Borrowed(
        "\"Noto Sans\", \"Noto Sans Arabic\", \"Noto Sans Hebrew\", \"Noto Sans Devanagari\", \"Noto Sans CJK SC\""
    ))));
    builder.push_default(StyleProperty::LineHeight(LineHeight::Absolute(size * 1.6)));
    builder.push_default(StyleProperty::OverflowWrap(OverflowWrap::Anywhere));
    let mut shaped: Layout<()> = builder.build(text);
    shaped.break_all_lines(Some(width));
    shaped.align(Alignment::Start, Default::default());
    let source: Arc<str> = Arc::from(text);
    let mut lines = Vec::new();
    for line in shaped.lines() {
        let metrics = line.metrics();
        if metrics.advance - metrics.hanging_advance > width + 0.1 {
            return Err(invalid("PDF text cannot fit the column; use CSV or XLSX"));
        }
        let mut x = metrics.inline_min_coord;
        let mut runs = Vec::new();
        for run in line.runs() {
            let start_x = x;
            let size = run.font_size();
            // Parley assigns every shaped atom's glyphs to its logical first
            // grapheme. Merge continuation ranges before changing visual order:
            // in RTL text a continuation is visited before its start visually.
            let mut atoms: Vec<TextAtom> = Vec::new();
            for cluster in run.clusters() {
                if cluster.is_ligature_continuation() {
                    let atom = atoms
                        .last_mut()
                        .ok_or_else(|| generation("Shaped text is missing its ligature start"))?;
                    atom.range.end = cluster.text_range().end;
                    continue;
                }
                let range = cluster.text_range();
                let mut glyphs = Vec::new();
                for glyph in cluster.glyphs() {
                    if glyph.id == 0 {
                        let missing = text.get(cluster.text_range()).unwrap_or("");
                        if !missing.chars().all(char::is_whitespace) {
                            let codes = missing
                                .chars()
                                .map(|c| format!("U+{:04X}", c as u32))
                                .collect::<Vec<_>>()
                                .join(" ");
                            return Err(invalid(format!(
                                "PDF font coverage is unavailable for {codes}; use CSV or XLSX to preserve this text"
                            )));
                        }
                    }
                    glyphs.push(KrillaGlyph::new(
                        GlyphId::new(glyph.id),
                        glyph.advance / size,
                        glyph.x / size,
                        glyph.y / size,
                        0.,
                        range.clone(),
                        None,
                    ));
                }
                atoms.push(TextAtom { range, glyphs });
            }
            if run.is_rtl() {
                atoms.reverse();
            }
            let mut glyphs = Vec::new();
            for atom in atoms {
                for mut glyph in atom.glyphs {
                    glyph.text_range = atom.range.clone();
                    x += glyph.x_advance * size;
                    glyphs.push(glyph);
                }
            }
            runs.push(DrawRun {
                font: run.font().clone(),
                x: start_x,
                size,
                glyphs,
            });
        }
        lines.push(Line {
            text: source.clone(),
            runs,
            baseline: metrics.baseline - metrics.block_min_coord,
        });
    }
    if lines.is_empty() {
        lines.push(Line {
            text: source,
            runs: Vec::new(),
            baseline: size,
        });
    }
    Ok(lines)
}
fn draw(
    surface: &mut Surface<'_>,
    _fonts: &mut FontSystem,
    cache: &mut HashMap<(u64, u32), Font>,
    line: &Line,
    x: f32,
    top: f32,
) -> Result<(), ApiError> {
    for run in &line.runs {
        let (data, id) = run.font.data.clone().into_raw_parts();
        let key = (id, run.font.index);
        let font = if let Some(font) = cache.get(&key) {
            font.clone()
        } else {
            let font = Font::new(data.into(), run.font.index)
                .ok_or_else(|| generation("Bundled export font is invalid"))?;
            cache.insert(key, font.clone());
            font
        };
        surface.draw_glyphs(
            Point::from_xy(x + run.x, top + line.baseline),
            &run.glyphs,
            font,
            &line.text,
            run.size,
            false,
        );
    }
    Ok(())
}

fn fill(surface: &mut Surface<'_>, color: rgb::Color) {
    surface.set_fill(Some(Fill {
        paint: color.into(),
        ..Default::default()
    }));
}
fn rectangle(surface: &mut Surface<'_>, x: f32, y: f32, w: f32, h: f32) {
    let mut path = PathBuilder::new();
    path.move_to(x, y);
    path.line_to(x + w, y);
    path.line_to(x + w, y + h);
    path.line_to(x, y + h);
    path.close();
    if let Some(path) = path.finish() {
        surface.draw_path(&path);
    }
}
struct Row {
    id: i64,
    cells: Vec<Vec<Line>>,
    lines: usize,
}
const WIDTHS: [f32; 5] = [110., 190., 230., 100., 156.];
const MARGIN: f32 = 28.;
const DATA_TOP: f32 = 100.;
const DATA_BOTTOM: f32 = 555.;
fn pdf(members: &[Member]) -> Result<Vec<u8>, ApiError> {
    let mut fonts = PDF_FONTS
        .lock()
        .map_err(|_| generation("Export font lock failed"))?;
    let mut cache = HashMap::new();
    let mut rows = Vec::new();
    for member in members {
        let cells = member
            .cells()
            .iter()
            .zip(WIDTHS)
            .map(|(text, width)| layout(&mut fonts, text, width - 12., 9.))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|mut error| {
                error.message = format!("Member {}: {}", member.id, error.message);
                error
            })?;
        let lines = cells.iter().map(Vec::len).max().unwrap_or(1);
        rows.push(Row {
            id: member.id,
            cells,
            lines,
        });
    }
    // Store row fragments, not flattened text, so arbitrarily wrapped records can
    // continue across pages with their identity and every field retained.
    let mut pages: Vec<Vec<(usize, usize, usize)>> = vec![Vec::new()];
    let mut y = DATA_TOP;
    for (index, row) in rows.iter().enumerate() {
        let mut start = 0;
        while start < row.lines {
            let available = ((DATA_BOTTOM - y - 10.) / 14.4).floor().max(0.) as usize;
            if available == 0 {
                pages.push(Vec::new());
                y = DATA_TOP;
                continue;
            }
            let end = (start + available).min(row.lines);
            pages
                .last_mut()
                .ok_or_else(|| generation("Missing export page"))?
                .push((index, start, end));
            y += ((end - start) as f32) * 14.4 + 10.;
            start = end;
            if start < row.lines {
                pages.push(Vec::new());
                y = DATA_TOP;
            }
        }
    }
    let mut document = Document::new();
    let generated = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    for (index, fragments) in pages.iter().enumerate() {
        let mut page = document.start_page_with(
            PageSettings::from_wh(842., 595.)
                .ok_or_else(|| generation("Invalid export page size"))?,
        );
        let mut surface = page.surface();
        fill(&mut surface, rgb::Color::new(23, 38, 59));
        for line in layout(&mut fonts, "Members export", 786., 18.)? {
            draw(&mut surface, &mut fonts, &mut cache, &line, MARGIN, 19.)?;
        }
        let caption = format!(
            "{} members | Generated {generated} | Created dates are recorded database values",
            members.len()
        );
        for line in layout(&mut fonts, &caption, 786., 9.)? {
            draw(&mut surface, &mut fonts, &mut cache, &line, MARGIN, 51.)?;
        }
        rectangle(&mut surface, MARGIN, 77., 786., 22.);
        fill(&mut surface, rgb::Color::new(255, 255, 255));
        let mut x = MARGIN;
        for (heading, width) in HEADERS.iter().zip(WIDTHS) {
            for line in layout(&mut fonts, heading, width - 12., 9.)? {
                draw(&mut surface, &mut fonts, &mut cache, &line, x + 6., 80.)?;
            }
            x += width;
        }
        let mut y = DATA_TOP;
        if rows.is_empty() {
            fill(&mut surface, rgb::Color::new(23, 38, 59));
            for line in layout(
                &mut fonts,
                "No members match the selected filters.",
                786.,
                10.,
            )? {
                draw(
                    &mut surface,
                    &mut fonts,
                    &mut cache,
                    &line,
                    MARGIN + 6.,
                    y + 6.,
                )?;
            }
        }
        for &(row_index, start, end) in fragments {
            let row = &rows[row_index];
            let height = (end - start) as f32 * 14.4 + 10.;
            if row_index % 2 == 0 {
                fill(&mut surface, rgb::Color::new(242, 245, 249));
                rectangle(&mut surface, MARGIN, y, 786., height);
            }
            fill(&mut surface, rgb::Color::new(23, 38, 59));
            let mut x = MARGIN;
            for (column, (cell, width)) in row.cells.iter().zip(WIDTHS).enumerate() {
                if column == 0 && start > 0 {
                    let identity = row.id.to_string();
                    let lines = layout(&mut fonts, &identity, width - 12., 8.)?;
                    for (offset, line) in lines.iter().enumerate() {
                        draw(
                            &mut surface,
                            &mut fonts,
                            &mut cache,
                            line,
                            x + 6.,
                            y + 5. + offset as f32 * 12.8,
                        )?;
                    }
                } else {
                    for (offset, line) in cell.iter().skip(start).take(end - start).enumerate() {
                        draw(
                            &mut surface,
                            &mut fonts,
                            &mut cache,
                            line,
                            x + 6.,
                            y + 5. + offset as f32 * 14.4,
                        )?;
                    }
                }
                x += width;
            }
            y += height;
        }
        fill(&mut surface, rgb::Color::new(75, 89, 108));
        for line in layout(
            &mut fonts,
            &format!("Page {} of {}", index + 1, pages.len()),
            786.,
            9.,
        )? {
            draw(&mut surface, &mut fonts, &mut cache, &line, MARGIN, 568.)?;
        }
        surface.finish();
        page.finish();
    }
    document.finish().map_err(generation)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shaped_ligatures_preserve_every_source_character() {
        let mut fonts = PDF_FONTS.lock().unwrap();
        for source in [
            "لا",
            "ليلى أحمد لا",
            "שרה כהן",
            "अर्जुन शर्मा",
            "Latin العربية हिन्दी 李明",
            "A ليلى 123 أحمد B",
            "שרה 123 Cohen",
        ] {
            let lines = layout(&mut fonts, source, 178., 9.).unwrap();
            let mut covered = vec![false; source.len()];
            for glyph in lines
                .iter()
                .flat_map(|line| &line.runs)
                .flat_map(|run| &run.glyphs)
            {
                assert!(source.get(glyph.text_range.clone()).is_some());
                covered[glyph.text_range.clone()].fill(true);
            }
            for (start, character) in source.char_indices() {
                if !character.is_whitespace() {
                    assert!(
                        covered[start..start + character.len_utf8()]
                            .iter()
                            .all(|value| *value),
                        "Shaped source character {character:?} is missing from {source:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn csv_preserves_punctuation_and_prevents_formulas() {
        assert_eq!(csv_cell("  =1+2".into()), "'  =1+2");
        assert_eq!(csv_cell("\tmalicious".into()), "'\tmalicious");
        assert_eq!(
            csv_cell("D'Ávila,\nquoted \"name\"".into()),
            "D'Ávila,\nquoted \"name\""
        );
    }
}
