use anyhow::Result;

pub type AppError<T = ()> = Result<T>;
