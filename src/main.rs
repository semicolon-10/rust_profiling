use std::thread::Thread;
use std::time::Duration;
use pyroscope::{PyroscopeAgent, Result};
use pyroscope_pprofrs::{pprof_backend, PprofConfig};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() -> Result<()> {
    let agent = PyroscopeAgent::builder(
        "http://localhost:4040",
        "dummy_app"
    )
        .backend(pprof_backend(PprofConfig::new().sample_rate(100)))
        .build()?;

    let agent_running = agent.start()?;

    let result = fibonacci(45);

    let agent_stopped = agent_running.stop()?;

    println!("{:?}",result);

    agent_stopped.shutdown();

    Ok(())
}
