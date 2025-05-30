
use std::path::{Path};
use walkdir::WalkDir;

pub struct PackageAnalysis {
    pub file_count: usize,
    pub loc: usize, 
}


/// Analyzes the code of a Laravel package located at the specified path.
/// 
/// # Arguments 
/// 
/// * `package_path`:  A reference to a `Path` object representing the package directory.
/// 
/// returns: Result<PackageAnalysis, String> 
/// 
/// # Examples 
/// 
/// ```
/// 
/// ```
pub fn analyze_package_code(package_path: &Path) -> Result<PackageAnalysis, String> {
    let mut file_count = 0;
    let mut loc = 0;

    for entry in WalkDir::new(package_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "php") {
            file_count += 1;
            if let Ok(content) = std::fs::read_to_string(path) {
                loc += content.lines().count();
            }
        }
    }

    Ok(PackageAnalysis {
        file_count,
        loc,
    })
}