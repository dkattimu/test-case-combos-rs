pub mod core {
    use crate::set::traits::ISet;
    use std::str::{FromStr};
    use std::fmt::Debug;
    #[derive(Default, Debug)]
    pub struct IntervalSet0 {
        pub reprn: String,
        pub left_symb: char,
        pub right_symb: char,
        pub lb: f64,
        pub ub: f64,
    }

    impl IntervalSet0 {
        pub fn create(&self) -> Self {
            parse_intervalset0(String::from(&self.reprn))
        }

        pub fn new(reprn: &String) -> IntervalSet0 {
            //parse_intervalset(str_reprn)
            let str_reprn = reprn.as_str();
            let trimmed_reprn = str_reprn.trim();
            let left_symb_ = trimmed_reprn.chars().nth(0).unwrap();
            let right_symb_ = trimmed_reprn.chars().nth(trimmed_reprn.len() - 1).unwrap();
            let parts: Vec<&str> = trimmed_reprn.split(',').collect();
            let (first_part, second_part) = (parts[0].trim(), parts[1].trim());
            let lb_strs: Vec<&str> = first_part.split(left_symb_).collect();
            let ub_strs: Vec<&str> = second_part.split(right_symb_).collect();

            let lb_ = lb_strs[1];
            let ub_ = ub_strs[0];
            //let kofi = "bobo"
            let lb__ = lb_.trim().parse::<f64>().unwrap().clone();
            let ub__ = ub_.trim().parse::<f64>().unwrap().clone();

            IntervalSet0 {
                reprn: str_reprn.to_string(), // else warning about ownership
                left_symb: left_symb_,
                right_symb: right_symb_,
                lb: lb__,
                ub: ub__,
            }
        }
    }

    impl PartialEq for IntervalSet0 {
        fn eq(&self, other: &Self) -> bool {
            self.left_symb == other.left_symb
                && self.right_symb == other.right_symb
                && self.lb == other.lb
                && self.ub == other.ub
        }
    }

    pub fn parse_intervalset0(str_reprn: String) -> IntervalSet0 {
        //let temp = "bobo";
        let trimmed_reprn = str_reprn.trim();
        let left_symb_ = trimmed_reprn.chars().nth(0).unwrap();
        let right_symb_ = trimmed_reprn.chars().nth(trimmed_reprn.len() - 1).unwrap();
        let parts: Vec<&str> = trimmed_reprn.split(',').collect();
        let (first_part, second_part) = (parts[0].trim(), parts[1].trim());
        let lb_strs: Vec<&str> = first_part.split(left_symb_).collect();
        let ub_strs: Vec<&str> = second_part.split(right_symb_).collect();

        let lb_ = lb_strs[1];
        let ub_ = ub_strs[0];
        //let kofi = "bobo"
        let lb__ = lb_.trim().parse::<f64>().unwrap().clone();
        let ub__ = ub_.trim().parse::<f64>().unwrap().clone();

        IntervalSet0 {
            reprn: str_reprn.trim().to_string(), // else warning about ownership
            left_symb: left_symb_,
            right_symb: right_symb_,
            lb: lb__,
            ub: ub__,
        }
    }

    // *************************** PROD VERSION ************************************
    #[derive(Default, Debug /*PartialEq */)]
    pub struct IntervalSet<'reprn, T: PartialEq + PartialOrd> /*where T: PartialEq */{
        pub reprn: &'reprn str,
        pub left_symb: char,
        pub right_symb: char,
        pub lb: T,
        pub ub: T,
    }

    impl<'reprn, T> PartialEq for IntervalSet<'reprn, T> where T: PartialEq + PartialOrd {
        fn eq(&self, other: &Self) -> bool {
            self.left_symb == other.left_symb
                && self.right_symb == other.right_symb
                && self.lb == other.lb
                && self.ub == other.ub
        }
    }

    impl<'reprn, T> Eq for IntervalSet<'reprn, T> where T: PartialEq + PartialOrd + FromStr{ }

    impl<'reprn, T> IntervalSet<'reprn, T> where 
        T: PartialEq + PartialOrd + FromStr + Clone,  <T as FromStr>::Err: Debug{
        pub fn create(&self) -> Self {
            parse_intervalset(&self.reprn)
        }

        /// new method that uses String to intantiate

        pub fn new<'str_reprn>(str_reprn: &'str_reprn str) -> IntervalSet<'str_reprn, T> where
        T: PartialEq + PartialOrd + Clone + FromStr, <T as FromStr>::Err: Debug {
            //parse_intervalset(str_reprn)
            let trimmed_reprn = str_reprn.trim();
            let left_symb_ = trimmed_reprn.chars().nth(0).unwrap();
            let right_symb_ = trimmed_reprn.chars().nth(trimmed_reprn.len() - 1).unwrap();
            let parts: Vec<&str> = trimmed_reprn.split(',').collect();
            let (first_part, second_part) = (parts[0].trim(), parts[1].trim());
            let lb_strs: Vec<&str> = first_part.split(left_symb_).collect();
            let ub_strs: Vec<&str> = second_part.split(right_symb_).collect();

            let lb_ = lb_strs[1];
            let ub_ = ub_strs[0];
            //let kofi = "bobo"
            let lb__ = lb_.trim().parse::<T>().unwrap().clone();
            let ub__ = ub_.trim().parse::<T>().unwrap().clone();

            IntervalSet::<T> {
                reprn: str_reprn, // else warning about ownership
                left_symb: left_symb_,
                right_symb: right_symb_,
                lb: lb__,
                ub: ub__,
            }
        }

        pub fn is_subset(&self, other: &IntervalSet::<T>) -> bool {
            let boundary_check = self.lb >= other.lb && self.ub <= other.ub;
            let check_lb = match (other.left_symb == '[', self.lb == other.lb) {
                (true, true) => true,
                (false, true) => {
                    if self.left_symb == '[' {
                        false // other.left_symb is '(')
                    } else {
                        true
                    }
                }
                /*
                (true, false) => true, // if boundary_check works, this should work if boundaries are different
                (false, false) => true  //dito  so we use construct below*/
                (_, _) => true,
            };
            let check_ub = match (other.right_symb == ']', self.ub == other.ub) {
                (true, true) => true,
                (false, true) => {
                    if self.left_symb == '[' {
                        false // other.left_symb is '(')
                    } else {
                        true
                    }
                }
                /*
                (true, false) => true, // if boundary_check works, this should work if boundaries are different
                (false, false) => true  //dito  so we use construct below*/
                (_, _) => true,
            };
            return boundary_check && check_lb && check_ub;
        }
    }

    impl<'reprn, T> ISet<T> for IntervalSet<'reprn, T> where
    T: PartialEq + PartialOrd + FromStr + Clone + Copy,  <T as FromStr>::Err: Debug
     {
        fn contains(&self, elt: T) -> bool {
            let in_closure = elt <= self.ub && elt >= self.ub;
            let check_lb = match elt == self.lb && self.left_symb == '(' {
                true => false,
                _ => true,
            };
            let check_ub = match elt == self.ub && self.right_symb == ')' {
                true => false,
                _ => true,
            };
            in_closure && check_lb && check_ub
        }

        fn get_lb(&self) -> Option<T> {
            Some(self.lb)
        }

        fn get_ub(&self) -> Option<T> {
            Some(self.ub)
        }

        fn intersection(&self, other: &dyn ISet<T>) -> &dyn ISet<T> {
            todo!()
        }

        fn is_superset(&self, other: &dyn ISet<T>) -> bool {
            todo!()
        }

        fn get_elements(&self) -> Option<Vec<T>>{
            None
        }
    }

    pub fn parse_intervalset<T>(str_reprn: &str) -> IntervalSet<T> where
    T: PartialEq + PartialOrd + FromStr + Clone,  <T as FromStr>::Err: Debug {
        //let temp = "bobo";
        let trimmed_reprn = str_reprn.trim();
        let left_symb_ = trimmed_reprn.chars().nth(0).unwrap();
        let right_symb_ = trimmed_reprn.chars().nth(trimmed_reprn.len() - 1).unwrap();
        let parts: Vec<&str> = trimmed_reprn.split(',').collect();
        let (first_part, second_part) = (parts[0].trim(), parts[1].trim());
        let lb_strs: Vec<&str> = first_part.split(left_symb_).collect();
        let ub_strs: Vec<&str> = second_part.split(right_symb_).collect();

        let lb_ = lb_strs[1];
        let ub_ = ub_strs[0];
        //let kofi = "bobo"
        let lb__ = lb_.trim().parse::<T>().unwrap().clone();
        let ub__ = ub_.trim().parse::<T>().unwrap().clone();

        IntervalSet {
            reprn: str_reprn.trim(), // else warning about ownership
            left_symb: left_symb_,
            right_symb: right_symb_,
            lb: lb__,
            ub: ub__,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::core::{parse_intervalset, parse_intervalset0, IntervalSet, IntervalSet0};

    use parameterized::parameterized;
    #[parameterized(
        reprn ={
        ("(4,5)").to_string(),
        ("(4, 5 ]").to_string(),
        ("[4,5) ").to_string(),
        ("[4,5]").to_string()},
        expected = {
            IntervalSet0 {reprn: "(4,5)".to_string(), left_symb: '(', right_symb : ')', lb:4.0, ub:5.0 },
            IntervalSet0 {reprn: "(4,5]".to_string(), left_symb: '(', right_symb : ']', lb:4.0, ub:5.0 },
            IntervalSet0 {reprn: "[4,5)".to_string(), left_symb: '[', right_symb : ')', lb:4.0, ub:5.0 },
            IntervalSet0 {reprn: "[4,5]".to_string(), left_symb: '[', right_symb : ']', lb:4.0, ub:5.0 }}
    )]
    fn test_intervalset0_create_impl(reprn: String, expected: IntervalSet0) {
        let temp = IntervalSet0 {
            reprn: String::from(reprn),
            ..Default::default()
        };
        let actual = temp.create();
        assert_eq!(actual, expected)
    }

    #[parameterized(
        reprn ={
        ("(4, 5)").to_string(),
        ("(4, 5 ]").to_string(),
        ("[4,5) ").to_string(),
        ("[4,5]").to_string()},
        expected = {
            IntervalSet0 {reprn: "(4,5)".to_string(), left_symb: '(', right_symb : ')', lb:4.0, ub:5.0 },
            IntervalSet0 {reprn: "(4,5]".to_string(), left_symb: '(', right_symb : ']', lb:4.0, ub:5.0 },
            IntervalSet0 {reprn: "[4,5)".to_string(), left_symb: '[', right_symb : ')', lb:4.0, ub:5.0 },
            IntervalSet0 {reprn: "[4,5]".to_string(), left_symb: '[', right_symb : ']', lb:4.0, ub:5.0 }
        }
    )]
    fn test_intervalset0_new_impl(reprn: String, expected: IntervalSet0) {
        //let temp = IntervalSet0{reprn:String::from(reprn), ..Default::default()};
        let actual = IntervalSet0::new(&reprn);
        assert_eq!(actual, expected)
    }
    //use super::utils{create_interval_set(str_reprn)};

    #[parameterized(str_reprn ={
        String::from("(4,5)"), String::from("(4,5]"), String::from("[4,5)"), String::from("[4,5]")},
     expected = {
        IntervalSet0 {reprn: "(4,5)".to_string(), left_symb: '(', right_symb : ')', lb:4.0, ub:5.0 },
        IntervalSet0 {reprn: "(4,5]".to_string(), left_symb: '(', right_symb : ']', lb:4.0, ub:5.0 },
        IntervalSet0 {reprn: "[4,5)".to_string(), left_symb: '[', right_symb : ')', lb:4.0, ub:5.0 },
        IntervalSet0 {reprn: "[4,5]".to_string(), left_symb: '[', right_symb : ']', lb:4.0, ub:5.0 }}
    )]
    fn test_creation_intervalset0(str_reprn: String, expected: IntervalSet0) {
        let actual = parse_intervalset0(str_reprn);

        assert_eq!(actual, expected)
    }
    #[parameterized(
        reprn = { "(4,5)","(4,5] ","[4,5)","[4,5]" },
        expected = {
            IntervalSet { reprn: "(4,5)", left_symb: '(', right_symb : ')', lb:4.0, ub:5.0 },
            IntervalSet { reprn: "(4,5]", left_symb: '(', right_symb : ']', lb:4.0, ub:5.0 },
            IntervalSet { reprn: "[4,5)", left_symb: '[', right_symb : ')', lb:4.0, ub:5.0 },
            IntervalSet { reprn: "[4,5]", left_symb: '[', right_symb : ']', lb:4.0, ub:5.0 } }
    )]
    fn test_intervalset_create_impl(reprn: &str, expected: IntervalSet<f64>) {
        let temp = IntervalSet {
            reprn,
            ..Default::default()
        };
        let actual = temp.create();
        assert_eq!(actual, expected)
    }

    #[parameterized(
      reprn = { "(4,5)","(4 , 5] ","[ 4,5)","[4 ,  5]" },
      expected = {
          IntervalSet { reprn: "(4,5)", left_symb: '(', right_symb : ')', lb:4.0, ub:5.0 },
          IntervalSet { reprn: "(4,5]", left_symb: '(', right_symb : ']', lb:4.0, ub:5.0 },
          IntervalSet { reprn: "[4,5)", left_symb: '[', right_symb : ')', lb:4.0, ub:5.0 },
          IntervalSet { reprn: "[4,5]", left_symb: '[', right_symb : ']', lb:4.0, ub:5.0 } }
    )]
    fn test_intervalset_new_impl(reprn: &str, expected: IntervalSet<f64>) {
        //let temp = IntervalSet{reprn, ..Default::default()};
        let actual = IntervalSet::new(reprn);
        assert_eq!(actual, expected)
    }

    #[parameterized(
        str_reprn ={ "(4,5)", "(4 ,5 ]", " [4 ,5) ", "[ 4,5]"},
        expected = {
            IntervalSet { reprn: "(4,5)", left_symb: '(', right_symb : ')', lb:4.0, ub:5.0 },
            IntervalSet { reprn: "(4,5]", left_symb: '(', right_symb : ']', lb:4.0, ub:5.0 },
            IntervalSet { reprn: "[4,5)", left_symb: '[', right_symb : ')', lb:4.0, ub:5.0 },
            IntervalSet { reprn: "[4,5]", left_symb: '[', right_symb : ']', lb:4.0, ub:5.0 } }
    )]
    fn test_creation_intervalset(str_reprn: &str, expected: IntervalSet<f64>) {
        let actual = parse_intervalset(str_reprn);

        assert_eq!(actual, expected)
    }
    #[parameterized(
        str_reprn1 = { &"(4,5)".to_string(), "(4 ,5 ]", " [4 ,5) ", "[ 1,2]", "(4,5)", "(4 ,5 ]", " [4 ,5) ", "[ 1,2]"},
        str_reprn2 = { "(4,5]", "[4 ,5 ]", " [2 ,5) ", "[ 4,5]" , "(4, 5)", "(4, 5)", "(4, 5)", "(-8,9)"},
        expected = { true, true, false, true, true, false, false, true}
    )]
    fn test_is_subset_impl(str_reprn1: &str, str_reprn2 : &str, expected: bool) {
        let inval_set1 =  IntervalSet::<f64>::new(str_reprn1);
        let inval_set2 = IntervalSet::<f64>::new(str_reprn2);
        let actual = inval_set1.is_subset(&inval_set2);

        assert_eq!(actual, expected)
    }
}
