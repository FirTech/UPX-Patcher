# UPX Patcher

[简体中文](README.zh.md) | [English](README.md)

UPX Patcher 是一个用于修改 UPX 压缩的可执行文件（`.exe`）的命令行工具。它将 UPX 文件中标准的内部信息（如段名和 DOS
存根信息）替换为预定义的、更具个性化的内容。

## 主要功能

- **修改 UPX 段名**：将 UPX 压缩文件中的内部段名（UPX0、UPX1、UPX2）更改为标准段名（.rsrc、.reloc、.debug）
- **修改 DOS 存根信息**：替换标准消息 "This program cannot be run in DOS mode."
- **版本块混淆**：随机化 UPX 版本块数据
- **入口点修补**：修补 32 位和 64 位可执行文件的入口点字节
- **备份支持**：修补前可选择创建 .bak 备份
- **选择性修补**：选择要执行的修补步骤

## 快速开始

### 前提条件

此程序需要**管理员权限**才能修改系统中的可执行文件。

### 安装

从 [Releases](../../releases) 页面下载最新版本或从源代码构建：

```bash
cargo build --release
```

编译后的可执行文件将位于 `target/release/UPX-Patcher.exe`。

### 使用方法

```bash
UPX-Patcher.exe [选项] <文件路径>
```

### 参数

| 参数            | 说明             |
|---------------|----------------|
| `<FILE_PATH>` | 要修补的 .exe 文件路径 |

### 选项

| 选项             | 简写   | 说明                      |
|----------------|------|-------------------------|
| `--backup`     | `-b` | 使用 .bak 扩展名备份原文件        |
| `--no-section` |      | 跳过节名称修补（UPX0/UPX1/UPX2） |
| `--no-version` |      | 跳过版本块混淆                 |
| `--no-dos`     |      | 跳过 DOS 存根消息替换           |
| `--no-entry`   |      | 跳过入口点修补                 |
| `--help`       | `-h` | 显示帮助信息                  |
| `--version`    | `-V` | 显示版本信息                  |

### 使用示例

**基本使用 - 执行所有修补：**

```bash
UPX-Patcher.exe myapp.exe
```

**带备份：**

```bash
UPX-Patcher.exe -b myapp.exe
# 或者
UPX-Patcher.exe --backup myapp.exe
```

**跳过特定修补步骤：**

```bash
# 跳过版本块和入口点修补
UPX-Patcher.exe --no-version --no-entry myapp.exe

# 仅修补节名称
UPX-Patcher.exe --no-version --no-dos --no-entry myapp.exe
```

**完整示例（带备份和选择性修补）：**

```bash
UPX-Patcher.exe -b --no-version myapp.exe
```

## 输出示例

```
14:32:15 -> Backup created: myapp.exe.bak
14:32:15 -> Sections confusing...
14:32:15 -> Version block confusing...
14:32:15 -> Replacing standard DOS Stub message...
14:32:15 -> EntryPoint patching...
14:32:15 -> Successfully patched!
```

## 错误处理

程序将在以下情况下退出并显示错误消息：

- 指定的文件不存在
- 文件不是有效的 Windows 可执行文件（缺少 MZ 头）
- 文件未使用 UPX 压缩
- 文件已被修补
- 备份创建失败（使用 `--backup` 时）

## 许可证

本程序采用 [MIT 许可证](LICENSE)。

## 贡献

欢迎贡献！请随时提交 Pull Request。
