use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "UPX-Patcher",
    about = "A command-line tool for modifying UPX-compressed executable files",
    long_about = "UPX Patcher modifies standard internal information within UPX files, \
                  such as section names and DOS stub information, with predefined content."
)]
pub struct Cli {
    /// Path to the .exe file to patch
    #[clap(value_name = "FILE_PATH")]
    pub file_path: String,

    /// Backup the original file with .bak extension
    #[clap(short = 'b', long = "backup")]
    pub backup: bool,

    /// Skip section name patching (UPX0/UPX1/UPX2)
    #[clap(long = "no-section")]
    pub no_section: bool,

    /// Skip version block confusing
    #[clap(long = "no-version")]
    pub no_version: bool,

    /// Skip DOS stub message replacement
    #[clap(long = "no-dos")]
    pub no_dos: bool,

    /// Skip entry point patching
    #[clap(long = "no-entry")]
    pub no_entry: bool,
}
