use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn main() -> Result<()> {
    Ok(())
}

pub mod input {
    use std::{fmt::{Debug, Display}, io::BufRead};

    type Result<T> = std::result::Result<T, InputError>;

    pub fn readln() -> Result<String> {
        let stdin = std::io::stdin();
        let mut stream = stdin.lock();
        let mut buf = String::new();

        stream.read_line(&mut buf).map_err(|_| InputError {})?;

        Ok(buf.trim_end().into())
    }

    pub struct InputError;

    impl Debug for InputError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("InputError").finish()
        }
    }

    impl Display for InputError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Debug::fmt(self, f)
        }
    }
}

pub mod output {
    use std::{error::Error, fmt::{Debug, Display}, io::Write};

    type Result<T> = std::result::Result<T, OutputError>;

    pub fn writeln(data: &str) -> Result<()> {
        let stdout = std::io::stdout();
        let mut stream = stdout.lock();

        writeln!(stream, "{}", data).map_err(|_| OutputError {})
    }

    pub struct OutputError;

    impl Debug for OutputError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("OutputError").finish()
        }
    }

    impl Display for OutputError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Debug::fmt(self, f)
        }
    }

    impl Error for OutputError {}
}
