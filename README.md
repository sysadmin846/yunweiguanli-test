# 运维管理 RustDesk 构建产物

本仓库用于存放运维管理平台配套的 RustDesk 客户端和 Windows server 构建产物。

## 当前构建

- 构建标记：`2026-0703-0841`
- 客户端来源：`sysadmin846/rs-client`
- 客户端 Actions Run：`https://github.com/sysadmin846/rs-client/actions/runs/28630738923`
- 客户端源 Release：`https://github.com/sysadmin846/rs-client/releases/tag/2026-0703-0841-12`
- 本仓库 Release：`https://github.com/sysadmin846/yunweiguanli-test/releases/tag/2026-0703-0841`
- 用途：配合运维管理后台“一键远程”功能使用。

## 直接下载打开

| 类型 | 文件 | 说明 |
| --- | --- | --- |
| Mac Apple Silicon | `rustdesk-1.4.4-aarch64-aarch64.dmg` | M1/M2/M3/M4 Mac，下载后双击打开安装 |
| Mac Intel | `rustdesk-1.4.4-x86_64-x86_64.dmg` | Intel Mac，下载后双击打开安装 |
| Windows 安装版 | `rustdesk-1.4.4-x86_64.msi` | Windows 标准安装包，下载后双击安装 |
| Windows 免安装版 | `rustdesk-1.4.4-x86_64.exe` | Windows 自解压/免安装启动文件，下载后双击打开 |
| Windows Server | `rustdesk-server-windows-x86_64.zip` | Windows Server 端，解压后运行 `hbbs.exe` 和 `hbbr.exe` |

校验值见：`releases/2026-0703-0841/SHA256SUMS.txt`。

## 功能说明

运维管理后台不会保存永久远程密码或永久 ticket。管理员在在线设备页面点击“一键远程”时，后台会实时签发短时一次性 RustDesk ticket，并通过 `rustdesk://connection/new/{id}?password=<ticket>&relay=true` 拉起本机 RustDesk 客户端。

## 文件说明

- `releases/2026-0703-0841/BUILD_INFO.md`：本次构建说明。
- `releases/2026-0703-0841/SHA256SUMS.txt`：安装包 SHA256 校验值。
- `.github/workflows/build-windows-server.yml`：Windows server 包构建流程。
- `sources/rustdesk-server/`：用于 Windows server 构建的 RustDesk server 源码快照。
