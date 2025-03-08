Vim�UnDo� "�~�2��J
�����>W���E�b����   �   'pub use crate::core::import::DemandRow;   �         	       	   	   	    g�@    _�                     �       ����                                                                                                                                                                                                                                                                                                                                                             g���     �   �   �   �    �   �   �   �    5��    �                      �                     5�_�                    �       ����                                                                                                                                                                                                                                                                                                                                                             g���    �   �   �   �      pub mod data;5��    �                    �                    5�_�                    �        ����                                                                                                                                                                                                                                                                                                                                                             g� R     �               �   /*!       # NOV Sourcing Library       O    This library implements the core functionality for the NOV Sourcing system,   X    including modules for core processing, business logic, platform-specific operations,   Z    and user interface components. It exposes a public API via re-exports and the prelude.   3    See the SRS for complete architectural details.   */       use crossterm::{       execute,   ^    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},   };   use std::io;       #[derive(Debug, PartialEq)]   pub enum AppState {       Running,       Configure,   2    Demand,         // Added for demand management   )    ImportSettings, // Add this new state       ConfirmQuit,       Quitting,   }       #[derive(Debug, PartialEq)]   pub enum MenuItem {       Demand,       Inventory,       Analysis,   }       pub struct App {       state: AppState,       selected_menu: MenuItem,   }       
impl App {   "    pub fn new() -> Result<Self> {           // Setup terminal           enable_raw_mode()?;   6        execute!(io::stdout(), EnterAlternateScreen)?;               Ok(Self {   %            state: AppState::Running,   ,            selected_menu: MenuItem::Demand,   
        })       }       #    pub fn running(&self) -> bool {   (        self.state != AppState::Quitting       }       &    pub fn state(&self) -> &AppState {           &self.state       }       $    pub fn request_quit(&mut self) {   +        self.state = AppState::ConfirmQuit;       }       $    pub fn confirm_quit(&mut self) {   (        self.state = AppState::Quitting;       }       #    pub fn cancel_quit(&mut self) {   '        self.state = AppState::Running;       }       !    pub fn next_menu(&mut self) {   7        self.selected_menu = match self.selected_menu {   4            MenuItem::Demand => MenuItem::Inventory,   6            MenuItem::Inventory => MenuItem::Analysis,   3            MenuItem::Analysis => MenuItem::Demand,   
        };       }       !    pub fn prev_menu(&mut self) {   7        self.selected_menu = match self.selected_menu {   3            MenuItem::Demand => MenuItem::Analysis,   4            MenuItem::Inventory => MenuItem::Demand,   6            MenuItem::Analysis => MenuItem::Inventory,   
        };       }       .    pub fn selected_menu(&self) -> &MenuItem {           &self.selected_menu       }       -    pub fn cleanup(&mut self) -> Result<()> {           // Cleanup terminal           disable_raw_mode()?;   6        execute!(io::stdout(), LeaveAlternateScreen)?;           Ok(())       }       $    pub fn enter_config(&mut self) {   )        self.state = AppState::Configure;       }       #    pub fn exit_config(&mut self) {   '        self.state = AppState::Running;       }       $    pub fn enter_demand(&mut self) {   &        self.state = AppState::Demand;       }       #    pub fn exit_demand(&mut self) {   '        self.state = AppState::Running;       }       -    pub fn enter_import_settings(&mut self) {   .        self.state = AppState::ImportSettings;       }       ,    pub fn exit_import_settings(&mut self) {   '        self.state = AppState::Running;       }   }       impl Drop for App {       fn drop(&mut self) {   4        // Ensure we clean up even if the app panics           let _ = self.cleanup();       }   }       impl Default for App {       fn default() -> Self {           Self::new().unwrap()       }   }       (pub type Result<T> = std::io::Result<T>;       -// Remove unused modules and simplify exports   pub mod core;   pub mod ui;   pub mod business;   pub mod data;   pub mod state;       %// Only export what's actually needed    pub use core::models::DemandRow;    pub use core::error::CoreResult;   #pub use core::demand::DemandRecord;       #[cfg(test)]   mod tests {       #[test]       fn it_works() {           assert_eq!(2 + 2, 4);       }   }5�5�_�                    �        ����                                                                                                                                                                                                                                                                                                                                                             g�G     �   �   �              #[cfg(test)]   mod tests {       #[test]       fn it_works() {           assert_eq!(2 + 2, 4);       }   }5��    �                            `               5�_�                    �        ����                                                                                                                                                                                                                                                                                                                                                             g�I    �   �   �   �      // pub mod util;  5��    �                     �                     5�_�                    �       ����                                                                                                                                                                                                                                                                                                                                                             g�;     �   �   �   �      'pub use crate::core::import::DemandRow;5��    �                     �                     5�_�                    �       ����                                                                                                                                                                                                                                                                                                                                                             g�>     �   �   �   �      "pub use crate::core::i::DemandRow;5��    �                     �                     5�_�      	              �       ����                                                                                                                                                                                                                                                                                                                                                             g�>     �   �   �   �      !pub use crate::core::::DemandRow;5��    �                     �                     5�_�                  	   �       ����                                                                                                                                                                                                                                                                                                                                                             g�?    �   �   �   �       pub use crate::core:::DemandRow;5��    �                     �                     5��