pub trait ResxPath: Sized {
    fn path(self) -> String;
    fn new(path: String) -> Self;
}
pub trait ResxRB: ResxPath {}
pub trait ResxInstanceRB: ResxPath {}