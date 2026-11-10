//! Runtime coverage for:
//! - `src/plugin/id_generator/snowflake.rs`
//! - `src/plugin/id_generator/object_id.rs`
//! - `src/plugin/page.rs`
//! - `src/plugin/intercept/intercept_log.rs`
//! - `src/plugin/intercept/intercept_page.rs`
//! - `src/plugin/intercept/mod.rs`
//! - `src/utils/table_util.rs` (remaining macro arms)

#[cfg(test)]
mod test {
    use async_trait::async_trait;
    use dark_std::sync::SyncVec;
    use log::{Level, LevelFilter};
    use rbatis::executor::Executor;
    use rbatis::id_generator::IdGenerator;
    use rbatis::intercept::{apply_after, apply_before, Intercept, ResultType};
    use rbatis::object_id::{Error as OidError, ObjectId, ObjectIdGenerator};
    use rbatis::plugin::intercept_log::LogInterceptor;
    use rbatis::plugin::intercept_page::PageIntercept;
    use rbatis::snowflake::{new_snowflake_id, Snowflake, SNOWFLAKE};
    use rbatis::{field_key, field_name, table_field_btree};
    use rbatis::{Action, Error, IPage, IPageRequest, Page, PageRequest, RBatis};
    use rbdc::db::ExecResult;
    use rbs::Value;
    use std::error::Error as StdError;
    use std::str::FromStr;
    use std::sync::atomic::Ordering;
    use std::sync::Arc;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    fn exec_ok(rows: u64) -> Result<ExecResult, Error> {
        Ok(ExecResult {
            rows_affected: rows,
            last_insert_id: Value::Null,
        })
    }

    // ------------------------------------------------------------------
    // Snowflake
    // ------------------------------------------------------------------

    #[test]
    fn test_snowflake_constructors_clone_and_trait() {
        assert_ne!(Snowflake::new(1, 2, 0).generate_id(), 0);
        assert_ne!(Snowflake::default().generate_id(), 0);
        assert_ne!(<Snowflake as Default>::default().generate_id(), 0);
        assert_ne!(
            Snowflake::with_epoch(1, 2, 0, UNIX_EPOCH).generate_id(),
            0
        );
        assert_ne!(
            Snowflake::with_last_timestamp(1, 2, 0, 1_726_417_159).generate_id(),
            0
        );
        assert!(Snowflake::get_time_millis(UNIX_EPOCH) > 0);

        // Clone impl (snowflake.rs:51-63)
        let sf = Snowflake::new(3, 4, 1);
        let cloned = sf.clone();
        assert_eq!(cloned.machine_id, 3);
        assert_eq!(cloned.node_id, 4);
        assert_eq!(cloned.mode, 1);
        assert_ne!(cloned.generate_id(), 0);

        // IdGenerator impl + module level helpers
        let sf = Snowflake::new(1, 1, 0);
        assert_ne!(IdGenerator::generate(&sf), 0);
        assert_ne!(new_snowflake_id(), 0);
        assert_ne!(SNOWFLAKE.generate_id(), 0);
    }

    #[test]
    fn test_snowflake_fast_mode_counter_wrap() {
        // mode = 0: idx wrapping at 4096 advances last_timestamp instead of
        // waiting for the wall clock (snowflake.rs:143-146).
        let sf = Snowflake::new(1, 1, 0);
        let ids: Vec<i64> = (0..8200).map(|_| sf.generate_id()).collect();
        for id in &ids {
            assert_ne!(*id, 0);
        }
        assert!(ids.windows(2).all(|w| w[0] < w[1]));
        let mut unique = ids.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(unique.len(), ids.len());
    }

    #[test]
    fn test_snowflake_realtime_mode_counter_wrap() {
        // mode = 1: realtime path (snowflake.rs:136-142), idx wraps twice.
        let sf = Snowflake::new(1, 1, 1);
        let ids: Vec<i64> = (0..8200).map(|_| sf.generate_id()).collect();
        for id in &ids {
            assert_ne!(*id, 0);
        }
        let mut unique = ids.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(unique.len(), ids.len());
    }

    #[test]
    fn test_snowflake_realtime_same_millisecond_biding() {
        // `epoch = now - 5ms` makes `get_time_millis(epoch)` right after
        // construction equal to `last_timestamp`, so the first id enters the
        // `now_millis == last_time` branch and runs `biding_time_conditions`
        // until the next millisecond (snowflake.rs:139-141, 166-174).
        for _ in 0..500 {
            let epoch = SystemTime::now() - Duration::from_millis(5);
            let sf = Snowflake::with_epoch(1, 1, 1, epoch);
            let id = sf.generate_id();
            assert_ne!(id, 0);
        }
        // Same branch through `with_last_timestamp` (lines 98/106/110-111).
        for _ in 0..500 {
            let now = Snowflake::get_time_millis(UNIX_EPOCH);
            let sf = Snowflake::with_last_timestamp(1, 1, 1, now);
            assert_ne!(sf.generate_id(), 0);
        }
    }

    // ------------------------------------------------------------------
    // ObjectId
    // ------------------------------------------------------------------

    #[test]
    fn test_object_id_basics_and_round_trips() {
        let oid = ObjectId::new();
        let hex = oid.to_hex();
        assert_eq!(hex.len(), 24);
        assert_eq!(ObjectId::with_string(&hex).unwrap(), oid);
        assert_eq!(ObjectId::from_str(&hex).unwrap(), oid);
        assert_eq!(ObjectId::with_bytes(oid.bytes()), oid);
        assert_eq!(oid.bytes(), oid.id);
        assert_eq!(ObjectId::with_u128(oid.u128()), oid);

        // Display / Debug / Default
        assert_eq!(format!("{}", oid), hex);
        assert_eq!(format!("{:?}", oid), format!("ObjectId({})", hex));
        assert_eq!(ObjectId::default().to_hex().len(), 24);

        // ObjectIdGenerator constructors and both IdGenerator impls
        let generator = ObjectIdGenerator::new();
        let _ = generator.generate_id();
        let _ = IdGenerator::generate(&generator);
        let default_generator = ObjectIdGenerator::default();
        let _ = default_generator.generate_id();
        let _ = IdGenerator::generate(&ObjectId::new());
    }

    #[test]
    #[allow(deprecated)]
    fn test_object_id_error_display_and_cause() {
        // invalid hex -> Error::FromHexError
        let bad_hex = ObjectId::with_string("zzzz").unwrap_err();
        assert!(matches!(bad_hex, OidError::FromHexError(_)));
        assert!(!format!("{}", bad_hex).is_empty());
        assert!(<OidError as StdError>::cause(&bad_hex).is_some());

        // valid hex, wrong length -> Error::ArgumentError
        let bad_len = ObjectId::with_string("00").unwrap_err();
        assert!(matches!(bad_len, OidError::ArgumentError { .. }));
        assert_eq!(
            format!("{}", bad_len),
            "Provided string must be a 12-byte hexadecimal string."
        );
        assert!(<OidError as StdError>::cause(&bad_len).is_none());

        // FromStr surfaces the same errors
        assert!("not-hex".parse::<ObjectId>().is_err());
        assert!("12".parse::<ObjectId>().is_err());
    }

    // ------------------------------------------------------------------
    // Page / PageRequest
    // ------------------------------------------------------------------

    #[test]
    fn test_page_request_constructors_and_builder_setters() {
        let req = PageRequest::new(0, 20);
        assert_eq!(req.page_no, 1); // page_no is clamped to 1
        assert_eq!(req.page_size, 20);
        assert_eq!(req.total, 0);
        assert!(req.do_count);

        let req = PageRequest::new_total(2, 5, 12);
        assert_eq!(req.page_no, 2);
        assert_eq!(req.page_size, 5);
        assert_eq!(req.total, 12);

        let req = req
            .set_total(30)
            .set_page_size(10)
            .set_page_no(3)
            .set_do_count(false);
        assert_eq!(req.total, 30);
        assert_eq!(req.page_size, 10);
        assert_eq!(req.page_no, 3);
        assert!(!req.do_count);

        let default = PageRequest::default();
        assert_eq!(default.total, 0);
        assert_eq!(default.page_no, 1);
        assert_eq!(default.page_size, 10);
        assert!(default.do_count);
    }

    #[test]
    fn test_page_request_trait_setters_and_getters() {
        // IPageRequest for PageRequest (page.rs:221-253)
        let mut req = PageRequest::default();
        IPageRequest::set_total(&mut req, 100);
        IPageRequest::set_page_size(&mut req, 25);
        IPageRequest::set_page_no(&mut req, 2);
        IPageRequest::set_do_count(&mut req, false);
        assert_eq!(IPageRequest::total(&req), 100);
        assert_eq!(IPageRequest::page_size(&req), 25);
        assert_eq!(IPageRequest::page_no(&req), 2);
        assert!(!IPageRequest::do_count(&req));
    }

    #[test]
    fn test_page_request_pages_offset_and_offset_limit() {
        // pages(): zero total, exact multiple and non-multiple (page.rs:86-95)
        assert_eq!(PageRequest::new(1, 10).set_total(0).pages(), 0);
        assert_eq!(PageRequest::new(1, 10).set_total(100).pages(), 10);
        assert_eq!(PageRequest::new(1, 10).set_total(101).pages(), 11);
        let mut zero_size = PageRequest::new(1, 10);
        zero_size.page_size = 0;
        assert_eq!(zero_size.pages(), 0);

        // offset(): page_no 0 (line 102), first page and beyond the last page
        let mut req = PageRequest::new(1, 10).set_total(100);
        assert_eq!(req.offset(), 0);
        req.page_no = 0;
        assert_eq!(req.offset(), 0);
        req.page_no = 11;
        assert_eq!(req.offset(), 100);

        // offset_limit(): capped at total (page.rs:107-113)
        assert_eq!(PageRequest::new(1, 10).set_total(5).offset_limit(), 5);
        assert_eq!(PageRequest::new(2, 10).set_total(100).offset_limit(), 20);
        assert_eq!(PageRequest::new(11, 10).set_total(100).offset_limit(), 100);
    }

    #[test]
    fn test_page_constructors_and_metadata() {
        let page: Page<i32> = Page::default();
        assert!(page.records.is_empty());
        assert_eq!(page.total, 0);
        assert_eq!(page.page_no, 1);
        assert_eq!(page.page_size, 10);
        assert!(page.do_count);

        // zero page_size and zero page_no are clamped
        let page = Page::new(0, 0, 3, vec![1, 2, 3]);
        assert_eq!(page.page_no, 1);
        assert_eq!(page.page_size, 10);
        assert_eq!(page.total, 3);

        let page: Page<i32> = Page::new_total(2, 5, 7);
        assert!(page.records.is_empty());
        assert_eq!(page.page_no, 2);
        assert_eq!(page.page_size, 5);
        assert_eq!(page.total, 7);

        // inherent builder setters
        let page = page
            .set_total(20)
            .set_page_size(4)
            .set_page_no(3)
            .set_do_count(false);
        assert_eq!(page.total, 20);
        assert_eq!(page.page_size, 4);
        assert_eq!(page.page_no, 3);
        assert!(!page.do_count);
    }

    #[test]
    fn test_page_trait_getters_and_setters() {
        // IPageRequest for Page (page.rs:389-421)
        let mut page = Page::new(2, 5, 7, vec![1, 2]);
        assert_eq!(IPageRequest::page_size(&page), 5);
        assert_eq!(IPageRequest::page_no(&page), 2);
        assert_eq!(IPageRequest::total(&page), 7);
        assert!(IPageRequest::do_count(&page));

        IPageRequest::set_total(&mut page, 50);
        IPageRequest::set_page_size(&mut page, 10);
        IPageRequest::set_page_no(&mut page, 4);
        IPageRequest::set_do_count(&mut page, false);
        assert_eq!(IPageRequest::total(&page), 50);
        assert_eq!(IPageRequest::page_size(&page), 10);
        assert_eq!(IPageRequest::page_no(&page), 4);
        assert!(!IPageRequest::do_count(&page));

        // pages()/offset()/offset_limit() inherit the default impls
        assert_eq!(page.pages(), 5);
        assert_eq!(page.offset(), 30);
        assert_eq!(page.offset_limit(), 40);
    }

    #[test]
    fn test_page_records_accessors() {
        // IPage for Page (page.rs:423-445)
        let mut page = Page::new(1, 10, 2, vec![1, 2]);
        assert_eq!(IPage::records(&page), &vec![1, 2]);
        IPage::records_mut(&mut page).push(3);
        assert_eq!(page.records, vec![1, 2, 3]);

        let taken = IPage::records_take(&mut page);
        assert_eq!(taken, vec![1, 2, 3]); // order is preserved
        assert!(page.records.is_empty());

        // taking from an empty page
        let empty_taken = IPage::records_take(&mut page);
        assert!(empty_taken.is_empty());

        // trait `set_records` (lines 432-434)
        let page = <Page<i32> as IPage<i32>>::set_records(page, vec![9, 8]);
        assert_eq!(IPage::records(&page), &vec![9, 8]);
    }

    #[test]
    fn test_page_make_pages_ranges_requests_and_from() {
        // make_pages: non-multiple, zero page_size fallback, empty input
        let pages = Page::make_pages((0..25).collect::<Vec<i32>>(), 10);
        assert_eq!(pages.len(), 3);
        assert_eq!(pages[0].records.len(), 10);
        assert_eq!(pages[1].records.len(), 10);
        assert_eq!(pages[2].records, vec![20, 21, 22, 23, 24]);

        let pages = Page::make_pages(vec![1, 2, 3], 0);
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].page_size, 10);
        assert_eq!(pages[0].records, vec![1, 2, 3]);

        let pages = Page::<i32>::make_pages(Vec::new(), 10);
        assert!(pages.is_empty());

        // make_ranges / make_page_requests
        assert_eq!(
            Page::<i32>::make_ranges(25, 10),
            vec![(0, 10), (10, 20), (20, 25)]
        );
        assert!(Page::<i32>::make_ranges(0, 10).is_empty());
        let requests = Page::<i32>::make_page_requests(25, 10);
        assert_eq!(requests.len(), 3);
        assert_eq!(requests[2].page_no, 3);

        // conversion between Page<T> and Page<V>
        let page = Page::new(2, 5, 6, vec![1i32, 2, 3]);
        let converted: Page<i64> = Page::from(page);
        assert_eq!(converted.records, vec![1i64, 2, 3]);
        assert_eq!(converted.page_no, 2);
        assert_eq!(converted.page_size, 5);
        assert_eq!(converted.total, 6);

        // Display
        let display = format!("{}", Page::new(1, 10, 1, vec![1i32]));
        assert!(display.contains("Page"));
    }

    // ------------------------------------------------------------------
    // LogInterceptor
    // ------------------------------------------------------------------

    #[test]
    fn test_log_interceptor_level_filters() {
        for (filter, expected, level) in [
            (LevelFilter::Off, LevelFilter::Off, None),
            (LevelFilter::Error, LevelFilter::Error, Some(Level::Error)),
            (LevelFilter::Warn, LevelFilter::Warn, Some(Level::Warn)),
            (LevelFilter::Info, LevelFilter::Info, Some(Level::Info)),
            (LevelFilter::Debug, LevelFilter::Debug, Some(Level::Debug)),
            (LevelFilter::Trace, LevelFilter::Trace, Some(Level::Trace)),
        ] {
            let interceptor = LogInterceptor::new(filter);
            assert_eq!(interceptor.get_level_filter(), expected);
            assert_eq!(interceptor.to_level(), level);
        }
    }

    #[test]
    fn test_log_interceptor_out_of_range_filter_and_clone_debug() {
        let interceptor = LogInterceptor::new(LevelFilter::Off);
        // out-of-range atomic value falls back to Off (intercept_log.rs:90)
        interceptor.level_filter.store(99, Ordering::SeqCst);
        assert_eq!(interceptor.get_level_filter(), LevelFilter::Off);
        assert_eq!(interceptor.to_level(), None);

        let interceptor = LogInterceptor::new(LevelFilter::Trace);
        let clone = interceptor.clone();
        assert_eq!(clone.get_level_filter(), LevelFilter::Trace);
        assert!(format!("{:?}", interceptor).contains("LogInterceptor"));
    }

    #[tokio::test]
    async fn test_log_interceptor_before_after_logging_paths() {
        let rb = RBatis::new();
        let interceptor = LogInterceptor::new(LevelFilter::Debug);
        let mut sql = "select * from t".to_string();

        // before with empty args and with several args (RbsValueDisplay)
        let mut args: Vec<Value> = Vec::new();
        let mut result = exec_ok(0);
        assert_eq!(
            interceptor
                .before(1, &rb, &mut sql, &mut args, ResultType::Exec(&mut result))
                .await
                .unwrap(),
            Action::Next
        );
        let mut args = vec![
            Value::I32(1),
            Value::String("a".to_string()),
            Value::Null,
        ];
        let mut query_result: Result<Value, Error> = Ok(Value::Null);
        assert_eq!(
            interceptor
                .before(
                    2,
                    &rb,
                    &mut sql,
                    &mut args,
                    ResultType::Query(&mut query_result)
                )
                .await
                .unwrap(),
            Action::Next
        );

        // after: Exec Ok, Exec Err, Query Ok, Query Err
        let mut exec_result = exec_ok(3);
        assert_eq!(
            interceptor
                .after(1, &rb, &mut sql, &mut args, ResultType::Exec(&mut exec_result))
                .await
                .unwrap(),
            Action::Next
        );
        let mut exec_err: Result<ExecResult, Error> = Err(Error::from("boom"));
        assert_eq!(
            interceptor
                .after(1, &rb, &mut sql, &mut args, ResultType::Exec(&mut exec_err))
                .await
                .unwrap(),
            Action::Next
        );
        let mut query_ok: Result<Value, Error> = Ok(Value::Array(vec![Value::I32(1)]));
        assert_eq!(
            interceptor
                .after(1, &rb, &mut sql, &mut args, ResultType::Query(&mut query_ok))
                .await
                .unwrap(),
            Action::Next
        );
        let mut query_err: Result<Value, Error> = Err(Error::from("boom"));
        assert_eq!(
            interceptor
                .after(1, &rb, &mut sql, &mut args, ResultType::Query(&mut query_err))
                .await
                .unwrap(),
            Action::Next
        );
    }

    #[tokio::test]
    async fn test_log_interceptor_off_short_circuits() {
        let rb = RBatis::new();
        let interceptor = LogInterceptor::new(LevelFilter::Off);
        let mut sql = "select 1".to_string();
        let mut args = vec![Value::I32(1)];
        let mut result = exec_ok(1);
        assert_eq!(
            interceptor
                .before(0, &rb, &mut sql, &mut args, ResultType::Exec(&mut result))
                .await
                .unwrap(),
            Action::Next
        );
        assert_eq!(
            interceptor
                .after(0, &rb, &mut sql, &mut args, ResultType::Exec(&mut result))
                .await
                .unwrap(),
            Action::Next
        );
    }

    // ------------------------------------------------------------------
    // PageIntercept
    // ------------------------------------------------------------------

    #[test]
    fn test_page_intercept_new_and_default() {
        let interceptor = PageIntercept::default();
        assert!(!interceptor.select_ids.contains_key(&12345));
        assert!(!interceptor.count_ids.contains_key(&12345));
        assert_eq!(
            interceptor.count_param_count("sqlite", "select ? from t where a = ? limit ?"),
            3
        );
        let _ = PageIntercept::new();
    }

    #[tokio::test]
    async fn test_page_intercept_before_paths() {
        let rb = RBatis::new();
        let interceptor = PageIntercept::new();
        let mut sql = "select * from t".to_string();

        // ResultType::Exec takes the early exit.
        let mut args = vec![Value::I32(1)];
        let mut exec_result = exec_ok(0);
        assert_eq!(
            interceptor
                .before(
                    0,
                    &rb,
                    &mut sql,
                    &mut args,
                    ResultType::Exec(&mut exec_result)
                )
                .await
                .unwrap(),
            Action::Next
        );

        // Query without a registered request: sql untouched.
        let mut query_result: Result<Value, Error> = Ok(Value::Null);
        assert_eq!(
            interceptor
                .before(
                    0,
                    &rb,
                    &mut sql,
                    &mut args,
                    ResultType::Query(&mut query_result)
                )
                .await
                .unwrap(),
            Action::Next
        );
        assert_eq!(sql, "select * from t");

        // Registered select request: limit/offset gets appended. `rb` is not
        // linked, so the driver falls back to the default empty name.
        interceptor
            .select_ids
            .insert(rb.id(), PageRequest::new(2, 5));
        let mut sql_select = "select * from t".to_string();
        let mut query_result: Result<Value, Error> = Ok(Value::Null);
        assert_eq!(
            interceptor
                .before(
                    0,
                    &rb,
                    &mut sql_select,
                    &mut args,
                    ResultType::Query(&mut query_result)
                )
                .await
                .unwrap(),
            Action::Next
        );
        assert!(sql_select.ends_with(" limit 5,5 "), "sql={}", sql_select);

        // Registered count request: rewritten to count(1), limit/order by and
        // their bound arguments removed.
        interceptor
            .count_ids
            .insert(rb.id(), PageRequest::new(1, 10));
        let mut sql_count =
            "select a, b from t where a = ? order by id desc limit ?".to_string();
        let mut count_args = vec![Value::I32(1), Value::I32(10)];
        let mut query_result: Result<Value, Error> = Ok(Value::Null);
        assert_eq!(
            interceptor
                .before(
                    0,
                    &rb,
                    &mut sql_count,
                    &mut count_args,
                    ResultType::Query(&mut query_result)
                )
                .await
                .unwrap(),
            Action::Next
        );
        assert_eq!(sql_count, "select count(1) as count from t where a = ?");
        assert_eq!(count_args, vec![Value::I32(1)]);

        // after is the trait default for PageIntercept.
        assert_eq!(
            interceptor
                .after(
                    0,
                    &rb,
                    &mut sql_count,
                    &mut count_args,
                    ResultType::Query(&mut query_result)
                )
                .await
                .unwrap(),
            Action::Next
        );
    }

    // ------------------------------------------------------------------
    // Intercept trait defaults + apply_before / apply_after
    // ------------------------------------------------------------------

    #[derive(Debug)]
    struct NextInterceptor;

    impl Intercept for NextInterceptor {}

    #[derive(Debug)]
    struct ReturnInterceptor;

    #[async_trait]
    impl Intercept for ReturnInterceptor {
        async fn before(
            &self,
            _task_id: i64,
            _rb: &dyn Executor,
            _sql: &mut String,
            _args: &mut Vec<Value>,
            _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Value, Error>>,
        ) -> Result<Action, Error> {
            Ok(Action::Return)
        }

        async fn after(
            &self,
            _task_id: i64,
            _rb: &dyn Executor,
            _sql: &mut String,
            _args: &mut Vec<Value>,
            _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Value, Error>>,
        ) -> Result<Action, Error> {
            Ok(Action::Return)
        }
    }

    #[tokio::test]
    async fn test_intercept_default_methods() {
        let rb = RBatis::new();
        let interceptor = NextInterceptor;
        let mut sql = "select 1".to_string();
        let mut args = vec![Value::I32(1)];
        let mut result = exec_ok(1);

        // default before/after (intercept/mod.rs:108, 123)
        assert_eq!(
            interceptor
                .before(0, &rb, &mut sql, &mut args, ResultType::Exec(&mut result))
                .await
                .unwrap(),
            Action::Next
        );
        assert_eq!(
            interceptor
                .after(0, &rb, &mut sql, &mut args, ResultType::Exec(&mut result))
                .await
                .unwrap(),
            Action::Next
        );
        assert!(interceptor.name().contains("NextInterceptor"));
    }

    #[tokio::test]
    async fn test_apply_before_and_after() {
        let rb = RBatis::new();
        let mut sql = "select 1".to_string();
        let mut args = vec![Value::I32(1)];

        // empty interceptor list
        let empty: SyncVec<Arc<dyn Intercept>> = SyncVec::new();
        let mut result = exec_ok(1);
        assert!(!apply_before(
            &empty,
            0,
            &rb,
            &mut sql,
            &mut args,
            ResultType::Exec(&mut result)
        )
        .await
        .unwrap());
        assert!(!apply_after(
            &empty,
            0,
            &rb,
            &mut sql,
            &mut args,
            ResultType::Exec(&mut result)
        )
        .await
        .unwrap());

        // Action::Next lets the loop finish
        let chain: SyncVec<Arc<dyn Intercept>> = SyncVec::new();
        chain.push(Arc::new(NextInterceptor));
        let mut query_result: Result<Value, Error> = Ok(Value::Null);
        assert!(!apply_before(
            &chain,
            0,
            &rb,
            &mut sql,
            &mut args,
            ResultType::Query(&mut query_result)
        )
        .await
        .unwrap());
        assert!(!apply_after(
            &chain,
            0,
            &rb,
            &mut sql,
            &mut args,
            ResultType::Query(&mut query_result)
        )
        .await
        .unwrap());

        // Action::Return stops the chain
        let returning: SyncVec<Arc<dyn Intercept>> = SyncVec::new();
        returning.push(Arc::new(ReturnInterceptor));
        let mut result = exec_ok(1);
        assert!(apply_before(
            &returning,
            0,
            &rb,
            &mut sql,
            &mut args,
            ResultType::Exec(&mut result)
        )
        .await
        .unwrap());
        let mut query_result: Result<Value, Error> = Ok(Value::Null);
        assert!(apply_after(
            &returning,
            0,
            &rb,
            &mut sql,
            &mut args,
            ResultType::Query(&mut query_result)
        )
        .await
        .unwrap());
    }

    // ------------------------------------------------------------------
    // table_util! macros: nested arms and btree
    // ------------------------------------------------------------------

    #[derive(Clone, Debug, Default, PartialEq)]
    struct Leaf {
        pub leaf: Option<String>,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct Mid {
        pub mid: Leaf,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct Top {
        pub id: Option<String>,
        pub inner: Mid,
    }

    #[test]
    fn test_field_name_and_key_nested_arms() {
        // field_name! 1/2/3 level arms (table_util.rs:186-205)
        assert_eq!(field_name!(Top.id), "id");
        assert_eq!(field_name!(Top.inner), "inner");
        assert_eq!(field_name!(Top.inner.mid), "mid");
        assert_eq!(field_name!(Top.inner.mid.leaf), "leaf");

        // field_key! 1/2/3 level arms (table_util.rs:219-238)
        assert_eq!(field_key!(Top::id), "id");
        assert_eq!(field_key!(Top::inner), "inner");
        assert_eq!(field_key!(Top::inner::mid), "mid");
        assert_eq!(field_key!(Top::inner::mid::leaf), "leaf");
    }

    #[test]
    fn test_table_field_btree_borrowed_and_owned() {
        let a = Top {
            id: Some("a".to_string()),
            inner: Mid {
                mid: Leaf {
                    leaf: Some("l".to_string()),
                },
            },
        };
        let b = Top {
            id: None,
            inner: Mid::default(),
        };

        // borrowed form: BTreeMap<String, &Top>
        let data = vec![a.clone(), b.clone()];
        let borrowed = table_field_btree!(&data, id);
        assert_eq!(borrowed.len(), 1);
        assert_eq!(borrowed["a"], &a);
        let nested = table_field_btree!(&data, inner.mid.leaf);
        assert_eq!(nested.len(), 1);
        assert_eq!(nested["l"], &a);

        // owned form: BTreeMap<String, Top>
        let owned = table_field_btree!(data, id);
        assert_eq!(owned.len(), 1);
        assert_eq!(owned["a"], a);
    }
}
