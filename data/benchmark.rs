use clap::Parser;
use poke_engine::mcts::perform_mcts;
use poke_engine::mcts_threaded::perform_mcts_shared_tree;
use poke_engine::state::State;
use std::process::exit;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    file_name: String,

    #[clap(short = 'i', long, default_value_t = 250000)]
    iterations: u32,

    #[clap(short = 'n', long, default_value_t = 1)]
    threads: usize,
}

fn main() {
    let args = Args::parse();
    if args.file_name.is_empty() {
        eprintln!("File name is required");
        exit(1);
    }

    let file_path = {
        let this_file = std::path::Path::new(file!());
        let this_dir = this_file.parent().unwrap();
        this_dir.join(&args.file_name)
    };
    let contents = std::fs::read_to_string(file_path).expect("Failed to read the file");
    let lines = contents.split("\n").collect::<Vec<&str>>();

    // The engine is generic over the generation; MCTS behaviour here is gen-agnostic, so
    // this benchmark runs as the newest supported generation. The gen dispatch layer maps
    // this to the correct engine (genx or a compile-time gen1/2/3 build).
    const GEN: u8 = 9;

    let mut states = Vec::with_capacity(lines.len());
    for line in lines {
        states.push(State::deserialize::<GEN>(&line))
    }

    let start_time = std::time::Instant::now();
    for (i, state) in states.iter_mut().enumerate() {
        let (side_one_options, side_two_options) =
            poke_engine::gen_dispatch::dispatch::root_get_all_options::<GEN>(state);

        if args.threads > 1 {
            perform_mcts_shared_tree::<GEN>(
                state,
                side_one_options,
                side_two_options,
                std::time::Duration::from_millis(0),
                args.iterations,
                args.threads,
            );
        } else {
            perform_mcts::<GEN>(
                state,
                side_one_options,
                side_two_options,
                std::time::Duration::from_millis(0),
                args.iterations,
            );
        }
        println!("{}", i);
    }

    let elapsed_time = start_time.elapsed().as_secs_f64();
    println!("Took: {} seconds", elapsed_time);
    exit(0);
}
