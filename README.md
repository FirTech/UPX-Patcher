# UPX Patcher

[简体中文](README.zh.md) | [English](README.md)

UPX Patcher is a command-line tool for modifying UPX-compressed executable (.exe) files. It replaces standard internal
information within UPX files, such as section names and DOS stub information, with predefined, more customized content.

## Main Features

- **Modify UPX section names**: Changes internal section names within UPX-compressed files (UPX0, UPX1, UPX2) to
  standard section names (.rsrc, .reloc, .debug)
- **Modify DOS stub information**: Replaces the standard message "This program cannot be run in DOS mode."
- **Version block confusing**: Randomizes UPX version block data
- **Entry point patching**: Patches entry point bytes for both 32-bit and 64-bit executables
- **Backup support**: Optionally create .bak backup before patching
- **Selective patching**: Choose which patching steps to perform

## Quick Start

### Prerequisites

This program requires **Administrator privileges** to modify executable files on your system.

### Installation

Download the latest release from the [Releases](../../releases) page or build from source:

```bash
cargo build --release
```

The compiled executable will be located at `target/release/UPX-Patcher.exe`.

### Usage

```bash
UPX-Patcher.exe [OPTIONS] <FILE_PATH>
```

### Arguments

| Argument      | Description                    |
|---------------|--------------------------------|
| `<FILE_PATH>` | Path to the .exe file to patch |

### Options

| Option         | Short | Description                                  |
|----------------|-------|----------------------------------------------|
| `--backup`     | `-b`  | Backup the original file with .bak extension |
| `--no-section` |       | Skip section name patching (UPX0/UPX1/UPX2)  |
| `--no-version` |       | Skip version block confusing                 |
| `--no-dos`     |       | Skip DOS stub message replacement            |
| `--no-entry`   |       | Skip entry point patching                    |
| `--help`       | `-h`  | Print help information                       |
| `--version`    | `-V`  | Print version information                    |

### Examples

**Basic usage - patch all:**

```bash
UPX-Patcher.exe myapp.exe
```

**With backup:**

```bash
UPX-Patcher.exe -b myapp.exe
# or
UPX-Patcher.exe --backup myapp.exe
```

**Skip specific patching steps:**

```bash
# Skip version block and entry point patching
UPX-Patcher.exe --no-version --no-entry myapp.exe

# Only patch section names
UPX-Patcher.exe --no-version --no-dos --no-entry myapp.exe
```

**Full example with backup and selective patching:**

```bash
UPX-Patcher.exe -b --no-version myapp.exe
```

## Output Example

```
14:32:15 -> Backup created: myapp.exe.bak
14:32:15 -> Sections confusing...
14:32:15 -> Version block confusing...
14:32:15 -> Replacing standard DOS Stub message...
14:32:15 -> EntryPoint patching...
14:32:15 -> Successfully patched!
```

## Error Handling

The program will exit with an error message if:

- The specified file does not exist
- The file is not a valid Windows executable (MZ header missing)
- The file is not packed with UPX
- The file has already been patched
- Backup creation fails (when `--backup` is used)

## License

This program is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
