/*
Use of a genetic algorithm to optimize the engine parameters
*/

// ----- LIBRARIES ------
use byteslayer_core::engine::choose_move;
use byteslayer_core::evaluation::{get_default_genome, Genome};
use byteslayer_core::transposition::TranspositionTable;
use chess::{Board, BoardStatus};
use std::fs::write;

// ----- GENETIC ALGORITHM -----
// Structure for an individual
struct Individual {
    genome: Genome,
    fitness: f64,
}

// Structure for the population
struct Population {
    population: Vec<Individual>,
}

// Initialization of the Population
fn init_population(size: usize) -> Population {
    let mut pop = Vec::new();

    let mut rng = SimpleRng::new(42);

    let default_genome = get_default_genome();

    let patient_zero = Individual {
        genome: default_genome.clone(),
        fitness: 0.0,
    };

    pop.push(patient_zero);

    for _ in 1..size {
        let mut mutant_genome = default_genome.clone();

        mutate_genome(&mut mutant_genome, &mut rng);

        pop.push(Individual {
            genome: mutant_genome,
            fitness: 0.0,
        });
    }

    Population { population: pop }
}

// Mutation function
fn mutate_genome(genome: &mut Genome, rng: &mut SimpleRng) {
    for j in 0..12 {
        for i in 0..64 {
            if rng.gen_range(1, 100) <= 15 {
                let variation = rng.gen_range(-5, 5);

                match j {
                    0 => genome.p_pst_mg[i] += variation as i32,
                    1 => genome.p_pst_eg[i] += variation as i32,
                    2 => genome.n_pst_mg[i] += variation as i32,
                    3 => genome.n_pst_eg[i] += variation as i32,
                    4 => genome.b_pst_mg[i] += variation as i32,
                    5 => genome.b_pst_eg[i] += variation as i32,
                    6 => genome.r_pst_mg[i] += variation as i32,
                    7 => genome.r_pst_eg[i] += variation as i32,
                    8 => genome.q_pst_mg[i] += variation as i32,
                    9 => genome.q_pst_eg[i] += variation as i32,
                    10 => genome.k_pst_mg[i] += variation as i32,
                    11 => genome.k_pst_eg[i] += variation as i32,
                    _ => {}
                }
            }
        }
    }
}

// XOR shift random number generation structure
struct SimpleRng {
    state: u64,
}

// XOR shift random number generation implementation
impl SimpleRng {
    // New function
    fn new(seed: u64) -> Self {
        SimpleRng { state: seed }
    }

    // XOR shift random number generation function
    fn gen_range(&mut self, min: i64, max: i64) -> i64 {
        if max <= min {
            return min;
        }

        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;

        let interval = (max - min + 1) as u64;

        (self.state % interval) as i64 + min
    }
}

// Function to simulate a game between two genomes
fn simulate_game(genome_a: &Genome, genome_b: &Genome) -> f64 {
    let mut board = Board::default();

    let mut tt = TranspositionTable::new(1 << 20);

    let mut max_moves = 300;

    while board.status() == BoardStatus::Ongoing && max_moves > 0 {
        let current_genome = if board.side_to_move() == chess::Color::White {
            genome_a
        } else {
            genome_b
        };

        let depth = 3;

        if let Some(best_move) = choose_move(depth, &board, None, &mut tt, current_genome) {
            board = board.make_move_new(best_move);
        }

        max_moves -= 1;
    }

    match board.status() {
        BoardStatus::Checkmate => {
            if board.side_to_move() == chess::Color::Black {
                1.0
            } else {
                0.0
            }
        }
        _ => 0.5,
    }
}

// Population evaluation function
fn evaluate_population(pop: &mut Population) {
    let default_genome = get_default_genome();

    for individual in pop.population.iter_mut().skip(1) {
        let mut total_fitness = 0.0;

        let white_score = simulate_game(&individual.genome, &default_genome);
        total_fitness += white_score;

        let black_score = simulate_game(&default_genome, &individual.genome);
        total_fitness += 1.0 - black_score;

        individual.fitness = total_fitness;
    }

    if !pop.population.is_empty() {
        pop.population[0].fitness = 1.0;
    }
}

// Utility function to transform a 64 element array into a 8x8 block
fn format_pst(name: &str, table: &[i32; 64]) -> String {
    let mut out = format!("    {}: [\n", name);
    for row in 0..8 {
        out.push_str("        ");
        for col in 0..8 {
            let idx = row * 8 + col;
            out.push_str(&format!("{:>5},", table[idx]));
        }
        out.push_str("\n");
    }
    out.push_str("    ],\n");
    out
}

// Main function
fn main() -> std::io::Result<()> {
    let population_size = 60;
    let generations = 150;

    println!("Launching genetic optimization...");
    let mut pop = init_population(population_size);

    for generation in 1..=generations {
        println!("\n--- Generation nº{}/{} ---", generation, generations);

        evaluate_population(&mut pop);

        pop.population
            .sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        let champion = &pop.population[0];
        println!("Best Fitness : {}/2.0", champion.fitness);

        let mut next_generation = Vec::new();

        for i in 0..5 {
            next_generation.push(Individual {
                genome: pop.population[i].genome.clone(),
                fitness: 0.0,
            });
        }

        let mut rng = SimpleRng::new(42 + generation as u64);

        while next_generation.len() < population_size {
            let index_parent = rng.gen_range(0, 4) as usize;
            let mut child_genome = pop.population[index_parent].genome.clone();

            mutate_genome(&mut child_genome, &mut rng);

            next_generation.push(Individual {
                genome: child_genome,
                fitness: 0.0,
            });
        }
        pop.population = next_generation;
    }
    println!("\nThe best genome was found!");

    let champion_genome = &pop.population[0].genome;

    let mut data = String::new();
    data.push_str("Genome {\n");

    data.push_str(&format_pst("p_pst_mg", &champion_genome.p_pst_mg));
    data.push_str(&format_pst("p_pst_eg", &champion_genome.p_pst_eg));
    data.push_str(&format_pst("n_pst_mg", &champion_genome.n_pst_mg));
    data.push_str(&format_pst("n_pst_eg", &champion_genome.n_pst_eg));
    data.push_str(&format_pst("b_pst_mg", &champion_genome.b_pst_mg));
    data.push_str(&format_pst("b_pst_eg", &champion_genome.b_pst_eg));
    data.push_str(&format_pst("r_pst_mg", &champion_genome.r_pst_mg));
    data.push_str(&format_pst("r_pst_eg", &champion_genome.r_pst_eg));
    data.push_str(&format_pst("q_pst_mg", &champion_genome.q_pst_mg));
    data.push_str(&format_pst("q_pst_eg", &champion_genome.q_pst_eg));
    data.push_str(&format_pst("k_pst_mg", &champion_genome.k_pst_mg));
    data.push_str(&format_pst("k_pst_eg", &champion_genome.k_pst_eg));

    data.push_str("}\n");

    write("../../../../transposition.txt", data)?;
    Ok(())
}
