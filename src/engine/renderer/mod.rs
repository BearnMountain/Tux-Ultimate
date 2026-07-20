pub mod pipeline;
pub mod mesh_builder;
// pub mod model_loader;
pub mod context;
pub mod bind_group_layout;
pub mod bind_group;

pub struct Renderer {
    graphics: context::RenderContext,
}

impl Renderer {
    pub fn new(graphics: context::RenderContext) -> Self {
        return Self {
            graphics,
        };
    }

    pub fn render(&mut self) -> anyhow::Result<bool> {

        return Ok(true);
    }
}
