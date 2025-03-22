Vim�UnDo� Y�۠b,7�Lm�TD@X�ְw�<���\�   �                                   gܳ	    _�                             ����                                                                                                                                                                                                                                                                                                                                                             gۆ'     �               u   .use crate::core::models::demand::DemandRecord;   $use serde::{Deserialize, Serialize};       5/// Struct to hold rules for which records to exclude   8#[derive(Debug, Clone, Serialize, Deserialize, Default)]   pub struct ExclusionRules {   (    /// Specific part numbers to exclude   *    pub exclude_part_numbers: Vec<String>,          <    /// Part number suffixes to exclude (e.g., "-77", "-78")   &    pub exclude_suffixes: Vec<String>,          D    /// Whether to apply exclusion rules automatically during import   %    pub auto_exclude_on_import: bool,   }       impl ExclusionRules {   7    /// Create a new ExclusionRules with default values       pub fn new() -> Self {           Self {   Q            // Modify this list to add or remove specific part numbers to exclude   _            exclude_part_numbers: vec!["10654891-014".to_string(), "10654891-015".to_string()],                  P            // Modify this list to add or remove part number suffixes to exclude   ]            exclude_suffixes: vec!["-77".to_string(), "-78".to_string(), "-OLD".to_string()],                  V            // Whether to apply exclusions during import or wait for manual processing   )            auto_exclude_on_import: true,   	        }       }          1    /// Check if a part number should be excluded   =    pub fn should_exclude(&self, part_number: &str) -> bool {   6        // Check if part number is in the exclude list   G        if self.exclude_part_numbers.iter().any(|p| p == part_number) {               return true;   	        }              E        // Check if part number ends with any of the exclude suffixes   U        if self.exclude_suffixes.iter().any(|suffix| part_number.ends_with(suffix)) {               return true;   	        }                      false       }   }       B/// Process demand records by applying filters and transformations   fpub fn process_demand_records(records: &[DemandRecord], rules: &ExclusionRules) -> Vec<DemandRecord> {   +    let mut processed_records = Vec::new();           for record in records {   $        // Apply the exclusion rules   7        if !rules.should_exclude(&record.part_number) {   3            processed_records.push(record.clone());   	        }       }           processed_records   }       $/// Filter out excluded part numbers   ppub fn filter_excluded_parts<'a>(record: &'a DemandRecord, rules: &ExclusionRules) -> Option<&'a DemandRecord> {   3    if !rules.should_exclude(&record.part_number) {           Some(record)       } else {           None       }   }       "/// Process a single demand record   epub fn process_demand_record(record: &DemandRecord, rules: &ExclusionRules) -> Option<DemandRecord> {   1    filter_excluded_parts(record, rules).cloned()   }       "/// Generate processing statistics   pub struct ProcessingStats {       pub original_count: usize,       pub processed_count: usize,       pub filtered_count: usize,   }       >/// Process records and return both the results and statistics   ]pub fn process_with_stats(records: &[DemandRecord]) -> (Vec<DemandRecord>, ProcessingStats) {   "    // Use default exclusion rules   &    let rules = ExclusionRules::new();          '    let original_count = records.len();   D    let processed_records = process_demand_records(records, &rules);   2    let processed_count = processed_records.len();          !    let stats = ProcessingStats {           original_count,           processed_count,   9        filtered_count: original_count - processed_count,       };              (processed_records, stats)   }       //// Process records with custom exclusion rules   !pub fn process_with_custom_rules(       records: &[DemandRecord],        rules: &ExclusionRules   +) -> (Vec<DemandRecord>, ProcessingStats) {   '    let original_count = records.len();   C    let processed_records = process_demand_records(records, rules);   2    let processed_count = processed_records.len();          !    let stats = ProcessingStats {           original_count,           processed_count,   9        filtered_count: original_count - processed_count,       };              (processed_records, stats)   }5�5�_�                       '    ����                                                                                                                                                                                                                                                                                                                                                             gۆU     �          �      V            exclude_order_types: vec!["Forecast".to_string(), "Planning".to_string()],5��       '                 �                    5�_�                       *    ����                                                                                                                                                                                                                                                                                                                                                             gۆj     �          �      U            exclude_order_types: vec!["On Hand".to_string(), "Planning".to_string()],5��       *                 �                    5�_�                    	        ����                                                                                                                                                                                                                                                                                                                                                             gۆ�    �      �              �   y   {              �   s   u              records: &[DemandRecord], �   m   o              �   g   i              �   c   e              �   K   M          5    if !rules.should_exclude(&record.part_number) && �   @   B          9        if !rules.should_exclude(&record.part_number) && �   3   5              �   0   2                  �   +   -                  �   $   &              �      !                      �                            �                            �                    �                    �      
   �          5��                          4                     �                          �                     �                                               �                          �                     �                          K                     �                          �                     �    $                      b                     �    +                      u                     �    0                      5                     �    3                      J                     �    @   8                  ~	                     �    K   4                  �
                     �    c                      �                     �    g                      W                     �    m                      �                     �    s                     ~                     �    y                      e                     �                          �                     5�_�                            ����                                                                                                                                                                                                                                                                                                                                                             gۋ�     �               �   .use crate::core::models::demand::DemandRecord;   $use serde::{Deserialize, Serialize};       5/// Struct to hold rules for which records to exclude   8#[derive(Debug, Clone, Serialize, Deserialize, Default)]   pub struct ExclusionRules {   (    /// Specific part numbers to exclude   *    pub exclude_part_numbers: Vec<String>,       <    /// Part number suffixes to exclude (e.g., "-77", "-78")   &    pub exclude_suffixes: Vec<String>,       =    /// Order types to exclude (e.g., "Forecast", "Planning")   )    pub exclude_order_types: Vec<String>,       D    /// Whether to apply exclusion rules automatically during import   %    pub auto_exclude_on_import: bool,   }       impl ExclusionRules {   7    /// Create a new ExclusionRules with default values       pub fn new() -> Self {           Self {   Q            // Modify this list to add or remove specific part numbers to exclude   _            exclude_part_numbers: vec!["10654891-014".to_string(), "10654891-015".to_string()],       P            // Modify this list to add or remove part number suffixes to exclude   ]            exclude_suffixes: vec!["-77".to_string(), "-78".to_string(), "-OLD".to_string()],       -            // Default order types to exclude   U            exclude_order_types: vec!["On hand".to_string(), "Planning".to_string()],       V            // Whether to apply exclusions during import or wait for manual processing   )            auto_exclude_on_import: true,   	        }       }       1    /// Check if a part number should be excluded   =    pub fn should_exclude(&self, part_number: &str) -> bool {   6        // Check if part number is in the exclude list   G        if self.exclude_part_numbers.iter().any(|p| p == part_number) {               return true;   	        }       E        // Check if part number ends with any of the exclude suffixes   U        if self.exclude_suffixes.iter().any(|suffix| part_number.ends_with(suffix)) {               return true;   	        }               false       }       1    /// Check if an order type should be excluded   G    pub fn should_exclude_order_type(&self, order_type: &str) -> bool {   @        self.exclude_order_types.iter().any(|t| t == order_type)       }   }       B/// Process demand records by applying filters and transformations   fpub fn process_demand_records(records: &[DemandRecord], rules: &ExclusionRules) -> Vec<DemandRecord> {   +    let mut processed_records = Vec::new();           for record in records {   C        // Apply the exclusion rules for part number and order type   8        if !rules.should_exclude(&record.part_number) &&   A           !rules.should_exclude_order_type(&record.order_type) {   3            processed_records.push(record.clone());   	        }       }           processed_records   }       4/// Filter out excluded part numbers and order types   ppub fn filter_excluded_parts<'a>(record: &'a DemandRecord, rules: &ExclusionRules) -> Option<&'a DemandRecord> {   4    if !rules.should_exclude(&record.part_number) &&   =       !rules.should_exclude_order_type(&record.order_type) {           Some(record)       } else {           None       }   }       "/// Process a single demand record   epub fn process_demand_record(record: &DemandRecord, rules: &ExclusionRules) -> Option<DemandRecord> {   1    filter_excluded_parts(record, rules).cloned()   }       "/// Generate processing statistics   pub struct ProcessingStats {       pub original_count: usize,       pub processed_count: usize,       pub filtered_count: usize,   }       >/// Process records and return both the results and statistics   ]pub fn process_with_stats(records: &[DemandRecord]) -> (Vec<DemandRecord>, ProcessingStats) {   "    // Use default exclusion rules   &    let rules = ExclusionRules::new();       '    let original_count = records.len();   D    let processed_records = process_demand_records(records, &rules);   2    let processed_count = processed_records.len();       !    let stats = ProcessingStats {           original_count,           processed_count,   9        filtered_count: original_count - processed_count,       };           (processed_records, stats)   }       //// Process records with custom exclusion rules   !pub fn process_with_custom_rules(       records: &[DemandRecord],       rules: &ExclusionRules   +) -> (Vec<DemandRecord>, ProcessingStats) {   '    let original_count = records.len();   C    let processed_records = process_demand_records(records, rules);   2    let processed_count = processed_records.len();       !    let stats = ProcessingStats {           original_count,           processed_count,   9        filtered_count: original_count - processed_count,       };           (processed_records, stats)   }5�5�_�                             ����                                                                                                                                                                                                                                                                                                                                                             gܳ     �                 .use crate::core::models::demand::DemandRecord;   $use serde::{Deserialize, Serialize};   use std::sync::{Arc, Mutex};       5/// Struct to hold rules for which records to exclude   8#[derive(Debug, Clone, Serialize, Deserialize, Default)]   pub struct ExclusionRules {   (    /// Specific part numbers to exclude   *    pub exclude_part_numbers: Vec<String>,       <    /// Part number suffixes to exclude (e.g., "-77", "-78")   &    pub exclude_suffixes: Vec<String>,       =    /// Order types to exclude (e.g., "Forecast", "Planning")   )    pub exclude_order_types: Vec<String>,       D    /// Whether to apply exclusion rules automatically during import   %    pub auto_exclude_on_import: bool,   }       impl ExclusionRules {   7    /// Create a new ExclusionRules with default values       pub fn new() -> Self {           Self {   Q            // Modify this list to add or remove specific part numbers to exclude   _            exclude_part_numbers: vec!["10654891-014".to_string(), "10654891-015".to_string()],       P            // Modify this list to add or remove part number suffixes to exclude   ]            exclude_suffixes: vec!["-77".to_string(), "-78".to_string(), "-OLD".to_string()],       -            // Default order types to exclude   U            exclude_order_types: vec!["On hand".to_string(), "Planning".to_string()],       V            // Whether to apply exclusions during import or wait for manual processing   )            auto_exclude_on_import: true,   	        }       }       1    /// Check if a part number should be excluded   =    pub fn should_exclude(&self, part_number: &str) -> bool {   6        // Check if part number is in the exclude list   G        if self.exclude_part_numbers.iter().any(|p| p == part_number) {               return true;   	        }       E        // Check if part number ends with any of the exclude suffixes   U        if self.exclude_suffixes.iter().any(|suffix| part_number.ends_with(suffix)) {               return true;   	        }               false       }       1    /// Check if an order type should be excluded   R    pub fn should_exclude_order_type(&self, order_type: &Option<String>) -> bool {           match order_type {   Y            Some(order_type) => self.exclude_order_types.iter().any(|t| t == order_type),   Q            None => false, // Don't exclude records with no order type by default   	        }       }   }       +/// Enum to track the state of data imports   "#[derive(Debug, Clone, PartialEq)]   pub enum ImportState {   (    /// No import has been attempted yet       NotStarted,   '    /// Import is currently in progress       InProgress,   %    /// Import completed successfully       Completed,   (    /// Import failed with error message       Failed(String),   }       8/// Struct to manage the import process and confirmation   pub struct ImportManager {   +    /// Current state of the import process   #    state: Arc<Mutex<ImportState>>,   ,    /// ExclusionRules to use for processing       rules: ExclusionRules,   }       impl ImportManager {   ?    /// Create a new ImportManager with default exclusion rules       pub fn new() -> Self {           Self {   A            state: Arc::new(Mutex::new(ImportState::NotStarted)),   )            rules: ExclusionRules::new(),   	        }       }       ;    /// Create an ImportManager with custom exclusion rules   6    pub fn with_rules(rules: ExclusionRules) -> Self {           Self {   A            state: Arc::new(Mutex::new(ImportState::NotStarted)),               rules,   	        }       }       $    /// Get the current import state   ,    pub fn get_state(&self) -> ImportState {   /        let state = self.state.lock().unwrap();           state.clone()       }       $    /// Set the current import state   -    fn set_state(&self, state: ImportState) {   ;        let mut current_state = self.state.lock().unwrap();           *current_state = state;       }       0    /// Check if reimport confirmation is needed   .    pub fn needs_confirmation(&self) -> bool {   %        let state = self.get_state();   H        matches!(state, ImportState::Completed | ImportState::Failed(_))       }            /// Start the import process   r    pub fn start_import(&self, records: &[DemandRecord]) -> Result<(Vec<DemandRecord>, ProcessingStats), String> {   8        // Check if we need confirmation before starting   &        if self.needs_confirmation() {   ]            return Err("Previous import exists. Call confirm_reimport() first.".to_string());   	        }       #        // Set state to in progress   0        self.set_state(ImportState::InProgress);               // Process the records   Y        let (processed_records, stats) = process_with_custom_rules(records, &self.rules);       '        // Update state based on result   *        if !processed_records.is_empty() {   3            self.set_state(ImportState::Completed);   *            Ok((processed_records, stats))           } else {   P            let error = "No records processed after applying rules".to_string();   ?            self.set_state(ImportState::Failed(error.clone()));               Err(error)   	        }       }       0    /// Confirm reimport and restart the process   =    pub fn confirm_reimport(&self, confirmed: bool) -> bool {   '        if !self.needs_confirmation() {               return false;   	        }               if confirmed {   -            // Reset the state to start fresh   4            self.set_state(ImportState::NotStarted);               true           } else {   6            // User declined, keep existing data/state               false   	        }       }       )    /// Import with confirmation handling   $    pub fn import_with_confirmation(           &self,   !        records: &[DemandRecord],            confirmed: Option<bool>,   ?    ) -> Result<(Vec<DemandRecord>, ProcessingStats), String> {   >        // If confirmation is needed and not provided/declined   &        if self.needs_confirmation() {               match confirmed {                   Some(true) => {   ?                    // User confirmed, reset state and continue   0                    self.confirm_reimport(true);                   }                    Some(false) => {   $                    // User declined   F                    return Err("Import canceled by user".to_string());                   }                   None => {   ;                    // No confirmation provided, ask for it   ]                    return Err("Previous import exists. Confirmation required.".to_string());                   }               }   	        }       +        // Start/restart the import process   "        self.start_import(records)       }   }       B/// Process demand records by applying filters and transformations   fpub fn process_demand_records(records: &[DemandRecord], rules: &ExclusionRules) -> Vec<DemandRecord> {   +    let mut processed_records = Vec::new();           for record in records {   C        // Apply the exclusion rules for part number and order type   8        if !rules.should_exclude(&record.part_number) &&   A           !rules.should_exclude_order_type(&record.order_type) {   3            processed_records.push(record.clone());   	        }       }           processed_records   }       4/// Filter out excluded part numbers and order types   ppub fn filter_excluded_parts<'a>(record: &'a DemandRecord, rules: &ExclusionRules) -> Option<&'a DemandRecord> {   4    if !rules.should_exclude(&record.part_number) &&   =       !rules.should_exclude_order_type(&record.order_type) {           Some(record)       } else {           None       }   }       "/// Process a single demand record   epub fn process_demand_record(record: &DemandRecord, rules: &ExclusionRules) -> Option<DemandRecord> {   1    filter_excluded_parts(record, rules).cloned()   }       "/// Generate processing statistics   pub struct ProcessingStats {       pub original_count: usize,       pub processed_count: usize,       pub filtered_count: usize,   }       >/// Process records and return both the results and statistics   ]pub fn process_with_stats(records: &[DemandRecord]) -> (Vec<DemandRecord>, ProcessingStats) {   "    // Use default exclusion rules   &    let rules = ExclusionRules::new();       '    let original_count = records.len();   D    let processed_records = process_demand_records(records, &rules);   2    let processed_count = processed_records.len();       !    let stats = ProcessingStats {           original_count,           processed_count,   9        filtered_count: original_count - processed_count,       };           (processed_records, stats)   }       //// Process records with custom exclusion rules   !pub fn process_with_custom_rules(       records: &[DemandRecord],       rules: &ExclusionRules   +) -> (Vec<DemandRecord>, ProcessingStats) {   '    let original_count = records.len();   C    let processed_records = process_demand_records(records, rules);   2    let processed_count = processed_records.len();       !    let stats = ProcessingStats {           original_count,           processed_count,   9        filtered_count: original_count - processed_count,       };           (processed_records, stats)   }5�5��