use errors::*;

pub trait ElementFormat: ::std::fmt::Write {
    fn new_line(&mut self) -> Result<()>;

    fn new_line_unless_empty(&mut self) -> Result<()>;

    fn indent(&mut self);

    fn unindent(&mut self);
}
