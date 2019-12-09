use snafu::Snafu;

/// Error kinds for CLI issues.
#[derive(Debug, Snafu)]
pub(crate) enum Error {
    #[snafu(display(
        "Received an invalid choice: {}. Valid choices are: {}",
        bad_choice,
        vec_as_comma_delimited(valid_choices)
    ))]
    ChoiceError { bad_choice: String, valid_choices: Vec<String> },
}
