#![allow(unused_imports)]

#[cfg(not(any(feature = "runtime-tokio", feature = "runtime-async-std")))]
compile_error!("'runtime-async-std' or 'runtime-tokio' features which one of must be enabled");

#[cfg(all(feature = "runtime-tokio", feature = "runtime-async-std"))]
compile_error!("'runtime-async-std' or 'runtime-tokio' features which one of must be enabled");

#[cfg(feature = "runtime-async-std")]
pub(crate) use async_std::{
    fs,
    future::timeout,
    io::prelude::ReadExt as AsyncReadExt,
    io::{Read as AsyncRead, Write as AsyncWrite},
    net::TcpStream,
    task::sleep,
    task::spawn,
    sync::Mutex,
    sync::MutexGuard,

    sync::RwLock,
    sync::RwLockReadGuard,
    sync::RwLockWriteGuard
};

#[cfg(feature = "runtime-tokio")]
pub(crate) use tokio::{
    fs,
    io::{AsyncRead, AsyncReadExt, AsyncWrite},
    net::TcpStream,
    task::spawn,
    time::delay_for as sleep,
    time::timeout,
    sync::Mutex,
    sync::MutexGuard,
    sync::RwLock,
    sync::RwLockReadGuard,
    sync::RwLockWriteGuard
};
