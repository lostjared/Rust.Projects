Simple Basic Snake Like Game written in Rust

Use the arrow keys to move the snake, eat the red dots (apples).
As you move over them the snake will grow. Avoid hitting the out edges of the screen
and coliding the snake with itself.

$ cargo run --release 

from this directory to run (needs path to font.ttf)
or to build documentation

$ cargo doc --open

to compile on Windows you will need to link to SDL_ttf 's lib file and set the path to its location
