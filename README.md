Run it with `WASMTIME_BACKTRACE_DETAILS=1 RUST_BACKTRACE=full RUST_LOG=debug cargo run` to see a crash:

```
2025-09-24T16:00:45.372716Z  INFO src/main.rs:47: will call_async
too many write (1025 > 1024)failed to prepare env info

thread 'main' panicked at src/main.rs:49:43:
called `Result::unwrap()` on an `Err` value: error while executing at wasm backtrace:
    0: 0x2db10a - <unknown>!<wasm function 3988>
    1: 0x2d2031 - <unknown>!<wasm function 3975>
    2: 0x2e065d - <unknown>!<wasm function 4010>
    3: 0x2d40f4 - <unknown>!<wasm function 3976>
    4:   0xfc57 - <unknown>!<wasm function 59>

Caused by:
    Exited with i32 exit status 1

Stack backtrace:
   0: std::backtrace_rs::backtrace::libunwind::trace
             at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/../../backtrace/src/backtrace/libunwind.rs:117:9
   1: std::backtrace_rs::backtrace::trace_unsynchronized
             at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/../../backtrace/src/backtrace/mod.rs:66:14
   2: std::backtrace::Backtrace::create
             at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/backtrace.rs:331:13
   3: anyhow::error::<impl core::convert::From<E> for anyhow::Error>::from
             at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/backtrace.rs:27:14
   4: <T as core::convert::Into<U>>::into
             at /Users/tomaszklak/.rustup/toolchains/1.90.0-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/convert/mod.rs:784:9
   5: <wasmtime_wasi::p1::WasiP1Ctx as wasmtime_wasi::p1::wasi_snapshot_preview1::WasiSnapshotPreview1>::proc_exit
             at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasmtime-wasi-37.0.1/src/p1.rs:2561:39
   6: wasmtime_wasi::p1::wasi_snapshot_preview1::proc_exit::{{closure}}
             at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasmtime-wasi-37.0.1/src/p1.rs:803:1
   7: tracing::span::Span::in_scope
             at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-0.1.41/src/span.rs:1102:9
   8: wasmtime_wasi::p1::wasi_snapshot_preview1::proc_exit
             at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasmtime-wasi-37.0.1/src/p1.rs:803:1
   9: wasmtime_wasi::p1::wasi_snapshot_preview1::add_to_linker::{{closure}}
             at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasmtime-wasi-37.0.1/src/p1.rs:803:1
  10: <F as wasmtime::runtime::func::IntoFunc<T,(wasmtime::runtime::func::Caller<T>,A1),R>>::into_func::{{closure}}
             at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasmtime-37.0.1/src/runtime/func.rs:1913:21
  11: wasmtime::runtime::func::HostContext::array_call_trampoline::{{closure}}
             at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasmtime-37.0.1/src/runtime/func.rs:2373:25
  12: core::ops::function::FnOnce::call_once
             at /Users/tomaszklak/.rustup/toolchains/1.90.0-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:253:5
  13: wasmtime::runtime::func::Caller<T>::with
             at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasmtime-37.0.1/src/runtime/func.rs:2062:23
  14: wasmtime::runtime::func::HostContext::array_call_trampoline::{{closure}}
             at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasmtime-37.0.1/src/runtime/func.rs:2406:17
  15: wasmtime::runtime::vm::instance::Instance::enter_host_from_wasm::{{closure}}
             at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasmtime-37.0.1/src/runtime/vm/instance.rs:252:53
  16: wasmtime::runtime::vm::traphandlers::catch_unwind_and_record_trap::{{closure}}
             at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasmtime-37.0.1/src/runtime/vm/traphandlers.rs:136:62
  17: <core::result::Result<T,E> as wasmtime::runtime::vm::traphandlers::HostResult>::maybe_catch_unwind::{{closure}}
             at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasmtime-37.0.1/src/runtime/vm/traphandlers.rs:262:31
  18: <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at /Users/tomaszklak/.rustup/toolchains/1.90.0-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/panic/unwind_safe.rs:272:9
  19: std::panicking::catch_unwind::do_call
             at /Users/tomaszklak/.rustup/toolchains/1.90.0-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/panicking.rs:589:40
  20: ___rust_try
  21: std::panicking::catch_unwind
             at /Users/tomaszklak/.rustup/toolchains/1.90.0-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/panicking.rs:552:19
  22: std::panic::catch_unwind
             at /Users/tomaszklak/.rustup/toolchains/1.90.0-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/panic.rs:359:14
  23: <core::result::Result<T,E> as wasmtime::runtime::vm::traphandlers::HostResult>::maybe_catch_unwind
             at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasmtime-37.0.1/src/runtime/vm/traphandlers.rs:271:19
  24: wasmtime::runtime::vm::traphandlers::catch_unwind_and_record_trap
             at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasmtime-37.0.1/src/runtime/vm/traphandlers.rs:136:25
  25: wasmtime::runtime::vm::instance::Instance::enter_host_from_wasm
             at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasmtime-37.0.1/src/runtime/vm/instance.rs:252:9
  26: wasmtime::runtime::func::HostContext::array_call_trampoline
             at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasmtime-37.0.1/src/runtime/func.rs:2404:13
  27: <unknown>
stack backtrace:
   0:        0x101d87438 - std::backtrace_rs::backtrace::libunwind::trace::h72f4b72e0962905d
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/../../backtrace/src/backtrace/libunwind.rs:117:9
   1:        0x101d87438 - std::backtrace_rs::backtrace::trace_unsynchronized::hff394536698b6b10
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/../../backtrace/src/backtrace/mod.rs:66:14
   2:        0x101d87438 - std::sys::backtrace::_print_fmt::h64d1e3035850353e
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/sys/backtrace.rs:66:9
   3:        0x101d87438 - <std::sys::backtrace::BacktraceLock::print::DisplayBacktrace as core::fmt::Display>::fmt::hf35f9734f9a29483
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/sys/backtrace.rs:39:26
   4:        0x101da1f30 - core::fmt::rt::Argument::fmt::hedf6f2a66f855f69
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/core/src/fmt/rt.rs:173:76
   5:        0x101da1f30 - core::fmt::write::h60ec6633daab7b35
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/core/src/fmt/mod.rs:1468:25
   6:        0x101d8479c - std::io::default_write_fmt::h0e30d7b1295222cb
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/io/mod.rs:639:11
   7:        0x101d8479c - std::io::Write::write_fmt::hc29709fdab2e34e2
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/io/mod.rs:1954:13
   8:        0x101d872ec - std::sys::backtrace::BacktraceLock::print::hca95bffd78053951
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/sys/backtrace.rs:42:9
   9:        0x101d88694 - std::panicking::default_hook::{{closure}}::h357ed4fbef22679d
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/panicking.rs:300:27
  10:        0x101d884ec - std::panicking::default_hook::h0a4e133b151d5758
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/panicking.rs:327:9
  11:        0x101d89134 - std::panicking::rust_panic_with_hook::h557a23724a5de839
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/panicking.rs:833:13
  12:        0x101d88d50 - std::panicking::begin_panic_handler::{{closure}}::h269cace6208fef05
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/panicking.rs:706:13
  13:        0x101d878e8 - std::sys::backtrace::__rust_end_short_backtrace::h5be0da278f3aaec7
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/sys/backtrace.rs:174:18
  14:        0x101d88a2c - __rustc[de2ca18b4c54d5b8]::rust_begin_unwind
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/panicking.rs:697:5
  15:        0x101e036f0 - core::panicking::panic_fmt::h477ff48eff31ffa4
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/core/src/panicking.rs:75:14
  16:        0x101e03af0 - core::result::unwrap_failed::h61c3c2f1df5908ff
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/core/src/result.rs:1765:5
  17:        0x10052076c - core::result::Result<T,E>::unwrap::h8eaa5c7ad08982d5
                               at /Users/tomaszklak/.rustup/toolchains/1.90.0-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:1167:23
  18:        0x10052076c - quux::main::{{closure}}::h74f79053eada947f
                               at /Users/tomaszklak/Development/quux/src/main.rs:49:43
  19:        0x100522d44 - <core::pin::Pin<P> as core::future::future::Future>::poll::h310927759ed14341
                               at /Users/tomaszklak/.rustup/toolchains/1.90.0-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/future/future.rs:133:9
  20:        0x100562588 - tokio::runtime::park::CachedParkThread::block_on::{{closure}}::h40dfd37e507e4ad1
                               at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.47.1/src/runtime/park.rs:285:71
  21:        0x100562414 - tokio::task::coop::with_budget::hfb87006dc35ddec0
                               at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.47.1/src/task/coop/mod.rs:167:5
  22:        0x100562414 - tokio::task::coop::budget::hd29a1c46a3780d94
                               at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.47.1/src/task/coop/mod.rs:133:5
  23:        0x100562414 - tokio::runtime::park::CachedParkThread::block_on::hd86e5ba45dc5070c
                               at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.47.1/src/runtime/park.rs:285:31
  24:        0x100525108 - tokio::runtime::context::blocking::BlockingRegionGuard::block_on::h960e9634341f4f26
                               at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.47.1/src/runtime/context/blocking.rs:66:14
  25:        0x100504468 - tokio::runtime::scheduler::multi_thread::MultiThread::block_on::{{closure}}::he34466b6ae018f67
                               at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.47.1/src/runtime/scheduler/multi_thread/mod.rs:87:22
  26:        0x1005485fc - tokio::runtime::context::runtime::enter_runtime::h187781eaee0f0df7
                               at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.47.1/src/runtime/context/runtime.rs:65:16
  27:        0x1005043bc - tokio::runtime::scheduler::multi_thread::MultiThread::block_on::h73af85bd4b202e49
                               at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.47.1/src/runtime/scheduler/multi_thread/mod.rs:86:9
  28:        0x100531d18 - tokio::runtime::runtime::Runtime::block_on_inner::h02a64e164f161185
                               at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.47.1/src/runtime/runtime.rs:358:50
  29:        0x100532118 - tokio::runtime::runtime::Runtime::block_on::h8400cd55e40dde5e
                               at /Users/tomaszklak/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.47.1/src/runtime/runtime.rs:328:18
  30:        0x10052fb54 - quux::main::h9cd635fcc9efb1fa
                               at /Users/tomaszklak/Development/quux/src/main.rs:51:23
  31:        0x10054a0d0 - core::ops::function::FnOnce::call_once::h2e658e94fd0c88ee
                               at /Users/tomaszklak/.rustup/toolchains/1.90.0-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:253:5
  32:        0x10050e0b8 - std::sys::backtrace::__rust_begin_short_backtrace::h55955c61078f4244
                               at /Users/tomaszklak/.rustup/toolchains/1.90.0-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/sys/backtrace.rs:158:18
  33:        0x100573928 - std::rt::lang_start::{{closure}}::h2c98d85747536a5b
                               at /Users/tomaszklak/.rustup/toolchains/1.90.0-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/rt.rs:206:18
  34:        0x101d7e8b4 - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::hbb2eb0e6976088d9
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/core/src/ops/function.rs:290:21
  35:        0x101d7e8b4 - std::panicking::catch_unwind::do_call::h93858ce5ba09f3d9
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/panicking.rs:589:40
  36:        0x101d7e8b4 - std::panicking::catch_unwind::h129a241a010f1b76
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/panicking.rs:552:19
  37:        0x101d7e8b4 - std::panic::catch_unwind::h5ca6b885cfe10586
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/panic.rs:359:14
  38:        0x101d7e8b4 - std::rt::lang_start_internal::{{closure}}::hed6353a412388a00
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/rt.rs:175:24
  39:        0x101d7e8b4 - std::panicking::catch_unwind::do_call::h6579b7caa3691f01
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/panicking.rs:589:40
  40:        0x101d7e8b4 - std::panicking::catch_unwind::h4557f88752b89087
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/panicking.rs:552:19
  41:        0x101d7e8b4 - std::panic::catch_unwind::h82809ba82b8374af
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/panic.rs:359:14
  42:        0x101d7e8b4 - std::rt::lang_start_internal::hdb28e94b6865fa11
                               at /rustc/1159e78c4747b02ef996e55082b704c09b970588/library/std/src/rt.rs:171:5
  43:        0x100573900 - std::rt::lang_start::h279bd5eef04ef878
                               at /Users/tomaszklak/.rustup/toolchains/1.90.0-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/rt.rs:205:5
  44:        0x10052fbe4 - _main
```
