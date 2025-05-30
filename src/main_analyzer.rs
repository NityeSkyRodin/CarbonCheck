use std::path::Path;
use crate::error::handle_error;
use crate::print_ln::{print_ln_with_color};

pub struct LaravelAnalyzer {
    pub project_path: String,
}

impl LaravelAnalyzer {
    /// Creates a new instance of `LaravelAnalyzer`.
    /// 
    /// # Arguments 
    /// 
    /// * `path`: A `String` representing the path to the Laravel project.
    /// 
    /// returns: LaravelAnalyzer 
    /// 
    /// # Examples 
    /// 
    /// ```
    /// 
    /// ```
    pub fn new(path: String) -> Self {
        Self { project_path: path }
    }

    
    pub fn run_analysis(&self) {
        let root = Path::new(&self.project_path);

        if !root.join("artisan").exists()
            || !root.join("composer.json").exists()
            || !root.join("app").is_dir()
            || !root.join("routes").is_dir()
        {
            handle_error("Not a valid Laravel project: required files or folders are missing.".to_string());
            std::process::exit(1);
        }


        let composer_path = root.join("composer.json");

        match crate::analyzer::composer::read_composer_json(composer_path.to_str().unwrap()) {
            Ok(data) => {
                print_ln_with_color("35", "Project", format!("Name: {}", data.name.unwrap_or_else(|| "unknown".to_string())));
                print_ln_with_color("35", "Dependencies", "Listing runtime dependencies...".to_string());

                if let Some(deps) = data.require {
                    let mut total_lines_of_code = 0;
                    let mut total_files_in_packages = 0;
                    let mut runtime_packages_count = 0;

                    for (name, version) in &deps {
                        print_ln_with_color("36", "Dependency", format!("{}: {}", name, version));
                        runtime_packages_count += 1;

                        let package_path = root.join("vendor").join(name);

                        if package_path.exists() && package_path.is_dir() {
                            match crate::analyzer::package::analyze_package_code(&package_path) {
                                Ok(package_analysis) => {
                                    print_ln_with_color("37", "Package Analysis", format!("{}: Files: {}, LoC: {}", name, package_analysis.file_count, package_analysis.loc));
                                    total_files_in_packages += package_analysis.file_count;
                                    total_lines_of_code += package_analysis.loc;
                                },
                                Err(e) => {
                                    handle_error(format!("Failed to analyze package {}: {}", name, e));
                                }
                            }
                        } else {
                            handle_error(format!("Package {} not found in vendor directory.", name));
                        }
                    }
                    let estimated_impact = (total_lines_of_code as f32 * 0.001) + (total_files_in_packages as f32 * 0.01); // Example formula
                    print_ln_with_color("33", "Estimate", format!(
                        "Estimated CO₂ impact: ~{:.2}g per 1000 requests (based on {} runtime dependencies, {} total files, {} LoC in packages)",
                        estimated_impact,
                        runtime_packages_count,
                        total_files_in_packages,
                        total_lines_of_code
                    ));
                }
            }
            Err(err) => {
                handle_error(format!("Failed to read composer.json: {}", err));
            }
        }

        print_ln_with_color("32 ", "Success", "Analysis completed successfully.".to_string());
    }

    pub fn generate_dummy_report() {
        print_ln_with_color("34", "Info", "Generating dummy report...".to_string());
    }
}
