mod sealed {
    pub trait Sealed {}
}

pub trait Mode: sealed::Sealed {}

pub struct Fixed;
impl sealed::Sealed for Fixed {}
impl Mode for Fixed {}

pub struct Variable;
impl sealed::Sealed for Variable {}
impl Mode for Variable {}
