#[cfg(test)]
mod tests {
    use rbatis::sql::{IPageRequest, PageRequest};

    fn make_pages() -> Vec<Vec<i32>> {
        vec![
            vec![1, 10, 100, 1],
            vec![2, 20, 200, 0],
            vec![3, 30, 300, 1],
        ]
    }
    #[test]
    fn test_page_request() {
        let data = make_pages();
        let page_req = PageRequest::default();
        for i in 0..data.len() {
            let (page_no, page_size, total, search_count) = (
                data[i][0] as u64,
                data[i][1] as u64,
                data[i][2] as u64,
                data[i][3] != 0,
            );
            let mut pr = PageRequest::new_total(page_no, page_size, total);
            pr = pr.set_do_count(search_count);
            assert_eq!(pr.page_size(), page_size);
            assert_eq!(pr.page_no(), if page_no < 1 { 1 } else { page_no });
            assert_eq!(pr.total(), total);
            assert_eq!(pr.do_count(), search_count);

            let mut pr = PageRequest::new_option(
                if page_no == 0 { None } else { Some(page_no) },
                if page_size == 0 {
                    None
                } else {
                    Some(page_size)
                },
            );
            pr = pr.set_do_count(search_count);
            assert_eq!(pr.page_size(), page_size);
            assert_eq!(pr.page_no(), if page_no < 1 { 1 } else { page_no });
            assert_eq!(pr.do_count(), search_count);

            let mut pr = page_req
                .clone()
                .set_page_no(page_no)
                .set_page_size(page_size)
                .set_total(total)
                .set_do_count(search_count);
            pr = pr.set_do_count(search_count);
            assert_eq!(pr.page_size(), page_size);
            assert_eq!(pr.page_no(), if page_no < 1 { 1 } else { page_no });
            assert_eq!(pr.total(), total);
            assert_eq!(pr.do_count(), search_count);
        }
    }
}
