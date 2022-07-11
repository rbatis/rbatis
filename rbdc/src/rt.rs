#[cfg(all(feature = "_tls-native-tls"))]
pub use native_tls;

//
// Actix *OR* Tokio
//
pub use tokio::{
    self, fs, io::AsyncRead, io::AsyncReadExt, io::AsyncWrite, io::AsyncWriteExt, io::ReadBuf,
    net::TcpStream, runtime::Handle, task::spawn, task::yield_now, time::sleep, time::timeout,
};

//unix
#[cfg(all(unix, any(feature = "_rt-tokio")))]
pub use tokio::net::UnixStream;

pub use tokio_runtime::{block_on, enter_runtime};

mod tokio_runtime {
    use once_cell::sync::Lazy;
    use tokio::runtime::{self, Runtime};

    // lazily initialize a global runtime once for multiple invocations of the macros
    static RUNTIME: Lazy<Runtime> = Lazy::new(|| {
        runtime::Builder::new_multi_thread()
            .enable_io()
            .enable_time()
            .build()
            .expect("failed to initialize Tokio runtime")
    });

    pub fn block_on<F: std::future::Future>(future: F) -> F::Output {
        RUNTIME.block_on(future)
    }

    pub fn enter_runtime<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let _rt = RUNTIME.enter();
        f()
    }
}

#[cfg(all(
    feature = "_tls-native-tls",
    any(feature = "_rt-tokio"),
    not(any(feature = "_tls-rustls")),
))]
pub use tokio_native_tls::{TlsConnector, TlsStream};

#[cfg(all(
    feature = "_tls-rustls",
    any(feature = "_rt-tokio"),
    not(any(feature = "_tls-native-tls")),
))]
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
