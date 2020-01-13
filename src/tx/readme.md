## 事务处理模块


定义serviceA.methodA()以PROPAGATION_REQUIRED修饰；
定义serviceB.methodB()以表格中三种方式修饰；
methodA中调用methodB
异常状态	PROPAGATION_REQUIRES_NEW
（两个独立事务）	PROPAGATION_NESTED
(B的事务嵌套在A的事务中)	PROPAGATION_REQUIRED
(同一个事务)
methodA抛异常
methodB正常	A回滚，B正常提交	A与B一起回滚	A与B一起回滚
methodA正常
methodB抛异常	1.如果A中捕获B的异常，并没有继续向上抛异常，则B先回滚，A再正常提交；
2.如果A未捕获B的异常，默认则会将B的异常向上抛，则B先回滚，A再回滚	B先回滚，A再正常提交	A与B一起回滚
methodA抛异常
methodB抛异常	B先回滚，A再回滚	A与B一起回滚	A与B一起回滚
methodA正常
methodB正常	B先提交，A再提交	A与B一起提交	A与B一起提交