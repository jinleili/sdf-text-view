use crate::prelude::*;

#[cfg(any(feature = "dx12", feature = "metal", feature = "vulkan"))]
pub fn run(
    event_loop: &mut EventsLoop,
    factory: &mut Factory<Backend>,
    mut graph: Graph<Backend, ()>,
) -> Result<(), failure::Error> {
    let started = std::time::Instant::now();

    std::thread::spawn(move || {
        while started.elapsed() < std::time::Duration::new(30, 0) {
            std::thread::sleep(std::time::Duration::new(1, 0));
        }

        std::process::abort();
    });

    let mut frames = 0u64..;
    let mut elapsed = started.elapsed();

    for _ in &mut frames {
        event_loop.poll_events(|_| ());
        graph.run(factory, &mut ());

        elapsed = started.elapsed();
        if elapsed >= std::time::Duration::new(15, 0) {
            break;
        }
    }

    let elapsed_ms = elapsed.as_secs() * 1_000 + elapsed.subsec_millis() as u64;

    log::info!(
        "Elapsed: {:?}. Frames: {}. FPS: {}",
        elapsed,
        frames.start,
        frames.start * 1_000 / elapsed_ms
    );

    graph.dispose(factory, &mut ());
    Ok(())
}
