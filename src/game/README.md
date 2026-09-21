# Client
Maintains visual interface and game io:
- handles user input
- UI
- gamestate rendering
- 


# Server
Maintains gamestate via authoritative server architecture:
- client input handling(only data that from clients that updates world)
- applies physics
- updates players
- collision
- other reconcilation
    


expand to:
├── characters/
├── combat/          // hitboxes, hurtboxes, damage, knockback
├── entities/        // players, projectiles, items, effects
├── gamemodes/
├── io/              // game-specific input mapping, replay input
├── items/
├── matchmaking/     // if game-specific
├── physics/         // collision rules, ledges, platforms
├── rules/           // stocks, timers, victory conditions
├── stages/
├── state/           // GameState, MatchState
├── systems/         // ECS-like systems if not using ECS
├── tick/            // match simulation
├── replay/
├── scripting/       // optional
└── ui/              // HUD, stock icons, damage %, etc.
