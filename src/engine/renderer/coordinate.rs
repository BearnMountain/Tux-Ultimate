use crate::util::config::Config;

pub struct Coordinate;

impl Coordinate {
    /// Converts a pixel position (x, y) into relative (-1..1) space
    pub fn position(x: f32, y: f32) -> (f32, f32, f32) {
        let global_config = Config::get().read().unwrap();
        return (
            (x / global_config.window.width as f32) * 2.0 - 1.0,
            1.0 - (y / global_config.window.height as f32) * 2.0, // flip Y: screen space is top-down, NDC is bottom-up
            0.0,
        );
    }

    /// Converts a pixel-space width/height (a size, not a point) into
    /// relative (-1..1) scale
    pub fn area(width: f32, height: f32) -> (f32, f32) {
        let global_config = Config::get().read().unwrap();
        return (
            (width / global_config.window.width as f32) * 2.0,
            (height / global_config.window.height as f32) * 2.0,
        );
    }
}
