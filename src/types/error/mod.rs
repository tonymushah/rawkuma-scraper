fn unwrap_failed(msg: &str, error: &dyn core::fmt::Debug) -> ! {
    panic!("{msg}: {error:?}")
}
pub enum RawKumaResult<T>{
    Ok(T),
    ReqwestError(reqwest::Error),
    Io(std::io::Error),
    Other(String)
}

impl<T> RawKumaResult<T> {
    pub fn unwrap(self) -> T
    {
        match self {
            RawKumaResult::Ok(d) => d,
            RawKumaResult::ReqwestError(err) => unwrap_failed("called `RawKumaResult::unwrap()` on an `ReqwestError` value", &err),
            RawKumaResult::Other(err) => unwrap_failed("called `RawKumaResult::unwrap()` on an `Other` value", &err),
            RawKumaResult::Io(err) => unwrap_failed("called `RawKumaResult::unwrap()` on an `Io` value", &err)
        }
    }
}

#[macro_export]
macro_rules! handle_reqwest_error {
    ($to_use:expr) => {
        match $to_use{
            Ok(d) => d,
            Err(err) => return RawKumaResult::ReqwestError(err)
        }
    };
}

#[macro_export]
macro_rules! handle_selector_error {
    ($to_use:expr) => {
        match $to_use{
            Ok(d) => d,
            Err(err) => return RawKumaResult::Other(err.to_string())
        }
    };
}

#[macro_export]
macro_rules! handle_other_error {
    ($to_use:expr) => {
        match $to_use{
            Ok(d) => d,
            Err(err) => return RawKumaResult::Other(err.to_string())
        }
    };
}

#[macro_export]
macro_rules! handle_io_error {
    ($to_use:expr) => {
        match $to_use{
            Ok(d) => d,
            Err(err) => return RawKumaResult::Io(err)
        }
    };
}

#[macro_export]
macro_rules! handle_rawkuma_result {
    ($to_use:expr) => {
        match $to_use{
            RawKumaResult::Ok(d) => d,
            RawKumaResult::ReqwestError(err) => return RawKumaResult::ReqwestError(err),
            RawKumaResult::Other(err) => return RawKumaResult::Other(err),
            RawKumaResult::Io(err) => return RawKumaResult::Io(err)
        }
    };
}