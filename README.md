# SIMCO - Simulation I Myself Code Overtime

**SIMCO** is a modular, real-time physics simulation engine, written in Rust, designed for maximum performance (or at least
as much as possible), extensibility and hands-on learning. Built as a long-term solo project, it brings together low-level programming, high-level
application design, and deep math/physics knowledge.

## Overview 
SIMCO is designed to simulate physical systems like:
- N-body gravitational systems
- Charged particles in fields
- Mass-spring chains
- Crowd and soft-body approximations
- Collisions and constraints

All while exposing the code engine via:
- Python bindings (for scripting, data export, Machine Learning)
- WASM bindings (for browser-based interactive GUIs)
- React Frontend (visual controls, real-time canvas output)

## Tech Stack

| Layer       | Stack                          |
|------------|---------------------------------|
| Simulation | **Rust** + `nalgebra`, `rayon` |
| Backend    | Python + PyO3 + FastAPI         |
| Frontend   | React + Vite + WASM             |
| Dev Tools  | Maturin, wasm-pack, Makefile    |


#### *NOTE* 
The tech stack is still under consideration, and may change

## Project Goals
- Build a high-performance, physics-accurate engine in Rust
- Expose simulations through modern frontend and backend bindings
- Serve as a platform to learn and teach physics concepts
- Stay modular and scalable: physics → scripting → maybe even games
- Stay fun — weekend project that respects my time

## Planned features
- Euler & Verlet integration
- Particle systems
- Force systems (gravity, EM)
- Collision detection (AABB, SAT)
- Mass-spring + constraint systems
- Data export (JSON, CSV)
- Real-time GUI with simulation control
