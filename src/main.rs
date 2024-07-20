#![feature(test)]
extern crate test;

use datadog_trace_protobuf::pb;
use datadog_trace_utils::msgpack::decoder;
use std::collections::HashMap;

pub fn create_test_span(
    trace_id: u64,
    span_id: u64,
    parent_id: u64,
    start: i64,
    is_top_level: bool,
) -> pb::Span {
    let mut span = pb::Span {
        trace_id,
        span_id,
        service: "test-service".to_string(),
        name: "test_name".to_string(),
        resource: "test-resource".to_string(),
        parent_id,
        start,
        duration: 5,
        error: 0,
        meta: HashMap::from([
            ("service".to_string(), "test-service".to_string()),
            ("env".to_string(), "test-env".to_string()),
            (
                "runtime-id".to_string(),
                "test-runtime-id-value".to_string(),
            ),
        ]),
        metrics: HashMap::new(),
        r#type: "".to_string(),
        meta_struct: HashMap::new(),
        span_links: vec![],
    };
    if is_top_level {
        span.metrics.insert("_top_level".to_string(), 1.0);
        span.meta
            .insert("_dd.origin".to_string(), "cloudfunction".to_string());
        span.meta
            .insert("origin".to_string(), "cloudfunction".to_string());
        span.meta.insert(
            "functionname".to_string(),
            "dummy_function_name".to_string(),
        );
        span.r#type = "serverless".to_string();
    }
    span
}

fn create_trace() -> Vec<pb::Span> {
    vec![
        // create a root span with metrics
        create_test_span(1234, 12341, 0, 1, true),
        create_test_span(1234, 12342, 12341, 1, false),
        create_test_span(1234, 12343, 12342, 1, false),
    ]
}

pub fn main() {
    let dummy_trace = create_trace();
    let expected = vec![dummy_trace.clone()];
    let payload = rmp_serde::to_vec_named(&expected).unwrap();

    let _res = decoder::from_slice(&mut payload.as_ref()).unwrap();
    let _res: Vec<Vec<pb::Span>>  = rmp_serde::from_slice(&payload).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use datadog_trace_utils::msgpack::decoder;

    #[bench]
    fn bench_serde(b: &mut Bencher) {
        let dummy_trace = create_trace();
        let traces = vec![dummy_trace.clone(), dummy_trace.clone(), dummy_trace.clone()];
        let payload = rmp_serde::to_vec_named(&traces).unwrap();

        b.iter(|| {
            let _result = decoder::from_slice(&mut payload.as_ref()).unwrap();
        })
    }

    #[bench]
    fn bench_rmp(b: &mut Bencher) {
        let dummy_trace = create_trace();
        let traces = vec![dummy_trace.clone(), dummy_trace.clone(), dummy_trace.clone()];
        let payload = rmp_serde::to_vec_named(&traces).unwrap();
        b.iter(|| {
            let _res: Vec<Vec<pb::Span>>  = rmp_serde::from_slice(&payload).unwrap();
        })
    }
}
