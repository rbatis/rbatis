## 事务处理模块

### 传播行为
    ///默认，表示如果当前事务存在，则支持当前事务。否则，会启动一个新的事务。have tx ? join : new tx()
    REQUIRED,
    ///表示如果当前事务存在，则支持当前事务，如果当前没有事务，就以非事务方式执行。  have tx ? join(): session.exec()
    SUPPORTS,
    ///表示如果当前事务存在，则支持当前事务，如果当前没有事务，则返回事务嵌套错误。  have tx ? join() : return error
    MANDATORY,
    ///表示新建一个全新Session开启一个全新事务，如果当前存在事务，则把当前事务挂起。 have tx ? stop old。  -> new session().new tx()
    REQUIRES_NEW,
    ///表示以非事务方式执行操作，如果当前存在事务，则新建一个Session以非事务方式执行操作，把当前事务挂起。  have tx ? stop old。 -> new session().exec()
    NOT_SUPPORTED,
    ///表示以非事务方式执行操作，如果当前存在事务，则返回事务嵌套错误。    have tx ? return error: session.exec()
    NEVER,
    ///表示如果当前事务存在，则在嵌套事务内执行，如嵌套事务回滚，则只会在嵌套事务内回滚，不会影响当前事务。如果当前没有事务，则进行与PROPAGATION_REQUIRED类似的操作。
    NESTED,
    ///表示如果当前没有事务，就新建一个事务,否则返回错误。  have tx ? return error: session.new tx()
    NOT_REQUIRED,




### 案例
定义serviceA.methodA()以PROPAGATION_REQUIRED修饰；
定义serviceB.methodB()以表格中三种方式修饰；
methodA中调用methodB

| 异常状态    | PROPAGATION_REQUIRES_NEW （两个独立事务） |  PROPAGATION_NESTED  (B的事务嵌套在A的事务中)   |  PROPAGATION_REQUIRED   (同一个事务)   |
| ------ | ------ | ------ | ------ |
|  methodA抛异常 methodB正常      |   A回滚，B正常提交     |    A与B一起回滚    |   A与B一起回滚     |
|    methodA正常methodB抛异常    |    1.如果A中捕获B的异常，并没有继续向上抛异常，则B先回滚，A再正常提交；2.如果A未捕获B的异常，默认则会将B的异常向上抛，则B先回滚，A再回滚    |   B先回滚，A再正常提交     |    A与B一起回滚    |
|    methodA抛异常methodB抛异常    |    B先回滚，A再回滚    |    A与B一起回滚    |     A与B一起回滚   |
|    methodA正常methodB正常   |    B先提交，A再提交    |    A与B一起提交    |    A与B一起提交    |