# 1 建议使用Intellij Clion克隆本项目

# 2 建议使用docker快速启动mysql数据库

```cmd
docker run -p 3306:3306 --name mysql -e MYSQL_ROOT_PASSWORD=123456 -d mysql:5.7
```

# 3 导入数据库表数据 example/database.sql

# 4 Intellij Clion打开后，example/crud_test.rs 和 main.rs 包含的测试代码均可 点击 绿色按钮 执行。



#压测建议

* 修改Cargo.toml,去除 rbatis  ["debug_mode"]
* main.rs  fast_log::init_log debug_mode 设置false