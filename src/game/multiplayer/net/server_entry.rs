use egui::Color32;



pub struct ServerEntry {
    pub thumb_color: Color32, // placeholder swap-in for a TextureHandle
    pub name: String,
    pub map: String,
    pub mode: String,
    pub players: u32,
    pub max_players: u32,
    pub ping: u8, // 1..=4 bars
    pub host: String,
    pub subtitle: String,
    pub tags: Vec<(&'static str, &'static str)>, // (left col, right col)
    pub map_preview_color: Color32,
    pub next_map: String,
}
