type CustomErrorResult<T> = std::result::Result<T, CustomErrors>;

impl From<SecondCustomErrors> for CustomErrors {
    fn from(value: SecondCustomErrors) -> Self {
        CustomErrors::SecondCustomErrors(value)
    }
}

impl From<FirstCustomErrors> for CustomErrors {
    fn from(value: FirstCustomErrors) -> Self {
        CustomErrors::FirstCustomErrors(value)
    }
}

pub fn run_error(variant: u8) -> CustomErrorResult<usize> {
    let _ = first_fn(variant)?;
    let second = second_fn(variant)?;
    Ok(second)
}

#[derive(Debug, PartialEq)]
pub enum CustomErrors {
    FirstCustomErrors(FirstCustomErrors),
    SecondCustomErrors(SecondCustomErrors),
}

#[derive(Debug, PartialEq)]
pub enum FirstCustomErrors {
    First,
    Second,
}

pub fn first_fn(variant: u8) -> Result<usize, FirstCustomErrors> {
    if variant == 0 {
        Err(FirstCustomErrors::First)
    } else if variant == 1 {
        Err(FirstCustomErrors::Second)
    } else {
        Ok(0)
    }
}

#[derive(Debug, PartialEq)]
pub enum SecondCustomErrors {
    First,
    Second,
}

fn second_fn(variant: u8) -> Result<usize, SecondCustomErrors> {
    if variant == 0 {
        Err(SecondCustomErrors::First)
    } else if variant == 1 {
        Err(SecondCustomErrors::Second)
    } else {
        Ok(0)
    }
}
