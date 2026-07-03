# RustDesk Build 2026-0703-0841

## 来源

- 源仓库：`https://github.com/sysadmin846/rs-client`
- Actions Run：`https://github.com/sysadmin846/rs-client/actions/runs/28630738923`
- Run 标题：`RustDesk Build #12 | 2026-0703-0841 | @sysadmin846`

## 当前状态

截至本说明创建时，Actions 仍在运行，最终客户端安装包尚未全部生成。

已完成的前置阶段：

- `generate-bridge`
- `build-RustDeskTempTopMostWindow`

最终产物生成后，会继续上传到本目录或同名 Release。

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
