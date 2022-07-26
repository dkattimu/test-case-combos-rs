
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
mod element_set;
pub mod interval_set;
pub mod set;
pub mod sets {
    //use crate::utils::create_intervalset;



    pub mod interval_set{
        #[derive(Default, Debug, PartialEq)]
        pub struct IntervalSet0 {
            pub reprn: String,
            pub left_symb: String,
            pub right_symb: String,
            pub lb: f64,
            pub ub: f64,
        }
        #[derive(Default, Debug, PartialEq)]
        pub struct IntervalSet<'a> {
            pub reprn: &'a str,
            pub left_symb: char,
            pub right_symb: char,
            pub lb: f64,
            pub ub: f64,
        }

        /*impl PartialEq for IntervalSet{
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.left_symb == other.left_symb &&
            self.right_symb == other.right_symb &&
            self.lb == other.lb &&
            self.ub == other.ub
        }
        }*/

        impl IntervalSet0{
            pub fn create(& self) -> Self {
            /*let trimmed_reprn = self.reprn.trim();
            let left_symb_ = String::from(trimmed_reprn.chars().nth(0).unwrap());
            let right_symb_ = String::from(trimmed_reprn.chars().nth(trimmed_reprn.len() - 1).unwrap());
            
            let reprn = String::from(&self.reprn);
            IntervalSet {
                reprn: reprn,
                left_symb: left_symb_,
                right_symb: right_symb_,
                lb: 0.0,
                ub: 4.0,
            }*/
                parse_intervalset0(String::from(&self.reprn))

            }
        }

        pub fn parse_intervalset0(str_reprn:String) ->  IntervalSet0 {
            //let temp = "bobo";
            let trimmed_reprn = str_reprn.trim();
            let left_symb_ = trimmed_reprn.chars().nth(0).unwrap().to_string();
            let right_symb_ = String::from(trimmed_reprn.chars().nth(trimmed_reprn.len() - 1).unwrap());
            let parts: Vec<&str> = trimmed_reprn.split(',').collect();
            let (first_part, second_part) = (parts[0].trim(), parts[1].trim());
            let lb_strs:Vec<&str> = first_part.split(left_symb_.as_str()).collect();
            let ub_strs: Vec<&str> =second_part.split(right_symb_.as_str()).collect();
            
            let lb_ = lb_strs[1];
            let ub_ = ub_strs[0];
            //let kofi = "bobo"
            let lb__ = lb_.parse::<f64>().unwrap().clone();
            let ub__ = ub_.parse::<f64>().unwrap().clone();

            IntervalSet0 {
                reprn: str_reprn.to_string(), // else warning about ownership
                left_symb: left_symb_,
                right_symb: right_symb_,
                lb: lb__,
                ub: ub__,
            }
        }

        impl IntervalSet<'_>{
            pub fn create(& self) -> Self {

                parse_intervalset(&self.reprn)

            }

            /// new method that uses String to intantiate

            pub fn new<'str_reprn>(
                &self, 
                str_reprn: &'str_reprn String) -> IntervalSet<'str_reprn> {
                //parse_intervalset(str_reprn)
                let trimmed_reprn = str_reprn.trim();
                let left_symb_ = trimmed_reprn.chars().nth(0).unwrap();
                let right_symb_ = trimmed_reprn.chars().nth(trimmed_reprn.len() - 1).unwrap();
                let parts: Vec<&str> = trimmed_reprn.split(',').collect();
                let (first_part, second_part) = (parts[0].trim(), parts[1].trim());
                let lb_strs:Vec<&str> = first_part.split(left_symb_).collect();
                let ub_strs: Vec<&str> =second_part.split(right_symb_).collect();
                
                let lb_ = lb_strs[1];
                let ub_ = ub_strs[0];
                //let kofi = "bobo"
                let lb__ = lb_.parse::<f64>().unwrap().clone();
                let ub__ = ub_.parse::<f64>().unwrap().clone();

                IntervalSet {
                    reprn: str_reprn.as_str(), // else warning about ownership
                    left_symb: left_symb_,
                    right_symb: right_symb_,
                    lb: lb__,
                    ub: ub__,
            }
            }
        }

        pub fn parse_intervalset(str_reprn: &str) ->  IntervalSet {
            //let temp = "bobo";
            let trimmed_reprn = str_reprn.trim();
            let left_symb_ = trimmed_reprn.chars().nth(0).unwrap();
            let right_symb_ = trimmed_reprn.chars().nth(trimmed_reprn.len() - 1).unwrap();
            let parts: Vec<&str> = trimmed_reprn.split(',').collect();
            let (first_part, second_part) = (parts[0].trim(), parts[1].trim());
            let lb_strs:Vec<&str> = first_part.split(left_symb_).collect();
            let ub_strs: Vec<&str> =second_part.split(right_symb_).collect();
            
            let lb_ = lb_strs[1];
            let ub_ = ub_strs[0];
            //let kofi = "bobo"
            let lb__ = lb_.parse::<f64>().unwrap().clone();
            let ub__ = ub_.parse::<f64>().unwrap().clone();

            IntervalSet {
                reprn: str_reprn, // else warning about ownership
                left_symb: left_symb_,
                right_symb: right_symb_,
                lb: lb__,
                ub: ub__,
            }
        }

    } 

    #[derive(Debug, PartialEq)]
    pub struct ElementSet<T,U> {
        // want to support string and floats in element set
        pub elements_t: Vec<T>, 
        pub elements_u : Vec<U>
    }
}

pub mod utils {
    //pub use super::sets::IntervalSet;


}
/* 
#[cfg(test)]
mod interval_set_test {
    use super::sets::interval_set::{IntervalSet0, IntervalSet, parse_intervalset0, parse_intervalset};
    
    use parameterized::parameterized;
    #[parameterized(
        reprn ={
        ("(4,5)").to_string(),
        ("(4,5]").to_string(),
        ("[4,5)").to_string(),
        ("[4,5]").to_string()},
        expected = { 
        &IntervalSet0 {reprn: "(4,5)".to_string(), left_symb: String::from("("), right_symb : String::from(")"), lb:4.0, ub:5.0 },
        &IntervalSet0 {reprn: "(4,5]".to_string(), left_symb: String::from("("), right_symb : String::from("]"), lb:4.0, ub:5.0 },
        &IntervalSet0 {reprn: "[4,5)".to_string(), left_symb: String::from("["), right_symb : String::from(")"), lb:4.0, ub:5.0 },
        &IntervalSet0 {reprn: "[4,5]".to_string(), left_symb: String::from("["), right_symb : String::from("]"), lb:4.0, ub:5.0 }}
    )]
    fn test_intervalset0_create_impl(reprn :String, expected: &IntervalSet0) {
        let temp = IntervalSet0{reprn:String::from(reprn), ..Default::default()};
        let actual = &temp.create();
        assert_eq!(actual, expected)
    }
    //use super::utils{create_interval_set(str_reprn)};

    #[parameterized(str_reprn ={
        String::from("(4,5)"), String::from("(4,5]"), String::from("[4,5)"), String::from("[4,5]")},
     expected = { 
         IntervalSet0 {reprn: String::from("(4,5)"),left_symb: String::from("("), right_symb : String::from(")"), lb:4.0, ub:5.0 },
         IntervalSet0 {reprn: String::from("(4,5]"),left_symb: String::from("("), right_symb : String::from("]"), lb:4.0, ub:5.0 },
         IntervalSet0 {reprn: String::from("[4,5)"),left_symb: String::from("["), right_symb : String::from(")"), lb:4.0, ub:5.0 },
         IntervalSet0 {reprn: String::from("[4,5]"), left_symb: String::from("["), right_symb : String::from("]"), lb:4.0, ub:5.0 }
    })]
    fn test_creation_intervalset0(str_reprn: String, expected: IntervalSet0) {
        let actual = parse_intervalset0(str_reprn);
        
        assert_eq!(actual, expected)
    }
    #[parameterized(
        reprn ={ "(4,5)","(4,5]","[4,5)","[4,5]" },
        expected = {
            &IntervalSet {reprn: "(4,5)", left_symb: '(', right_symb : ')', lb:4.0, ub:5.0 },
            &IntervalSet {reprn: "(4,5]", left_symb: '(', right_symb : ']', lb:4.0, ub:5.0 },
            &IntervalSet {reprn: "[4,5)", left_symb: '[', right_symb : ')', lb:4.0, ub:5.0 },
            &IntervalSet {reprn: "[4,5]", left_symb: '[', right_symb : ']', lb:4.0, ub:5.0 } }
    )]
    fn test_intervalset_create_impl(reprn :&str, expected: &IntervalSet) {
        let temp = IntervalSet{reprn, ..Default::default()};
        let actual = &temp.create();
        assert_eq!(actual, expected)
    }
    //use super::utils{create_interval_set(str_reprn)};
    #[parameterized(
        str_reprn ={ "(4,5)", "(4,5]", " [4,5) ", "[4,5]"},
        expected = {
            IntervalSet {reprn: "(4,5)", left_symb: '(', right_symb : ')', lb:4.0, ub:5.0 },
            IntervalSet {reprn: "(4,5]", left_symb: '(', right_symb : ']', lb:4.0, ub:5.0 },
            IntervalSet {reprn: "[4,5)", left_symb: '[', right_symb : ')', lb:4.0, ub:5.0 },
            IntervalSet {reprn: "[4,5]", left_symb: '[', right_symb : ']', lb:4.0, ub:5.0 } }
)]
    fn test_creation_intervalset_(str_reprn: &str, expected: IntervalSet) {
        let actual = parse_intervalset(str_reprn);
        
        assert_eq!(actual, expected)
    }
}
*/