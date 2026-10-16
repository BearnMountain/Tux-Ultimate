/// GameSession handles all rendering and gameplay for all 
/// different session versions:
/// - Single: full simulation + input handling + interaction + etc
/// - Multi: connect to server + hosting options(play w/ friends)
/// - Hosting: full server handling + commentator viewport
pub trait GameSession {
    /// advance gameplay a timestep + backend processing
    /// - multi: reconcilliation + blending
    fn tick(&mut self);

    /// Monitor refresh rate
    fn frame(&mut self);

    /// cleanly shutdown session own resources
    fn shutdown(&mut self);
}
