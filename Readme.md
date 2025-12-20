# Front-end toolkit

## install

```zsh
# 注意需要匹配最新的安装脚本才能安装最新版本
curl -fsSL https://raw.githubusercontent.com/cyhndaugust/fekit-cli/refs/heads/0.1/install.sh | sh
```

安装后可使用 `fekit` 或 `fk` 执行命令（`fk` 由安装脚本创建别名）。

`--version` 输出优先使用构建时的 tag 版本（例如 `v0.1.5`），若未注入则回退到 `Cargo.toml` 版本。

## 升级

```zsh
fekit upgrade
```

执行后将自动下载最新版本并替换当前可执行文件（部分系统可能需要退出后手动替换）。

## 自动创建下一个 Tag

```zsh
./scripts/next-tag.sh
```

脚本要求当前分支名为 `X.Y` 格式；否则将提示无法创建 tag。它会基于当前分支已合并的最新 `vX.Y.Z` tag 自动补丁 +1 并推送到远程，从而触发 GitHub Actions 打包；如果当前分支没有 `vX.Y.*` tag，则从 `vX.Y.0` 起步。

## 开发操作
```zsh
# push tag 到远程
git tag v0.1.5 && git push origin v0.1.5
```

## 子命令功能

### tag 功能描述（生产使用说明）

#### 命令详情
```zsh
fk tag [version] # 版本号可选，如 1.0.0 或 xxx/1.0.0
fk tag # 不带版本号时根据规则预生成下一个版本
fk tag --push # --push 推送版本到远程
```

#### 作用
为前端项目生成并发布 git tag，同时同步 `package.json` 的 `version`。

#### 前置条件
- 当前目录必须是前端工程：存在 `package.json` 且包含 `version` 字段。
- 非前端工程会提示并停止执行。

#### 预生成 tag 规则
执行 `tag` 时只进行“预生成”，不会立即创建 tag。规则如下：
- 如果命令带版本号（格式：`1.0.0` 或 `xxx/1.0.0`），先校验格式；不匹配则退出。
- `version: "1.0"` -> 预生成 `"1.0.0"`（若未创建过 tag）。
- `version: "1.0.0"` -> 预生成 `"1.0.1"`（已存在 tag 则补丁版本 +1）。
- `version: "xxx/1.0"` -> 预生成 `"xxx/1.0.0"`（兼容非标准前缀）。
- `version: "xxx/1.0.0"` -> 预生成 `"xxx/1.0.1"`。
- 若远程已存在该 tag，提示并退出，等待用户调整 version 后重试。

#### 确认与发布流程
预生成成功后会提示当前版本与目标 tag。
用户输入 `y` 继续，`n` 取消，`q` 退出。确认后执行以下动作：
1) 将 `package.json` 的 `version` 更新为目标 tag 版本。
2) 提交版本变更（提交信息格式：`tag@1.0.0`）。
3) 创建对应 tag。

#### 远程推送
默认仅创建本地提交与 tag，不会推送远程。若需推送，请在命令中显式指定：
`fekit tag --push`
