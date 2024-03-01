use std::{
    io::{stdin, stdout, Write},
    str::FromStr,
};

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn quit() -> Result<bool, std::io::Error> {
    return match get_input("play again? (Enter yes to play again)\n")?
        .to_lowercase()
        .as_str()
    {
        "y" | "yes" => Ok(false),
        _ => Ok(true),
    };
}

#[derive(Debug)]
pub enum CellAction {
    Reveal,
    Flag,
    Unflag,
    Cancel,
}
impl FromStr for CellAction {
    type Err = Box<dyn std::error::Error>;
    /// Defines how a CellAction is parsed from a string
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.to_lowercase().as_str() {
            "r" | "reveal" => Ok(CellAction::Reveal),
            "f" | "flag" => Ok(CellAction::Flag),
            "u" | "unflag" => Ok(CellAction::Unflag),
            "c" | "cancel" => Ok(CellAction::Cancel),
            invalid => Err(format!("{} is not a valid cell action.\n either use the first letter or type the whole action", invalid).into()),
        };
    }
}

/// <b> This function will call [get_input] forever until the [String] returned can be `parsed` into a `T`. </b>
/// # Generics
/// - `T`: This type must implement [FromStr] meaning it can be parsed.
///   - The [FromStr::Err] type associated with `T` must also implement the [std::fmt::Display] trait to be shown to the user
/// # Errors
/// - When [get_input] fails
pub fn get_parsed_input<T>(prompt: &str) -> Result<T, std::io::Error>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    loop {
        let input = get_input(prompt)?; // get input

        // attempt to parse input
        match input.parse() {
            Ok(parsed_input) => return Ok(parsed_input), // if the input could be parsed return it
            Err(parse_error) => println!("\nInvalid input\n{}\n", parse_error), // otherwise print an error and continue the loop
        }
    }
}

/// This function will display the `prompt` to the user using `standard output stream` ([std::io::Stdout]). <br>
/// A line of input is read from `standard input stream` ([std::io::Stdin]) and returned.
/// - Trailing and leading whitespace is trimmed from the input line.
/// # Errors
/// - If cannot write `prompt` to `standard output stream`
/// - If cannot flush `standard output stream`
/// - If cannot read input from `standard input stream`
pub fn get_input(prompt: &str) -> Result<String, std::io::Error> {
    // prompt the user
    stdout().write(prompt.as_bytes())?; // write the prompt to `stdout`
    stdout().flush()?; // flush the standard output stream (ensure all data reaches its destination ie the terminal)

    // read a line of input
    let mut input = String::new(); // create a [String] to hold user input
    stdin().read_line(&mut input)?; // read a line from `stdin` into `input`
    let input = input.trim().to_string(); // shadow `input` with a clone that doesn't include leading or trailing whitespace

    return Ok(input);
}
