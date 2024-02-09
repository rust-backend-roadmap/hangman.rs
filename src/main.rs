use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn main() -> Result<()> {
    output::writeln("dad")?;
    
    Ok(())
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
