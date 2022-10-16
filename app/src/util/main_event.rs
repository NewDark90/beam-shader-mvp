/* 
pub trait MainEvent<T> {
    fn should_run(&self, params: T) -> bool;
    fn run(&self) -> ();
}
*/
pub type ShouldRunFn<T> = fn(T) -> bool;
pub type RunFn = fn() -> ();

pub struct MainEvent<T> {
    pub should_run: ShouldRunFn<T>,
    pub run: RunFn
}