`delete from activity where delete_flag = 0`
                  if name != '':
                    ` and name=#{name}`
                  if !ids.is_empty():
                    ` and id in `
                    ${ids.sql()}
