# UPX Patcher

[简体中文](README.zh.md) [English](README.md)

UPX Patcher is a command-line tool for modifying UPX-compressed executable (.exe) files. It replaces standard internal information within UPX files, such as section names and DOS stub information, with predefined, more customized content.

## Main Features

- Modify UPX section names: Changes internal section names within UPX-compressed files, such as UPX0, UPX1, and UPX2, to make them appear more standard or less obscure.
- Modify DOS stub information: Replaces the standard message "This program cannot be run in DOS mode."

## Quick Start

### Prerequisites

This program requires **Administrator privileges** to modify executable files on your system.

### Instructions

Place `UPX-Patcher.exe` in the same directory as the `.exe` file you want to modify and run it from the command line.

```bash
UPX-Patcher.exe <file_path>
```

### Example:

```bash
UPX-Patcher.exe myapp.exe
```

## License

This program is licensed under the MIT License.
