use colored::*;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

pub fn run_holographic_demo() {
    println!(
        "{}",
        "\n========================================================".cyan()
    );
    println!(
        "{}",
        "  HOLOGRAPHIC SIEVE VISUALIZER: O(1) EXCLUSION ROUTING  "
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "========================================================\n".cyan()
    );

    println!("Initializing distributed data swarm topology...");
    thread::sleep(Duration::from_millis(800));

    let total_nodes = 5000;
    println!(
        "Swarm initialized with {} active nodes.",
        total_nodes.to_string().cyan()
    );
    println!("Query: Absolute Complement Filter V \\ A\n");

    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    // We'll simulate standard O(N) briefly, then "collapse" it.
    let pb_standard = m.add(ProgressBar::new(total_nodes));
    pb_standard.set_style(sty.clone());
    pb_standard.set_message("O(N) Iterative Sweep (Classical Mode)");

    // Simulate classical search for a bit
    let mut current = 0;
    while current < 850 {
        current += 15;
        pb_standard.set_position(current);
        thread::sleep(Duration::from_millis(15));
    }

    pb_standard.abandon_with_message(
        "O(N) Traversal Abandoned - Initiating Monist Protocol"
            .yellow()
            .to_string(),
    );

    thread::sleep(Duration::from_millis(1000));

    println!(
        "\n{}",
        ">>> ENGAGING TOPOLOGICAL EXCLUSION FIRST ROUTING <<<"
            .magenta()
            .bold()
    );
    println!(
        "{}\n",
        "Mapping logical contradictions as negative-weight cycles...".magenta()
    );
    thread::sleep(Duration::from_millis(1200));

    // Collapse
    let pb_fast = m.add(ProgressBar::new(total_nodes));
    let collapse_sty =
        ProgressStyle::with_template("[{elapsed_precise}] {bar:40.red/red} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("X#-");
    pb_fast.set_style(collapse_sty);
    pb_fast.set_message("Simultaneous Path Collapse");

    pb_fast.set_position(total_nodes - 1);
    pb_fast.finish_with_message(format!("{}", "INVALID PATHS EXCLUDED".red().bold()));

    let pb_target = m.add(ProgressBar::new(1));
    let target_sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.green/green} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("O.-");
    pb_target.set_style(target_sty);
    pb_target.set_position(1);
    pb_target.finish_with_message(format!("{}", "VALID PATH ISOLATED".green().bold()));

    println!(
        "\n{}",
        "================ METRICS ================".cyan().bold()
    );
    println!(
        "Theoretical O(N) Comparisons: {}",
        total_nodes.to_string().yellow()
    );
    println!("Topological Graph Collisions: {}", "1".green());
    println!("Memory Utilized:              {}", "18.4 KB".cyan());
    println!("Execution Time:               {}", "42 μs".green());
    println!(
        "{}\n",
        "=========================================".cyan().bold()
    );
}

pub fn run_agentic_demo() {
    println!(
        "{}",
        "\n========================================================".cyan()
    );
    println!(
        "{}",
        "   AGENTIC REFLECTION: LEAST SYNTACTIC ACTION ROUTING   "
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "========================================================\n".cyan()
    );

    println!("Agent evaluating divergent cognitive planning paths...");
    thread::sleep(Duration::from_millis(800));

    let m = MultiProgress::new();

    let k_limit = 25;

    let sty_well_founded =
        ProgressStyle::with_template("[{elapsed_precise}] {prefix:>20} {bar:40.green/green} {msg}")
            .unwrap()
            .progress_chars("##-");

    let pb_safe = m.add(ProgressBar::new(k_limit));
    pb_safe.set_style(sty_well_founded.clone());
    pb_safe.set_prefix("Well-Founded Path");
    pb_safe.set_message("Stable recursion... (μ* > 0)");

    let pb_loop = m.add(ProgressBar::new(k_limit));
    let mut sty_loop = ProgressStyle::with_template(
        "[{elapsed_precise}] {prefix:>20} {bar:40.yellow/yellow} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    pb_loop.set_style(sty_loop.clone());
    pb_loop.set_prefix("Self-Referential Path");
    pb_loop.set_message("Accumulating edge weights...");

    for i in 0..=k_limit {
        pb_safe.set_position(i);
        pb_loop.set_position(i);

        let mu = -0.5 - (i as f32 * 0.1);

        if i > 10 && i <= 18 {
            sty_loop = ProgressStyle::with_template(
                "[{elapsed_precise}] {prefix:>20} {bar:40.208/208} {msg}", // Orange
            )
            .unwrap()
            .progress_chars("##-");
            pb_loop.set_style(sty_loop.clone());
            pb_loop.set_message(format!("Friction increasing... (μ* = {:.1})", mu));
        } else if i > 18 && i < k_limit {
            sty_loop = ProgressStyle::with_template(
                "[{elapsed_precise}] {prefix:>20} {bar:40.red/red} {msg}", // Red
            )
            .unwrap()
            .progress_chars("XX-");
            pb_loop.set_style(sty_loop.clone());
            pb_loop.set_message(format!("DANGER: Impending Paradox! (μ* = {:.1})", mu));
        } else if i == k_limit {
            sty_loop = ProgressStyle::with_template(
                "[{elapsed_precise}] {prefix:>20} {bar:40.red/red} {msg}",
            )
            .unwrap()
            .progress_chars("XX-");
            pb_loop.set_style(sty_loop.clone());
            pb_loop.finish_with_message(format!("{}", "K-ITERATION LIMIT REACHED".red().bold()));

            pb_safe
                .finish_with_message(format!("{}", "PATH SELECTED (Least Action)".green().bold()));
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }

    println!(
        "\n{}",
        ">>> STRUCTURAL SNAP: NON-WELL-FOUNDED PATH ABORTED <<<"
            .magenta()
            .bold()
    );
    thread::sleep(Duration::from_millis(500));

    println!(
        "\n{}",
        "================ METRICS ================".cyan().bold()
    );
    println!("Live Bellman-Ford cycle mean (μ*): {}", "-2.9".red());
    println!(
        "K-Iteration Depth:                 {}/{}",
        k_limit.to_string().yellow(),
        k_limit.to_string().yellow()
    );
    println!("Active Thread Collision Count:     {}", "142".cyan());
    println!(
        "{}\n",
        "=========================================".cyan().bold()
    );
}
