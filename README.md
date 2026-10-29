# mypwd

一个单向的密码生成和查询工具：将输入文本与本地盐值拼接，计算 MD5，再把摘要映射为固定的 16 个字符。

相同文本和相同盐值会得到相同密码。查询时通过重新计算复现密码，不保存生成的密码，也不提供反向解密。

## 构建与运行

需要安装支持 Rust 2024 edition 的 Rust 工具链和 Cargo。通过标准库获取用户主目录，支持 macOS、Linux 和 Windows。

在项目目录运行：

```bash
cargo run -- "hello"
cargo run -- --help
```

这里的 `--` 用于分隔 Cargo 的参数与 mypwd 的参数。

构建发布版本：

```bash
cargo build --release
./target/release/mypwd "hello"
```

Windows 下在 PowerShell 中运行：

```powershell
cargo build --release
.\target\release\mypwd.exe "hello"
.\target\release\mypwd.exe -i "my-salt"
```

也可以安装到 Cargo 的可执行文件目录：

```bash
cargo install --path .
mypwd --help
```

运行 `mypwd` 前，需要确保 Cargo 的可执行文件目录已加入 `PATH`（macOS / Linux 默认是 `~/.cargo/bin`，Windows 默认是 `%USERPROFILE%\.cargo\bin`）。

## 参数

```text
mypwd [--init <salt>] [--force] [text]
mypwd --history-clear
```

| 参数 | 说明 |
| --- | --- |
| `text` | 要计算的文本；提供 `--init` 或执行独立的历史清理时可省略 |
| `-i, --init <salt>` | 指定初始化盐值；默认复用已有盐文件 |
| `-f, --force` | 备份旧盐后强制写入盐文件；未指定 `--init` 时生成新随机盐 |
| `-c, --history-clear` | 警告并确认后删除历史盐备份，保留当前盐；不能与其他操作参数或文本组合 |
| `-h, --help` | 显示帮助 |

`--init` / `-i` 后必须提供盐值。单独执行 `mypwd` 或 `mypwd -f` 会报参数错误。

## 使用示例

```bash
# 自动初始化或复用盐，输出计算结果
mypwd "hello world"

# 只初始化盐，不输出计算结果
mypwd -i "my-salt"

# 初始化盐并计算；已有盐文件时仍优先复用
mypwd --init "my-salt" "hello"

# 强制替换盐，只做初始化
mypwd -i "new-salt" -f

# 强制替换盐并计算
mypwd --init "new-salt" --force "hello"

# 使用新随机盐覆盖盐文件并计算
mypwd -f "hello"

# 删除历史盐备份（长参数为 --history-clear）
mypwd -c
```

文本包含空格时应使用引号。在 Bash、Zsh 或 PowerShell 中，包含 `$` 等 shell 特殊字符时，可使用单引号。例如：`mypwd 'hello$world'`。Windows CMD 不支持这种单引号用法，建议使用 PowerShell 运行上述示例。

## 盐文件

盐值以明文保存在用户主目录下的 `.mypwd/salt`，父目录不存在时会自动创建：

- macOS / Linux：通常为 `$HOME/.mypwd/salt`。
- Windows：通常为 `C:\Users\用户名\.mypwd\salt`。

跨系统复现密码时，需要使用相同的输入文本和盐值。可复制原盐文件，保留内容不变，不要添加换行。

| 文件状态 | 是否指定 `--force` | 行为 |
| --- | --- | --- |
| 已存在 | 否 | 读取并复用，忽略传入的 `--init` 值 |
| 已存在 | 是 | 先备份旧盐，再使用 `--init` 的值覆盖；未提供则生成随机盐 |
| 不存在 | 任意 | 使用 `--init` 的值创建；未提供则生成随机盐 |

随机盐长 32 个字符，取自 ASCII 编码 33–126（`!` 到 `~`），不含空格和控制字符。手动传入的盐不受这一字符范围或长度限制。

强制替换已有盐时，旧内容保存在同目录的 `salt.datetime.bak` 文件中，例如 `salt.20261029-153045.123456789.bak`。日期时间使用本地时区，格式为 `年月日-时分秒.纳秒`。每次备份均保留；同名备份已存在或备份失败时，停止替换，不覆盖当前盐。首次创建和普通复用不生成备份。

使用 `mypwd -c` 或 `mypwd --history-clear` 删除符合上述日期时间格式的备份文件。清理保留当前 `salt`、其他文件、目录和符号链接，不创建或替换盐值；盐目录不存在时直接返回。删除后无法再通过这些备份找回旧盐。

执行清理前会显示警告并询问 `确认清除？[y/N]`。仅输入 `y` 或 `yes`（不区分大小写）后执行删除；直接回车、其他输入或输入结束均取消清理。

替换或丢失盐文件后，不能依靠相同文本复现原来的结果。`~/.mypwd` 必须是目录；如果它已经是普通文件，初始化会报错。

## 计算方式

1. 按 UTF-8 编码拼接 `text + salt`，中间不添加分隔符。
2. 计算 MD5，得到 16 个原始字节。
3. 对每个字节计算 `字节值 % CHARSET.len()`，取对应字符。
4. 检查字母、数字、符号三类是否齐全，缺少时根据 MD5 确定替换位置和字符，只替换出现次数超过一次的类别中的字符。
5. 返回 16 个字符，并在终端输出一行。结果至少包含一个字母、一个数字和一个符号，不强制同时包含大写和小写。

输出字符表定义在 `src/hashing.rs`，包含大小写字母、数字和 ASCII 标点，排除了 `i I l L 1 o O 0`。随机盐不使用这个字符表。

字符补齐过程不使用随机数，相同文本和盐仍得到相同密码。相比未做类别补齐的版本，原先缺少某类字符的密码会改变，已经包含三类字符的密码保持不变。

MD5 加盐及取模映射不适合作为生产环境的密码存储方案；取模映射也存在分布偏差。

## 代码结构

```text
src/
├── main.rs     # 解析参数并调度初始化、摘要计算和输出
├── cli.rs      # clap 参数声明
├── salt.rs     # 盐文件路径、生成、读写和初始化
└── hashing.rs  # 加盐 MD5、字符表和摘要映射
```

跨模块使用的接口通过 `pub(crate)` 开放，其余函数保持模块私有。

检查格式和编译：

```bash
cargo fmt --check
cargo check
cargo test
```
