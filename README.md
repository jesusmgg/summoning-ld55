# Ludum Dare 55 entry
Theme: Summoning

Base code is from my bespoke engine `pipe-warp`, which is a 2D game engine built on top of Rust, Macroquad and Tiled.


## Engine lifecycle functions
Systems should implement these functions as required.
The `GameMgr` calls them in the required order and the `main` function implements the main game loop.

### new
Structure constructors, run only once for each system at the game start. Passing dependencies should be avoided here and almost no logic should take place, other than assigning initial parameters like speeds, positions, scales, and similar ones.

### init
System initialization happens here, including logic that requires dependency passing. For example, passing the `SpriteMgr` to the `Player` so that it can create the corresponding sprite.

Runs only once after initial instantiation.

### spawn
Runs at scene activation. Initialization logic that should be repeated after a new scene is activated should take place here. For example, the `Player` system activating its sprite at the position set in the scene.

### input
Runs each frame.

Input collection and processing should happen here.

### update
Runs each frame.
Main game logic should be done here. A frame time variable `dt` is available to be passed for framerate independent logic for any system that requires it.

### render
Runs each frame
All rendering logic should happen here.


## Scene format
Scenes are implement using the Tiled level editor. The layers are expect to reflect exactly this definition.
### Layers
- Tile layers: contains graphics tiles that will be rendered exactly as in the editor.
- Object layers: contains any kind of object that is expected to be managed by a game system. The objects have a name and a class (strings) that identifies them, alongside a spawning position.
