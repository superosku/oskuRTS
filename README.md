
oskuRTS is my RTS project. It is really early in development

Install dependencies

```
brew install rust
brew install sdl2 sdl2_gfx sdl2_ttf sdl2_mixer sdl2_image sdl2_ttf sdl2_gfx
```

Build and run the game

```
# Debug build:

cargo run

# Optimized build:

cargo build --release && ./target/release/rust-game
```


Instructions

```
WASD -> move the camrea
arrows -> move the guy (deprecated)
mouse click and drag -> select units (only debug print for now)
N -> add unit to mouse position (hold to add quickly)
IO -> zoom in/out
KL -> make tile water/land
```

Features

 - Rendering tilemap
 - Zooming and other camera handling
 - Unit collision detection with other units and the map
 - Much more

Todo next

 - Rename entity to unit in code
 - Handle units that end up inside blocked tiles better
 - Drawing guy graphics for units
 - Selecting units
 - Path finding for selected group

