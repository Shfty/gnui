use std::{
    ffi::OsStr,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::Args;
use crossbeam_channel::Receiver;

pub type InputReceiver = Receiver<String>;

pub fn delimiter_from_str(delim: &str) -> std::result::Result<u8, &'static str> {
    match delim {
        "\\t" => Ok(b'\t'),
        "\\n" => Ok(b'\n'),
        "\\r" => Ok(b'\r'),
        "\\0" => Ok(b'\0'),
        "\\" => Ok(b'\\'),
        _ => delim.bytes().next().ok_or("Empty delimiter"),
    }
}

fn input_file_from_os_str(os_str: &OsStr) -> InputFile {
    InputFile(match os_str.to_str().unwrap() {
        "-" => None,
        i => Some(i.into()),
    })
}

#[derive(Debug)]
struct InputFile(Option<PathBuf>);

impl std::ops::Deref for InputFile {
    type Target = Option<PathBuf>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for InputFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Option<PathBuf>> for InputFile {
    fn from(i: Option<PathBuf>) -> Self {
        InputFile(i)
    }
}

impl From<InputFile> for Option<PathBuf> {
    fn from(i: InputFile) -> Self {
        i.0
    }
}

#[derive(Debug, Args)]
pub struct InputThread {
    /// Input file; if -, read from standard input
    #[clap(parse(from_os_str = input_file_from_os_str), default_value = "-")]
    input: InputFile,

    /// Input delimiter
    #[clap(short, long, parse(try_from_str = delimiter_from_str), default_value = "\0", help_heading = "OPTIONS-INPUT")]
    delimiter: u8,
}

impl InputThread {
    pub fn spawn(self) -> InputReceiver {
        let InputThread {
            input: file,
            delimiter,
        } = self;

        let (stdin_tx, stdin_rx) = crossbeam_channel::unbounded::<String>();

        std::thread::spawn(move || {
            let mut file = file
                .as_ref()
                .map(|path| std::fs::File::open(path).expect("Failed to open input file"))
                .map(BufReader::new);

            loop {
                let mut buf = vec![];

                let input = if let Some(file) = &mut file {
                    file.read_until(delimiter, &mut buf)
                } else {
                    std::io::stdin().lock().read_until(delimiter, &mut buf)
                };

                // Remove trailing delimiter
                buf.pop();

                match input {
                    Ok(count) => {
                        if count > 0 {
                            stdin_tx.send(String::from_utf8(buf).unwrap()).unwrap()
                        }
                    }
                    Err(e) => panic!("{e:}"),
                }
            }
        });

        stdin_rx
    }
}
