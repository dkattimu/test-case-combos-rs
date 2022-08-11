pub mod traits {
   pub trait ISet<T> {
       fn contains(&self, other: T) -> bool;
       fn intersection(&self, other: &dyn ISet<T>) -> &dyn ISet<T>;
       //fn is_subset(&self, other: &dyn ISet<T>) -> bool;
       fn is_superset(&self, other: &dyn ISet<T>) -> bool;

       fn get_lb(&self) -> Option<T>{ None }
       fn get_ub(&self) -> Option<T> { None}
       fn get_elements(&self) -> Option<Vec<T>>;

   }
}