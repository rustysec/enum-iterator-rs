#[macro_use]
extern crate enum_iterator;

#[derive(Debug, EnumNameIterator, EnumDefaultValueIterator)]
pub enum Foods {
    Pizza,
    Steak,
    Beer,
}

impl Default for Foods {
    fn default() -> Foods {
        Foods::Beer
    }
}

#[derive(Debug, EnumNameIterator, EnumDefaultValueIterator)]
pub enum Hobbies {
    Computers,
    Guitars,
    Drums(Option<String>),
    Food(Foods),
    Others(u32, u32),
}

fn main() {
    let default_value_iter = Hobbies::enum_default_value_iter();
    for i in Hobbies::enum_name_iter() {
        println!(
            "Default value for {} = {:?}",
            i,
            default_value_iter.default_from_name(&i)
        );
    }

    println!(
        "Missing: {:?}",
        Hobbies::enum_default_value_iter().default_from_name("NotHere")
    );
}
