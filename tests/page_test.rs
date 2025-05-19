use rbatis::Page;
#[test]
fn test_page_into_range() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let ranges = Page::<i32>::make_ranges(v.len() as u64, 3);
    let mut new_v = vec![];
    for (offset, limit) in ranges {
        for i in offset..limit {
            new_v.push(i + 1);
        }
    }
    assert_eq!(v, new_v);
}

#[test]
fn test_page_into_range_zero() {
    let mut v = vec![1];
    v.clear();
    let ranges = Page::<i32>::make_ranges(v.len() as u64, 3);
    let mut new_v = vec![];
    for (offset, limit) in ranges {
        for i in offset..limit {
            new_v.push(i + 1);
        }
    }
    assert_eq!(v, new_v);
}

#[test]
fn test_page_into_pages() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let pages = Page::make_pages(v.clone(), 3);
    assert_eq!(pages.len(), 3);
    let mut new_v = vec![];
    for x in pages {
        for i in x.records {
            new_v.push(i);
        }
    }
    assert_eq!(v, new_v);
}

#[test]
fn test_page_into_pages_more_than() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let pages = Page::make_pages(v.clone(), 18);
    assert_eq!(pages.len(), 1);
    let mut new_v = vec![];
    for x in pages {
        for i in x.records {
            new_v.push(i);
        }
    }
    assert_eq!(v, new_v);
}

#[test]
fn test_page_into_pages_zero() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let pages = Page::make_pages(v.clone(), 1);
    assert_eq!(pages.len(), 9);
    let mut new_v = vec![];
    for x in pages {
        for i in x.records {
            new_v.push(i);
        }
    }
    assert_eq!(v, new_v);
}

#[test]
fn test_page_into_pages_8() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let pages = Page::make_pages(v.clone(), 3);
    assert_eq!(pages.len(), 3);
    let mut new_v = vec![];
    for x in pages {
        for i in x.records {
            new_v.push(i);
        }
    }
    assert_eq!(v, new_v);
}

#[test]
fn test_page_into_pages_10() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let pages = Page::make_pages(v.clone(), 3);
    assert_eq!(pages.len(), 4);
    let mut new_v = vec![];
    for x in pages {
        for i in x.records {
            new_v.push(i);
        }
    }
    assert_eq!(v, new_v);
}

#[test]
fn test_page_into_pages_one() {
    let v = vec![1];
    let pages = Page::make_pages(v.clone(), 1);
    let mut new_v = vec![];
    for x in pages {
        for i in x.records {
            new_v.push(i);
        }
    }
    assert_eq!(v, new_v);
}