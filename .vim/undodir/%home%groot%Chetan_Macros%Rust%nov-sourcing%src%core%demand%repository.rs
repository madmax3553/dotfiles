Vim�UnDo� �ö:e�츣y�|6����W;hΔ��.֍�   d   1                .map(|desc| desc.contains(query))   `   *                       g��w    _�                        $    ����                                                                                                                                                                                                                                                                                                                               $          %       v   %    g���     �         X      '        fs::create_dir_all(&data_dir)?;5��       $                  �                     5�_�                    2        ����                                                                                                                                                                                                                                                                                                                               $          %       v   %    g���    �   P   R          6                record.part_number.contains(query) || �   D   F                      �   A   C                      �   4   6                  �   1   3   X                  5��    1                      \                     �    4                      �                     �    A                      e                     �    D                      �                     �    P   5                  	                     5�_�                       $    ����                                                                                                                                                                                                                                                                                                                               $          %       v   %    g���    �         X      %        fs::create_dir_all(&data_dir;5��       $                  �                     5�_�                            ����                                                                                                                                                                                                                                                                                                                               $          %       v   %    g��v    �               X   use super::DemandRecord;   0use crate::core::error::{CoreError, CoreResult};   $use serde::{Deserialize, Serialize};   use std::collections::HashMap;   use std::fs;   use std::path::PathBuf;       (#[derive(Debug, Serialize, Deserialize)]   pub struct DemandRepository {       data_dir: PathBuf,   +    records: HashMap<String, DemandRecord>,       #[serde(skip)]       modified: bool,   }       impl DemandRepository {   7    pub fn new(data_dir: PathBuf) -> CoreResult<Self> {   &        fs::create_dir_all(&data_dir);           Ok(Self {               data_dir,   $            records: HashMap::new(),               modified: false,   
        })       }       P    pub fn save_batch(&mut self, records: Vec<DemandRecord>) -> CoreResult<()> {           for record in records {   D            self.records.insert(record.part_number.clone(), record);   	        }           self.modified = true;           self.persist()       }       C    pub fn get(&self, part_number: &str) -> Option<&DemandRecord> {   %        self.records.get(part_number)       }       1    pub fn get_all(&self) -> Vec<&DemandRecord> {   '        self.records.values().collect()       }       .    pub fn load(&mut self) -> CoreResult<()> {   0        let file_path = self.get_storage_path();            if !file_path.exists() {               return Ok(());   	        }       4        let content = fs::read_to_string(&file_path)   W            .map_err(|e| CoreError::IO(format!("Failed to read demand data: {}", e)))?;       5        self.records = serde_json::from_str(&content)   [            .map_err(|e| CoreError::Parse(format!("Failed to parse demand data: {}", e)))?;               self.modified = false;           Ok(())       }       )    fn persist(&self) -> CoreResult<()> {           if !self.modified {               return Ok(());   	        }       0        let file_path = self.get_storage_path();   >        let json = serde_json::to_string_pretty(&self.records)   c            .map_err(|e| CoreError::Serialize(format!("Failed to serialize demand data: {}", e)))?;       #        fs::write(&file_path, json)   X            .map_err(|e| CoreError::IO(format!("Failed to write demand data: {}", e)))?;               Ok(())       }       +    fn get_storage_path(&self) -> PathBuf {   )        self.data_dir.join("demand.json")       }       =    pub fn search(&self, query: &str) -> Vec<&DemandRecord> {           self.records               .values()               .filter(|record| {   5                record.part_number.contains(query) ||   +                record.description.as_ref()   5                    .map(|desc| desc.contains(query))   %                    .unwrap_or(false)               })               .collect()       }   }5�5�_�                    b       ����                                                                                                                                                                                                                                                                                                                               $          %       v   %    g���     �   a   b              }5��    a                      3                     5�_�                    b        ����                                                                                                                                                                                                                                                                                                                               $          %       v   %    g���     �   a   b          }5��    a                      3                     5�_�                    a       ����                                                                                                                                                                                                                                                                                                                               $          %       v   %    g���    �   a               �   a            5��    a                      3                     �    a                     3                    5�_�      	                      ����                                                                                                                                                                                                                                                                                                                               $          %       v   %    g��s     �               b   use super::DemandRecord;   0use crate::core::error::{CoreError, CoreResult};   $use serde::{Deserialize, Serialize};   use std::collections::HashMap;   use std::fs;   use std::path::PathBuf;       (#[derive(Debug, Serialize, Deserialize)]   pub struct DemandRepository {       data_dir: PathBuf,   +    records: HashMap<String, DemandRecord>,       #[serde(skip)]       modified: bool,   }       impl DemandRepository {   7    pub fn new(data_dir: PathBuf) -> CoreResult<Self> {   '        fs::create_dir_all(&data_dir)?;           Ok(Self {               data_dir,   $            records: HashMap::new(),               modified: false,   
        })       }       >    pub fn save_batch(&mut self, records: Vec<DemandRecord>) {           for record in records {   D            self.records.insert(record.part_number.clone(), record);   	        }           self.modified = true;       }       9    pub fn persist_if_modified(&self) -> CoreResult<()> {           if self.modified {               self.persist()           } else {   "            self.modified = false;               Ok(())   B    pub fn get_all(&self) -> impl Iterator<Item = &DemandRecord> {           self.records.values()       }   C    pub fn get(&self, part_number: &str) -> Option<&DemandRecord> {   %        self.records.get(part_number)       }       1    pub fn get_all(&self) -> Vec<&DemandRecord> {   '        self.records.values().collect()       }       .    pub fn load(&mut self) -> CoreResult<()> {   0        let file_path = self.get_storage_path();            if !file_path.exists() {               return Ok(());   	        }       4        let content = fs::read_to_string(&file_path)   W            .map_err(|e| CoreError::IO(format!("Failed to read demand data: {}", e)))?;       &        if content.trim().is_empty() {   *            self.records = HashMap::new();           } else {   9            self.records = serde_json::from_str(&content)   _                .map_err(|e| CoreError::Parse(format!("Failed to parse demand data: {}", e)))?;   	        }               self.modified = false;           Ok(())       }       )    fn persist(&self) -> CoreResult<()> {           if !self.modified {               return Ok(());   	        }       0        let file_path = self.get_storage_path();   >        let json = serde_json::to_string_pretty(&self.records)   c            .map_err(|e| CoreError::Serialize(format!("Failed to serialize demand data: {}", e)))?;       #        fs::write(&file_path, json)   X            .map_err(|e| CoreError::IO(format!("Failed to write demand data: {}", e)))?;               Ok(())       }       +    fn get_storage_path(&self) -> PathBuf {   )        self.data_dir.join("demand.json")       }   N    pub fn search(&self, query: &str) -> impl Iterator<Item = &DemandRecord> {           self.records               .values()   #            .filter(move |record| {   5                record.part_number.contains(query) ||   +                record.description.as_ref()   5                    .map(|desc| desc.contains(query))   %                    .unwrap_or(false)               })       }   }5�5�_�      
           	   #        ����                                                                                                                                                                                                                                                                                                                               $          %       v   %    g� 6     �               o   use super::DemandRecord;   0use crate::core::error::{CoreError, CoreResult};   $use serde::{Deserialize, Serialize};   use std::collections::HashMap;   use std::fs;   use std::path::PathBuf;       (#[derive(Debug, Serialize, Deserialize)]   pub struct DemandRepository {       data_dir: PathBuf,   +    records: HashMap<String, DemandRecord>,       #[serde(skip)]       modified: bool,   }       impl DemandRepository {   7    pub fn new(data_dir: PathBuf) -> CoreResult<Self> {   '        fs::create_dir_all(&data_dir)?;           Ok(Self {               data_dir,   $            records: HashMap::new(),               modified: false,   
        })       }       >    pub fn save_batch(&mut self, records: Vec<DemandRecord>) {   d        self.records.extend(records.into_iter().map(|record| (record.part_number.clone(), record)));           self.modified = true;       }       =    pub fn persist_if_modified(&mut self) -> CoreResult<()> {           if self.modified {               self.persist()           } else {   "            self.modified = false;               Ok(())   	        }       }       C    pub fn get(&self, part_number: &str) -> Option<&DemandRecord> {   %        self.records.get(part_number)       }       C    pub fn get(&self, part_number: &str) -> Option<&DemandRecord> {       1    pub fn get_all(&self) -> Vec<&DemandRecord> {   '        self.records.values().collect()       }       .    pub fn load(&mut self) -> CoreResult<()> {   0        let file_path = self.get_storage_path();            if !file_path.exists() {               return Ok(());   4        let content = fs::read_to_string(&file_path)   S            .map_err(|_| CoreError::IO("Failed to read demand data".to_string()))?;   4        let content = fs::read_to_string(&file_path)   W            .map_err(|e| CoreError::IO(format!("Failed to read demand data: {}", e)))?;   4        let content = fs::read_to_string(&file_path)   S            .map_err(|_| CoreError::IO("Failed to read demand data".to_string()))?;   *            self.records = HashMap::new();           } else {   9            self.records = serde_json::from_str(&content)   _                .map_err(|e| CoreError::Parse(format!("Failed to parse demand data: {}", e)))?;   	        }               self.modified = false;   )    fn persist(&self) -> CoreResult<()> {   )    fn persist(&self) -> CoreResult<()> {   0        let file_path = self.get_storage_path();   >        let json = serde_json::to_string_pretty(&self.records)   c            .map_err(|e| CoreError::Serialize(format!("Failed to serialize demand data: {}", e)))?;       #        fs::write(&file_path, json)   T            .map_err(|_| CoreError::IO("Failed to write demand data".to_string()))?;               Ok(())       }       #        fs::write(&file_path, json)   X            .map_err(|e| CoreError::IO(format!("Failed to write demand data: {}", e)))?;               Ok(())       }   )    fn persist(&self) -> CoreResult<()> {           if !self.modified {               return Ok(());   )    fn persist(&self) -> CoreResult<()> {   0        let file_path = self.get_storage_path();   >        let json = serde_json::to_string_pretty(&self.records)   c            .map_err(|e| CoreError::Serialize(format!("Failed to serialize demand data: {}", e)))?;       #        fs::write(&file_path, json)   X            .map_err(|e| CoreError::IO(format!("Failed to write demand data: {}", e)))?;               Ok(())       }       }   N    pub fn search(&self, query: &str) -> impl Iterator<Item = &DemandRecord> {           self.records               .values()   #            .filter(move |record| {   5                record.part_number.contains(query) ||           self.records               .iter()   (            .filter(move |(_, record)| {   5                record.part_number.contains(query) ||   +                record.description.as_ref()   5                    .map(|desc| desc.contains(query))   %                    .unwrap_or(false)               })   &            .map(|(_, record)| record)5�5�_�   	              
   +       ����                                                                                                                                                                                                                                                                                                                                                             gĠ"    �   *   ,   Z      1    pub fn get_all(&self) -> Vec<&DemandRecord> {5��    *                     Q                     5�_�   
                 O        ����                                                                                                                                                                                                                                                                                                                                                             gģ�     �   O   Q   [          �   O   Q   Z    5��    O                      �                     �    O                      �                     �    O                     �                     �    O                    �                    5�_�                    P   
    ����                                                                                                                                                                                                                                                                                                                                                             gģ�     �   O   T   [          pub fn �   P   Q   [    5��    O   
                  �                     �    O                    �              �       5�_�                    S       ����                                                                                                                                                                                                                                                                                                                                                             gģ�     �   R   U   ^          } 5��    R                    �	                     �    S                      �	                     5�_�                    S        ����                                                                                                                                                                                                                                                                                                                                                             gģ�    �   R   T   _          } 5��    R                     �	                     5�_�                    P       ����                                                                                                                                                                                                                                                                                                                                                             gģ�    �   O   Q   _      8    pub fn add_record(&mut self, record: DemandRecord) {5��    O                     �                     5�_�                            ����                                                                                                                                                                                                                                                                                                                                                             g�>�   
 �               _   use super::DemandRecord;   0use crate::core::error::{CoreError, CoreResult};   $use serde::{Deserialize, Serialize};   use std::collections::HashMap;   use std::fs;   use std::path::PathBuf;       (#[derive(Debug, Serialize, Deserialize)]   pub struct DemandRepository {       data_dir: PathBuf,   +    records: HashMap<String, DemandRecord>,       #[serde(skip)]       modified: bool,   }       impl DemandRepository {   7    pub fn new(data_dir: PathBuf) -> CoreResult<Self> {   '        fs::create_dir_all(&data_dir)?;           Ok(Self {               data_dir,   $            records: HashMap::new(),               modified: false,   
        })       }       >    pub fn save_batch(&mut self, records: Vec<DemandRecord>) {   d        self.records.extend(records.into_iter().map(|record| (record.part_number.clone(), record)));           self.modified = true;       }       =    pub fn persist_if_modified(&mut self) -> CoreResult<()> {           if self.modified {               self.persist()           } else {               Ok(())   	        }       }       C    pub fn get(&self, part_number: &str) -> Option<&DemandRecord> {   %        self.records.get(part_number)       }       9    pub fn get_all_records(&self) -> Vec<&DemandRecord> {   '        self.records.values().collect()       }       .    pub fn load(&mut self) -> CoreResult<()> {   0        let file_path = self.get_storage_path();            if !file_path.exists() {   *            self.records = HashMap::new();   "            self.modified = false;               return Ok(());   	        }       4        let content = fs::read_to_string(&file_path)   W            .map_err(|e| CoreError::IO(format!("Failed to read demand data: {}", e)))?;       5        self.records = serde_json::from_str(&content)   [            .map_err(|e| CoreError::Parse(format!("Failed to parse demand data: {}", e)))?;               self.modified = false;           Ok(())       }       )    fn persist(&self) -> CoreResult<()> {   0        let file_path = self.get_storage_path();   >        let json = serde_json::to_string_pretty(&self.records)   c            .map_err(|e| CoreError::Serialize(format!("Failed to serialize demand data: {}", e)))?;       #        fs::write(&file_path, json)   X            .map_err(|e| CoreError::IO(format!("Failed to write demand data: {}", e)))?;               Ok(())       }       +    fn get_storage_path(&self) -> PathBuf {   )        self.data_dir.join("demand.json")       }       9    pub fn add_records(&mut self, record: DemandRecord) {   @        self.records.insert(record.part_number.clone(), record);           self.modified = true;       }       N    pub fn search(&self, query: &str) -> impl Iterator<Item = &DemandRecord> {           self.records               .values()   #            .filter(move |record| {   5                record.part_number.contains(query) ||   +                record.description.as_ref()   5                    .map(|desc| desc.contains(query))   %                    .unwrap_or(false)               })       }   }5�5�_�                    Y   L    ����                                                                                                                                                                                                                                                                                                                                                             g���     �   X   Z   c      N    pub fn search(&self, query: &str) -> impl Iterator<Item = &DemandRecord> {5��    X   L                  
                     5�_�                    Y   O    ����                                                                                                                                                                                                                                                                                                                                                             g���     �   X   Z   c      Q    pub fn search(&self, query: &str) -> impl Iterator<Item = &DemandRecord> +  {5��    X   O                  
                     5�_�                    Y   P    ����                                                                                                                                                                                                                                                                                                                                                             g���     �   X   Z   c      S    pub fn search(&self, query: &str) -> impl Iterator<Item = &DemandRecord> + '' {5��    X   P                 
                    5�_�                    Y   S    ����                                                                                                                                                                                                                                                                                                                                                             g���     �   X   [   c      S    pub fn search(&self, query: &str) -> impl Iterator<Item = &DemandRecord> + '_ {5��    X   S                 
                     �    Y                  &   
             &       5�_�                    Z   &    ����                                                                                                                                                                                                                                                                                                                                                             g��     �   Y   [   d      &        let query = query.to_lowercase5��    Y   &                  6
                     5�_�                    Z   (    ����                                                                                                                                                                                                                                                                                                                                                             g��     �   Y   \   d      (        let query = query.to_lowercase()5��    Y   (                  8
                     �    Y   )                 9
              	       �    Z                      :
                     5�_�                    ]   .    ����                                                                                                                                                                                                                                                                                                                                                             g��8     �   \   ^   e      .            record.part_number.contains(query)                   || record�   \   ^   e      .            record.part_number.contains(query)5��    \   .                  �
                     �    \   .                  �
                     �    \   .                  �
                     �    \   .                  �
                     �    \   .                  �
                     �    \   .                  �
                     �    \   .                  �
                     �    \   .                  �
                     �    \   .                  �
                     �    \   .                  �
                     �    \   .                  �
                     �    \   .                  �
                     �    \   .                  �
                     �    \   .                  �
                     �    \   .                  �
                     �    \   .                  �
                     5�_�                    ]   1    ����                                                                                                                                                                                                                                                                                                                                                             g��@     �   \   _   d      8            record.part_number.contains(query) || record5��    \   1                  �
                     �    \   1                 �
                     5�_�                    ^       ����                                                                                                                                                                                                                                                                                                                                                             g��D    �   ]   _   e                  record                        .description�   ]   _   e                  record5��    ]                     �
                     �    ]                     �
                     �    ]                     �
                     �    ]                     �
                     �    ]                     �
                     �    ]                     �
                     �    ]                     �
                     �    ]                     �
                     �    ]                     �
                     �    ]                     �
                     �    ]                     �
                     �    ]                     �
                     �    ]                     �
                     �    ]                     �
                     �    ]                     �
                     �    ]                     �
                     �    ]                     �
                     �    ]                     �
                     �    ]                     �
                     �    ]                     �
                     �    ]                     �
                     5�_�                    X        ����                                                                                                                                                                                                                                                                                                                                                             g��`     �   W   e              S    pub fn search(&self, query: &str) -> impl Iterator<Item = &DemandRecord> + '_ {   )        let query = query.to_lowercase();       4        self.records.values().filter(move |record| {   1            record.part_number.contains(query) ||               record.description                       .as_ref()   5                    .map(|desc| desc.contains(query))   %                    .unwrap_or(false)   
        })       }   }5��    ^                     �
                    �    _                     �
                    �    `                                         5�_�                    ]   (    ����                                                                                                                                                                                                                                                                                                                                                             g��p     �   \   ^   d      1            record.part_number.contains(query) ||5��    \   (                  �
                     5�_�                     `   *    ����                                                                                                                                                                                                                                                                                                                                                             g��v    �   _   a   d      1                .map(|desc| desc.contains(query))5��    _   *                                       5��