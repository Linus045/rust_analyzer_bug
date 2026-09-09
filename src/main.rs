//use std::sync::OnceLock;
//
//use tauri::{AppHandle, Manager, async_runtime::block_on};
//
//static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

//fn app_handle<'a>() -> &'a AppHandle {
//    APP_HANDLE.get().unwrap()
//}

use mdns_sd::ServiceDaemon;
use tauri::async_runtime::block_on;

pub struct Test {
    pub service_daemon: Option<ServiceDaemon>,
}

pub struct Service {
    pub test: Option<Test>,
}

//pub fn example1() {
//    let app = app_handle();
//
//    let discover_service: tauri::State<'_, tokio::sync::Mutex<Service>> = app.state();
//
//    block_on(async {
//        let discover_service = discover_service.lock().await;
//
//        let _ = discover_service.test;
//    });
//}

//////////////////////////////////////////////////////////
// Copies Tauri::State to create TestState
pub struct TestState<'r, T: Send + Sync + 'static>(&'r T);

impl<'r, T: Send + Sync + 'static> TestState<'r, T> {
    /// Retrieve a borrow to the underlying value with a lifetime of `'r`.
    /// Using this method is typically unnecessary as `TestState` implements
    /// [`std::ops::Deref`] with a [`std::ops::Deref::Target`] of `T`.
    #[inline(always)]
    pub fn inner(&self) -> &'r T {
        self.0
    }
}

// ERROR: Uncommenting this function breaks rust analyzer
impl<T: Send + Sync + 'static> std::ops::Deref for TestState<'_, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &'_ T {
        self.0
    }
}
//////////////////////////////////////////////////////////

pub fn example2(discover_service: &TestState<'_, tokio::sync::Mutex<Service>>) {
    block_on(async {
        // inner().lock() does not resolve - no info is shown by rust analyzer
        let discover_service = discover_service.inner().lock().await;

        // .test does also not resolve
        let _ = discover_service.test;
    });
}

pub fn example3(discover_service: &TestState<'_, tokio::sync::Mutex<Service>>) {
    block_on(async {
        // inner() does not resolve - no info is shown by rust analyzer
        let mutex: &tokio::sync::Mutex<Service> = discover_service.inner();

        // but here lock() can be resolved and info is shown by rust analyzer
        let discover_service = mutex.lock().await;

        let _ = discover_service.test;
    });
}
pub fn example4(discover_service: &TestState<'_, tokio::sync::Mutex<Service>>) {
    block_on(async {
        // accessing .0 directly seems to make rust_analyzer resolve the type correctly
        let discover_service = discover_service.0.lock().await;

        let _ = discover_service.test;
    });
}

fn main() {
    let mutex = tokio::sync::Mutex::new(Service { test: None });

    let discover_service = TestState(&mutex);

    example2(&discover_service);

    example3(&discover_service);
}
