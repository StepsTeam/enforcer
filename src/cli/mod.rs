// Each CLI processing step (train in → train out)
pub mod configure_cli;
pub mod set_file_path;
pub mod set_app_logs_dir;
pub mod set_data_files_dir;

// The CLI orchestrator function
pub mod track_cli;

// Re-export so main.rs can just call cli::track_cli()
pub use track_cli::track_cli;
