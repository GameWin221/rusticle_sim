# TO DO:
1. Wrapping
    - A particle can teleport from one edge to another
    - A particle is updated agains other particles that belong to cells on the other side of the map


2. World partitioning
    - The world is divided into cells of size `max_r`
    - A particle belongs to only one cell
    - A particle is updated agains other particles in the cell it belongs to and the neighbouring cells
    - Before each physics update, there will be a partition update which assigns particles to cells
    - A partition holds indexes to certain particles

# CELLS AREN'T ALWAYS THE SAME, THEY ARE SMALLER/BIGERR ON TOP AND RIGHT EDGE BECAUSE OF HOW IT IS CALCULATED


3. Variable particle smoothness
    - Add a push constant for smoothstep min bound


4. UI
    - Physics properties panel
    - Color properties
    - Rendering
    - Debug and metrics