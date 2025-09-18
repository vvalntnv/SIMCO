# Day 1 - (15-06-2025)

## Particles
Today I have managed to create a simple 2d particle, on which I want to apply forces and see how its position
changes. We will use Newton's formulas and laws to achieve this

## Movements
At first, I want to create a simple bounce. We will keep the 2d vectors and particles, but as soon as 
i get up and running a horizontal movement simulation, or "falling from a hill" simulation, I will move onto the 3d world, since that is what we are trying to simulate

## Time?
Perhaps Time would be important, after all it is considered the 4th dimension in our world.

However, I do not know how to represent it still and what we will need in order to track it. 

### Tick
For now, the program will work in "ticks". A single tick means **the time needed the main loop to iterate \*ONCE\* **
Then, when we get the frequency done, we will move on to actually calculate time in the simulation.

But most probably, the goal will be to get something of the sorts of 60Hz (60 main-loop ticks per secongs), which will really improve how we handle time.

And that should be adjustable. It's logical that a simulation with 120Hz frequency will be WAY MORE accuare than the one with 60Hz, since we can calculate things more granurally. Although I do not know if that can possibly stack more errors, especially if the mathematical operations are not accurate enough and we miss on some small piece when dividing, for example.

This can add up and potentially result in a more "not-ok" simulation...

I'll have to research that one out!
