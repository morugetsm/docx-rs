use serde::Serialize;
use std::io::Write;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use super::*;
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Serialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TableCellProperty {
    // 1. w:cnfStyle
    // TODO: Add CnfStyle type
    // 2. w:tcW
    width: Option<TableCellWidth>,
    // 3. w:gridSpan
    grid_span: Option<GridSpan>,
    // 4. w:hMerge
    // TODO: Add HMerge type
    // 5. w:vMerge
    vertical_merge: Option<VMerge>,
    // 6. w:tcBorders
    borders: Option<TableCellBorders>,
    // 7. w:shd
    shading: Option<Shading>,
    #[serde(skip_serializing_if = "Option::is_none")]
    // 8. w:noWrap
    // TODO: Add NoWrap type
    // 9. w:tcMar
    margins: Option<CellMargins>,
    // 10. w:textDirection
    text_direction: Option<TextDirection>,
    // 11. w:tcFitText
    // TODO: Add TcFitText type
    // 12. w:vAlign
    vertical_align: Option<VAlign>,
    // 13. w:hideMark
    // TODO: Add HideMark type
    // 14. w:cellIns
    // TODO: Add CellIns type
    // 15. w:cellDel
    // TODO: Add CellDel type
    // 16. w:cellMerge
    // TODO: Add CellMerge type
    // 17. w:tcPrChange
    // TODO: Add TcPrChange type
}

impl TableCellProperty {
    pub fn new() -> TableCellProperty {
        Default::default()
    }

    pub fn width(mut self, v: usize, t: WidthType) -> TableCellProperty {
        self.width = Some(TableCellWidth::new(v, t));
        self
    }

    pub fn vertical_merge(mut self, t: VMergeType) -> TableCellProperty {
        self.vertical_merge = Some(VMerge::new(t));
        self
    }

    pub fn vertical_align(mut self, t: VAlignType) -> TableCellProperty {
        self.vertical_align = Some(VAlign::new(t));
        self
    }

    pub fn text_direction(mut self, t: TextDirectionType) -> Self {
        self.text_direction = Some(TextDirection::new(t));
        self
    }

    pub fn grid_span(mut self, v: usize) -> TableCellProperty {
        self.grid_span = Some(GridSpan::new(v));
        self
    }

    pub fn shading(mut self, s: Shading) -> Self {
        self.shading = Some(s);
        self
    }

    pub fn set_borders(mut self, borders: TableCellBorders) -> Self {
        self.borders = Some(borders);
        self
    }

    pub fn set_border(mut self, border: TableCellBorder) -> Self {
        self.borders = Some(self.borders.unwrap_or_default().set(border));
        self
    }

    pub fn clear_border(mut self, position: TableCellBorderPosition) -> Self {
        self.borders = Some(self.borders.unwrap_or_default().clear(position));
        self
    }

    pub fn clear_all_border(mut self) -> Self {
        self.borders = Some(self.borders.unwrap_or_default().clear_all());
        self
    }

    pub fn margins(mut self, margins: CellMargins) -> Self {
        self.margins = Some(margins);
        self
    }

    pub fn margin_top(mut self, v: usize, t: WidthType) -> Self {
        if let Some(margins) = self.margins {
            self.margins = Some(margins.margin_top(v, t));
        } else {
            let margins = CellMargins::new();
            self.margins = Some(margins.margin_top(v, t));
        }
        self
    }

    pub fn margin_right(mut self, v: usize, t: WidthType) -> Self {
        if let Some(margins) = self.margins {
            self.margins = Some(margins.margin_right(v, t));
        } else {
            let margins = CellMargins::new();
            self.margins = Some(margins.margin_right(v, t));
        }
        self
    }

    pub fn margin_bottom(mut self, v: usize, t: WidthType) -> Self {
        if let Some(margins) = self.margins {
            self.margins = Some(margins.margin_bottom(v, t));
        } else {
            let margins = CellMargins::new();
            self.margins = Some(margins.margin_bottom(v, t));
        }
        self
    }

    pub fn margin_left(mut self, v: usize, t: WidthType) -> Self {
        if let Some(margins) = self.margins {
            self.margins = Some(margins.margin_left(v, t));
        } else {
            let margins = CellMargins::new();
            self.margins = Some(margins.margin_left(v, t));
        }
        self
    }
}

impl BuildXML for TableCellProperty {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_table_cell_property()?
            .add_optional_child(&self.width)?
            .add_optional_child(&self.grid_span)?
            .add_optional_child(&self.vertical_merge)?
            .add_optional_child(&self.borders)?
            .add_optional_child(&self.shading)?
            .add_optional_child(&self.margins)?
            .add_optional_child(&self.text_direction)?
            .add_optional_child(&self.vertical_align)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_default() {
        let c = TableCellProperty::new();
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:tcPr />"#);
    }

    #[test]
    fn test_grid_span() {
        let c = TableCellProperty::new().grid_span(3);
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tcPr><w:gridSpan w:val="3" /></w:tcPr>"#
        );
    }

    #[test]
    fn test_vmerge() {
        let c = TableCellProperty::new().vertical_merge(VMergeType::Continue);
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tcPr><w:vMerge w:val="continue" /></w:tcPr>"#
        );
    }

    #[test]
    fn test_valign() {
        let c = TableCellProperty::new().vertical_align(VAlignType::Center);
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tcPr><w:vAlign w:val="center" /></w:tcPr>"#
        );
    }

    #[test]
    fn test_shd() {
        let c = TableCellProperty::new()
            .shading(Shading::new().shd_type(ShdType::Clear).fill("FF0000"));
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tcPr><w:shd w:val="clear" w:color="auto" w:fill="FF0000" /></w:tcPr>"#
        );
    }

    #[test]
    fn test_table_cell_prop_json() {
        let c = TableCellProperty::new()
            .vertical_merge(VMergeType::Continue)
            .grid_span(3)
            .width(200, WidthType::Dxa);
        assert_eq!(
            serde_json::to_string(&c).unwrap(),
            r#"{"width":{"width":200,"widthType":"dxa"},"gridSpan":3,"verticalMerge":"continue","borders":null,"shading":null,"textDirection":null,"verticalAlign":null}"#
        );
    }

    #[test]
    fn test_table_cell_prop_json_with_valign() {
        let c = TableCellProperty::new().vertical_align(VAlignType::Center);
        assert_eq!(
            serde_json::to_string(&c).unwrap(),
            r#"{"width":null,"gridSpan":null,"verticalMerge":null,"borders":null,"shading":null,"textDirection":null,"verticalAlign":"center"}"#
        );
    }
}
