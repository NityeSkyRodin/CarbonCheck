mod cli;
mod main_analyzer;
mod error;
mod print_ln;

mod analyzer;

use clap::Parser;
use cli::{Cli, Commands};
use main_analyzer::LaravelAnalyzer;
use error::handle_error;


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { path } => {
            println!("Analyzing Laravel project at: {}", path);
            if !std::path::Path::new(&path).is_dir() {
                handle_error(format!("404 NOT FOUND: The specified path does not exist or is not a directory: {}", path));
                std::process::exit(1);
            }

            let analyzer = LaravelAnalyzer::new(path);
            analyzer.run_analysis();
        }
        Commands::Report { output } => {
            match output {
                Some(file) => println!("Generating report to: {}", file),
                None => println!("🖥Printing report to standard output..."),
            }
            LaravelAnalyzer::generate_dummy_report();
        }
    }
}
