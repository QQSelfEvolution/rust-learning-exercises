use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};
use std::time::Instant;

use md5::{Digest, Md5};
use sha2::{Sha256, Sha512};
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "hash_tool")]
#[command(about = "File hash checker for integrity verification", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compute hash of file(s)
    Hash {
        /// Single file to hash
        #[arg(short, long)]
        file: Option<String>,
        /// Multiple files to hash
        #[arg(short = 'F', long)]
        files: Option<Vec<String>>,
        /// Directory to scan
        #[arg(short, long)]
        directory: Option<String>,
        /// Hash algorithm
        #[arg(short, long, value_enum, default_value_t = Algorithm::Sha256)]
        algo: Algorithm,
        /// Recursively scan directories
        #[arg(short, long, default_value_t = false)]
        recursive: bool,
        /// Output format
        #[arg(short, long, default_value = "text")]
        output: OutputFormat,
    },
    /// Verify file against known hash
    Verify {
        /// File to verify
        #[arg(short, long)]
        file: String,
        /// Known hash value
        #[arg(short, long)]
        hash: String,
        /// Hash algorithm
        #[arg(short, long, value_enum, default_value_t = Algorithm::Sha256)]
        algo: Algorithm,
    },
    /// Compare two files by hash
    Compare {
        /// First file
        #[arg(short = '1', long)]
        file1: String,
        /// Second file
        #[arg(short = '2', long)]
        file2: String,
        /// Hash algorithm
        #[arg(short, long, value_enum, default_value_t = Algorithm::Sha256)]
        algo: Algorithm,
    },
    /// Batch verify files using a manifest
    Batch {
        /// Manifest file with known hashes
        #[arg(short, long)]
        manifest: String,
        /// Directory containing files
        #[arg(short, long)]
        directory: Option<String>,
    },
}

#[derive(Copy, Clone, ValueEnum, Debug)]
enum Algorithm {
    Md5,
    Sha256,
    Sha512,
}

impl Algorithm {
    fn to_string(&self) -> &'static str {
        match self {
            Algorithm::Md5 => "MD5",
            Algorithm::Sha256 => "SHA256",
            Algorithm::Sha512 => "SHA512",
        }
    }
}

#[derive(Copy, Clone, ValueEnum, Debug)]
enum OutputFormat {
    Text,
    Json,
    Csv,
}

struct HashResult {
    file: String,
    hash: String,
    algorithm: String,
    size: u64,
    time_ms: u64,
}

fn compute_hash<P: AsRef<Path>>(path: P, algo: Algorithm) -> io::Result<(String, u64, u64)> {
    let start = Instant::now();
    let file = File::open(path.as_ref())?;
    let file_size = file.metadata()?.len();
    let mut reader = BufReader::with_capacity(1024 * 1024, file); // 1MB buffer
    
    let hash = match algo {
        Algorithm::Md5 => {
            let mut hasher = Md5::new();
            let mut buffer = [0u8; 8192];
            loop {
                let bytes_read = reader.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }
                hasher.update(&buffer[..bytes_read]);
            }
            format!("{:x}", hasher.finalize())
        }
        Algorithm::Sha256 => {
            let mut hasher = Sha256::new();
            let mut buffer = [0u8; 8192];
            loop {
                let bytes_read = reader.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }
                hasher.update(&buffer[..bytes_read]);
            }
            format!("{:x}", hasher.finalize())
        }
        Algorithm::Sha512 => {
            let mut hasher = Sha512::new();
            let mut buffer = [0u8; 8192];
            loop {
                let bytes_read = reader.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }
                hasher.update(&buffer[..bytes_read]);
            }
            format!("{:x}", hasher.finalize())
        }
    };
    
    let elapsed = start.elapsed().as_millis() as u64;
    Ok((hash, file_size, elapsed))
}

fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    
    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

fn process_file(path: &str, algo: Algorithm) -> Result<HashResult, String> {
    let path = PathBuf::from(path);
    
    if !path.exists() {
        return Err(format!("File not found: {}", path.display()));
    }
    
    match compute_hash(&path, algo) {
        Ok((hash, size, time)) => Ok(HashResult {
            file: path.to_string_lossy().to_string(),
            hash,
            algorithm: algo.to_string().to_string(),
            size,
            time_ms: time,
        }),
        Err(e) => Err(format!("Error processing {}: {}", path.display(), e)),
    }
}

fn hash_command(files: Vec<String>, algo: Algorithm, output: OutputFormat) {
    let mut results = Vec::new();
    let mut errors = Vec::new();
    
    for file in files {
        match process_file(&file, algo) {
            Ok(result) => results.push(result),
            Err(e) => errors.push(e),
        }
    }
    
    match output {
        OutputFormat::Text => {
            for result in &results {
                println!("{}  {}  ({} in {}ms)", 
                    result.hash, result.file, format_size(result.size), result.time_ms);
            }
        }
        OutputFormat::Json => {
            println!("{{");
            println!("  \"algorithm\": \"{}\",", algo.to_string());
            println!("  \"files\": [");
            for (i, result) in results.iter().enumerate() {
                println!("    {{");
                println!("      \"file\": \"{}\",", result.file);
                println!("      \"hash\": \"{}\",", result.hash);
                println!("      \"size\": {},", result.size);
                println!("      \"time_ms\": {}", result.time_ms);
                print!("    }}");
                if i < results.len() - 1 {
                    println!(",");
                } else {
                    println!();
                }
            }
            println!("  ]");
            println!("}}");
        }
        OutputFormat::Csv => {
            println!("file,hash,size,time_ms");
            for result in &results {
                println!("{},{},{},{}", result.file, result.hash, result.size, result.time_ms);
            }
        }
    }
    
    if !errors.is_empty() {
        eprintln!("\nErrors:");
        for error in errors {
            eprintln!("  {}", error);
        }
    }
}

fn verify_command(file: &str, expected_hash: &str, algo: Algorithm) {
    match process_file(file, algo) {
        Ok(result) => {
            let computed_hash = result.hash.to_lowercase();
            let expected = expected_hash.to_lowercase().trim();
            
            println!("File: {}", result.file);
            println!("Algorithm: {}", algo.to_string());
            println!("Computed: {}", computed_hash);
            println!("Expected: {}", expected);
            println!();
            
            if computed_hash == expected {
                println!("✓ VERIFIED: File integrity check passed!");
            } else {
                println!("✗ FAILED: File integrity check failed!");
                println!("  WARNING: File may have been tampered with!");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn compare_command(file1: &str, file2: &str, algo: Algorithm) {
    println!("Comparing files using {}...", algo.to_string());
    println!();
    
    let result1 = process_file(file1, algo);
    let result2 = process_file(file2, algo);
    
    match (result1, result2) {
        (Ok(r1), Ok(r2)) => {
            println!("File 1: {}", r1.file);
            println!("  Hash: {}", r1.hash);
            println!("  Size: {}", format_size(r1.size));
            println!();
            println!("File 2: {}", r2.file);
            println!("  Hash: {}", r2.hash);
            println!("  Size: {}", format_size(r2.size));
            println!();
            
            if r1.hash == r2.hash {
                println!("✓ FILES ARE IDENTICAL (same hash)");
            } else {
                println!("✗ FILES ARE DIFFERENT (different hashes)");
            }
        }
        (Err(e), _) => {
            eprintln!("Error with file 1: {}", e);
            std::process::exit(1);
        }
        (_, Err(e)) => {
            eprintln!("Error with file 2: {}", e);
            std::process::exit(1);
        }
    }
}

fn batch_command(manifest: &str, directory: Option<String>) {
    let manifest_content = fs::read_to_string(manifest)
        .map_err(|e| format!("Cannot read manifest: {}", e))
        .unwrap();
    
    let mut known_hashes: Vec<(String, String)> = Vec::new();
    
    for line in manifest_content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        // Parse "hash  filename" format
        let parts: Vec<&str> = line.splitn(2, char::is_whitespace).collect();
        if parts.len() == 2 {
            known_hashes.push((parts[0].to_string(), parts[1].trim().to_string()));
        }
    }
    
    let base_dir = directory.unwrap_or_else(|| ".".to_string());
    
    println!("Verifying {} files from manifest...", known_hashes.len());
    println!();
    
    let mut passed = 0;
    let mut failed = 0;
    let mut missing = 0;
    
    for (expected_hash, filename) in known_hashes {
        let file_path = PathBuf::from(&base_dir).join(&filename);
        
        match process_file(file_path.to_str().unwrap_or(&filename), Algorithm::Sha256) {
            Ok(result) => {
                let computed = result.hash.to_lowercase();
                let expected = expected_hash.to_lowercase().trim();
                
                if computed == expected {
                    println!("✓ {} - VERIFIED", filename);
                    passed += 1;
                } else {
                    println!("✗ {} - FAILED (hash mismatch)", filename);
                    println!("  Expected: {}", expected);
                    println!("  Got:      {}", computed);
                    failed += 1;
                }
            }
            Err(_) => {
                println!("✗ {} - FILE NOT FOUND", filename);
                missing += 1;
            }
        }
    }
    
    println!();
    println!("=== Summary ===");
    println!("Passed:  {}", passed);
    println!("Failed:  {}", failed);
    println!("Missing: {}", missing);
    
    if failed > 0 || missing > 0 {
        std::process::exit(1);
    }
}

fn collect_files_from_dir(dir: &str, recursive: bool) -> Vec<String> {
    let mut files = Vec::new();
    let path = PathBuf::from(dir);
    
    if recursive {
        for entry in walkdir::WalkDir::new(&path) {
            if let Ok(entry) = entry {
                if entry.file_type().is_file() {
                    files.push(entry.path().to_string_lossy().to_string());
                }
            }
        }
    } else {
        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries.flatten() {
                if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                    files.push(entry.path().to_string_lossy().to_string());
                }
            }
        }
    }
    
    files
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Hash { file, files, directory, algo, recursive, output } => {
            let mut file_list = Vec::new();
            
            if let Some(f) = file {
                file_list.push(f);
            }
            
            if let Some(fs) = files {
                file_list.extend(fs);
            }
            
            if let Some(dir) = directory {
                let dir_files = collect_files_from_dir(&dir, recursive);
                file_list.extend(dir_files);
            }
            
            if file_list.is_empty() {
                eprintln!("Error: No files specified. Use --file, --files, or --directory");
                std::process::exit(1);
            }
            
            hash_command(file_list, algo, output);
        }
        
        Commands::Verify { file, hash, algo } => {
            verify_command(&file, &hash, algo);
        }
        
        Commands::Compare { file1, file2, algo } => {
            compare_command(&file1, &file2, algo);
        }
        
        Commands::Batch { manifest, directory } => {
            batch_command(&manifest, directory);
        }
    }
}
