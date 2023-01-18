# Rusticle Simulator
Just a simple Particle Life simulation made in Rust.

I made this project after I got inspired by [this great video by Tom Mohr](https://www.youtube.com/watch?v=p4YirERTVF0).

This was mostly a quick experiment so the code definitely won't be in a perfect shape.

## Main goals of this project:
  - Interesting simulations:
    - Particle wrapping
    - High particle count
  - Optimization:
    - World partitioning
    - Partially multithreaded (thanks to `rayon` crate)
    - Instanced rendering
  - Customizability:
    - Runtime customizable settings 
    - Setting presets save/load system
  - Possibly more to come later!

# Performance
10000 particles simulated at about 2ms / frame on Intel i5-12600K (16 threads)

[<img src="https://user-images.githubusercontent.com/72656547/213253052-80e923ca-bf12-468b-8061-5dea9737a4b3.png" width="600"/>]()


# How to run?
Simply use the `run` command in the crate's directory. **Don't forget about the `--release` flag**, otherwise the performance will be much worse.

```
cargo run --release
```

# Showcase:

[<img src="https://user-images.githubusercontent.com/72656547/213254209-cc0475d1-5bf1-4230-a654-f06de8c133e1.png" width="500"/>]()
[<img src="https://user-images.githubusercontent.com/72656547/213255494-0d7bf096-4957-498e-a708-79bdc55c634c.png" width="500"/>]()
[<img src="https://user-images.githubusercontent.com/72656547/213255544-f41af102-2dae-45f6-9917-bf5c66fa9874.png" width="500"/>]()
[<img src="https://user-images.githubusercontent.com/72656547/213255851-64a34450-57d8-4f18-90f6-aefc286f267b.png" width="500"/>]()
[<img src="https://user-images.githubusercontent.com/72656547/213255868-f32b5fe2-b13f-482c-b3d4-21440fd1d72c.png" width="500"/>]()
[<img src="https://user-images.githubusercontent.com/72656547/213255910-2cdb0e0b-459a-4e00-8dbb-1ce0ffa39d72.png" width="500"/>]()
[<img src="https://user-images.githubusercontent.com/72656547/213255935-7759f5db-912b-4473-b696-bedaa0e20d3c.png" width="500"/>]()
[<img src="https://user-images.githubusercontent.com/72656547/213255944-f92c7016-09ae-4e21-898c-33c2c56e6cc8.png" width="500"/>]()
[<img src="https://user-images.githubusercontent.com/72656547/213255683-b0c9df51-b4c0-4d02-8d17-f349d922e7f8.png" width="500"/>]()
[<img src="https://user-images.githubusercontent.com/72656547/213255731-e6ca76fd-4570-4879-b045-0956ce85dd72.png" width="500"/>]()
[<img src="https://user-images.githubusercontent.com/72656547/213255958-84344b60-ef9a-4a52-8506-968f8c402f5d.png" width="500"/>]()
[<img src="https://user-images.githubusercontent.com/72656547/213255992-a25b5f62-a6d5-4cdb-bf70-34f328638d1e.png" width="500"/>]()


