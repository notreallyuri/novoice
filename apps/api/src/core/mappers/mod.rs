pub trait FromDomain<T> {
    fn from_domain(value: T) -> Self;
}

pub trait IntoDomain<T> {
    fn into_domain(self) -> T;
}

impl<T, U> IntoDomain<U> for T
where
    U: FromDomain<T>,
{
    fn into_domain(self) -> U {
        U::from_domain(self)
    }
}

pub mod channels;
pub mod enums;
pub mod roles;
pub mod users;
