
use anyhow::Result;
use javy::{quickjs::JSValue, Runtime};

use crate::{APIConfig, JSApiSet};

#[link(wasm_import_module="dylibso_observe")]
extern "C" {
    //fn instrument_span_enter(ptr: u64, len: u64);
    fn instrument_span_record(ptr: u64, len: u64);
}

pub struct Observe;

impl JSApiSet for Observe {
    fn register(&self, runtime: &Runtime, _config: &APIConfig) -> Result<()> {
        let ctx = runtime.context();

        let global = ctx.global_object()?;

        // global.set_property(
        //     "__dylibso_observe_instrument_span_enter", 
        //     ctx.wrap_callback(|_ctx, _this, args| {
        //         let span = args.first().unwrap();

        //         let name = args.first().unwrap().to_string();
        //         let name_ptr = name.as_ptr() as *const i8;
        //         let name_len = name.len();
        //         unsafe { instrument_span_enter(name_ptr as u64, name_len as u64) };
        //         drop(name);
        //         Ok(JSValue::Null)
        //     })?,
        // )?;

        global.set_property(
            "__dylibso_observe_instrument_span_record", 
            ctx.wrap_callback(|_ctx, this, args| {
                let span = unsafe { args.first().unwrap().inner_value() };
                let this = unsafe { this.inner_value() };
                let span_ctx = span.get_property("spanContext")?.call(&this, &[])?;
                let trace_id = span_ctx.get_property("traceId")?;
                let trace_id = trace_id.as_str()?;
                let span_id = span_ctx.get_property("spanId")?;
                let span_id = span_id.as_str()?;
                let parent_span_id = span.get_property("parentSpanId")?;
                let parent_span_id = parent_span_id.as_str()?;

                let payload = format!("{{ \"traceId\": \"{trace_id}\", \"spanId\": \"{span_id}\", \"parentSpanId\": \"{parent_span_id}\" }}");

                let span_ptr = payload.as_ptr() as *const i8;
                let span_len = payload.len();

                unsafe { instrument_span_record(span_ptr as u64, span_len as u64) };
                drop(span);
                Ok(JSValue::Null)
            })?,
        )?;

        Ok(())
    }
}
