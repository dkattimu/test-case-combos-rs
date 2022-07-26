pub mod core {
   #[derive(Debug, PartialEq)]
   pub struct ElementSet<T,U> {
       // want to support string and floats in element set
       pub elements_t: Vec<T>, 
       pub elements_u : Vec<U>
   }
}