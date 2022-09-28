# exert sqlserver

## Docker Linux

```bash
# 2022
docker pull mcr.microsoft.com/mssql/server:2022-latest
```

## 提示

```bash
# 在项目里面设置代理
git config http.proxy socks5://127.0.0.1:1080
git config https.proxy socks5://127.0.0.1:1080
```

```bash
/opt/mssql-tools/bin/sqlcmd -S localhost -U SA -P sa.pass00.word11
```