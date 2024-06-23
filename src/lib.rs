pub mod lazy;

#[cfg(test)]
mod tests {
    use crate::lazy::Lazy;

    #[test]
    fn it_works() {
        //In this example, we create a person struct to use as our initialized state. By default, the uninitialized state is simply a u32.
        //The data will be entirely uninitialized and will be a value of 22 until we call the 'get' function on the lazy.
        //Once this occurs, the data will

        #[derive(Debug)]
        struct Person {
            name: String,
            age: u32
        }

        //Create the lazy person, with an uninitialized "age" of 22.
        let mut lazy_person: Lazy<u32, Person> = Lazy::Uninitialized(22);

        //Check to ensure that the lazy_person is an uninitialized value of 22.
        assert_eq!(&format!("{:?}", lazy_person), "Uninitialized(22)");

        //Get the lazy person, but before the get can occur, the 'initialization' step will have to occur, transforming the lazy_person into a Person, with the name of 'Preston' and an age of the uninitialized u32 "22".
        lazy_person.get(|age| {
            Person {
                name: "Preston".to_string(),
                age: *age,
            }
        });

        assert_eq!(&format!("{:?}", lazy_person), "Initialized(Person { name: \"Preston\", age: 22 })")
    }
}