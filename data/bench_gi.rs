// Micro-benchmark for generate_instructions_from_move_pair on a fixed 6v6 state.
//
// Deserializes one fixed serialized state (data/bench_state.txt) and repeatedly runs
// generate_instructions_from_move_pair::<5> with a fixed move pair, reporting the average
// per-call time. Used to compare the const-generic conversion against the pre-conversion
// baseline for gen 5.
//
// Build:  cargo build --release --no-default-features --bin bench_gi
// (The pre-conversion baseline built this with `--features gen5`; post-conversion gen5 is
//  selected by the ::<5> instantiation instead.)

// This bench targets the genx const-generic engine; it is a no-op for the standalone
// gen1/2/3 engines (whose entry points are not generic).
#[cfg(not(any(feature = "gen1", feature = "gen2", feature = "gen3")))]
fn main() {
    use poke_engine::engine::generate_instructions::generate_instructions_from_move_pair;
    use poke_engine::engine::state::MoveChoice;
    use poke_engine::state::{PokemonMoveIndex, State};

    const ITERS: u32 = 2_000_000;

    let this_file = std::path::Path::new(file!());
    let this_dir = this_file.parent().unwrap();
    let state_path = this_dir.join("bench_state.txt");
    let contents = std::fs::read_to_string(&state_path).expect("read bench_state.txt");
    let line = contents.lines().next().expect("at least one state line");

    // Gen 5 is the pinned/benchmarked generation for the converted-vs-baseline comparison.
    let mut state = State::deserialize::<5>(line);
    let s1 = MoveChoice::Move(PokemonMoveIndex::M0);
    let s2 = MoveChoice::Move(PokemonMoveIndex::M0);

    // Warmup (also forces the gen 5 MOVES table to build).
    for _ in 0..10_000 {
        let _ = generate_instructions_from_move_pair::<5>(&mut state, &s1, &s2, true);
    }

    let start = std::time::Instant::now();
    let mut sink = 0usize;
    for _ in 0..ITERS {
        let instrs = generate_instructions_from_move_pair::<5>(&mut state, &s1, &s2, true);
        sink = sink.wrapping_add(instrs.len());
    }
    let elapsed = start.elapsed();
    let per_call_us = elapsed.as_secs_f64() * 1e6 / ITERS as f64;
    println!("iters={ITERS} sink={sink}");
    println!(
        "total={:.4}s per_call={:.5} us",
        elapsed.as_secs_f64(),
        per_call_us
    );
}

#[cfg(any(feature = "gen1", feature = "gen2", feature = "gen3"))]
fn main() {
    eprintln!("bench_gi targets the genx const-generic engine; build without gen1/2/3");
}
