Vim�UnDo� �흐ژ��jJ'�'��p ��R���w�g.n  f                                   g�{�     _�                      	        ����                                                                                                                                                                (                                                                                                                                                                                           g�{�     �              �   pub mod column_mapping;   pub mod types;   pub mod validation;       &use crate::core::config::ImportConfig;   0use crate::core::error::{CoreError, CoreResult};   .use crate::core::models::demand::DemandRecord;   .use crate::core::models::demand::DemandStatus;   Kuse calamine::{open_workbook, DataType, Reader, Xlsx}; // Add DataType here   use chrono::{Local, Utc};   use rust_decimal::Decimal;   !use std::fs::{File, OpenOptions};   use std::io::Write;   use std::path::PathBuf;   use std::sync::mpsc::Sender;   use uuid::Uuid;       // Re-export key types   pub use self::types::DemandRow;       +/// Import status reporting for UI feedback   #[derive(Debug, Clone)]   pub enum ImportProgress {       Starting,   0    Processing { current: usize, total: usize },   $    Complete { stats: ImportStats },       Error(String),   }        #[derive(Debug, Clone, Default)]   pub struct ImportStats {       pub total_rows: usize,       pub valid_rows: usize,       pub error_rows: usize,   ]    pub first_error_row: Option<(usize, Vec<String>)>, // Add this field to track first error   }       #[derive(Debug, Default)]   pub struct ImportResult {       pub stats: ImportStats,   #    pub records: Vec<DemandRecord>,       pub errors: Vec<String>,   0    pub rows: Vec<Vec<String>>, // Raw data rows   }       2// Helper function to make cell extraction cleaner   @fn extract_cell(row: &[DataType], col: &str) -> Option<String> {   G    // Use the column_mapping module's function for accurate conversion   @    let col_index = match column_mapping::column_to_index(col) {           Ok(idx) => idx,           Err(e) => {   [            crate::core::log::error(&format!("Invalid column reference '{}': {}", col, e));               return None;   	        }       };       #    // Bounds check to avoid panics       if col_index >= row.len() {   5        // Log this issue instead of silently failing   (        crate::core::log::info(&format!(   =            "Column '{}' (index {}) out of bounds (max: {})",               col,               col_index,               row.len()           ));           return None;       }           match &row[col_index] {            DataType::Empty => None,   ;        DataType::String(s) if s.trim().is_empty() => None,   /        DataType::String(s) => Some(s.clone()),   2        DataType::Float(f) => Some(f.to_string()),   0        DataType::Int(i) => Some(i.to_string()),   1        DataType::Bool(b) => Some(b.to_string()),   Q        DataType::DateTime(dt) => Some(dt.to_string()), // Handle datetime values   Q        DataType::Duration(d) => Some(d.to_string()),   // Handle duration values   .        _ => Some(row[col_index].to_string()),       }   }       ^// Add a new function to extract cells based on the predefined constants with enhanced logging   Vfn extract_cell_by_constant(row: &[DataType], column_index: usize) -> Option<String> {       // Bounds check   "    if column_index >= row.len() {   7        // Log with more detail to help diagnose issues   E        let col_name = column_mapping::get_column_name(column_index);   )        crate::core::log::error(&format!(   S            "Column index {} (column {}) is out of bounds (max: {}). Row data: {}",               column_index,               col_name,               row.len() - 1,   K            &column_mapping::row_to_debug_string(&row[0..row.len().min(5)])           ));           return None;       }           match &row[column_index] {            DataType::Empty => None,   ;        DataType::String(s) if s.trim().is_empty() => None,   /        DataType::String(s) => Some(s.clone()),   2        DataType::Float(f) => Some(f.to_string()),   0        DataType::Int(i) => Some(i.to_string()),   1        DataType::Bool(b) => Some(b.to_string()),   7        DataType::DateTime(dt) => Some(dt.to_string()),   5        DataType::Duration(d) => Some(d.to_string()),   1        _ => Some(row[column_index].to_string()),       }   }       9// Add this helper function to convert Excel serial dates   3fn format_excel_date(date_string: &str) -> String {   C    // Try to parse the string as a float first (Excel serial date)   9    if let Ok(serial_date) = date_string.parse::<f64>() {   N        // Excel dates start from January 0, 1900 (actually December 31, 1899)   #        // with 1 = January 1, 1900   Q        let excel_epoch = chrono::NaiveDate::from_ymd_opt(1899, 12, 31).unwrap();       G        // Calculate days since epoch, handling the Excel leap year bug   9        // (Excel incorrectly treats 1900 as a leap year)   *        let days = if serial_date > 59.0 {   <            // After February 29, 1900 (which doesn't exist)   "            serial_date as i64 - 1           } else {               serial_date as i64   
        };       %        // Add days to the epoch date   Z        if let Some(date) = excel_epoch.checked_add_days(chrono::Days::new(days as u64)) {   !            // Format as DD/MM/YY   7            return date.format("%d/%m/%y").to_string();   	        }       }       M    // If parsing fails or date calculation fails, return the original string       date_string.to_string()   }       b// RECOMMENDATION #1: Create a unified import function with a strategy pattern for cell extraction   v// This would eliminate the duplication between import_demand_file_with_progress and import_demand_file_with_constants       R/// Cell extraction strategy - allows swapping different column mapping approaches   trait CellExtractionStrategy {       fn extract_cell(           &self,           row: &[DataType],           config: &ImportConfig,            column_type: ColumnType,       ) -> Option<String>;   }       enum ColumnType {       PartNumber,       Quantity,       Description,       OrderType,       Location,       DateRequired,   }       '// Then implement different strategies:   struct MappingBasedExtraction;   struct ConstantBasedExtraction;       8impl CellExtractionStrategy for MappingBasedExtraction {       fn extract_cell(           &self,           row: &[DataType],           config: &ImportConfig,            column_type: ColumnType,       ) -> Option<String> {   %        let col = match column_type {   B            ColumnType::PartNumber => &config.columns.part_number,   =            ColumnType::Quantity => &config.columns.quantity,   L            ColumnType::Description => config.columns.description.as_ref()?,   I            ColumnType::OrderType => config.columns.order_type.as_ref()?,   F            ColumnType::Location => config.columns.location.as_ref()?,   O            ColumnType::DateRequired => config.columns.date_required.as_ref()?,   
        };           extract_cell(row, col)       }   }       9impl CellExtractionStrategy for ConstantBasedExtraction {       fn extract_cell(           &self,           row: &[DataType],           config: &ImportConfig,            column_type: ColumnType,       ) -> Option<String> {   +        let col_index = match column_type {   Q            ColumnType::PartNumber => column_mapping::DemandColumns::PART_NUMBER,   L            ColumnType::Quantity => column_mapping::DemandColumns::QUANTITY,   R            ColumnType::Description => column_mapping::DemandColumns::DESCRIPTION,   O            ColumnType::OrderType => column_mapping::DemandColumns::ORDER_TYPE,   L            ColumnType::Location => column_mapping::DemandColumns::LOCATION,   U            ColumnType::DateRequired => column_mapping::DemandColumns::DATE_REQUIRED,   
        };   0        extract_cell_by_constant(row, col_index)       }   }       Q// RECOMMENDATION #2: Improve the Excel date conversion with more robust handling   @fn improved_excel_date_conversion(date_string: &str) -> String {   .    // Try to parse as Excel serial date first   9    if let Ok(serial_date) = date_string.parse::<f64>() {   8        // Calculate days since Excel epoch (1899-12-31)   *        let days = if serial_date > 60.0 {   >            // Account for Excel's incorrect leap year in 1900   &            (serial_date - 1.0) as i64           } else {               serial_date as i64   
        };       O        if let Some(excel_date) = chrono::NaiveDate::from_ymd_opt(1899, 12, 31)   U            .and_then(|epoch| epoch.checked_add_days(chrono::Days::new(days as u64)))   	        {   =            return excel_date.format("%d/%m/%Y").to_string();   	        }       }       B    // If it's already a date string, try to parse and reformat it   ,    // Add additional date formats as needed   E    for format in &["%Y-%m-%d", "%d/%m/%Y", "%m/%d/%Y", "%d-%m-%Y"] {   Y        if let Ok(parsed_date) = chrono::NaiveDate::parse_from_str(date_string, format) {   >            return parsed_date.format("%d/%m/%Y").to_string();   	        }       }       3    // Return original if all parsing attempts fail       date_string.to_string()   }       N// RECOMMENDATION #3: Improve progress reporting with percentage-based updates   fn send_progress_update(   (    tx: &Option<Sender<ImportProgress>>,       current: usize,       total: usize,       last_percentage: &mut i32,   ) -> bool {       if let Some(tx) = tx {   '        // Calculate current percentage   P        let current_percentage = (current as f64 / total as f64 * 100.0) as i32;       F        // Send update if percentage has changed or for first/last row   [        if current_percentage != *last_percentage || current == 0 || current == total - 1 {   2            *last_percentage = current_percentage;   K            let _ = tx.send(ImportProgress::Processing { current, total });               return true;   	        }       }   	    false   }       ?// RECOMMENDATION #4: Add better validation for decimal parsing   =fn parse_quantity(qty_str: &str) -> Result<Decimal, String> {   S    // Handle common Excel issues like thousands separators, currency symbols, etc.       let cleaned = qty_str           .trim()   7        .replace(",", "") // Remove thousand separators   4        .replace("$", "") // Remove currency symbols           .replace("€", "")           .replace("£", "");       -    match Decimal::from_str_exact(&cleaned) {           Ok(qty) => Ok(qty),   O        Err(e) => Err(format!("Invalid quantity format '{}': {}", qty_str, e)),       }   }       G/// Import demand data from a file with a channel for progress tracking   (pub fn import_demand_file_with_progress(       path: PathBuf,   !    import_config: &ImportConfig,   0    progress_tx: Option<Sender<ImportProgress>>,   ) -> CoreResult<ImportResult> {       // Create/open log file   ,    let log_file = setup_import_log_file()?;   -    let mut result = ImportResult::default();           // Send starting event   $    if let Some(tx) = &progress_tx {   2        let _ = tx.send(ImportProgress::Starting);       }       3    // Log import start to file instead of printing       log_message(           &log_file,           &format!(   ;            "Starting import of file: {:?} with sheet: {}",   *            path, import_config.sheet_name   
        ),       );           // Open the Excel workbook   <    let mut workbook: Xlsx<_> = match open_workbook(&path) {           Ok(wb) => wb,           Err(e) => {   D            let err_msg = format!("Failed to open workbook: {}", e);   -            log_message(&log_file, &err_msg);   3            return Err(CoreError::Import(err_msg));   	        }       };   /    let sheet_name = &import_config.sheet_name;           // Get the worksheet   <    let range = match workbook.worksheet_range(sheet_name) {   !        Some(Ok(range)) => range,           Some(Err(e)) => {   T            let err_msg = format!("Failed to access sheet '{}': {}", sheet_name, e);   -            log_message(&log_file, &err_msg);   3            return Err(CoreError::Import(err_msg));   	        }           None => {   F            let err_msg = format!("Sheet '{}' not found", sheet_name);   -            log_message(&log_file, &err_msg);   3            return Err(CoreError::Import(err_msg));   	        }       };       G    // Count rows for progress reporting (exclude header if configured)   E    let total_rows = if import_config.import_options.skip_first_row {   ,        range.rows().len().saturating_sub(1)       } else {           range.rows().len()       };           // Send total count info   $    if let Some(tx) = &progress_tx {   4        let _ = tx.send(ImportProgress::Processing {               current: 0,               total: total_rows,           });       }           let now = Utc::now();       let mut row_index = 0;       $    // Skip header row if configured   4    if import_config.import_options.skip_first_row {           row_index = 1;       }       8    // Log import parameters to file instead of printing       log_message(           &log_file,           &format!(   I            "Import parameters: total_rows={}, columns: part={}, qty={}",   Y            total_rows, import_config.columns.part_number, import_config.columns.quantity   
        ),       );       I    // Log import parameters more clearly including all column references       log_message(           &log_file,           &format!(   u            "Import parameters: total_rows={}, columns: part={}, qty={}, desc={:?}, order={:?}, loc={:?}, date={:?}",               total_rows,    /            import_config.columns.part_number,    +            import_config.columns.quantity,   .            import_config.columns.description,   -            import_config.columns.order_type,   +            import_config.columns.location,   /            import_config.columns.date_required   
        ),       );       -    // Add more debugging for column mappings       log_message(           &log_file,           &format!(   e            "Column mappings: part={} ({}), qty={} ({}), desc={:?}, order={:?}, loc={:?}, date={:?}",   /            import_config.columns.part_number,    �            column_mapping::column_to_index(&import_config.columns.part_number).map_or_else(|_| "invalid".to_string(), |i| i.to_string()),   +            import_config.columns.quantity,   �            column_mapping::column_to_index(&import_config.columns.quantity).map_or_else(|_| "invalid".to_string(), |i| i.to_string()),   �            import_config.columns.description.as_ref().map(|c| format!("{}:{}", c, column_mapping::column_to_index(c).map_or_else(|_| "invalid".to_string(), |i| i.to_string()))),   �            import_config.columns.order_type.as_ref().map(|c| format!("{}:{}", c, column_mapping::column_to_index(c).map_or_else(|_| "invalid".to_string(), |i| i.to_string()))),   �            import_config.columns.location.as_ref().map(|c| format!("{}:{}", c, column_mapping::column_to_index(c).map_or_else(|_| "invalid".to_string(), |i| i.to_string()))),   �            import_config.columns.date_required.as_ref().map(|c| format!("{}:{}", c, column_mapping::column_to_index(c).map_or_else(|_| "invalid".to_string(), |i| i.to_string())))   
        ),       );           // Process each row   >    for (i, row) in range.rows().skip(row_index).enumerate() {   %        result.stats.total_rows += 1;       (        // Report progress every 10 rows   (        if let Some(tx) = &progress_tx {   3            if i % 10 == 0 || i == total_rows - 1 {   <                let _ = tx.send(ImportProgress::Processing {   #                    current: i + 1,   &                    total: total_rows,                   });               }   	        }       5        // Log first few rows for diagnostic purposes           if i < 5 {               log_message(                   &log_file,                   &format!(   #                    "Row {}: {:?}",                       i + 1,   N                    row.iter().map(|c| c.to_string()).collect::<Vec<String>>()                   ),               );   	        }       -        // Extract data using column mappings   P        let part_number = extract_cell(row, &import_config.columns.part_number);   J        let quantity = extract_cell(row, &import_config.columns.quantity);       -        // Log extracted values for debugging           if i < 5 {               log_message(                   &log_file,                   &format!(   >                    "Row {}: part_number={:?}, quantity={:?}",                       i + 1,                        part_number,                       quantity                   ),               );   	        }       '        let description = import_config               .columns               .description               .as_ref()   4            .and_then(|col| extract_cell(row, col));   &        let order_type = import_config               .columns               .order_type               .as_ref()   4            .and_then(|col| extract_cell(row, col));   $        let location = import_config               .columns               .location               .as_ref()   4            .and_then(|col| extract_cell(row, col));   )        let date_required = import_config               .columns               .date_required               .as_ref()   4            .and_then(|col| extract_cell(row, col));       :        // Log the full extracted row for better debugging           if i < 5 {               log_message(                   &log_file,                   &format!(   p                    "Extracted data for Row {}: part={:?}, qty={:?}, desc={:?}, type={:?}, loc={:?}, date={:?}",                       i + 1,                        part_number,                       quantity,                        description,                       order_type,                       location,   !                    date_required                   ),               );   	        }       H        // Store the error parts that will be needed if validation fails   =        let part_number_debug = format!("{:?}", part_number);   7        let quantity_debug = format!("{:?}", quantity);               // Store raw row data   U        let raw_row: Vec<String> = row.iter().map(|cell| cell.to_string()).collect();   *        result.rows.push(raw_row.clone());       #        // Validate required fields   J        if let (Some(part_num), Some(qty_str)) = (part_number, quantity) {   P            // Try to handle different formats of quantity (common excel issues)   >            let cleaned_qty = qty_str.trim().replace(",", "");   9            match Decimal::from_str_exact(&cleaned_qty) {   $                Ok(decimal_qty) => {   <                    // Log the parsed quantity for debugging                        log_message(   "                        &log_file,   Y                        &format!("Parsed quantity '{}' as {}", cleaned_qty, decimal_qty),                       );   /                    let record = DemandRecord {   7                        id: Uuid::new_v4().to_string(),   .                        part_number: part_num,   $                        description,   #                        order_type,   .                        quantity: decimal_qty,   !                        location,   <                        // Format date_required if it exists   4                        date_required: date_required   A                            .map(|date| format_excel_date(&date))   1                            .unwrap_or_default(),   '                        priority: None,   $                        notes: None,   5                        source: "import".to_string(),   (                        created_at: now,   (                        updated_at: now,   2                        status: DemandStatus::New,   #                        cost: None,                       };   0                    result.records.push(record);   1                    result.stats.valid_rows += 1;                   }                   Err(e) => {   1                    result.stats.error_rows += 1;   (                    let error = format!(   =                        "Row {}: Invalid quantity '{}' - {}",   *                        row_index + i + 1,                            qty_str,                           e                       );   3                    log_message(&log_file, &error);   .                    result.errors.push(error);       G                    // Store first error row if this is the first error   ?                    if result.stats.first_error_row.is_none() {   V                        result.stats.first_error_row = Some((row_index + i, raw_row));                       }                   }               }           } else {   )            result.stats.error_rows += 1;                let error = format!(   O                "Row {}: Missing required fields: part_number={}, quantity={}",   "                row_index + i + 1,   "                part_number_debug,                   quantity_debug               );   +            log_message(&log_file, &error);   &            result.errors.push(error);       ?            // Store first error row if this is the first error   7            if result.stats.first_error_row.is_none() {   N                result.stats.first_error_row = Some((row_index + i, raw_row));               }   	        }       }       H    // Make sure the import is marked as complete, even if we had errors       log_message(           &log_file,           &format!(   A            "Import summary: {} total rows, {} valid, {} errors",   U            result.stats.total_rows, result.stats.valid_rows, result.stats.error_rows   
        ),       );       8    // Report completion - make sure this always happens   $    if let Some(tx) = &progress_tx {   ;        // Just send the message without logging the result   2        let _ = tx.send(ImportProgress::Complete {   (            stats: result.stats.clone(),           });       }           Ok(result)   }       8/// Import demand data using predefined column constants   )pub fn import_demand_file_with_constants(       path: PathBuf,   !    import_config: &ImportConfig,   0    progress_tx: Option<Sender<ImportProgress>>,   ) -> CoreResult<ImportResult> {       // Create/open log file   ,    let log_file = setup_import_log_file()?;   -    let mut result = ImportResult::default();           // Send starting event   $    if let Some(tx) = &progress_tx {   2        let _ = tx.send(ImportProgress::Starting);       }       3    // Log import start to file instead of printing       log_message(           &log_file,           &format!(   ;            "Starting import of file: {:?} with sheet: {}",   *            path, import_config.sheet_name   
        ),       );           // Open the Excel workbook   <    let mut workbook: Xlsx<_> = match open_workbook(&path) {           Ok(wb) => wb,           Err(e) => {   D            let err_msg = format!("Failed to open workbook: {}", e);   -            log_message(&log_file, &err_msg);   3            return Err(CoreError::Import(err_msg));   	        }       };           // Get the worksheet   K    let range = match workbook.worksheet_range(&import_config.sheet_name) {   !        Some(Ok(range)) => range,           Some(Err(e)) => {   "            let err_msg = format!(   2                "Failed to access sheet '{}': {}",   +                import_config.sheet_name, e               );   -            log_message(&log_file, &err_msg);   3            return Err(CoreError::Import(err_msg));   	        }           None => {   T            let err_msg = format!("Sheet '{}' not found", import_config.sheet_name);   -            log_message(&log_file, &err_msg);   3            return Err(CoreError::Import(err_msg));   	        }       };       G    // Count rows for progress reporting (exclude header if configured)   E    let total_rows = if import_config.import_options.skip_first_row {   ,        range.rows().len().saturating_sub(1)       } else {           range.rows().len()       };           // Send total count info   $    if let Some(tx) = &progress_tx {   4        let _ = tx.send(ImportProgress::Processing {               current: 0,               total: total_rows,           });       }           let now = Utc::now();   D    let row_index = if import_config.import_options.skip_first_row {   	        1       } else {   	        0       };       &    // Log which constants we're using       log_message(           &log_file,           &format!(   o            "Using column constants: PART_NUMBER={}, QUANTITY={}, DESCRIPTION={}, ORDER_TYPE={}, LOCATION={}",    7            column_mapping::DemandColumns::PART_NUMBER,   4            column_mapping::DemandColumns::QUANTITY,   7            column_mapping::DemandColumns::DESCRIPTION,   6            column_mapping::DemandColumns::ORDER_TYPE,   3            column_mapping::DemandColumns::LOCATION   
        ),       );           // Process each row   >    for (i, row) in range.rows().skip(row_index).enumerate() {   %        result.stats.total_rows += 1;       %        // Report progress for the UI   (        if let Some(tx) = &progress_tx {   3            if i % 10 == 0 || i == total_rows - 1 {   <                let _ = tx.send(ImportProgress::Processing {   #                    current: i + 1,   &                    total: total_rows,                   });               }   	        }       P        // Add enhanced logging for the first several rows and then periodically   B        // This helps diagnose issues that occur later in the file   :        if i < 10 || i % 100 == 0 || i >= total_rows - 5 {               log_message(                   &log_file,                   &format!(   !                    "Row {}: {}",   &                    row_index + i + 1,   :                    column_mapping::debug_row_columns(row)                   ),               );   	        }       B        // Extract data using column constants instead of mappings   d        let part_number = extract_cell_by_constant(row, column_mapping::DemandColumns::PART_NUMBER);   ^        let quantity = extract_cell_by_constant(row, column_mapping::DemandColumns::QUANTITY);   d        let description = extract_cell_by_constant(row, column_mapping::DemandColumns::DESCRIPTION);   b        let order_type = extract_cell_by_constant(row, column_mapping::DemandColumns::ORDER_TYPE);   ^        let location = extract_cell_by_constant(row, column_mapping::DemandColumns::LOCATION);           let date_required =   X            extract_cell_by_constant(row, column_mapping::DemandColumns::DATE_REQUIRED);       1        // Log the extracted values for debugging   :        if i < 10 || i % 100 == 0 || i >= total_rows - 5 {               log_message(                   &log_file,                   &format!(   r                    "Extracted values for Row {}: part={:?}, qty={:?}, desc={:?}, type={:?}, loc={:?}, date={:?}",   &                    row_index + i + 1,                        part_number,                       quantity,                        description,                       order_type,                       location,   !                    date_required                   ),               );   	        }               // Store raw row data   U        let raw_row: Vec<String> = row.iter().map(|cell| cell.to_string()).collect();   *        result.rows.push(raw_row.clone());       M        // Store these for error reporting if needed - avoid the borrow issue   =        let part_number_debug = format!("{:?}", part_number);   7        let quantity_debug = format!("{:?}", quantity);       #        // Validate required fields   Z        if let (Some(part_num), Some(qty_str)) = (part_number.clone(), quantity.clone()) {   P            // Try to handle different formats of quantity (common excel issues)   >            let cleaned_qty = qty_str.trim().replace(",", "");   9            match Decimal::from_str_exact(&cleaned_qty) {   $                Ok(decimal_qty) => {   <                    // Log the parsed quantity for debugging                        log_message(   "                        &log_file,   Y                        &format!("Parsed quantity '{}' as {}", cleaned_qty, decimal_qty),                       );   /                    let record = DemandRecord {   7                        id: Uuid::new_v4().to_string(),   .                        part_number: part_num,   $                        description,   #                        order_type,   .                        quantity: decimal_qty,   !                        location,   <                        // Format date_required if it exists   4                        date_required: date_required   A                            .map(|date| format_excel_date(&date))   1                            .unwrap_or_default(),   '                        priority: None,   $                        notes: None,   5                        source: "import".to_string(),   (                        created_at: now,   (                        updated_at: now,   2                        status: DemandStatus::New,   #                        cost: None,                       };   0                    result.records.push(record);   1                    result.stats.valid_rows += 1;                   }                   Err(e) => {   1                    result.stats.error_rows += 1;   (                    let error = format!(   =                        "Row {}: Invalid quantity '{}' - {}",   *                        row_index + i + 1,                            qty_str,                           e                       );   3                    log_message(&log_file, &error);   .                    result.errors.push(error);       G                    // Store first error row if this is the first error   ?                    if result.stats.first_error_row.is_none() {   V                        result.stats.first_error_row = Some((row_index + i, raw_row));                       }                   }               }           } else {   )            result.stats.error_rows += 1;                let error = format!(   O                "Row {}: Missing required fields: part_number={}, quantity={}",   "                row_index + i + 1,   "                part_number_debug,                   quantity_debug               );   +            log_message(&log_file, &error);   &            result.errors.push(error);       ?            // Store first error row if this is the first error   7            if result.stats.first_error_row.is_none() {   N                result.stats.first_error_row = Some((row_index + i, raw_row));               }   	        }       }       H    // Make sure the import is marked as complete, even if we had errors       log_message(           &log_file,           &format!(   A            "Import summary: {} total rows, {} valid, {} errors",   U            result.stats.total_rows, result.stats.valid_rows, result.stats.error_rows   
        ),       );       8    // Report completion - make sure this always happens   $    if let Some(tx) = &progress_tx {   ;        // Just send the message without logging the result   2        let _ = tx.send(ImportProgress::Complete {   (            stats: result.stats.clone(),           });       }           // Return the result       Ok(result)   }       C// Update the original import function to use the constants version   dpub fn import_demand_file(path: PathBuf, import_config: &ImportConfig) -> CoreResult<ImportResult> {   (    // Use the constants version instead   @    import_demand_file_with_constants(path, import_config, None)   }       ,/// Setup the log file for import operations   0fn setup_import_log_file() -> CoreResult<File> {   0    // Create logs directory if it doesn't exist   )    let logs_dir = PathBuf::from("logs");       if !logs_dir.exists() {   *        std::fs::create_dir_all(&logs_dir)   `            .map_err(|e| CoreError::Import(format!("Failed to create logs directory: {}", e)))?;       }       %    // Create log file with timestamp       let now = Local::now();   I    let filename = format!("import_{}.log", now.format("%Y%m%d_%H%M%S"));   +    let log_path = logs_dir.join(filename);           OpenOptions::new()           .create(true)           .append(true)           .open(log_path)   R        .map_err(|e| CoreError::Import(format!("Failed to open log file: {}", e)))   }       (/// Log a message to the import log file   0fn log_message(log_file: &File, message: &str) {       let now = Local::now();   8    let timestamp = now.format("%Y-%d-%m %H:%M:%S%.3f");       if let Err(_) = writeln!(           &mut log_file               .try_clone()   D            .unwrap_or_else(|_| File::create("/dev/null").unwrap()),           "[{}] {}",           timestamp,           message       ) {   7        // Don't print errors - they'll mess up the TUI       }   }       4/// Validate column mappings against the import file   cpub fn validate_import_mappings(path: &PathBuf, config: &ImportConfig) -> CoreResult<Vec<String>> {   "    let mut messages = Vec::new();   ;    let mut workbook: Xlsx<_> = match open_workbook(path) {           Ok(wb) => wb,           Err(e) => {   U            return Err(CoreError::Import(format!("Failed to open workbook: {}", e)));   	        }       };       *    // Check if the specified sheet exists   (    let sheet_name = &config.sheet_name;   <    let range = match workbook.worksheet_range(sheet_name) {   !        Some(Ok(range)) => range,           Some(Err(e)) => {   U            messages.push(format!("Failed to access sheet '{}': {}", sheet_name, e));                return Ok(messages);   	        }           None => {   "            messages.push(format!(   :                "Sheet '{}' not found. Available sheets:",                   sheet_name               ));   6            for sheet_name in workbook.sheet_names() {   ;                messages.push(format!("- {}", sheet_name));               }                return Ok(messages);   	        }       };       /    // Get the maximum column index in the file       let max_col_idx = range           .rows()           .next()           .map(|row| row.len())           .unwrap_or(0)   ,        .saturating_sub(1); // 0-based index           // Check required columns   H    match column_mapping::column_to_index(&config.columns.part_number) {           Ok(idx) => {   "            if idx > max_col_idx {   &                messages.push(format!(   R                    "Part number column {} (index {}) is out of bounds (max: {})",   @                    config.columns.part_number, idx, max_col_idx                   ));               }   	        }   (        Err(e) => messages.push(format!(   2            "Invalid part number column '{}': {}",   )            config.columns.part_number, e           )),       }       E    match column_mapping::column_to_index(&config.columns.quantity) {           Ok(idx) => {   "            if idx > max_col_idx {   &                messages.push(format!(   O                    "Quantity column {} (index {}) is out of bounds (max: {})",   =                    config.columns.quantity, idx, max_col_idx                   ));               }   	        }   (        Err(e) => messages.push(format!(   /            "Invalid quantity column '{}': {}",   &            config.columns.quantity, e           )),       }           // Check optional columns   4    if let Some(col) = &config.columns.description {   4        match column_mapping::column_to_index(col) {               Ok(idx) => {   &                if idx > max_col_idx {   *                    messages.push(format!(   V                        "Description column {} (index {}) is out of bounds (max: {})",   -                        col, idx, max_col_idx                       ));                   }               }   \            Err(e) => messages.push(format!("Invalid description column '{}': {}", col, e)),   	        }       }       6    // Continue checking remaining optional columns...           Ok(messages)   }5�5��