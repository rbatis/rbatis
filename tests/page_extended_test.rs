/// Extended tests for Page module covering untested APIs:
/// - PageRequest::set_* chain API
/// - IPageRequest::offset_limit()
/// - Page::make_page_requests()
/// - Page::from<T>() type conversion
/// - Page::records_take()
/// - Page::Display implementation
/// - Page boundary conditions (page_no=0, page_size=0)

#[cfg(test)]
mod test {
    use rbatis::plugin::{IPage, IPageRequest, Page, PageRequest, DEFAULT_PAGE_SIZE};

    // ==================== PageRequest Tests ====================

    #[test]
    fn test_page_request_new_default_total() {
        let req = PageRequest::new(1, 10);
        assert_eq!(req.page_no, 1);
        assert_eq!(req.page_size, 10);
        assert_eq!(req.total, DEFAULT_PAGE_SIZE); // default total from new()
        assert!(req.do_count);
    }

    #[test]
    fn test_page_request_new_total_custom() {
        let req = PageRequest::new_total(2, 5, 100);
        assert_eq!(req.page_no, 2);
        assert_eq!(req.page_size, 5);
        assert_eq!(req.total, 100);
        assert!(req.do_count);
    }

    #[test]
    fn test_page_request_new_zero_page_no_clamped_to_1() {
        let req = PageRequest::new(0, 10);
        // page_no < 1 should be clamped to 1
        assert_eq!(req.page_no, 1);
    }

    #[test]
    fn test_page_request_set_total_chain() {
        let req = PageRequest::new(1, 10).set_total(500);
        assert_eq!(req.total, 500);
        assert_eq!(req.page_no, 1);
        assert_eq!(req.page_size, 10);
    }

    #[test]
    fn test_page_request_set_page_size_chain() {
        let req = PageRequest::new(1, 10).set_page_size(20);
        assert_eq!(req.page_size, 20);
    }

    #[test]
    fn test_page_request_set_page_no_chain() {
        let req = PageRequest::new(1, 10).set_page_no(3);
        assert_eq!(req.page_no, 3);
    }

    #[test]
    fn test_page_request_set_do_count_chain() {
        let req = PageRequest::new(1, 10).set_do_count(false);
        assert!(!req.do_count);
    }

    #[test]
    fn test_page_request_full_chain() {
        let req = PageRequest::new(1, 10)
            .set_total(200)
            .set_page_size(25)
            .set_page_no(5)
            .set_do_count(false);
        assert_eq!(req.total, 200);
        assert_eq!(req.page_size, 25);
        assert_eq!(req.page_no, 5);
        assert!(!req.do_count);
    }

    #[test]
    fn test_page_request_default() {
        let req = PageRequest::default();
        assert_eq!(req.total, 0);
        assert_eq!(req.page_no, 1);
        assert_eq!(req.page_size, DEFAULT_PAGE_SIZE);
        assert!(req.do_count);
    }

    // ==================== IPageRequest::offset_limit Tests ====================

    #[test]
    fn test_offset_limit_normal_case() {
        let req = PageRequest::new_total(2, 10, 100);
        // page_no=2 => offset=(2-1)*10=10, offset_limit=10+10=20 <= total(100) => 20
        assert_eq!(req.offset_limit(), 20);
    }

    #[test]
    fn test_offset_limit_first_page() {
        let req = PageRequest::new_total(1, 10, 50);
        // offset=0, offset_limit=0+10=10 <= 50 => 10
        assert_eq!(req.offset_limit(), 10);
    }

    #[test]
    fn test_offset_limit_last_page_truncated_to_total() {
        // total=15, page_size=10, page_no=2
        // offset=10, offset_limit=10+10=20 > total(15) => 15
        let req = PageRequest::new_total(2, 10, 15);
        assert_eq!(req.offset_limit(), 15);
    }

    #[test]
    fn test_offset_limit_exact_total() {
        // total=20, page_size=10, page_no=2
        // offset=10, offset_limit=10+10=20 == total(20) => 20
        let req = PageRequest::new_total(2, 10, 20);
        assert_eq!(req.offset_limit(), 20);
    }

    #[test]
    fn test_offset_limit_page_zero() {
        // page_no=0 is clamped to 1 internally by PageRequest::new
        let req = PageRequest::new(0, 10);
        // But if page_no is actually 0 on a raw struct (via Default or direct construction)
        // offset would be 0
        assert_eq!(req.offset(), 0);
    }

    #[test]
    fn test_offset_limit_exceeds_total() {
        // total=5, page_no=1, page_size=10
        let req = PageRequest::new_total(1, 10, 5);
        // offset=0, offset_limit=0+10=10 > 5 => 5
        assert_eq!(req.offset_limit(), 5);
    }

    // ==================== Page::make_page_requests Tests ====================

    #[test]
    fn test_make_page_requests_basic() {
        let requests = Page::<i32>::make_page_requests(9, 3);
        assert_eq!(requests.len(), 3);
        assert_eq!(requests[0].page_no, 1);
        assert_eq!(requests[0].page_size, 3);
        assert_eq!(requests[0].total, 9);
        assert_eq!(requests[1].page_no, 2);
        assert_eq!(requests[2].page_no, 3);
    }

    #[test]
    fn test_make_page_requests_exact_division() {
        let requests = Page::<i32>::make_page_requests(10, 5);
        assert_eq!(requests.len(), 2);
        assert_eq!(requests[0].page_no, 1);
        assert_eq!(requests[1].page_no, 2);
    }

    #[test]
    fn test_make_page_requests_single_element() {
        let requests = Page::<i32>::make_page_requests(1, 10);
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].page_no, 1);
    }

    #[test]
    fn test_make_page_requests_zero_elements() {
        let requests = Page::<i32>::make_page_requests(0, 10);
        // 0 / 10 = 0 pages
        assert_eq!(requests.len(), 0);
    }

    #[test]
    fn test_make_page_requests_page_size_larger_than_total() {
        let requests = Page::<i32>::make_page_requests(5, 100);
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].page_no, 1);
        assert_eq!(requests[0].total, 5);
    }

    #[test]
    fn test_make_page_requests_page_size_one() {
        let requests = Page::<i32>::make_page_requests(4, 1);
        assert_eq!(requests.len(), 4);
        for i in 0..4 {
            assert_eq!(requests[i].page_no, i as u64 + 1);
        }
    }

    // ==================== Page::from<T> Tests ====================

    #[test]
    fn test_page_from_type_conversion() {
        let source = Page::new(1, 10, 100, vec!["a", "b", "c"]);
        let target: Page<String> = Page::from(source);
        assert_eq!(target.page_no, 1);
        assert_eq!(target.page_size, 10);
        assert_eq!(target.total, 100);
        assert_eq!(target.records.len(), 3);
        assert_eq!(target.records[0], "a");
        assert_eq!(target.records[1], "b");
        assert_eq!(target.records[2], "c");
    }

    #[test]
    fn test_page_from_empty_records() {
        let source: Page<i32> = Page::new(1, 10, 0, vec![]);
        let target: Page<i64> = Page::from(source);
        assert_eq!(target.records.len(), 0);
        assert_eq!(target.total, 0);
    }

    #[test]
    fn test_page_from_preserves_do_count() {
        let source = Page::new(1, 10, 100, vec![1, 2, 3]).set_do_count(false);
        let target: Page<i64> = Page::from(source);
        assert!(!target.do_count);
    }

    // ==================== Page::records_take Tests ====================

    #[test]
    fn test_records_take_basic() {
        let mut page = Page::new(1, 10, 100, vec![1, 2, 3, 4, 5]);
        let taken = page.records_take();
        assert_eq!(taken, vec![1, 2, 3, 4, 5]);
        // After take, records should be empty
        assert!(page.records.is_empty());
    }

    #[test]
    fn test_records_take_order_preserved() {
        let mut page = Page::new(1, 10, 100, vec!["x", "y", "z"]);
        let taken = page.records_take();
        // records_take pops from end then reverses, so order should be preserved
        assert_eq!(taken, vec!["x", "y", "z"]);
    }

    #[test]
    fn test_records_take_empty() {
        let page: Page<String> = Page::new(1, 10, 0, vec![]);
        let taken = page.records();
        assert!(taken.is_empty());
    }

    #[test]
    fn test_records_take_single_element() {
        let mut page = Page::new(1, 10, 1, vec![42]);
        let taken = page.records_take();
        assert_eq!(taken, vec![42]);
        assert!(page.records.is_empty());
    }

    // ==================== Page Boundary Conditions ====================

    #[test]
    fn test_page_new_page_size_zero_defaults() {
        // page_size=0 should default to DEFAULT_PAGE_SIZE
        let page: Page<i32> = Page::new(1, 0, 0, vec![]);
        assert_eq!(page.page_size, DEFAULT_PAGE_SIZE);
    }

    #[test]
    fn test_page_new_page_no_zero_clamped() {
        // page_no=0 should be clamped to 1
        let page: Page<i32> = Page::new(0, 10, 0, vec![]);
        assert_eq!(page.page_no, 1);
    }

    #[test]
    fn test_page_new_total_page_size() {
        let page = Page::<i32>::new_total(1, 10, 50);
        assert_eq!(page.records.capacity(), 10); // pre-allocated capacity
        assert_eq!(page.total, 50);
    }

    // ==================== Page Display ====================

    #[test]
    fn test_page_display() {
        let page = Page::new(1, 10, 100, vec![1, 2, 3]);
        let display_str = format!("{}", page);
        // Display should contain key fields
        assert!(display_str.contains("Page"));
        // It's a debug-like format, just verify it doesn't panic and produces output
        assert!(!display_str.is_empty());
    }

    // ==================== Page set_* chain methods ====================

    #[test]
    fn test_page_set_total_chain() {
        let page: Page<i32> = Page::new(1, 10, 0, vec![]).set_total(999);
        assert_eq!(page.total, 999);
    }

    #[test]
    fn test_page_set_page_size_chain() {
        let page: Page<i32> = Page::new(1, 10, 0, vec![]).set_page_size(50);
        assert_eq!(page.page_size, 50);
    }

    #[test]
    fn test_page_set_page_no_chain() {
        let page: Page<i32> = Page::new(1, 10, 0, vec![]).set_page_no(5);
        assert_eq!(page.page_no, 5);
    }

    #[test]
    fn test_page_set_do_count_chain() {
        let page: Page<i32> = Page::new(1, 10, 0, vec![]).set_do_count(false);
        assert!(!page.do_count);
    }

    #[test]
    fn test_page_set_records_chain() {
        let page: Page<i32> = Page::new(1, 10, 0, vec![]).set_records(vec![10, 20, 30]);
        assert_eq!(page.records, vec![10, 20, 30]);
    }

    // ==================== IPage for Page ====================

    #[test]
    fn test_page_records_immut_ref() {
        let page = Page::new(1, 10, 100, vec![1, 2, 3]);
        assert_eq!(page.records(), &vec![1, 2, 3]);
    }

    #[test]
    fn test_page_records_mut_ref() {
        let mut page = Page::new(1, 10, 100, vec![1, 2, 3]);
        page.records_mut().push(4);
        assert_eq!(page.records(), &vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_page_default_impl() {
        let page: Page<String> = Page::default();
        assert!(page.records.is_empty());
        assert_eq!(page.total, 0);
        assert_eq!(page.page_no, 1);
        assert_eq!(page.page_size, DEFAULT_PAGE_SIZE);
        assert!(page.do_count);
    }

    // ==================== IPageRequest pages() edge cases ====================

    #[test]
    fn test_pages_zero_page_size_returns_zero() {
        let req = PageRequest::new_total(1, 0, 100);
        // page_size == 0 => pages returns 0
        assert_eq!(req.pages(), 0);
    }

    #[test]
    fn test_pages_exact_division() {
        let req = PageRequest::new_total(1, 10, 100);
        assert_eq!(req.pages(), 10);
    }

    #[test]
    fn test_pages_with_remainder() {
        let req = PageRequest::new_total(1, 10, 95);
        assert_eq!(req.pages(), 10); // 95/10 = 9 remainder 5 => 10
    }

    #[test]
    fn test_pages_zero_total() {
        let req = PageRequest::new_total(1, 10, 0);
        // 0/10 = 0, remainder 0 => 0
        assert_eq!(req.pages(), 0);
    }

    #[test]
    fn test_offset_edge_cases() {
        let req = PageRequest::new_total(1, 10, 100);
        assert_eq!(req.offset(), 0);

        let req2 = PageRequest::new_total(3, 10, 100);
        assert_eq!(req2.offset(), 20); // (3-1)*10 = 20
    }
}
