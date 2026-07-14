Project Pantheon

A Modular, Multi-Domain Autonomous Systems Simulation Framework in Rust

Executive Summary

Project Pantheon is a high-performance, modular simulation sandbox for autonomous vehicles, robotics, and aeronautics. Built entirely in Rust, it leverages the language's memory safety and fearless concurrency to model complex kinematics, sensor arrays, and control systems. The project is designed with a "Core and Plugins" architecture, utilizing a Roman mythology naming convention to separate physical domains into distinct, swappable sub-systems.

🏛️ The Architecture & Naming Convention

The framework is divided into a centralized world engine and specialized domain modules. This allows the simulation to handle everything from a 2D ground vehicle to a 6-DoF (Degrees of Freedom) quadcopter using the same underlying physics loop.

The Core Engine

Pantheon (pantheon-core): The overarching workspace and central nervous system. It defines the universal traits (interfaces) that all vehicles and sensors must implement. (Named after the iconic Roman temple of all the gods).

Caelum (caelum-sim): The simulation environment/world state where entities interact. (The Roman personification of the heavens and sky).

Saturn (saturn-engine): The physics loop and time-step (dt) manager. (The Roman god of time, generation, and cycles).

The Domain Modules ("The Gods")

Mercury (mercury-auto): The automotive/ground vehicle dynamics module (2D physics, steering, kinematic bicycle models). (The Roman god of travel, speed, and roads).

Caelus (caelus-aero): The aeronautics and drone module (3D physics, quaternions, multi-rotor thrust vectors). (The primordial Roman god of the sky).

Janus (janus-vision): The perception and sensor module (LiDAR raycasting, ultrasonic sensors, basic computer vision). (The two-faced Roman god who can see both forward and backward simultaneously—perfect for omnidirectional sensors).

Vulcan (vulcan-kinematics): The robotics and mechanical joint module (for robotic arms or articulated suspension). (The Roman god of the forge, metalworking, and mechanical craftsmanship).

Neptune (neptune-marine): Future expansion for ROVs and marine dynamics. (The Roman god of the sea).

🛠️ The Technology Stack (Rust Crates)

To avoid reinventing the wheel, Pantheon will utilize industry-standard Rust libraries:

bevy: The primary ECS (Entity-Component-System) engine for scalable world management and rendering.

nalgebra: For linear algebra, matrices, and vector math.

rapier2d / rapier3d: For blazing-fast collision detection and raycasting (Sensor simulation).

rayon: For data-parallelism and concurrent sensor processing.

rerun: For advanced telemetry visualization and debugging.

🗺️ Phased Development Roadmap

To minimize the "Math Tax" and ensure a smooth learning curve for Rust, the project will be built in strict phases. Attempting 3D flight physics before mastering Rust's borrow checker is a recipe for burnout.

Phase 1: Foundation & Ground (Mercury)

Focus: 2D Space, 3 Degrees of Freedom (X, Y, Yaw).

Goal: Create a headless (no GUI initially) simulation loop that updates a car's position based on throttle and steering inputs.

Rust Concepts: Structs, basic Enums, Ownership, and basic math modeling.

Deliverable: A kinematic vehicle that can move towards a fixed waypoint using a simple PID controller for steering.

Phase 2: Perception & Sensors (Janus)

Focus: Environmental awareness.

Goal: Implement a 2D raycasting system to act as a simulated LiDAR sensor, preventing the car from hitting virtual walls.

Rust Concepts: Traits (creating generic Sensor interfaces) and Lifetimes (passing environment references safely).

Deliverable: The car dynamically routes around basic obstacles.

Phase 3: The Brain & Concurrency (Core)

Focus: Advanced autonomy.

Goal: Implement A* or RRT pathfinding algorithms.

Rust Concepts: Fearless concurrency. Processing multiple sensor raycasts simultaneously on different threads using rayon.

Deliverable: A vehicle that calculates complex paths through a maze.

Phase 4: Aeronautics Expansion (Caelus)

Focus: 3D Space, 6 Degrees of Freedom (X, Y, Z, Roll, Pitch, Yaw).

Goal: Introduce quadcopters. This requires transitioning from simple Euler angles to Quaternions (to avoid Gimbal Lock) and implementing cascaded PID controllers (Altitude, Position, and Attitude).

Rust Concepts: Advanced Traits and Generics.

Deliverable: A drone that can hover, tilt, and navigate to 3D waypoints.

Phase 5: ECS Refactor & Scalability

Focus: Mass simulation.

Goal: Transition the architecture fully into bevy's ECS.

Rust Concepts: Macros, ECS architectures, and advanced data streaming.

Deliverable: Simulating dozens of heterogeneous vehicles (cars and drones) interacting in the same environment simultaneously, with telemetry piped out to rerun.

🧬 Core Design Philosophy: The SimulatedEntity Trait

The secret to making this multi-domain architecture work is Rust's Trait system. The Pantheon core should never hardcode a "Car" or a "Drone". Instead, it will process any struct that implements a specific trait:

// pantheon-core/src/traits.rs
pub trait SimulatedEntity {
    fn update_physics(&mut self, delta_time: f32, inputs: &ControlInputs);
    fn get_position(&self) -> (f32, f32, f32); // Universally returns 3D coords (Cars just leave Z as 0)
    fn get_heading(&self) -> Quaternion; 
}


By building Mercury (Car) to implement this trait, and later building Caelus (Drone) to implement the exact same trait, the core engine seamlessly simulates both without requiring a massive rewrite.

