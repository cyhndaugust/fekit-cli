# Repository Guidelines

## 项目结构与模块组织
- `src/main.rs` 负责 CLI 解析与库入口调用。
- `src/lib.rs` 包含命令分发与共享逻辑。
- `src/config.rs` 定义 CLI 参数与子命令（基于 `clap`）。
- `.github/workflows/release.yml` 在打 tag 时构建发布产物。
- `target/` 为构建输出目录（生成物，不要编辑或提交）。

## 构建、测试与开发命令
- `cargo build` 构建调试版二进制到 `target/debug`。
- `cargo build --release --locked` 构建锁定依赖的发布版（CI 使用）。
- `cargo run -- tag --version 1.2.3` 本地运行 CLI 并执行 `tag` 子命令。
- `cargo fmt` 在安装 `rustfmt` 时格式化代码。

## 编码风格与命名规范
- 使用 Rust 2024 edition，默认 `rustfmt` 风格（4 空格缩进）。
- 文件/模块采用 `snake_case.rs`，类型使用 `CamelCase`。
- CLI 帮助文案应简洁、面向用户；新增文案默认用英文，除非需与现有中文提示保持一致。

## 测试指南
- 当前暂无自动化测试。若新增测试，单元测试放在模块文件内，集成测试放在 `tests/`。
- 使用描述性测试名，如 `test_tag_requires_version`，并说明通过 `cargo test` 运行。

## 提交与 PR 规范
- 现有提交信息为简短小写摘要（如 `config`, `structure optim`）。
- 延续该风格：单行、动词/摘要导向、末尾不加句号。
- PR 需包含简要说明、变更动机，以及行为变化时的 CLI 输出或截图。

## 发布与分发说明
- 以 `v*` tag 触发发布，并为 macOS/Linux/Windows 打包 `fekit` 二进制。
- 若修改 CLI 参数，确保帮助文案与发布流程同步更新。
