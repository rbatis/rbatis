select * from biz_activity where delete_flag = 0
    if name != '':
      and name=#{name}