
/// Returns the result of the given function and the time it took to execute it.
///
/// # Arguments
///
/// * `f` - The function to execute.
///
/// # Returns
///
/// * `(T, std::time::Duration)` - The result of the given function and the time it took to execute it.
///
/// # Examples
///
/// ```
/// use utils::time_it;
///
/// let (result, duration) = time_it(|| 1 + 1);
///
/// assert_eq!(result, 2);
/// ```
pub fn time_it<T, F: Fn() -> T>(f: F) -> (T, std::time::Duration) {
    let start = std::time::Instant::now();
    let result = f();

    (result, start.elapsed())
}
