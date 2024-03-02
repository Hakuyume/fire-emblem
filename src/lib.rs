pub trait Params {
    type Item;
    type Params<T>: Params<Item = T>;
    fn map<T, F>(self, f: F) -> Self::Params<T>
    where
        F: FnMut(Self::Item) -> T;
    fn zip<T>(self, other: Self::Params<T>) -> Self::Params<(Self::Item, T)>;
}

macro_rules! params {
    ($vis:vis, $name:ident, $($param:ident),*) => {
        #[derive(Clone, Copy, Debug)]
        $vis struct $name<T> { $($vis $param: T),* }

        impl<T> crate::Params for $name<T> {
            type Item = T;
            type Params<U> = $name<U>;

            fn map<U, F>(self, mut f: F) -> Self::Params<U>
            where
                F: FnMut(Self::Item) -> U
            {
                $name { $($param: f(self.$param)),* }
            }

            fn zip<U>(self, other: Self::Params<U>) -> Self::Params<(Self::Item, U)> {
                $name { $($param: (self.$param, other.$param)),* }
            }
        }
    };
}
