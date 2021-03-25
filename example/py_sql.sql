--fn:py_select_file
select * from biz_activity where delete_flag = 0
    if name != '':
      and name=#{name}
--fn:py_select_file2
select * from biz_activity where delete_flag = 0
    if name != '':
      and name=#{name}