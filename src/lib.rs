mod gba;

pub trait Stats {
    type Item;
    type Stats<T>: Stats<Item = T>;
    fn map<T, F>(self, f: F) -> Self::Stats<T>
    where
        F: FnMut(Self::Item) -> T;
    fn zip<T>(self, other: Self::Stats<T>) -> Self::Stats<(Self::Item, T)>;
}

macro_rules! stats {
    ($name:ident, $($param:ident: $comment:literal,)*) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $name<T> {
            $(#[doc = $comment] pub $param: T),*
        }

        impl<T> crate::Stats for $name<T> {
            type Item = T;
            type Stats<U> = $name<U>;

            fn map<U, F>(self, mut f: F) -> Self::Stats<U>
            where
                F: FnMut(Self::Item) -> U
            {
                $name { $($param: f(self.$param)),* }
            }

            fn zip<U>(self, other: Self::Stats<U>) -> Self::Stats<(Self::Item, U)> {
                $name { $($param: (self.$param, other.$param)),* }
            }
        }
    };
}
pub mod the_blazing_blade;
