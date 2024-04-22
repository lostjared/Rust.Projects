# Rust Pong Game

This project is a simple implementation of the classic Pong game written in Rust using the SDL2 library. It features a basic 2D game where two players control paddles to hit a ball back and forth across the screen.

## Features

- 2D Pong game logic.
- Two player mode using keyboard controls:
  - Player 1 controls: 'A' (up), 'S' (down).
  - Player 2 controls: Up Arrow (up), Down Arrow (down).
- Random ball direction changes when hitting paddles or borders.
- Basic collision detection and response.
- Score resetting when the ball goes out of the horizontal bounds.

## Requirements

To run this game, you need to have Rust and SDL2 installed on your system. 

### Dependencies

- `rustc` (Rust Compiler)
- `sdl2` - Used for creating the window, handling events, and rendering.

## Installation

1. **Install Rust:**

   Follow the instructions to install Rust for your operating system from the [official Rust site](https://www.rust-lang.org/tools/install).

2. **SDL2 Installation:**

   - **Linux:** Install SDL2 using your package manager. For example, on Ubuntu:
     ```bash
     sudo apt-get install libsdl2-dev
     ```

   - **Windows:** Download the SDL2 development libraries from the [SDL website](https://www.libsdl.org/download-2.0.php) and set up according to your project setup guide.

   - **macOS:** Install using Homebrew:
     ```bash
     brew install sdl2
     ```

4. **Build and Run:**

   Compile and run the project using Cargo:
   ```bash
   cargo run
   ```

## How to Play

- Run the program.
- Use the defined keys to move the paddles up and down.
- Try to hit the ball such that it passes the opponent's paddle.

## Contributing

Contributions are welcome! Feel free to submit pull requests or open issues to improve the game or fix bugs.

## License

This project is open-sourced under the GPLv3 License. See the LICENSE file for more details.
