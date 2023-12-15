use std::{ffi::OsStr, fs::OpenOptions};

pub fn backend_from_os_str(backend: &OsStr) -> std::io::Result<Backend> {
    Ok(match backend.to_str().unwrap() {
        "-" => Backend::Stdout(std::io::stdout()),
        _ => Backend::File(
            OpenOptions::new()
                .append(true)
                .create(false)
                .truncate(false)
                .open(backend)?,
        ),
    })
}

#[derive(Debug)]
pub enum Backend {
    Stdout(std::io::Stdout),
    File(std::fs::File),
}

impl From<std::io::Stdout> for Backend {
    fn from(stdout: std::io::Stdout) -> Self {
        Backend::Stdout(stdout)
    }
}

impl From<std::fs::File> for Backend {
    fn from(stdout: std::fs::File) -> Self {
        Backend::File(stdout)
    }
}

impl std::io::Write for Backend {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Backend::Stdout(backend) => backend.write(buf),
            Backend::File(backend) => backend.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Backend::Stdout(backend) => backend.flush(),
            Backend::File(backend) => backend.flush(),
        }
    }

    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
        match self {
            Backend::Stdout(backend) => backend.write_vectored(bufs),
            Backend::File(backend) => backend.write_vectored(bufs),
        }
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        match self {
            Backend::Stdout(backend) => backend.write_all(buf),
            Backend::File(backend) => backend.write_all(buf),
        }
    }

    fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> std::io::Result<()> {
        match self {
            Backend::Stdout(backend) => backend.write_fmt(fmt),
            Backend::File(backend) => backend.write_fmt(fmt),
        }
    }
}
