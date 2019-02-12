
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
WASD -> Move the camrea
mouse left click and drag -> Select units
mouse right click -> Order selected units to go to mouse location
P -> Toggle debug view
N -> Add unit to mouse position (hold to add many quickly)
IO -> Zoom in/out
KL -> Make tile water/land
```

Features

 - Rendering tilemap
 - Zooming and other camera handling
 - Unit collision detection with other units and the map
 - Selecting units
 - Path finding for selected group
 - Much more

Todo next

 - Drawing graphics for units
 - Fix issues with path finding and large unit groups behaving funnyly
 - Different kind of units
 - Different teams and separate colors for teams units
 - Units attacking other team units

