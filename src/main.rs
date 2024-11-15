use minifb::{Key, Window, WindowOptions};
use rand::Rng;

// Constants for the window dimensions
const WIDTH: usize = 800;
const HEIGHT: usize = 600;

/// Represents a single ball in the simulation
struct Ball {
    x: i32,           // X position of the ball
    y: i32,           // Y position of the ball
    dx: i32,          // X velocity of the ball
    dy: i32,          // Y velocity of the ball
    radius: i32,      // Radius of the ball
    color: u32,       // Color of the ball
}

impl Ball {
    /// Creates a new ball with given properties
    fn new(x: i32, y: i32, dx: i32, dy: i32, radius: i32, color: u32) -> Self {
        Self { x, y, dx, dy, radius, color }
    }

    /// Updates the position of the ball, bouncing off the walls if needed
    fn update_position(&mut self) {
        self.x += self.dx;
        self.y += self.dy;

        // Bounce off left and right walls
        if self.x - self.radius < 0 || self.x + self.radius > WIDTH as i32 {
            self.dx = -self.dx;
        }
        // Bounce off top and bottom walls
        if self.y - self.radius < 0 || self.y + self.radius > HEIGHT as i32 {
            self.dy = -self.dy;
        }
    }

    /// Checks if this ball collides with another ball
    fn collides_with(&self, other: &Ball) -> bool {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let distance_squared = dx * dx + dy * dy;
        let radius_sum = self.radius + other.radius;
        distance_squared <= radius_sum * radius_sum
    }

    /// Resolves collision with another ball by swapping velocities
    fn resolve_collision(&mut self, other: &mut Ball) {
        if self.collides_with(other) {
            // Swap velocities
            std::mem::swap(&mut self.dx, &mut other.dx);
            std::mem::swap(&mut self.dy, &mut other.dy);
        }
    }

    /// Draws the ball onto the framebuffer
    fn draw(&self, buffer: &mut Vec<u32>) {
        for y in -self.radius..=self.radius {
            for x in -self.radius..=self.radius {
                if x * x + y * y <= self.radius * self.radius {
                    let draw_x = self.x + x;
                    let draw_y = self.y + y;
                    if draw_x >= 0 && draw_x < WIDTH as i32 && draw_y >= 0 && draw_y < HEIGHT as i32 {
                        buffer[(draw_y as usize) * WIDTH + draw_x as usize] = self.color;
                    }
                }
            }
        }
    }
}

/// Manages the simulation of multiple balls
struct BallSimulation {
    balls: Vec<Ball>, // A collection of balls
}

impl BallSimulation {
    /// Creates a new simulation with a specified number of random balls
    fn new(num_balls: usize) -> Self {
        let mut rng = rand::thread_rng();
        let balls = (0..num_balls)
            .map(|_| {
                Ball::new(
                    rng.gen_range(50..WIDTH as i32 - 50),
                    rng.gen_range(50..HEIGHT as i32 - 50),
                    rng.gen_range(-3..3),
                    rng.gen_range(-3..3),
                    20,
                    rng.gen_range(0xFFFFFF..0xFFFFFF + 1),
                )
            })
            .collect();

        Self { balls }
    }

    /// Updates the simulation by moving balls and resolving collisions
    fn update(&mut self) {
        for i in 0..self.balls.len() {
            // Split into two slices to avoid mutable borrow conflicts
            let (left, right) = self.balls.split_at_mut(i + 1);
            let ball_a = &mut left[i];
    
            for ball_b in right {
                ball_a.resolve_collision(ball_b);
            }
        }
    
        for ball in &mut self.balls {
            ball.update_position();
        }
    }
    

    /// Renders all balls into the framebuffer
    fn render(&self, buffer: &mut Vec<u32>) {
        for ball in &self.balls {
            ball.draw(buffer);
        }
    }
}

/// Entry point of the program
fn main() {
    let mut window = Window::new(
        "Ball Simulation with Collisions - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Framebuffer to hold the screen pixels
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // Create a simulation with 10 random balls
    let mut simulation = BallSimulation::new(50);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the screen
        for pixel in buffer.iter_mut() {
            *pixel = 0x000000; // Black
        }

        // Update the simulation
        simulation.update();

        // Render the simulation
        simulation.render(&mut buffer);

        // Display the updated framebuffer
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
