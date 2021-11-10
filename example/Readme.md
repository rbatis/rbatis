# 1 It is recommended to clone this project using Intellij Clion
# 2 Use docker to quickly start mysql database
```cmd
Docker run -p 3306:3306 --name mysql -e MYSQL_ROOT_PASSWORD=123456 -d mysql:5.7
```
SQL > select * from database
# 4 When Intellij Clion is opened, the test code contained in Example /crud_test.rs and main.rs can be executed by clicking the green button.

# Pressure test recommendations

* Modify Cargo. Toml to remove rbatis ["debug_mode"]

* main.rs fast_log::init_log debug_mode Set false to make sure Console does not print logs


# 1 建议使用Intellij Clion克隆本项目

# 2 建议使用docker快速启动mysql数据库

```cmd
docker run -p 3306:3306 --name mysql -e MYSQL_ROOT_PASSWORD=123456 -d mysql:5.7
```

# 3 导入数据库表数据 example/database.sql

# 4 Intellij Clion打开后，example/crud_test.rs 和 main.rs 包含的测试代码均可 点击 绿色按钮 执行。



#压测建议

* 修改Cargo.toml,去除 rbatis  ["debug_mode"]
* main.rs  fast_log::init_log debug_mode 设置false,确认Console不打印日志
