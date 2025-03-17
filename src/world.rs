//! This file defines the World struct, which brings together all the components
//! of our simulation: the space (grid), input handling, and simulation logic.

// Import the components we need for our world
/// The grid where cells live
use crate::space::Space;
/// Tracks user input
use crate::input::InputTracker;
/// Simulation algorithms
use crate::simulator::{ Simulator, SwappingSim, CellularSim };

/// The World struct is the main container for our simulation
/// It coordinates all the different parts and represents the entire game state
pub struct World {
    /// Whether the simulation is currently running or paused
    pub run: bool,
    /// The grid containing all our cells
    pub space: Space,
    /// Tracks user mouse input
    pub input: InputTracker,
    /// The simulation algorithm to use (boxed trait object)
    simulator: Box<dyn Simulator>,
    /// Last mouse x position for line drawing
    last_x: Option<i32>,
    /// Last mouse y position for line drawing
    last_y: Option<i32>,
}

impl World {
    /// Creates a new world with the given dimensions
    pub fn new(width: u32, height: u32) -> World {
        World {
            run: true,                        // Start with the simulation running
            space: Space::new(width, height), // Create a new empty space with the given dimensions
            input: InputTracker::new(),       // Initialize input tracking
            simulator: Box::new(SwappingSim { }), // Use the SwappingSim algorithm
            //simulator: Box::new(CellularSim { }), // Alternative simulator (commented out)
            last_x: None,                     // No previous x position yet
            last_y: None,                     // No previous y position yet
        }
    }

    /// Toggles whether the simulation is running or paused
    pub fn toggle_run(&mut self) {
        self.run = !self.run; // Flip the boolean value
    }

    /// Returns whether the simulation is currently running
    pub fn is_running(&self) -> bool {
        self.run
    }

    // Draws a line of cells between two points using Bresenham's line algorithm
    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, cell_type: crate::cells::CellType) {
        // Calculate delta values and determine primary direction
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        
        // Determine step direction (positive or negative for each axis)
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        
        // Error value for tracking the accumulated error
        let mut err = dx - dy;
        
        // Current position
        let mut x = x0;
        let mut y = y0;
        
        loop {
            // Add a cell at the current position
            self.space.add(x, y, cell_type);
            
            // Exit if we've reached the end point
            if x == x1 && y == y1 {
                break;
            }
            
            // Calculate double the error to avoid using floating point
            let e2 = 2 * err;
            
            // Adjust error and x if moving horizontally
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            
            // Adjust error and y if moving vertically
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Advances the simulation by one step
    pub fn advance_simulation(&mut self) {
        // If the mouse is down, get its position and add cells
        if let Some((x, y)) = self.input.get_pos() {
            // Add a small offset every other frame for a nicer drawing effect
            let offset = if self.space.get_generation() % 2 == 0 { 0 } else { 1 };
            
            // Get the cell type the user wants to place
            let cell_type = self.input.get_selected_type();
            
            // If we have a previous position, draw a line from it to the current position
            if let (Some(last_x), Some(last_y)) = (self.last_x, self.last_y) {
                // Only draw a line if the position has changed
                if last_x != x || last_y != y {
                    self.draw_line(last_x + offset, last_y, x + offset, y, cell_type);
                } else {
                    // If position hasn't changed, just add cells at the current position
                    self.space.add(x + offset, y, cell_type);
                }
            } else {
                // If there's no previous position, just add cells at the current position
                self.space.add(x + offset, y, cell_type);
            }
            
            // Save the current position for the next frame
            self.last_x = Some(x);
            self.last_y = Some(y);
        } else {
            // If the mouse is not down, clear the last position
            self.last_x = None;
            self.last_y = None;
        }
        
        // Run one tick of the simulation using the current simulator
        self.simulator.tick(&mut self.space);
    }
}

