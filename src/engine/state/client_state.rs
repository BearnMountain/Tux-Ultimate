

/*
Authoritative server cares only about user input:
- all keybinds and inputs are registered for processing by server
*/



pub struct ClientState {
    actions: Vec<GameActions>,
}
