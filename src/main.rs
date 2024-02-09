use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn main() -> Result<()> {
    hangman::start()
}

pub mod hangman {
    use crate::{menu, output};

    pub fn start() -> crate::Result<()> {
        output::writeln("Welcome to hangman game!")?;
        output::writeln("Opening menu..")?;

        menu::open()
    }
}

pub mod menu {
    use crate::{input, output};

    const START: &str = "S";
    const EXIT: &str = "E";

    pub fn open() -> crate::Result<()> {
        let mut exit = false;

        while !exit {
            self::print_options()?;

            let option = input::readln()?;

            match option.as_str() {
                START => {
                    output::writeln("Starting new round..")?;
                },

                EXIT => {
                    output::writeln("Exiting from menu..")?;
                    exit = true;
                },

                _ => {}
            }
        }
        
        Ok(())
    }

    pub fn print_options() -> crate::Result<()> {
        output::write_empty()?;
        output::writeln("Menu options:")?;
        output::writeln(&format!("   [{}] - to start new round", START))?;
        output::writeln(&format!("   [{}] - to exit from menu", EXIT))?;

        Ok(())
    }
}

pub mod dictionary {
    use std::{error::Error, fmt::{Debug, Display}, fs::File, io::{BufRead, BufReader}};

    use rand::Rng;

    const DEFAULT: &str = "dictionaries/default.txt";

    type Result<T> = std::result::Result<T, DictionaryError>;

    pub fn next_word() -> Result<String> {
        let file = File::open(DEFAULT)
            .map_err(|_| DictionaryError::NotFound)?;

        let mut stream = BufReader::new(file);
        let mut buf = String::new();

        stream.read_line(&mut buf).map_err(|_| DictionaryError::ReadError)?;
        let words = buf.trim_end().parse::<i32>()
            .map_err(|_| DictionaryError::WrongFormat)?;

        let choosen = choose_number(words);
        for _ in 0..=choosen {
            buf.clear();
            stream.read_line(&mut buf).map_err(|_| DictionaryError::ReadError)?;
        }

        Ok(buf.trim_end().into())
    }

    fn choose_number(words: i32) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..words)
    }

    pub enum DictionaryError {
        NotFound,
        WrongFormat,
        ReadError
    }

    impl Debug for DictionaryError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::NotFound => write!(f, "DictionaryNotFound"),
                Self::WrongFormat => write!(f, "WrongDictionaryFormat"),
                Self::ReadError => write!(f, "ReadDictionaryError"),
            }
        }
    }

    impl Display for DictionaryError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Debug::fmt(self, f)
        }
    }

    impl Error for DictionaryError {}
}

pub mod input {
    use std::{error::Error, fmt::{Debug, Display}, io::BufRead};

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

    impl Error for InputError {}
}

pub mod output {
    use std::{error::Error, fmt::{Debug, Display}, io::Write};

    type Result<T> = std::result::Result<T, OutputError>;

    pub fn writeln(data: &str) -> Result<()> {
        let stdout = std::io::stdout();
        let mut stream = stdout.lock();

        writeln!(stream, "{}", data).map_err(|_| OutputError {})
    }

    pub fn write_empty() -> Result<()> {
        writeln("")
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
