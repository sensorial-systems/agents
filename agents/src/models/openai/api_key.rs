use std::io::{Write, BufRead};

#[derive(Clone, PartialEq)]
pub enum OpenAIKeySrc {
    DOTENV,
    SYSENV,
    USERIN,
    CUSTOM(String),
}

pub fn prompt_user_for_key() -> Result<String, InputErr> {
    print!("\x1b[0;1;35mPlease enter your OpenAI API key\x1b[0m: ");
    let _ = std::io::stdout().flush();
    let mut buf = String::new();
    let mut stdin: std::io::StdinLock = std::io::stdin().lock();
    let res = stdin.read_line(&mut buf)
        .or_else(|_| Err(InputErr{ source: InputError("Error reading input to buffer") }));
    match (res, buf.trim()) {
        (Err(e), _) => Err(e),
        (Ok(_), "") => Err(InputErr{ source: InputError("Empty string given") }),
        (Ok(_), _) => Ok(buf),
    }
}

pub fn handle_fallback<E: std::error::Error>(
    r: Result<String, E>, 
    fb: Option<OpenAIKeySrc>, 
    human_input: bool
) -> String {
    r.unwrap_or_else(|e| {
        eprintln!("\x1b[0;1;31mError:\x1b[0m {}", e);
        if let Some(key_src) = fb {
            match key_src {
                OpenAIKeySrc::DOTENV => {
                    handle_fallback(
                        dotenv::var("OPENAI_API_KEY"),
                        match human_input {
                            true => Some(OpenAIKeySrc::USERIN),
                            false => None,
                        },
                        false
                    )
                }
                OpenAIKeySrc::SYSENV => {
                    handle_fallback(
                        std::env::var("OPENAI_API_KEY"),
                        match human_input {
                            true => Some(OpenAIKeySrc::USERIN),
                            false => None,
                        },
                        false
                    )
                }
                OpenAIKeySrc::USERIN => {
                    handle_fallback(
                        prompt_user_for_key(),
                        match human_input {
                            true => Some(OpenAIKeySrc::USERIN),
                            false => None,
                        },
                        true    // INFINITELY GETS USERINPUT UNTIL THEY GIVE A STRING VALUE
                    )
                }
                OpenAIKeySrc::CUSTOM(s) => {
                    String::from(s)
                }
            }
        } else {
            eprintln!("No fallback option for getting OpenAI API Key given. Terminating run.");
            // TODO: change this panic to send a termination signal to tokio once clean termination is
            // implemented <https://tokio.rs/tokio/topics/shutdown>
            panic!("ERROR GETTING OPENAI_API_KEY"); 
        }
    })
}

#[derive(Debug)]
pub struct InputErr {
    source: InputError
}

impl std::fmt::Display for InputErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.source.fmt(f)
    }
}

impl std::error::Error for InputErr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(Debug)]
struct InputError(&'static str);

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Input Error: {}", self)
    }
}

impl std::error::Error for InputError {}
