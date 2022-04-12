# 1 It is recommended to clone this project using Intellij Clion

# 2 When Intellij Clion is opened, the test code contained in Example /crud_test.rs and main.rs can be executed by clicking the green button.

# Pressure test recommendations

* Modify Cargo. Toml to remove rbatis ["debug_mode"]
* main.rs fast_log::init_log debug_mode Set false to make sure Console does not print logs


# 1 建议使用Intellij Clion克隆本项目

# 2 Clion打开后，example/crud_test.rs 和 main.rs 包含的测试代码均可 点击 绿色按钮 执行。

# 3 执行测试后，可用Clion打开target/sqlite.db，ide右侧打开sqlite数据库面板查看数据

# 压测建议

* 修改Cargo.toml,去除 rbatis  ["debug_mode"]
* main.rs fast_log::init_log debug_mode 设置false,确认Console不打印日志
