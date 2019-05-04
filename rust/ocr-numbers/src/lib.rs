// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let n_cols = verify_columns(input)?;
    let n_rows = verify_rows(input)?;

    // Collect the input into rows of chars
    let chars_by_row = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut digits = vec![];

    // Iterate left to right at 4 rows x 3 column steps
    (0..n_rows).step_by(4).for_each(|row_idx: usize| {
        (0..n_cols).step_by(3).for_each(|col_idx: usize| {
            // Select the current digit segment and parse into `Digit`
            let digit_string = chars_by_row
                .iter()
                .skip(row_idx)
                .take(4)
                .map(|row: &Vec<char>| row.iter().skip(col_idx).take(3))
                .flat_map(|c| c)
                .collect::<String>();
            digits.push(Digit::from(digit_string.as_str()))
        });

        // Add comma only if there are more rows to come
        if row_idx + 4 < n_rows {
            digits.push(Digit::Comma)
        }
    });

    let mut final_reading = String::new();
    digits
        .into_iter()
        .for_each(|digit| final_reading.push_str(digit.to_string().as_str()));
    Ok(final_reading)
}

// Verify the input has columns in a multiple of 3, returning total columns
fn verify_columns(input: &str) -> Result<usize, Error> {
    let n_cols = match input.lines().map(|l| l.chars().count()).last() {
        Some(n) => n,
        None => return Err(Error::InvalidColumnCount(0)),
    };

    match n_cols % 3 {
        0 => Ok(n_cols),
        _ => Err(Error::InvalidColumnCount(n_cols)),
    }
}

// Verify the input has rows in a multiple of 4, returning total rows
fn verify_rows(input: &str) -> Result<usize, Error> {
    let n_rows = input.lines().count();
    match n_rows % 4 {
        0 => Ok(n_rows),
        _ => Err(Error::InvalidRowCount(n_rows)),
    }
}

#[derive(Debug)]
pub enum Digit {
    Comma,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Unknown,
}

impl<'a> From<&'a str> for Digit {
    fn from(input: &'a str) -> Self {
        if input == zero() {
            Digit::Zero
        } else if input == one() {
            Digit::One
        } else if input == two() {
            Digit::Two
        } else if input == three() {
            Digit::Three
        } else if input == four() {
            Digit::Four
        } else if input == five() {
            Digit::Five
        } else if input == six() {
            Digit::Six
        } else if input == seven() {
            Digit::Seven
        } else if input == eight() {
            Digit::Eight
        } else if input == nine() {
            Digit::Nine
        } else {
            Digit::Unknown
        }
    }
}
impl ToString for Digit {
    fn to_string(&self) -> String {
        let s = match self {
            Digit::Comma => ",",
            Digit::Zero => "0",
            Digit::One => "1",
            Digit::Two => "2",
            Digit::Three => "3",
            Digit::Four => "4",
            Digit::Five => "5",
            Digit::Six => "6",
            Digit::Seven => "7",
            Digit::Eight => "8",
            Digit::Nine => "9",
            Digit::Unknown => "?",
        };
        s.to_string()
    }
}

#[rustfmt::skip]
fn zero() -> String {
    " _ ".to_string() +
    "| |" +
    "|_|" +
    "   "
}

#[rustfmt::skip]
fn one() -> String {
    "   ".to_string() +
    "  |" +
    "  |" +
    "   "
}

#[rustfmt::skip]
fn two() -> String {
    " _ ".to_string() +
    " _|" +
    "|_ " +
    "   "
}

#[rustfmt::skip]
fn three() -> String {
    " _ ".to_string() +
    " _|" +
    " _|" +
    "   "
}

#[rustfmt::skip]
fn four() -> String {
    "   ".to_string() +
    "|_|" +
    "  |" +
    "   "
}

#[rustfmt::skip]
fn five() -> String {
    " _ ".to_string() +
    "|_ " +
    " _|" +
    "   "
}

#[rustfmt::skip]
fn six() -> String {
    " _ ".to_string() +
    "|_ " +
    "|_|" +
    "   "
}

#[rustfmt::skip]
fn seven() -> String {
        " _ ".to_string() +
        "  |" +
        "  |" +
        "   "
}

#[rustfmt::skip]
fn eight() -> String {
    " _ ".to_string() +
    "|_|" +
    "|_|" +
    "   "
}

#[rustfmt::skip]
fn nine() -> String {
    " _ ".to_string() +
    "|_|" +
    " _|" +
    "   "
}
