# UPX Patcher

[简体中文](README.zh.md) [English](README.md)

UPX Patcher 是一个用于修改 UPX 压缩的可执行文件（`.exe`）的命令行工具。它将 UPX 文件中标准的内部信息（如段名和 DOS 存根信息）替换为预定义的、更具个性的内容。

## 主要功能

- 修改 UPX 段名： 更改 UPX 压缩文件中的内部段名，如 `UPX0`、`UPX1` 和 `UPX2`，以使其看起来更常规或更隐蔽。
- 修改 DOS 存根信息： 替换 `This program cannot be run in DOS mode.` 这条标准信息。

## 快速开始

### 前提条件

这个程序需要**管理员权限**才能修改系统中的可执行文件。

### 使用方法

将 `UPX-Patcher.exe` 放置在你需要修改的 `.exe` 文件所在的目录，并在命令行中运行。

```bash
UPX-Patcher.exe <file_path>
```

### 示例：

```bash
UPX-Patcher.exe myapp.exe
```

## 许可证

本程序遵循 MIT 许可证。
