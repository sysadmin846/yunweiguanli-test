# RustDesk Build 2026-0703-0841

## 来源

- 客户端源仓库：`https://github.com/sysadmin846/rs-client`
- 客户端 Actions Run：`https://github.com/sysadmin846/rs-client/actions/runs/28630738923`
- 客户端源 Release：`https://github.com/sysadmin846/rs-client/releases/tag/2026-0703-0841-12`
- Windows server Actions Run：`https://github.com/sysadmin846/yunweiguanli-test/actions/runs/28633328741`
- 目标 Release：`https://github.com/sysadmin846/yunweiguanli-test/releases/tag/2026-0703-0841`

## 已上传文件

| 文件 | 类型 | 打开方式 |
| --- | --- | --- |
| `rustdesk-1.4.4-aarch64-aarch64.dmg` | Mac Apple Silicon 客户端 | 双击 DMG 安装 |
| `rustdesk-1.4.4-x86_64-x86_64.dmg` | Mac Intel 客户端 | 双击 DMG 安装 |
| `rustdesk-1.4.4-x86_64.msi` | Windows 安装版客户端 | 双击 MSI 安装 |
| `rustdesk-1.4.4-x86_64.exe` | Windows 免安装版客户端 | 双击 EXE 直接打开 |
| `rustdesk-server-windows-x86_64.zip` | Windows Server 端 | 解压后运行 `hbbs.exe` 和 `hbbr.exe` |
| `SHA256SUMS.txt` | 校验文件 | 用于核对下载文件完整性 |

## 使用说明

1. 下载对应系统的安装包或压缩包。
2. Windows/macOS 客户端安装或直接打开。
3. Windows Server 包解压后运行 `hbbs.exe` 和 `hbbr.exe`。
4. 登录运维管理后台，进入 RustDesk 在线设备页面。
5. 点击“一键远程”，浏览器会拉起本机 RustDesk 客户端并使用后台签发的一次性 ticket 发起连接。

## 安全说明

- 不保存永久密码。
- 不保存长期 ticket。
- 后台每次点击实时签发短时 ticket。
- 后台审计日志记录 `request_source=admin_launch`。
