1. Post Processing:
    - Bloom

2. Seed for particle colors and positions
    - https://rust-random.github.io/book/guide-seeding.html

3. UI
    - List of saved color tables etc. in the 'saved' folder
    - Saving and loading
    - Better color table
    - World size
    - Buttons for control instead of keyboards keys
    - Time step input field

4. Saving and loading

### Add an option to set a fixed timestep because deltatime glitches the sim at lower framerates
### Maybe bounce particles for the 'Barrier' wrapping mode instead of just clamping it (velocity.(x/y) *= -1.0)

# Bugs and errors:
### MINIMIZING THE WINDOW CRASHES IT
### UI WRAP MODE CANT BE CHANGED BECAUSE UI IS UPDATED FOR A TOO SHORT PEROID OF TIME
### MULTITHREADING DOESN'T GIVE AS MUCH BENEFITS AS IT SHOULD - USE A THREADPOOL