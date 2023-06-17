#[cfg(all(feature = "tls-native-tls"))]
pub use native_tls;

//
// Actix *OR* Tokio
//
pub use tokio::{
    self, fs, io::AsyncRead, io::AsyncReadExt, io::AsyncWrite, io::AsyncWriteExt, io::ReadBuf,
    net::TcpStream, runtime::Handle, task::spawn, task::yield_now, time::sleep, time::timeout,
};

//unix
#[cfg(unix)]
pub use tokio::net::UnixStream;

pub use tokio_runtime::{block_on, enter_runtime};

mod tokio_runtime {
    use std::sync::OnceLock;
    use tokio::runtime::{self, Runtime};

    // lazily initialize a global runtime once for multiple invocations of the macros
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();

    pub fn block_on<F: std::future::Future>(future: F) -> F::Output {
        RUNTIME.get_or_init(|| init_tokio()).block_on(future)
    }

    pub fn enter_runtime<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let _rt = RUNTIME.get_or_init(|| init_tokio()).enter();
        f()
    }
    fn init_tokio() -> Runtime {
        runtime::Builder::new_multi_thread()
            .enable_io()
            .enable_time()
            .build()
            .expect("failed to initialize Tokio runtime")
    }
}

#[cfg(feature = "tls-native-tls")]
pub use tokio_native_tls::{TlsConnector, TlsStream};

#[cfg(feature = "tls-rustls")]
pub use tokio_rustls::{client::TlsStream, TlsConnector};

//
// tokio
//

#[macro_export]
macro_rules! blocking {
    ($($expr:tt)*) => {
        $crate::tokio::task::spawn_blocking(move || { $($expr)* })
            .await.expect("Blocking task failed to complete.")
    };
}

#[macro_export]
macro_rules! block_on {
    ($ex:expr) => {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on($ex);
    };
}
