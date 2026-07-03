# 运维管理 RustDesk 构建产物

本仓库用于存放运维管理平台配套的 RustDesk 客户端构建产物和说明。

## 当前构建

- 构建来源：`sysadmin846/rs-client`
- GitHub Actions Run：`https://github.com/sysadmin846/rs-client/actions/runs/28630738923`
- 构建标记：`2026-0703-0841`
- 用途：配合运维管理后台“一键远程”功能使用。

## 功能说明

运维管理后台不会保存永久远程密码或永久 ticket。管理员在在线设备页面点击“一键远程”时，后台会实时签发 60 秒有效的一次性 RustDesk ticket，并通过 `rustdesk://connection/new/{id}?password=<ticket>&relay=true` 拉起本机 RustDesk 客户端。

## 文件位置

最终 RustDesk 安装包会放在：

```text
releases/2026-0703-0841/
```

如果目录内只有说明文件，表示上游 GitHub Actions 构建仍在运行，最终安装包尚未生成。
