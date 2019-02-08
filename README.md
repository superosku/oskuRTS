
oskuRTS is my RTS project. It is really early in development

Install dependencies

```
brew install rust
brew install sdl2 sdl2_gfx sdl2_ttf sdl2_mixer sdl2_image sdl2_ttf sdl2_gfx
```

Build and run the game

```
cargo run
```

Instructions

```
WASD -> move the camrea
arrows -> move the guy (deprecated)
mouse click -> add unit to mouse position
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

