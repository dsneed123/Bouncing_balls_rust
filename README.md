# Bouncing Balls 
A Rust-based graphical simulation of bouncing balls using the `minifb` crate. Balls move independently, bounce off walls, and collide with each other, with their velocities adjusted upon impact. The simulation includes random initialization of ball positions, speeds, and colors, offering smooth and interactive visuals.

## Features
- **Realistic Ball Movement**: Balls move independently with configurable speeds and directions.
- **Wall Collisions**: Balls bounce off the edges of the screen.
- **Ball Collisions**: When balls collide, their velocities are swapped to simulate elastic collisions.
- **Random Initialization**: Balls are initialized at random positions, velocities, and colors.
- **Efficient Rendering**: Optimized rendering using a framebuffer for smooth visuals.

## Technologies Used
- **Rust**: A modern systems programming language with safety and performance guarantees.
- **minifb**: A lightweight window and framebuffer library for graphics rendering.
- **rand**: A Rust crate used for random number generation.

## How to Run
1. **Install Rust**:
   Ensure you have Rust installed. If not, download it from [rust-lang.org](https://www.rust-lang.org/).

2. **Clone the Repository**:
   ```bash
   git clone https://github.com/your-username/bouncing-balls-simulation.git
   cd bouncing-balls-simulation
