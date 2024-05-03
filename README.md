# rust_profiling
Profiling Rust Application Using Pyroscope

# Stack
- RUST
- Profiling
- Pyroscope

# How to run

### Spin up Pyroscope
```bash
docker run -d --name pyroscope -p 4040:4040 grafana/pyroscope:latest
```

### Run app
```bash
cargo run
```

### Profile 
- Go to http://localhost:4040

### Documentation
- http://grafana.com/docs/pyroscope/latest/view-and-analyze-profile-data/profiling-types/

# Author
Semicolon

### Youtube

www.youtube.com/@Semicolon10

