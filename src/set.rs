pub mod traits {
   pub trait ISet<T> {
       fn contains(&self, other: T) -> bool;
       fn intersection(&self, other: &dyn ISet<T>) -> &dyn ISet<T>;
       fn is_subset(&self, other: &dyn ISet<T>) -> bool;
       fn is_superset(&self, other: &dyn ISet<T>) -> bool;
   }
}