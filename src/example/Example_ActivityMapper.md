## selectByIds
select * from biz_activity where delete_flag = 1
where:
  if id != null:
     #{id}   
  if create != null:
     and create_time = #{create_time}      
  if ids!= null:
    trim ',':
      for index,item in ids:
        #{item} 