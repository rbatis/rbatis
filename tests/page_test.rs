#[cfg(test)]
mod test {
    use rbatis::plugin::page::{IPage, IPageRequest, Page, RbatisReplacePagePlugin};

    #[test]
    pub fn test_page() {
        let mut page: Page<i32> = Page::new(2, 10, None);
        page.records.push(12);
        let s = serde_json::to_string(&page).unwrap();
        println!("{}", s.clone());
        let r: Page<i32> = serde_json::from_str(s.as_str()).unwrap();
        println!("{:?}", r);

        println!("offset:{}", page.offset());
        println!("get_pages:{}", page.get_pages());
        println!("page_string:{}", page.to_string());
        assert_eq!(page.offset(), 10);
    }

    #[test]
    fn test_make_count() {
        let plugin = RbatisReplacePagePlugin {};
        let sql = plugin
            .make_count_sql("biz_activity where id = 1 and order by id DESC and order by id DESC");
        println!("sql:{}", sql);
    }
}
