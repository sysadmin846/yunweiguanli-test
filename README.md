# 运维管理 RustDesk 构建产物

本仓库用于存放运维管理平台配套的 RustDesk 客户端构建产物和说明。

## 当前构建

- 构建来源：`sysadmin846/rs-client`
- GitHub Actions Run：`https://github.com/sysadmin846/rs-client/actions/runs/28630738923`
- 源 Release：`https://github.com/sysadmin846/rs-client/releases/tag/2026-0703-0841-12`
- 本仓库 Release：`https://github.com/sysadmin846/yunweiguanli-test/releases/tag/2026-0703-0841`
- 构建标记：`2026-0703-0841`
- 用途：配合运维管理后台“一键远程”功能使用。

## 已上传文件

- `rustdesk-1.4.4-aarch64-aarch64.dmg`

校验值见：

```text
releases/2026-0703-0841/SHA256SUMS.txt
```

## 功能说明

运维管理后台不会保存永久远程密码或永久 ticket。管理员在在线设备页面点击“一键远程”时，后台会实时签发 60 秒有效的一次性 RustDesk ticket，并通过 `rustdesk://connection/new/{id}?password=<ticket>&relay=true` 拉起本机 RustDesk 客户端。

## 文件说明

- `releases/2026-0703-0841/BUILD_INFO.md`：本次构建说明。
- `releases/2026-0703-0841/SHA256SUMS.txt`：安装包 SHA256 校验值。
- Release `2026-0703-0841`：实际安装包下载入口。
