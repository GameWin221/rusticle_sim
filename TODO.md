# TO DO:
1. Wrapping
    - A particle can teleport from one edge to another
    - A particle is updated agains other particles that belong to cells on the other side of the map
    - Cells from the other side of the map should be somehow mapped/teleported as if they were a neighbor

2. Post Processing:
    - Bloom

3. UI
    - Better color table

4. Saving and loading

### Maybe get rid of deltatime and introduce a fixed timestep? Deltatime glitches the sim at lower fps

# Bugs and errors:
### MULTITHREADING DOESN'T GIVE AS MUCH BENEFITS AS IT SHOULD - USE A THREADPOOL