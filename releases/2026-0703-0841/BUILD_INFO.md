# RustDesk Build 2026-0703-0841

## 来源

- 源仓库：`https://github.com/sysadmin846/rs-client`
- Actions Run：`https://github.com/sysadmin846/rs-client/actions/runs/28630738923`
- 源 Release：`https://github.com/sysadmin846/rs-client/releases/tag/2026-0703-0841-12`
- 目标 Release：`https://github.com/sysadmin846/yunweiguanli-test/releases/tag/2026-0703-0841`
- Run 标题：`RustDesk Build #12 | 2026-0703-0841 | @sysadmin846`

## 已上传文件

| 文件 | 说明 |
| --- | --- |
| `rustdesk-1.4.4-aarch64-aarch64.dmg` | macOS Apple Silicon / arm64 安装包 |
| `SHA256SUMS.txt` | 已上传安装包的 SHA256 校验值 |

## 构建状态

当前已上传源 Release 中已经生成的 macOS Apple Silicon 安装包。源 GitHub Actions Run 可能仍在继续生成其他平台产物；Windows 或 macOS x86_64 产物生成后，可继续追加到同一个目标 Release。

## 使用说明

1. 下载对应系统的安装包。
2. 安装或覆盖安装 RustDesk 客户端。
3. 登录运维管理后台，进入 RustDesk 在线设备页面。
4. 点击“一键远程”，浏览器会拉起本机 RustDesk 客户端并使用后台签发的一次性 ticket 发起连接。

## 安全说明

- 不保存永久密码。
- 不保存长期 ticket。
- 后台每次点击实时签发短时 ticket。
- 后台审计日志记录 `request_source=admin_launch`。
