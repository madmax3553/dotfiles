Vim�UnDo� ;�e��Pu��9D�~1��UIk��-@r��:u   V                                   g�(N    _�                             ����                                                                                                                                                                                                                                                                                                                                                             g��    �                pub mod settings;5��                          3                      5�_�                             ����                                                                                                                                                                                                                                                                                                                                                             g�(M     �               K   pub mod common;   pub mod demand;   pub mod inventory;       use std::path::PathBuf;       1/// Actions returned by views when handling input   #[derive(Debug, Clone)]   pub enum ViewAction {   	    None,   
    Close,       SelectFile(PathBuf),       SaveConfig,   	    Save,   D    Complete, // Added for signaling completion with records to view   }       A/// Trait for UI components that can be rendered and handle input   pub trait ViewComponent {   R    fn render(&mut self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect);   V    fn handle_input(&mut self, key: crossterm::event::KeyEvent) -> Option<ViewAction>;       fn title(&self) -> &str;   +    fn set_focus(&mut self, focused: bool);   }       /// State shared by all views   #[derive(Debug, Default)]   pub struct ViewState {       pub active: bool,       pub focused: bool,       pub error: Option<String>,        pub message: Option<String>,   }       :/// Pagination state for views that display lists of items   #[derive(Debug, Default)]   pub struct Pagination {       pub current_page: usize,       pub items_per_page: usize,       pub total_pages: usize,   }       impl Pagination {   /    pub fn new(items_per_page: usize) -> Self {           Self {               current_page: 0,   Q            items_per_page: if items_per_page > 0 { items_per_page } else { 20 },               total_pages: 0,   	        }       }       !    pub fn next_page(&mut self) {   C        if self.current_page < self.total_pages.saturating_sub(1) {   #            self.current_page += 1;   	        }       }       !    pub fn prev_page(&mut self) {   "        if self.current_page > 0 {   #            self.current_page -= 1;   	        }       }       8    pub fn update_total(&mut self, total_items: usize) {   $        if self.items_per_page > 0 {   a            self.total_pages = (total_items as f64 / self.items_per_page as f64).ceil() as usize;   &            if self.total_pages == 0 {   %                self.total_pages = 1;               }   6            if self.current_page >= self.total_pages {   9                self.current_page = self.total_pages - 1;               }   	        }       }   }5�5��