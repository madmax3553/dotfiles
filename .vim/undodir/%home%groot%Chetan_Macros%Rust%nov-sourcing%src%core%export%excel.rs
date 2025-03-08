Vim�UnDo� �2��@�ѭ�1P���b�����^�F[qJZ�   �              t                           g�@�    _�                             ����                                                                                                                                                                                                                                                                                                                                                             g��     �               �   (//! Provides Excel export functionality.       ?use crate::core::export::formats::{ExportFormat, FormatConfig};   !use crate::core::{Error, Result};   use async_trait::async_trait;   use serde::Serialize;   use std::path::Path;       #[cfg(windows)]   6use rust_xlsxwriter::{Format, FormatBorder, Workbook};       pub struct ExcelExporter {       config: FormatConfig,       output_path: String,   }       impl ExcelExporter {   V    pub fn new(output_path: impl Into<String>, config: FormatConfig) -> Result<Self> {   7        // Ensure that the provided config is for Excel           match config.format {   ,            ExportFormat::Excel => Ok(Self {                   config,   0                output_path: output_path.into(),               }),   O            _ => Err(Error::Export("Invalid format for ExcelExporter".into())),   	        }       }           #[cfg(windows)]   D    fn write_workbook<T: Serialize>(&self, data: &T) -> Result<()> {   '        let workbook = Workbook::new();   1        let mut sheet = workbook.add_worksheet();               // Create header format   )        let header_format = Format::new()               .set_bold()   +            .set_border(FormatBorder::Thin)   "            .set_background_color(                   self.config                       .options   !                    .header_style   %                    .background_color                       .clone()   >                    .unwrap_or_else(|| "#D8E4BC".to_string()),               );       6        // Serialize data to JSON for generic handling   0        let value = serde_json::to_value(data)?;   7        if let serde_json::Value::Array(rows) = value {   D            if self.config.options.use_headers && !rows.is_empty() {   H                if let serde_json::Value::Object(first_row) = &rows[0] {   D                    for (col, key) in first_row.keys().enumerate() {   \                        sheet.write_string_with_format(0, col as u16, key, &header_format)?;                       }                   }               }   ;            for (row_idx, row) in rows.iter().enumerate() {   =                if let serde_json::Value::Object(obj) = row {   F                    for (col_idx, value) in obj.values().enumerate() {   ;                        let cell_value = value.to_string();   +                        sheet.write_string(   $                            (row_idx   F                                + if self.config.options.use_headers {   %                                    1   (                                } else {   %                                    0   *                                }) as u32,   +                            col_idx as u16,   (                            &cell_value,                           )?;                       }                   }               }   	        }   *        workbook.save(&self.output_path)?;           Ok(())       }           #[cfg(not(windows))]   E    fn write_workbook<T: Serialize>(&self, _data: &T) -> Result<()> {   B        Err(Error::Export("Excel export requires Windows".into()))       }   }       #[async_trait]   6impl crate::core::export::Exporter for ExcelExporter {   P    async fn export<T: Serialize + Send + Sync>(&self, data: &T) -> Result<()> {   !        self.write_workbook(data)       }       "    fn get_format(&self) -> &str {           "xlsx"       }   }       #[cfg(test)]   mod tests {       use super::*;       use serde::Serialize;       use tempfile::tempdir;           #[derive(Serialize)]       struct TestData {           id: i32,           name: String,       }           #[tokio::test]   0    async fn test_excel_export() -> Result<()> {           let dir = tempdir()?;   0        let path = dir.path().join("test.xlsx");       #        let config = FormatConfig {   (            format: ExportFormat::Excel,   (            options: Default::default(),   
        };       W        let exporter = ExcelExporter::new(path.to_string_lossy().to_string(), config)?;               let data = vec![               TestData {                   id: 1,   %                name: "Test1".into(),               },               TestData {                   id: 2,   %                name: "Test2".into(),               },   
        ];       &        exporter.export(&data).await?;               assert!(path.exists());           Ok(())       }   }5�5�_�                     t        ����                                                                                                                                                                                                                                                                                                                                                             g�@�    �   �   �                  �   z   |                  �   x   z                  �   s   u   �              5��    s                      �                     �    x                      h                     �    z                      �                     �    �                      n                     5��