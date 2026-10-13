/// GameSession handles all rendering and gameplay for all 
/// different session versions:
/// - Single: full simulation + input handling + interaction + etc
/// - Multi: connect to server + hosting options(play w/ friends)
/// - Hosting: full server handling + commentator viewport
pub trait GameSession {
    /// advance gameplay a timestep
    fn tick(&mut self, dt: f32);

    /// process incoming input and network(if multi) updates
    fn update(&mut self, dt: f32);

    /// cleanly shutdown session own resources
    fn shutdown(&mut self);
}
