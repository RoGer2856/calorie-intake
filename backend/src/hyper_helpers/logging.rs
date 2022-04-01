pub trait LogError<T, E> {
    fn log_error(self, log_function: impl FnOnce() -> ()) -> Self;
}

impl<T, E> LogError<T, E> for Result<T, E> {
    fn log_error(self, log_function: impl FnOnce() -> ()) -> Self {
        if self.is_err() {
            log_function();
        }
        self
    }
}
