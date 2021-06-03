// MIT/Apache2 License

pin_project_lite::pin_project! {
    /// Future that takes a `Result`, and then processes another future if the result is `Ok`.
    #[derive(Debug)]
    #[must_use = "futures do nothing unless you .poll or await them"]
    pub enum AndThen
}
