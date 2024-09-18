use std::str::FromStr;

use xml::{attribute::OwnedAttribute, reader::XmlEvent};

mod account;
mod split;
mod transaction;
mod gnucash;

// trait XmlElement {
//     const CHILD_PREFIX: Option<&'static str>;
//     const NAME: &'static str;
// }


trait FromString: FromStr {
    fn from_string(value: String) -> Result<Self, <Self as FromStr>::Err>;
}

impl<T> FromString for T 
where T: FromStr {
    fn from_string(value: String) -> Result<Self, <Self as FromStr>::Err> {
        T::from_str(&value)
    }
}

trait Update<T> {
    fn update<F>(&mut self, updater:F)
    where F: FnMut(&mut T);
}

impl<T> Update<T> for Option<T> {
    fn update<F>(&mut self, mut updater:F)
    where F: FnMut(&mut T) {
        if let Some(value) = self {
            updater(value)
        }
    }
}