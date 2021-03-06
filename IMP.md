POINTS TO REMEMBER:


------------------------------------------------------------ BOROWING ------------------------------------------------------------


* We can have any number of immutable references to a varaible

* We can have at most one mutable refereence to a variable in a perticular scope

* We can't have a immutable as well as a mutable reference to a varaible in the same scope

* Rust automatically detects that the scopr of a mutable reference has ended when the reference is not longer used later
and thus is allows defining a mutable reference to the varaible since the immutable reference is no longer in scope

```
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
```

* Rust we not allow us to have dangling pointers to varaibles which have gone out of scope, the code wont compile.
A reference can only exists as long the varaible is there in memory, if the varaible goes out of scope and we try to
use the reference to it, rust will detect this at compile time.


------------------------------------------------------------ The Slice type ------------------------------------------------------------

* Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

* A string literal is nothing but a slice (a pointer) to a binary stored in memory,
A slice type contains the starting memory location of the slice, and its length
(This is another reason why a string literal is immutable since its a slice which is a immutable reference)

```
let s = "Hello, world!"; // The type of s here is &str (slice)
```

* A string literal is actual of type slice, eg: let s = "hello  world";

* When we use slices it means we are creating an immutable reference to our varaible (or a reference to a part of the varaible)
  This means we cant create a mutable reference (as we can't have a mutable and immutable reference togther in same scope)

```
  fn main() {
    let mut s = String::from("Hello world !");

    let sl = &s[1..2]; // Creates a immutable reference to s

    let x = & mut s; // BUG ! Creates a mutable reference to s

    println!("{}", sl); // Without this print statement the code would compile since then the immutable reference "sl" would go out of scope since its never used
}
```

* We also have general slice types like over arrays...

```
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

------------------------------------------------------------ STRUCTS ------------------------------------------------------------

* Defining and copying:

```
// define
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// use, if property and value is same use shorthand like this
User {
  email,
  username,
  active: true,
  sign_in_count: 1,
}

// shorthand for copying ans specifying only properties that have changed
User {
  email,
  username,
  active: true,
  sign_in_count: 1,
}
```

* Tuple Structs

```
fn main() {
    struct Color(i32, i32, i32);
    let black = Color(0, 2, 4);

    let Color(a, b, c) = black; // destructuring a tuple struct

    println!("{}", black.1); //2
    println!("{} {} {}", a, b, c); // 0 2 4
}
```

* printing structs

to print our struct must implement `std::fmt::Display` or `std::fmt::Debug`...

We can do that later by...

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// some code...

println!("{:?}", rect1); // { width: 30, height: 50 }

println!("{:#?}", rect1); // using {:#?} prints in separate lines
```

* Unit struct, we can define a struct without any properties like:

```
struct QuitMessage;
```

* Struct methods

We can define methods(not fucntions) tied to a struct, it will receive self (instance of the struct) as the first argument.
In the method we can receive self as `&self` or `& mut self` if we need to mutate it. We can also take owner shif of the struct
by simplying receiveing  only sellf instaed of its reference but this is not usually done.

Also, a struct may have multiple impl blocks, its perfectly valid.

Eg:

```
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

* Associated functions, inside impl blocks we can have methods which receive self as the first arg, also we can have a
  associated function, which are like normal fucntion (they don't receive self), often these are used as contructors, which give a instance of that struct.
  To call an associated function we use, struct::func().

  Associated functions let you namespace functionality that is particular to your struct without having an instance available.

  Eg:

```
  impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
  } // Can be called using Rectangle::square(3);
```

------------------------------------------------------------ ENUMS ------------------------------------------------------------

* You can define an enum in rust, and enum can also optioanlly have values associated with it (you need to specify the type of these values) when defining
  the enum.

Example:

```
enum Message {
    Quit,                       // has no data associated with it at all.
    Move { x: i32, y: i32 },    // includes an anonymous struct inside it
    Write(String),              // includes a single String
    ChangeColor(i32, i32, i32), // includes three i32 values
}

// Like Structs you can also have methods on Enum, which receive the enum instance as the first argument
impl Message {
    fn call(&self) {
        match self {
            Message::Write(value) => println!("value: {}", value), // Destructure the value from the enum and print it
            _ => println!("Something else"),
        }
    }
}

let m = Message::Write(String::from("hello")); // Create a new enum of of type `Message::Write` with value "hello"
m.call(); // invoke the call() method on the enum instance
```

* The Option<T> enum in rust is used to avoid problems with null values

Since Option<T> is very usefull its already present in scope and so are its variants: you can use Some and None, you don't need to get it.
You can directly without the Option:: prefix.

```
enum Option<T> { // <T> here is a generic term, that can be anything
    Some(T),
    None,
}

let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None;
```

If you have a need where a varaible might contain nothing/null at a perticular option you can use the Option<T> enum to wrap it.
Doing so, elimintaes errors which occur due to null values.
Bacause using Option<T> instead of T (where T can be any type), the compiler won’t let us use an Option<T> value as if it were DEFINITELY a valid value.

Thus, Everywhere that a value has a type that isn’t an Option<T>, you can safely assume that the value isn’t null.

For example, this code won’t compile because it’s trying to add an i8 to an Option<i8>

```
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y; // ERROR ! no implementation for `i8 + std::option::Option<i8>
```

If a varaible might contain None, using Options rust forces us to define what to do in case its null rather than crashing our code..

Eg:

```
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5); // Option::Some(5)
    let six = plus_one(five);
    let none = plus_one(None); // Now adding 1 to null doesn't lead to an error since we have handled that case
```

Rust will ensure that we have handled all possible cases in our match clause, for example if we excluded the `None` case in the above example rust would say...

ERROR!
`ensure that all possible cases are being handled, possibly by adding wildcards or more match arms`

So...

Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid. Especially in the case of Option<T>, when Rust prevents us from forgetting to explicitly handle the None case, it protects us from assuming that we have a value when we might have null, thus making error due to null impossible.



* if let

If you have a situation in which your program has logic that is too verbose to express using a match, you can use "if let".

"if let" is like syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
You can add a else block to handle all other cases for a "if let" statement, here the else blocks works like the "_" wildcard for match statements.

Eg:

```
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
```

Thus, When enum values have data inside them, you can use match or if let to extract and use those values, depending on how many cases you need to handle.


----------------------------------------------------------- PACKAGES, CRATES & MODULES -----------------------------------------------------------

(Idea on how to import code from different modules: https://learning-rust.github.io/docs/d3.modules.html)

(https://stackoverflow.com/questions/46829539/how-to-include-files-from-same-directory-in-a-module-using-cargo-rust)

* A package is one or more crates that provide a set of functionality. A package contains a Cargo.toml file that describes how to build those   crates.

* There can be 2 types of crates, library crates and binary crates. You can have max one library crate but any number of binary crates in a package

A binary crate is an executable project that has a main() method. A library crate is a group of components that can be reused in other projects.
Unlike a binary crate, a library crate does not have an entry point (main() method).

The cargo install command allows you to install and use binary crates locally.
Note that you can only install packages that have binary targets.
A binary target is the runnable program that is created if the crate has a src/main.rs file or another file specified as a binary,
as opposed to a library target that isn’t runnable on its own but is suitable for including within other programs.

All binaries installed with cargo install are stored in the installation root’s(path where RUST is installed) bin folder.

* By deafult when we create a new package using `cargo new package-name`, Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package.

* Likewise, Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root.

* We can create a new library named restaurant by running cargo new --lib restaurant, this will create a new library which has its own `src/lib.rs`
Now we might have a folder structure like:

├───resturant
│   └───src
|       |__ lib.rs
├───src
|   |__ main.rs
|
|__ Cargo.toml

* Modules let us organize code within a crate into groups for readability and easy reuse.

We define a module by starting with the mod, modules can contain other modules or functions, structs, enums, constants, traits, etc.

Eg:

```
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
    }
}

```

*  src/main.rs and src/lib.rs are called crate roots. The reason for their name is that the contents of either of these two files form a module named crate at the root of the crate’s module structure, known as the module tree. The entire module tree is rooted under the implicit module named crate

crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     └── serving
         ├── take_order
         ├── serve_order


* By default the code we write is implicitley inside the crate root module, everthing defined inside a module is private by default unless explicitly made public
using the `pub` keyword. Modules can be accessed via a abosulte (starting from the crate root module) or relative path.

You can access a module if its public or if its a sibling of the current module.

```
// `front_of_house` is not public
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // The front_of_house module isn’t public, but because the eat_at_restaurant function is defined in the same module(the implicit crate module) as front_of_house
    // (that is, eat_at_restaurant and front_of_house are siblings), we can refer to front_of_house from eat_at_restaurant.
    front_of_house::hosting::add_to_waitlist();
}
```

* We can construct relative paths that begin in the parent module by using super at the start of the path.

```
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // This is same as `crate::serve_order()` since the parent module of `back_of_house` is the implicit `crate` root module
    }

    fn cook_order() {}
}
```

* a struct inside a module is by default private and can be made public using the `pub` keyword.

HOWEVER, even if the struct is made public the properties of the struct remain private and have to be explicitly made public.

```
mod back_of_house {

    pub struct Breakfast {
        pub toast: String,      // This property is public
        seasonal_fruit: String, // This property is private
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast { // A public associated function which acts like a constructor to make "Breakfast" instances.
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

let mut meal = back_of_house::Breakfast::summer("Rye");

meal.toast = String::from("Wheat");                 // OK !

meal.seasonal_fruit = String::from("blueberries"); // ERROR ! cannot access private field `seasonal_fruit` of the `Breakfast` struct

```

* Unlike structs where we need to mae the invidual properties of the struct public for enums
 if we declare and enum as public it means all of its variants are then public.

 ```
 mod back_of_house {
    pub enum Appetizer {
        Soup,  // Here we don't need to explicitly declare `Soup` and `Salad` as public.
        Salad, // Since `Appetizer` is public we can directly use `back_of_house::Appetizer::Soup`
    }
}
```


* The `use` keyword is used to bring some other PUBLIC code into current scope, it can be thoughh of as an alias
and helps us to avoid writing completed path names again and again.

To import items relative to the current and parent modules, use the `self::` and `super::` prefixes, respectively.

The rules of visibility also applies qually when brining items into scope via "use"

```

use crate::front_of_house::hosting;
use self::front_of_house::hosting;  // From the current module bring "front_of_house::hosting" into scope
use std::collections::HashMap;


use std::fmt::Result;
use std::io::Result as IoResult; // import as to avoid name collisions

/*
Re-exporting Names with "pub use"

When we bring a name into scope with the use keyword, the name available in the new scope is private.
To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine pub and use.
*/

pub use crate::front_of_house::hosting;


use std::{cmp::Ordering, io}; // // Bring multiple items from same std module

/*
Intead of this....

use std::io;
use std::io::Write;

We can write...

*/
use std::io::{self, Write};


// To bring all public items defined in a path into scope, we can specify that path followed by *
use std::collections::*;

```


* Keeping modules in separarte files...

Using a semicolon after `mod filename;` rather than using a block tells Rust to load the contents of the
file.

Eg:

To include the code from hello.rs in your main.rs, use mod hello;. It gets expanded to the code that is in hello.rs.

use is just for alias, while mod pulls in the file. You would use use, for example, to be able to call the print_hello function without having to prefix with the namespace


----------------------------------------------------------- VECTORS, STRINGS and HASHMAPS -----------------------------------------------------------

* A vector stores homogenous values contigously in memeory, its stored in the heap so elements can be added or removed from it at runtime.
Vectors can only store values of the same type.

```
let v: Vec<i32> = Vec::new(); // creating a vector using Vec::new() requires a type annotaion

let v = vec![1, 2, 3]; // Here we don't need type annotation when using the vec! macro

let v = vec![]; // however this won't work, type annotations needed for `std::vec::Vec<T>`

------------------------------------------------------
// To add elements to a vector it must be mutable...
let mut v = Vec::new();
v.push(5);
v.push(6);
v.pop();

------------------------------------------------------
// Reading from a Vector, there are 2 ways...

let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2]; // Get the 3rd element, this works but the code will crash if we try to access a invalid or out of bound index

// using the get method we obtain a Option<T> so even if the index is invalid it returns None and therefore this is safe
match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
------------------------------------------------------

/*
You can't have a mutable reference to a vector item while inserting/deleting from the vector
adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space,
if there isn’t enough room to put all the elements next to each other where the vector currently is.
In that case, the reference to the first element would be pointing to deallocated memory.
The borrowing rules prevent programs from ending up in that situation.
*/

let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0]; // immutable borrow occurs here

v.push(6); // the push() needs a mutable borrow here, ERROR ! can't have both immutable and mutable borrow at same scope.

------------------------------------------------------

// Iterating on a vector

let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}

// Iterate over mutable references to each element in a mutable vector in order to make changes to all the elements.
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;      // To change the value that the mutable reference refers to, we
                   // have to use the dereference operator (*) to get to the value in i before we can use the += operator.
}

// Update a value at index

let mut v = vec![1, 2, 3, 4, 5, 6];
let got = std::mem::replace(&mut v[3], 42); // [1, 2, 3, 42, 5, 6]

------------------------------------------------------
// You can store different type of values ina vector using a Enum

enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];

```

* Strings in rust are not so simple !


In rust Strings are UTF-8 encoded text. So this works: `String::from("नमस्ते");`
They are actaully a wrapper over a Vec<u8>.

Many of the same operations available with Vec<T> are available with String as well, like the new function to create a string.

```

// Creating a new string
let mut s = String::new();

// Get a string from a string literal(&str) or some preexisting data
// to_string() works on anything that implements the Display trait.
let s = "initial contents".to_string();

// Also, we can do...
let s = String::from("initial contents"); // this has exactly same outcome as "initial contents".to_string()

-------------------------------------
// Appending

let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2);

// Here the push-str method takes a slice(&str) so the ownership os s2 is not lost

// To push a single characterr we can use...
let mut s = String::from("lo");
s.push('l');
--------------------------------------

// Concating using `+` operator

let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;

/*

The `+` operator uses the `fn add(self, s: &str) -> String` method.
add() takes ownership of the first string, so in the example ownsership of `s1` is lost after `s1 + &s2`. But ownership for s2 is not lost since its borrowed.
Also, the 2nd parameter to add() is a slice(&str) but in the example we passed a reference to a string (&String).
Here rust uses automatic `deref coercoin` which turns &s2 into &s2[..] thus giving us a slice

*/
------------------------------

// Using format! macro, we get a string without losing ownership of any of the parameters

let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

* Indexing into string

In rust a string is a series of bytes. A ut8 chracter may take 1 byte, 2byte...

So index a string directly is not possible, although index via range is possible but if we end up at a invalid char boundary our code will panicand crash...
`thread 'main' panicked at 'byte index 1 is not a char boundary;`

If we do `len` on string it gives us a the number of bytes,so...

String::from("Hola"); // Length is 4 since each letter is one byte

String::from("Здравствуйте"); // Length is 24 although there are 12 chracters since each character here is 2 bytes

There are 3 ways to read a string, for a hindi string “नमस्ते” ...

- Reading each byte. (vector of u8) Eg: [224, 164, 168, ... 135] (18 bytes)

- Reading as unicode scalar values or as rusts `char` type.
  Eg: ['न', 'म', 'स', '्', 'त', 'े']

- Reading a grapheme clusters, Eg: ["न", "म", "स्", "ते"]

Also, index into string directly is again not possible since indexing should be a O(1) operation but for stings Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.


```
let s1 = String::from("hello");
let h = s1[0]; // ERROR ! `std::ops::Index<{integer}>` is not implemented for `std::string::String`


let hello = "Здравствуйте";
&hello[0..2]; // Reading first 2 bytes gives `З`
&hello[2..4]; // Reading 2nd 2 bytes gives `д`
&hello[4..5]; // thread 'main' panicked at 'byte index 5 is not a char boundary;


// Iterating on string...

for b in "नमस्ते".bytes() {
    println!("{}", b);  // [224, 164, 168, ... 135]
}

for c in "नमस्ते".chars() {
    println!("{}", c);   // ['न', 'म', 'स', '्', 'त', 'े']
}
```
Getting grapheme clusters from strings is complex, so this functionality is not provided by the standard library.
Use this, to get this functionality: https://crates.io/crates/unicode-segmentation


* HashMap<K, V>

A hash map stores key value pairs, where both the keys and values must be of homogenous types(all of the keys must have the same type, and all of the values must have the same type).

Just like vectors, hash maps store their data on the heap.

By default, a hashmap uses a “cryptographically strong” hash function, which provides security but might be slow for some applications, you can provide your own hash function(A hasher is a type that implements the BuildHasher trait.), many implementations are available on crates.io

Unlike strings and vectors a Hash map not automatically available in scope we need to import it like `use std::collections::HashMap`

```
use std::collections::HashMap;

let mut scores = HashMap::new(); // There is no built-in macro, like for vectors `vec![1, 2, 3]`

// Here the type (keys of type String and values of type i32) is automatically understood by rust as we insert values

scores.insert(String::from("Blue"), 10); // Adding keys and values
scores.insert(String::from("Yellow"), 50);



// Create a hash map using iterators and the collect method on a vector of tuples


let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

// Here zip method to create a vector of tuples
// Rust can infer the types that the hash map contains based on the types of the data in the vectors.
let mut scores: HashMap<_, _> =
    teams.into_iter().zip(initial_scores.into_iter()).collect();
```

== IMPORTANT ==
HERE WE DID NOT HAVE TO SPECIFY THE TYPES <K,V> WHEN DEFINING THE HashMap LIKE
HashMap::new() since we added vavlues ot it later `scores.insert(String::from("Blue"), 10);` , so the compiler infered types that the HashMap has keys of type String and values of type i32.

----------

Ownership:

For datat types which implement the Copy trait like i32, values are copied into the hash map when inserting.

*IMPORTANT:*
For owned values like String, the values will be moved and the hash map will be the owner of those values

If we insert references to values into the hash map, the values won’t be moved into the hash map, however the values that the references point to must be valid for at least as long as the hash map is valid.

Accessing values:

We use the get() method that gives us a `Option<&V>` that is a Option enum
which wraps up a reference to the value, if the key does not exists it gives us `None`.

```
let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
let team_name = String::from("Blue");
let score = scores.get(&team_name);  // Gives a Some(&10)


// Iterating over key value paires in a hash using for loop

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

Updating Values:

By default inserting a value for a key that already exists will overwrtie its value.

ENTRY:

Hash maps have a special API for this called entry that takes the key you want to check as a parameter. The return value of the entry method is an enum called Entry that represents a value that might or might not exist.

Example...

```
scores.entry(String::from("Yellow")).or_insert(50);
```

`scores.entry` gives us a `Entry`, the `or_insert` method on Entry gives us a mutable reference for the corresponding value if the key exists or if there is no value then it inserts a value and then returns a mutable reference to it.

Since or_insert gives us a mutable reference we can use it to update the value inside the hash, this is usefull if we want to updating a Value Based on the Old Value...

```
// Count occurance of each word in a sentence

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1; // in order to assign to that value, we must first dereference count using the asterisk (*)
}
```

----------------------------------------------------------- ERROR HANDLING -----------------------------------------------------------

* The panic! macro makes our code panic, When the panic! macro executes, your program will print a failure message, unwind and clean up the stack, and then quit.

  By default on panic our code will unwind which means Rust walks back up the stack and cleans up the data from each function it encounters, but this might be slow,
  alternative is to immediately abort, which ends the program without cleaning up.
  Memory that the program was using will then need to be cleaned up by the operating system.

  You can change the behaviour on panic, like in order to `abort` instead of unwinding in release mode, add a profile for release mode in 'Cargo.toml' file like..
  (aborting instead of unwinding can make our release binary smaller)

  ```
  [profile.release]
  panic = 'abort'
  ```

We can call panic like...

```
fn main() {
    panic!("crash and burn"); // thread 'main' panicked at 'crash and burn', src/main.rs:2:5
}
```

When we try to do something illegal, like access a array index not available a rust core library method might call the panic! macro for us...
'thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2806:10'

We can see a complete backtrace when our code panics by running it like 'RUST_BACKTRACE=1 cargo run'

* Recovering from erros with Result<T, E>

The Result enum has two values, Ok(T) or Err(E) here T represents the type of the value that will be returned in a success case within the Ok variant,
and E represents the type of the error that will be returned in a failure case within the Err variant. Just like the Options enum the Result enum
and its variants are automatically brought into scope and we can use them directly.

Examples...

```

// Using match

use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}


-------------------------------------

// Create a File if it does not exists

// io::Error struct provided by the standard library, has a kind() method that returns an io::ErrorKind value.
// The enum io::ErrorKind has variants representing the different kinds of errors that might result from an io operation.
// The variant we want to use is ErrorKind::NotFound here...

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}


---------------------------------------
// Shortcuts...

use std::fs::File;

fn main() {

    // Unwraps and returns T from Result::Ok<T> that is the file handle in this case otherwise in case of Err it calls panic! macro
    let f = File::open("hello.txt").unwrap();

    // OR

    // Same as above but allows us to preovide a custom message incase of panic
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

Propagating erros and `?` operator shortcut...
----------------------------------------------

Instead of calling panic! or handling the Result::Err we can return it to the caller in that case our fucntion must return a type like
`Result<String, io::Error>` and we can have `Err(e) => return Err(e)` in case of error.


* If the `?` operator is placed after a Result, and the value of the Result is an Ok, the value inside the Ok will get *returned from this expression*,and the program will continue.
Otherwise the Err will be *returned from the whole function* as if we had used the return keyword so the error value gets propagated to the calling code.

*For error case the ? operator returns the Result<E> from the function itself while for Result<v> it just returns V after evalutaing the expression it DOES NOT return from the function itself.*

Example...

```
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

```

Here if `File::open("hello.txt")` is success `?` returns the file handle T inside Result::Ok<T> over which we call `read_to_string` which loads up the content
of the file in the mutable string `s`. If at any point we recieve Result::Err<E> the `?` operator will resturn from the fucntion with Result::Err<E>.

This is common so there is shotcut method like `fs::read_to_string("hello.txt")` which does the same thing as above.

--------------------------------------------------------------
DIFFERENSE BETWEEN `return Err<E>` and using `?` operator...
--------------------------------------------------------------

Instead of directly returning a `Result::Err<E>` the `?` operator calls the `from()` function of the `From` trait in the standard library.
from() converts the error type received into the error type defined in the return type of the current function.

his is useful when a function returns one error type to represent all the ways a function might fail.
As long as each error type implements the from function to define how to convert itself to the returned error type,
the ? operator takes care of the conversion automatically.

Finally the `?` operator can only be used in a function that returns Result or Option or another type that implements std::ops::Try.

The main() function either returns `()`, and conveniently, another valid return type is Result<T, E>. So we can do...

```
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> { // Box<dyn Error> type is called a trait object, it means “any kind of error.”
    let f = File::open("hello.txt")?;

    Ok(())
}
```

So, Using `?` in a main function is allowed only if the return type is `Result`.


----------------------------------------------------------- GENERICS -----------------------------------------------------------


* We can use generics in function signatures, structs or enums.

Exampls...

```
/*
This function largest is generic over some type T. This function has one parameter named list, which is a slice of values of type T. The largest function will return a reference to a value of the same type T.
*/

fn largest<T>(list: &[T]) -> &T {
    let mut largest = list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

---------------------
// Generic struct

struct Point<T, U> {
    x: T,
    y: U,
}

...
let both_float = Point { x: 1.0, y: 4.0 };
let integer_and_float = Point { x: 5, y: 4.0 };

---------------------
// Generic enums

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

--------------------
// Generics for 'impl'

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {

    // A simple getter
    fn x(&self) -> &T {
        &self.x
    }
}

```

It is possible to implment a method only for a specific generic type of the struct like, this method is only available for 'Point<f32>' structs

```
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

```

Generics in impl and generics in method are different..

Here a new method is defined for 'Point<T, U>' so we had to specify the generic parameters as 'impl<T, U>'.

Next the method additionalty took a 'Point<V, W>' as a argument, which is of some other type <V,W> different from <T,U>
so we had to define the method like 'mixup<V, W>'.

```

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

...
let p1 = Point { x: 5, y: 10.4 };
let p2 = Point { x: "Hello", y: 'c' };

let p3 = p1.mixup(p2); // Point { x: 5, y: 'c' };
```

== PERFORMANCE OF GENERICS ==

Rust uses a technique called 'Monomorphization' which replaces all generics with concrete types at compile time and therefore there is absolutely NO COST at runtime.

Eg:

If we create 2 Option<T> types like 'Some(5);' and 'Some(5.0);'

Rust will expands the generic definition of Option<T> into Option_i32 and Option_f64, thereby replacing the generic definition with the specific ones.


```

// Compile time Monomorphization, expanding generics:

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```


====== TRAITS ======

Traits are similar to a feature often called interfaces in other languages, where you define a method and overrride it by implementing the interface.

Definign a trait:

```
pub trait Summary {
    fn summarize(&self) -> String;
}
```

Implementing a trait, syntax like: 'impl TRAIT for TYPE'

```
// Implementing the Summary trait for the NewsArticle struct
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// Now we can all summarize on an instance of NewsArticle like..
article.summarize()

```

 == Restriction when implementing traits ==

For implementing the trait on a type, either the type or the trait must be local to our crate. For example, we can implement the Display trait(external trait must be public) which is an external trait on our local type like 'NewsArticle'.

Similary we can implement our local trait like 'Summary' on an external type like String.

** However, WE CANNOT IMPLEMENT EXTRNAL TRAITS ON EXTERNAL TYPES.
Eg: We can't implement the external DIsplay trait on the external Vec<T> type.

Reason:

This restriction is part of a property of programs called coherence, and more specifically the orphan rule,
so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa.
Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.


* We can have a default implementations for a trait, if a type implementing the trait does not specify its own implementation the default is used.

Eg:

```
pub trait Summary {
    fn summarize_author(&self) -> String;

    // Default implementation of summarize
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

```

In the above example if a type only defines 'summarize_author' when implementing the 'Summary' trait then its also gets the 'summarize'
method without having to implement it since there is a default implementation.


* Traits as parameters

When using generics in function signature we can use traits like..

```
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

'<T: Summary>' says thtat the function receives a generic type T that must implement the Summary trait.

An equivalent, shortcut to the above code is..

```
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

Sometimes the above shotcut cannot be used to express our intentions like...

```
pub fn notify<T: Summary>(item1: &T, item2: &T) { ... }

// is not the same as...

pub fn notify(item1: &impl Summary, item2: &impl Summary) {... }

// since it inforces that both item1, item2 must be of same generic type T
```


* Specifying Multiple Trait Bounds with the + Syntax

```
// Here, 'item' must implement both Display and Summary traits

pub fn notify<T: Summary + Display>(item: &T) { ... }

// or shotcut

pub fn notify(item: &(impl Summary + Display)) { ... }
```

Having multiple items having different traits might be confussing a cleaner synax is using the 'where' clause like...


```
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {...}

// The above can be written like...

fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{ ... }
```

* Returning Types that Implement Traits

We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait

```
fn returns_summarizable() -> impl Summary { ... }
```

WARNING: When specifying the return type of a function implements a specific trait
it DOES NOT mean you can conditionally return DIFFERENT TYPEs at just beacuse those
types implement the trait.

Your function can only return a specific type but specifying the return type as a
trait only means that the retuned type must implement the trait.

So, this is INVALD ! Gives error like: '`if` and `else` have incompatible types'

```
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {...}
    } else {
        Tweet { ... }
    }
}
```
The above code wont compile, since it conditionally returns either a 'NewsArticle' or a 'Tweet'
although both of them implement the 'SUmmary' trait.
While you can accept different types T you can only return a perticular type at all circumstances from the function.


* Conditionally implementing methods

By using a trait bound with an impl block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.

Eg:

Here, we implement the 'cmp_display' method for only those types T of Pair<T> that implement both traits 'Display' and 'PartialOrd'

(This similar to how we implement a method for only a perticular type like Pair<f32>.)

```
...

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

```

Also, we can conditionally implement a trait for any type that implements another trait, this is called 'blanket'.

For example, the standard library implements the ToString trait on any type that implements the Display trait.
The impl block in the standard library looks similar to this code:

```
impl<T: Display> ToString for T { // Implement the 'ToString' function for any Type T that implements the 'Display' trait
    // --snip--
}
```

Because the standard library has this blanket implementation, we can call the to_string method defined by the ToString trait on
any type that implements the Display trait.

For example, `let s = 3.to_string();`



Advantage over other languages:
-------------------------------
In dynamically typed languages, we would get an error at runtime if we called a method on a type which didn’t define the method.

But Rust moves these errors to compile time so we’re forced to fix the problems before our code is even able to run.
Additionally, we don’t have to write code that checks for behavior at runtime because we’ve already checked at compile time.
Doing so improves performance without having to give up the flexibility of generics.


Blanked trait implementations:
------------------------------

https://users.rust-lang.org/t/what-are-blanket-implementations/49904

It is an implement of a trait either for all types, or for all types that match some condition.

For example, the stdandard library has this impl:

It is a blanket impl that implements ToString for all types that implement the Display trait.
(Rust adds the T: Sized bound to all generics by default. Adding ?Sized removes that bound, allowing non-sized types too.
The Sized trait means we know something's size at compile time. If you ever want to store something as a
local variable the compiler needs to know how much space to set aside, and adding a ?Sized to your where clause relaxes that restriction.)

```
impl<T> ToString for T where
    T: Display + ?Sized,
{ ... }
```

* LIFETIMES IN RUST

Most of the time, lifetimes are implicit and inferred by rust just like types but in certain cases we need to annotate lifetimes when the lifetimes of references could be related in a few different ways.


Borrow Checker:

The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid.

This code is invalid and gives error: `x` does not live long enough
```
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+

```

LIFETIME ANNOTATIONS:

Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

Lifetime parameters must start with an apostrophe (')

===VERY IMPORTANT===

*Lifetime annotations don’t change how long any of the references live.*

*At compile time only the compiler MUST be able to infer the lifetimes of all the references in the function signature either by itself(using the 3 life time rules) or by the help of lifetime annotations given by us, otherwise the code WONT compile*

*Every reference has a lifetime and you need to specify lifetime parameters for functions or structs that use references.*

*When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.*
(The returned reference from the function cannot be a value created within the function since in that case the reference becomes a dangling reference, once the function scope ends)

Example:

As with generic type parameters, we need to declare generic lifetime parameters inside angle brackets between the function name and the parameter list.

```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

```

The above code will not compile without the lifetime annotations because the compiler cannot automatically determine what will be the lifetime of the returned
reference since we are conditionally returning `x` or `y`.

When a function has references to or from code outside that function, it becomes almost impossible for Rust to figure out the lifetimes of the parameters or return values on its own. The lifetimes might be different each time the function is called. This is why we need to annotate the lifetimes manually.

By adding a lifetime annotation to the returned value `&'a str` we makke the compiler understand that the function returns a reference having lifetime `'a`
The compiler also further chehcks both `x` and `y` have lifetime `'a` since we hhave written `x: &'a str, y: &'a str` and thus the code is valid !

(In other words..
The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a. The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a.)

* Lifetimes for structs:

```
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```


The compiler uses three rules to figure out what lifetimes references have when there aren’t explicit annotations.

1) The first rule is that each parameter that is a reference gets its own lifetime parameter.

In other words, a function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32);`
 a function with two parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32);` and so on.

2) The second rule is if there is exactly one input lifetime parameter,
that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32.`

3) The third rule is if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.

4) Lifetime Elision rules: There are some common patterns were a situation is predictable and followed a few deterministic patterns. The developers programmed these patterns into the compiler’s code so the borrow checker could infer the lifetimes in these situations and wouldn’t need explicit annotations.


==Example==

Example 1:

Given signature: `fn first_word<'a>(s: &'a str) -> &str {`

After compiler applies rule 1: `fn first_word<'a>(s: &'a str) -> &str {`

After compiler applies rule 2: `fn first_word<'a>(s: &'a str) -> &'a str {`

Now all the references in this function signature have lifetimes, and the compiler can continue its analysis without needing the programmer to annotate the lifetimes in this function signature.

---

Example 2:

`fn longest(x: &str, y: &str) -> &str {`

here after rule 1 is applied: `fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {`

rule 2 does not work as there are 2 arguments, rule 3 does not apply as this is a function NOT a method so none of its arguments are `self`

So, After working through all three rules, we still haven’t figured out what the return type’s lifetime is.
Compiler gives error, and asks for explicit annotations!
---

Example 3:

```
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes. Then, because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for !
---

* Static lifetime:

One special lifetime we need to discuss is 'static, which means that this reference can live for the entire duration of the program. All string literals have the 'static lifetime like:

`let s: &'static str = "I have a static lifetime.";`

The text of this string is stored directly in the program’s binary, which is always available. Therefore, the lifetime of all string literals is 'static.


Example with Generic Type Parameters, Trait Bounds, and Lifetimes

```
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(         // function will use a lifetime 'a and generic parameter T
    x: &'a str,
    y: &'a str,                                 // x, y are two string slices
    ann: T,                                     // a generic parameter of of type T
) -> &'a str                                    // returns a string slices having lifetime 'a
where
    T: Display,                                 // generic type T, can be filled in by any type that implements the Display trait
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

----------------------------------------------------------- TESTING IN RUST -----------------------------------------------------------

Even though Rust’s type system and ownership rules help prevent some kinds of bugs, tests are still important to reduce logic bugs having to do with how your code is expected to behave.

To change a function into a test function, add #[test] annotation on the line before fn so the test runner knows to treat this function as a test.

We run tests using the cargo test command.

* Rust builds a test runner binary that runs the functions annotated with the test attribute and reports on whether each test function passes or fails. *
* Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed. *

If you are not using a separate module for tests then you don't need to use `#[cfg(test)]`.
Functions marked with #[test] are already excluded from non-test builds. This can be verified very easily:

```
#[test]
fn test() {}

fn main() {
    test(); // error[E0425]: cannot find function `test` in this scope
}
```



A test fails when the test function panics, either when panic! is called explicitly by us or when its called by an assert macro.

If the value is false, the assert! macro calls the panic! macro.

We can use `assert!`, `assert_eq!` and `assert_ne!` in tests to write assertions.

The tests module is a regular module that follows the usual visibility rules.
Because the tests module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module.
We use `use super::*;` inside the test module so anything we define in the outer module is available to this tests module.


Example:

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        // --snip--
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

fn main() {}
```


When the assertions fail, the `assert..` macros print their arguments using debug formatting,
which means the values being compared must implement the `PartialEq` and `Debug` traits.
All the primitive types and most of the standard library types implement these traits.
For structs and enums that you define, you’ll need to implement `PartialEq` to assert that values of those types are equal or not equal.

You’ll need to implement Debug to print the values when the assertion fails.

Because both traits are derivable traits, this is usually as straightforward as adding the `#[derive(PartialEq, Debug)]` annotation to your struct or enum definition.


* Adding custom faliure messages:

Add a custom message to be printed with the failure message as optional arguments to the assert!, assert_eq!, and assert_ne! macros.
This messages are passed to panic! when a test fails.

This messages are passed along to the format! macro, so you can pass a format string that contains {} placeholders and values to go in those placeholders.

Example:

```
assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
```

If this assertion fails we get an error like `thread 'main' panicked at 'Greeting did not contain name, value was `Hello!`', src/lib.rs:12:9`


You can mark that a test should panic by using the `should_panic` annotation along with the `test` annotation.

Example:
```
#[test]
#[should_panic]
fn greater_than_100() {
    Guess::new(200);
}
```

A should_panic test would pass even if the test panics for a different reason from the one we were expecting to happen. To make should_panic tests more precise, we can add an optional expected parameter to the should_panic attribute

The should_panic attribute’s expected parameter has to be a *substring* of the message that the function panics with.

Example:
```
#[test]
#[should_panic(expected = "Guess value must be less than or equal to 100")]
fn greater_than_100() {
    Guess::new(200);
}
```

* Using Result<T, E> in Tests

If a test function has a return type of Result<T, E> it means
we return `Ok` when the test passes and an `Err` when the test fails.

This enables us to use the question mark operator, which can be a convenient way to write tests that should fail if any operation within them returns an Err variant.

(If the `?` operator is placed after a Result, and the value of the Result is an Ok, the value inside the Ok will get returned from this expression,
and the program will continue. Otherwise the Err will be returned from the whole function as if we had used the return keyword so the error value
gets propagated to the calling code.)

We can’t use the `#[should_panic]` annotation on tests that use `Result<T, E>`,
instead we must return `Err` value directly when the test should fail.

Example:
```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

* Controlling Test runs

Just as cargo run compiles your code and then runs the resulting binary, cargo test compiles your code in test mode and runs the resulting test binary.

Some command line options go to cargo test, and some go to the resulting test binary.

To separate these two types of arguments, you list the arguments that go to cargo test followed by the separator -- and then the ones that go to the test binary.
(Running cargo test --help displays the options you can use with cargo test, and running cargo test -- --help displays the options you can use after the separator --)

By default they run in parallel using threads, if they are iterdependent this might be a problem run them sequentially by using only one thread like..

`cargo test -- --test-threads=1`

If the test prints some value then only values printed by failed tests are shown by default, you can see all printed values if..
`cargo test -- --show-output`

Running a Subset of Tests by Name: `cargo test add` , this runs test function which have the substring 'add' in there name.

Ignoring Some Tests: Add the `#[ignore]` attribute to exclude some tests like...

```
#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

Run only the ignored tests: `cargo test -- --ignored`

Run all tests in a specific file(tests/integration_test.rs): `cargo test --test integration_test`

* Organizing tests

= Unit tests =

You’ll put unit tests in the src directory in each file with the code that they’re testing.
The convention is to create a module named tests in each file to contain the test functions and to annotate the module with cfg(test).

What is [cfg(test)]?

The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test, not when you run cargo build.
This saves compile time when you only want to build the library and saves space in the resulting compiled artifact because the tests are not included.

By using the cfg attribute, Cargo compiles our test code only if we actively run the tests with cargo test.

The attribute cfg stands for configuration and tells Rust that the following item should only be included given a certain configuration option.
---

Example of a unit test for src/lib.rs

```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

fn main() {}
```

Testing private functions:

Because tests are just Rust code and the tests module is just another module, you can bring private functions into a test’s scope and call it.

Eg:

```
// Private function
fn internal_adder(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*; // brings the private functions into a test’s scope

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

```

== Integrastion tests ==

We create a tests directory at the top level of our project directory, next to src. Cargo knows to look for integration test files in this directory.

Example:

In tests/integration_test.rs..

```
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

*Each file in the tests directory is a separate crate*

So we need to bring the code we want to test in the crate's scope by `use adder;` this was not required for unit tests.

We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)].
Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test


* Submodules with integration tests

We might have some helper functions which many of our integration tests use, this helper function are not tests by themself but rust does not know that.

To avoid having helper files appear in the test output, instead of creating tests/common.rs, we’ll create tests/common/mod.rs.
This is an alternate naming convention that Rust also understands.

Naming the file this way tells Rust NOT to treat the common module as an integration test file.

*Files in subdirectories of the tests directory don’t get compiled as separate crates or have sections in the test output.*

Example:

Here we use functionality from our helper tests/common/mod.rs in a integration test

```
use adder;

mod common; // module declaration for using the helper functions

#[test]
fn it_adds_two() {
    common::setup();                        // Calling the helper function 'setup' in the helper module 'common'
    assert_eq!(4, adder::add_two(2));
}
```

---

* Integration tests are NOT available for binary crates

Lastly, for binary crates that only contains a src/main.rs file and doesn’t have a src/lib.rs file.
In this case, we can’t create integration tests in the tests directory and bring functions defined in the src/main.rs
file into scope with a use statement beacuse
*only library crates expose functions that other crates can use; binary crates are meant to be run on their own.*

To use integration tests for binary crates, We have to have a straightforward src/main.rs file that calls logic that lives in the src/lib.rs file.
Using that structure, integration tests can test the library crate with use to make the important functionality available.


----------------------------------------------------------- CLOSURES IN RUST -----------------------------------------------------------

Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.
You can create the closure in one place and then call the closure to evaluate it in a different context.
Unlike functions, closures can capture values from the scope in which they’re defined.

Examples:

```
fn  add_one_v1   (x: u32) -> u32 { x + 1 }      // Function
let add_one_v2 = |x: u32| -> u32 { x + 1 };     // Closure with annotaion
let add_one_v3 = |x|             { x + 1 };     // Closure without annotaion
let add_one_v4 = |x|               x + 1  ;     // Closure with braces since it has only one statement in its body
```

Closures can capture values from their environment in three ways:-

1) Taking ownership 2) Borrowing mutably 3) Borrowing immutably

The Fn traits are provided by the standard library. All closures implement at least one of the traits:

* Fn -> Fn borrows values from the environment immutably.

* FnMut -> FnMut can change the environment because it mutably borrows values.

* FnOnce ->  Consumes/takes ownership of varaibles in the closure’s environment. The Once part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.

Use FnOnce as a bound when you want to accept a parameter of function-like type and only need to call it once. If you need to call the parameter repeatedly, use FnMut as a bound; if you also need it to not mutate state, use Fn.

If you want to force the closure to take ownership of the values it uses in the environment, you can use the move keyword before the parameter list. This technique is mostly useful when passing a closure to a new thread to move the data so it’s owned by the new thread.


Eg:

```
fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x; // The closure as taken ownership of "x" here, if this closure is never called then ownership is not taken here

    println!("can't use x here: {:?}", x); // cannot use varaible "x" here since its ownership was taken by the closure

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
```

Closure Type annotations:

Closures are usually short and relevant only within a narrow context unlike functions.
The compiler is reliably able to infer the types of the parameters and the return type.

But for the compiler to automatically infer the closures types we must call it from somewhere otherwise explicity specify type annotaions.
The first time we call the closure with some value, the compiler infers the types and those types are then locked in to the closure
now we will a type error if we try to use a different type with the same closure.

Example:
```
fn main() {
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    let n = example_closure(5); // ERROR ! since the compiler has previously infered that the closure takes a String
}
```


-------------------------------------------------------- ITERATORS IN RUST --------------------------------------------------------

The iterator pattern allows you to perform some task on a sequence of items in turn.

In Rust, iterators are *lazy*, meaning they have no effect until you call methods that consume the iterator to use it up.

```
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter(); // This creates an iterator and does nothing since iterators are lazy

    for val in v1_iter {            // The "for" consumes the itertor here
        println!("Got: {}", val);
    }
```

Now look at this:

```
let v1 = vec![1, 2, 3];

let mut v1_iter = v1.iter();

v1_iter.next();
v1_iter.next();

```

In the above example "v1_iter" mutable since calling the next method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence.

So, calling next() uses up or consumes the iterator.

In case of the for-loop, the loop took ownership of v1_iter and made it mutable behind the scenes so we did not have to make v1_iter mutable

* Methods that Consume the Iterator

 There are 2 types of methods on iterators...

 1) CONSUMING adaptors: Methods that call next, these must takes ownership of the iterator and iterates through the items
    by repeatedly calling next, thus consuming the iterator.

    Example:

    ```
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum(); // sum() is a consuming adapter


        v1_iter.next() // INVALID ! because ownership of "v1_iter" was lost to "sum()"

    ```

2) ITERATOR adaptors: These take a iterator and return a new iterator, we can chain multiple calls to iterator adaptors to perform complex actions in a readable way.

   *But because all iterators are lazy, you have to call one of the consuming adaptor methods to get results from calls to iterator adaptors.*

    Example:

    ```
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // Here collect is a consuming adapter that consumed the iterator given by map()

    assert_eq!(v2, vec![2, 3, 4]);

    ```

    collect() can take anything iterable, and turn it into a relevant collection.
    The type is infered on or can be specified in many ways...

    ```
    let a = [1, 2, 3];

    let doubled: Vec<i32> = a.iter() .map(|&x| x * 2).collect();
    // Here type is infered from "doubled: Vec<i32>", without this code wont work since type cannot be infered

    let doubled = a.iter().map(|x| x * 2).collect::<Vec<i32>>(); // Using the 'turbofish' on "collect" instead of annotating doubled

    let doubled = a.iter().map(|x| x * 2).collect::<Vec<_>>(); // collect() only cares about what you're collecting into, so <Vec<_>> also works
    ```


TYPES OF ITERATORS:
(https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter)

The iter method produces an iterator over immutable references.

If we want to create an iterator that takes ownership of the data and returns owned values, we can call into_iter instead of iter.

Similarly, if we want to iterate over mutable references, we can call iter_mut instead of iter.

* iter() iterates over the items by reference. Will iterates over &T.

* into_iter() iterates over the items, moving them into the new scope. Will iterates over &mut T.

* iter_mut() iterates over the items, giving a mutable reference to each item. Will iterates over T.

So for x in my_vec { ... } is essentially equivalent to my_vec.into_iter().for_each(|x| ... ) - both move the elements of my_vec into the ... scope.

If you just need to "look at" the data, use iter, if you need to edit/mutate it, use iter_mut, and if you need to give it a new owner, use into_iter.

Example:

```
struct Counter {
    pub count: u32,
}

impl Counter {
    fn new(arg: u32) -> Counter {
        Counter { count: arg }
    }
}

fn main() {
    let mut v1: Vec<Counter> = vec![Counter::new(1), Counter::new(2), Counter::new(3)];

    let mut v2: Vec<_> = v1
        .iter_mut()
        .map(|x| {
            x.count = 5; // Not allowed if "iter()" was used instead of "iter_mut()"
            x * 2
        })
        .collect();
}

```


* Implementing "Iterator" Trait

All iterators implement a trait named Iterator that is defined in the standard library.

You must have a associated type "Item" and a "next()" function when implementing the iterator.
The next function must returns a `Option<Self::Item>` here "item" is the associated type you defined.

Once a type implements the "Iterator" trait all library methods like map(), zip(), etc are avilable on that type.

A complete example of implementing iterators:

```
struct Counter { // A custom type, with private struct varaible "count"
    count: u32,
}

impl Counter {
    fn new() -> Counter {     // A "new" method to create instances of "Counter"
        Counter { count: 0 }
    }
}

impl Iterator for Counter {  // Implement "Iterator" for "Counter"

    type Item = u32;          // The associated Item type for our iterator is u32, meaning the iterator will return u32 values.


    // Create an iterator that will only ever count from 1 to 5.

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// Since "Counter" implements iterator trait all iterator methods are available on it
#[test]
fn using_other_iterator_trait_methods() {

    let sum: u32 = Counter::new()    // take the values produced by an instance of Counter
        .zip(Counter::new().skip(1)) // pair them with values produced by another Counter instance after skipping the first value
                                        (zip produces only four pairs; the theoretical fifth pair (5, None) is never produced because zip returns None when either of its input iterators return None)

        .map(|(a, b)| a * b)         // multiply each pair together
        .filter(|x| x % 3 == 0)      // keep only those results that are divisible by 3
        .sum();                      // add all the resulting values together
    assert_eq!(18, sum);
}
fn main() {}
```

* PERFORMANCE OF ITERATORS VS LOOPS

Iterators, although a high-level abstraction, get compiled down to roughly the same code as if you’d written the lower-level code yourself using loops.

Iterators are one of Rust’s zero-cost abstractions, by which we mean using the abstraction imposes no additional runtime overhead.

Closures and iterators are Rust features inspired by functional programming language ideas. They contribute to Rust’s capability to clearly express high-level ideas at low-level performance. The implementations of closures and iterators are such that runtime performance is not affected. This is part of Rust’s goal to strive to provide zero-cost abstractions.


----------------------------------------------------------- SMART POINTERS-----------------------------------------------------------

Unlike general `&` references used to borrow values, smart pointers have additional capabilities.

Smart pointers, are data structures that not only act like a pointer but also have additional metadata and capabilities. The difference between references
and smart pointers is that references are pointers that only borrow data; in contrast, in many cases, smart pointers own the data they point to.

Some examples of smart pointers are String and Vec<T>. They own some memory and allow you to manipulate it. They also have metadata (such as their capacity) and extra capabilities or guarantees (such as with String ensuring its data will always be valid UTF-8).

Smart pointers are usually implemented using structs.
The characteristic that distinguishes a smart pointer from an ordinary struct is that smart pointers implement the Deref and Drop traits.

* The Deref trait allows an instance of the smart pointer struct to behave like a reference so you can write code that works with either references or smart pointers.

* The Drop trait allows you to customize the code that is run when an instance of the smart pointer goes out of scope.


== Using Box<T> to Point to Data on the Heap ==

Box<T> is a most straightforward smart pointer. Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data.

Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack. But they don’t have many extra capabilities either.

Just like any owned value, when a box goes out of scope, it will be deallocated. The deallocation happens for the box (stored on the stack) and the data it points to (stored on the heap).

The Box<T> type is a smart pointer because it implements the Deref trait, which allows Box<T> values to be treated like references. When a Box<T> value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the Drop trait implementation.

Boxes are used in situations like...

1) When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so:

    transferring ownership of a large amount of data can take a long time because the data is copied around on the stack. To improve performance in this situation, we can store the large amount of data on the heap in a box. Then, only the small amount of pointer data is copied around on the stack, while the data it references stays in one place on the heap.

2) When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
    - This deals with the trait object

3) When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size

Example of 3rd case:

A cons list iis a recursive type(One type whose size can’t be known at compile time is a recursive type) used commonly in functional laguages.

Like in elixir: `[1 | [2 | [3 | [] ] ] ] # [1, 2, 3]`

Rust impllementation of Cons list

```
// The enum has two values List::Cons and List::Nil
enum List {
    Cons(i32, List), // The first value is a i32 and second is a `List` which is another instance of our `List` enum
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // Trying to define a cons list using the enum we defined
    // The first Cons value holds 1 and another List value. This List value is another Cons value that holds 2 and another List value and so on..
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}

```

This gives us an error like:

```
error[E0072]: recursive type `List` has infinite size
= help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `List` representable
```

Why?

In order to determine how much space to allocated for a Enum value, Rust goes through each of the variants to see which variant needs the most space.

When Rust tries to determine how much space a recursive type like the List enum we defined. The compiler starts by looking at the Cons variant, which holds a value of type i32 and a value of type List. Therefore, Cons needs an amount of space equal to the size of an i32 plus the size of a List. To figure out how much memory the List type needs, the compiler looks at the variants, starting with the Cons variant. The Cons variant holds a value of type i32 and a value of type List, and this process continues infinitely.

Solution using Box<T> smart pointer by storing list in Heap:

```
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

```

The Cons variant will need the size of an i32 plus the space to store the box’s pointer data.
By using a box, we’ve broken the infinite, recursive chain, so the compiler can figure out the size it needs to store a List value.

Because a Box<T> is a pointer, Rust always knows how much space a Box<T> needs: a pointer’s size doesn’t change based on the amount of data it’s pointing to.
This means we can put a Box<T> inside the Cons variant instead of another List value directly.
The Box<T> will point to the next List value that will be on the heap rather than inside the Cons variant.

Boxes provide only the indirection and heap allocation; they don’t have any other special capabilities, like those we’ll see with the other smart pointer types. They also don’t have any performance overhead that these special capabilities incur, so they can be useful in cases like the cons list where the indirection is the only feature we need.

== DEREF Trait ==

Implementing the Deref trait allows you to customize the behavior of the dereference operator, *

Using the Deref trait a smart pointer can be treated like a regular reference, you can write code that operates on references and use that code with smart pointers too.

---------------------------
COMPLETE EXAMPLE IMPORTANT:
---------------------------
```
use std::ops::Deref;

// Struct with single generic parameter <T>
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Creating our own smart pointer by implementing deref
impl<T> Deref for MyBox<T> {
    type Target = T; // associated type for the Deref trait to use

    fn deref(&self) -> &T {
        &self.0 // Returns the first value of the enum, We return a reference to the underlying type which can be directly dereferenced !
    }
}

fn main() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let a = MyBox::new(x);


    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z); // Possible beacuse Box implements deref trait

    assert_eq!(5, *a); // Possible since MyBox implements deref trait, *a -> *(a.deref()) -> *(&self.0) -> self -> 5
}
```

Without the Deref trait, the compiler can only dereference & references. The deref method gives the compiler the ability to take a value of any type that implements Deref and call the deref method to get a & reference that it knows how to dereference.

So, when we do *a, Rust substitutes the * operator with a call to the deref method. So, `*a becomes *(a.deref())`
This Rust feature lets us write code that functions identically whether we have a regular reference or a type that implements Deref.

The deref methods returns a plain reference and the derefence(*) is still needed due to ownership system.
(If the deref method returned the value directly instead of a reference to the value, the value would be moved out of self. We don’t want to take ownership of the inner value inside MyBox<T> in this case or in most cases where we use the dereference operator.)

== Implicit Deref Coercions ==

Deref coercion is a convenience that Rust performs on arguments to functions and methods. Deref coercion works only on types that implement the Deref trait.
Deref coercion converts such a type into a reference to another type.

Deref coercion happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition. A sequence of calls to the deref method converts the type we provided into the type the parameter needs.

When the Deref trait is defined for the types involved, Rust will analyze the types and use Deref::deref as many times as necessary to get a reference to match the parameter’s type. The number of times that Deref::deref needs to be inserted is resolved at compile time, so there is no runtime penalty for taking advantage of deref coercion!

Examples:

deref coercion can convert &String to &str because String implements the Deref trait such that it returns str.

```
// Defination of MyBox that implements deref from previous example

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));

    hello(&m); // This Works !
}

```
hello() takes a &str but we passed it a &MyBox<String>. This works since rust will call deref multiple times on &MyBox<String> and convert it to &str
(It works because, the standard library provides an implementation of Deref on String that returns a string slice)


Without deref we had to do:

`hello(&(*m)[..]);`

Explaination: (This is what deref automatically does for us here):
------------------------------------------------------------------

*m -> *(MyBox::new(String)) -> *(MyBox::new(String).deref()) -> *(&String) -> String

&(*m)[..] -> &(String) -> &String[..] -> &str

-----------

Similar to how you use the Deref trait to override the * operator on immutable references, you can use the DerefMut trait to override the * operator on mutable references.

That is if you have a mutable type T that implements `T: DerefMut<Target=U>` so on call to deref we get a `&mut U`

so `*(mut T) -> *(mut T.deref()) -> *(&mut U) -> U`

Also, Rust can coerce a mutable reference to an immutable one that is for a mutable type T, `mut T.deref() -> &U` and not `&mut U`
but reverse is not possible: immutable references will never coerce to mutable references due to borrowing rules
(That is if we convert a immutable refernece to mutable which is okay BUT there might exist other immutable referces that is not allowed).

== DROP TRAIT ==

Drop trait requires us to implement the `drop` method which gets called automatically when a value goes out of scope.
The Drop trait is included in the prelude, so we don’t need to bring it into scope.
The body of the drop function is where you would place any logic that you wanted to run when an instance of your type goes out of scope, the memory will
automatically get cleared by rust but the drop method allows you to do additional tasks like release resources like files or network connections, etc.

The drop method will be called automatically when a value goes out of scope, it is **NOT** possible for you to call the Drop trait’s drop method manually
beacause Rust would still automatically call drop on the value at the end of its scope. This would be a double free error !

However you can force clean up early by calling `std::mem::drop` method and passing the value we want to force to be dropped early as an argument.
This function is in prelude so we don't need to include it. Calling it will invoke our drop implementaion.

Example:

```
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {

    // Implement drop method of Drop Trait
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };

    // c.drop(); This wont work cant call drop method of drop trait directly, will give error: "explicit destructor calls not allowed"

    // Called std::mem::drop to force drop value, "Dropping CustomSmartPointer with data `some data`!" is printed !
    // If this was not called Rust would call drop() method from Drop trait automatically after the main() method ends
    drop(c);
}

```

== Rc<T>, the Reference Counted Smart Pointer ==

https://doc.rust-lang.org/stable/rust-by-example/std/rc.html
https://doc.rust-lang.org/std/rc/

*Single-threaded reference-counting pointers*

Usually we can have only one owner to a value however there might be instances where we require a value to be owned by multiple owners.

Rc<T> or reference counting provides shared ownership of a value of type T, allocated in the heap and keeps track of the number of references to a value which determines whether or not a value is still in use.

If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

Example using cons list:

Think of cons list which is a part of two other cons list like:

```
a = 5 -> 10 -> nil

b = 3 -> a

c = 4-> b
```

Here both b and c use the cons list a as there part, if we try to implement this like...

```
enum List {
    Cons(i32, Box<List>), // Recursive data structure where 2nd item is a Box of itself(List)
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));

    let b = Cons(3, Box::new(a)); // ERROR ! value moved here,              owner ship of 'a' given to 'b'

    let c = Cons(4, Box::new(a)); // ERROR ! value used here after move,    'c' tries to own a but 'a' belongs to 'b'
}
```

We could change the definition of Cons to hold references instead to avoid passing ownership, but then we would have to specify lifetime parameters.
But this would create other problems like:

By specifying lifetime parameters, we would be specifying that every element in the list will live at least as long as the entire list.
Also, `let a = Cons(10, &Nil);` is invalid now since temporary `Nil` value would be dropped before a could take a reference to it.

Other option is to create copys of 'a' for both 'b' and 'c' but that would be bad for performance.

USING Rc<T> to solve this problem:
----------------------------------

```
enum List {
    Cons(i32, Rc<List>), // Now we wrap the 2nd type in Rc<> to enable reference counting
    Nil,
}

use crate::List::{Cons, Nil};

use std::rc::Rc; // use statement to bring Rc<T> into scope because it’s not in the prelude

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil))))); // Wrap 'a' in a Rc<> to enable having multiple owners of 'a'

    // we call the Rc::clone function and pass a reference to the Rc<List> in 'a' as an argument.
    // Calling Rc::clone() creates a new owner of 'a' and increases ownership count by 1
    // Calling a.clone() would also work here but would produce a deep copy of all the data BUT calling Rc::clone(&a) only increases reference count

    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    println!("Reference count of a = {}", Rc::strong_count(&a)); // 3 (Because 'a' has 3 owners 'a', 'b' and 'c')
}

```

IMPORTANT NOTES:

1) Reference count of an Rc increases by 1 whenever an Rc is cloned and produces a new pointer to the same allocation in the heap,
Also, reference count is decreases by 1 whenever one cloned Rc is dropped out of the scope. When an Rc's reference count becomes zero,
which means there are no owners remained, both the Rc and the value are all dropped.

2) Invoking clone on Rc produces a new pointer to the same allocation in the heap. When the last Rc pointer to a given allocation is destroyed,
the value stored in that allocation (often referred to as "inner value") is also dropped.

3) Cloning an Rc never performs a deep copy. Cloning creates just another pointer to the wrapped value, and increments the count.

4) Two `Rc`s are equal if their inner values are equal, Eg: `rc_a.eq(&rc_b)` // true if both wrap same value

5) We can use methods of a value directly, Eg: If `rc_a` is a string we can do `rc_a.len()`



== RefCell<T> and the Interior Mutability Pattern ==
(https://users.rust-lang.org/t/help-with-understanding-refcell-t-and-interior-mutability-pattern/49657)

Shared references in Rust disallow mutation by default, and Rc is no exception: you cannot generally obtain a mutable reference to something inside an Rc. If you need mutability, put a Cell or RefCell inside the Rc.

Interior mutability is a design pattern in Rust that *allows you to mutate data even when there are immutable references to that data*; normally, this action is disallowed by the borrowing rules.

NOTES:

* Unlike Rc<T>, the RefCell<T> type represents single ownership over the data it holds.

    So RcfCell<T> is similar to Box<T> ?

* With references and Box<T>, the borrowing rules’ invariants are enforced at compile time. With RefCell<T>, these invariants are enforced at runtime.

* With references, if you break these rules, you’ll get a compiler error. With RefCell<T>, if you break these rules, your program will panic and exit.

* The advantages of checking the borrowing rules at compile time are that errors will be caught sooner in the development process. The advantage of checking the borrowing rules at runtime instead is that certain memory-safe scenarios are then allowed, whereas they are disallowed by the compile-time checks.

* The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.

* Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in a multithreaded context.


So....

Box<T> allows immutable or mutable borrows checked at compile time;
Rc<T> allows only immutable borrows checked at compile time;
RefCell<T> allows immutable or mutable borrows() checked at *runtime*.

How RefCell<T> Works?
--------------------

When creating normal immutable and mutable references, we use the & and &mut syntax, respectively.
With RefCell<T>, we use the *borrow* and *borrow_mut* methods, which are part of the safe API that belongs to RefCell<T>.

The borrow method returns the smart pointer type Ref<T>, and borrow_mut returns the smart pointer type RefMut<T>.
Ref<T> and RefMut<T> are just a wrapper type for a immutably and mutably borrowed value from a RefCell<T> respectively.

Both types implement Deref, so we can treat them like regular references.
(Often we can directly call methods on `Ref` or `RefMut` and Rust automatically derefences and calls the methods on the underlying value which
`Ref` or `RefMut` holds)

The RefCell<T> keeps track of how many Ref<T> and RefMut<T> smart pointers are currently active.

Every time we call borrow, the RefCell<T> increases its count of how many immutable borrows are active. When a Ref<T> value goes out of scope, the count of immutable borrows goes down by one.

Just like the compile-time borrowing rules, *RefCell<T> lets us have many immutable borrows or one mutable borrow at any point in time.*

If we try to violate these rules, rather than getting a compiler error as we would with references, the implementation of RefCell<T> *will panic at runtime*.

Example:

```
use std::cell::RefCell;

fn main() {
   let val2 = Vec::new();

   val2.push(10); // cannot borrow `val2` as mutable, as it is not declared as mutablev

   // Using RefCell<T>

   let val1 = RefCell::new(Vec::new());

   let mut borrow_1 = val1.borrow_mut(); // The borrow_mut method returns a RefMut<T> smart pointer

   // We can directly call methods on the RefMut<T> smart pointer and they are called on the inner value
   // Here the compiler first looks for a push method on RefMut<Vec<T>>.
   // Because there isn’t one, it automatically dereferences the value and looks for it on Vec<T>, which succeeds.
   // So, borrow_1.push(10); -> automatic dereferencing -> (*borrow_1).push() -> (*RefMut<Vec<T>>).push() -> <Vec<T>>.push()
   borrow_1.push(10);

   val1.borrow();    // RUNTIME ERROR ! thread 'main' panicked at 'already mutably borrowed: BorrowError'

   let mut borrow_2 = val1.borrow_mut();

   borrow_2.push(12); // RUNTIME ERROR ! thread 'main' panicked at 'already borrowed: BorrowMutError'
}
```

Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
------------------------------------------------------------------------

If you have an Rc<T> that holds a RefCell<T>, you can get a value that can have multiple owners and that you can mutate safely!
This is beacuse Rc<T> allows multiple owners and RefCell<T> allows us to mutate the data.

COMPLETE EXAMPLE OF CONS LIST USING RefCell<T>:-

```
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>), // Our 'List' Enum now holds its data as a Rc<RefCell<i32>> and the rest of the 'List' as a Rc<List>
    Nil,
}

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5)); // Wrap the value '5' in a RefCell<T> and then wrap it in a Rc<T>

    // Here we use Rc::clone() to so both 'a' and 'value' have ownership of the inner 5 value
    // rather than transferring ownership from 'value' to 'a' or having a borrow from value.
    // Next, we wrap the `List` again in a Rc<T> since we later wish to give ownership of 'a' to 'b' and 'c'
    let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));

    // Add a new value '3' and '4' to 'b' and 'c' respectively using `Rc::new(RefCell::new())`
    // Also, adding both 'b' and 'c' as the owners of 'a' by `Rc::clone(&a)`
    let b = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = List::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // At this point Rc<T> for '5' is owned by 2 owners 'value' and 'a'
    // and Rc<T> for 'a' is owned by 3 owners 'a', 'b' and 'c'


    // Here calling borrow_mut on 'value', uses the automatic dereferencing to dereference the Rc<T> to the inner RefCell<T>
    // Then borrow_mut method on RefCell<T> returns a RefMut<T> smart pointer
    // Now we use the dereference operator(*) on it and change the inner value.

    // HOW IT WORKS:-
    // *value.borrow_mut() -> *(Rc<RefCell<T>>.borrow_mut()) ->
    // Now automatic dereferencing happens here since borrow_mut() is for RefCell<T> so it automatically deferences Rc<RefCell<T>> to RefCell<T> then ...
    // -> *(RefCell<T>.borrow_mut()) -> *(RefMut<T>) -> 5 (since RefMut implements DerefMut)
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a); // a after = Cons(RefCell { value: 15 }, Nil)
    println!("b after = {:?}", b); // b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
    println!("c after = {:?}", c); // c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))

}
```

== Reference Cycles and the Weak type ==

It is possible to create a memeory leadk in Rust by creating a reference cycle.

Example of Cons List with a Cycle...

```
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

// The second element in the Cons variant is now RefCell<Rc<List>>,
// Rc<<List> allows mutliple owners of the List and
// RefCell<Rc<List>> allows mutiple owners to have mutable access and ability to update the List
// This helps if we want to modify the 2nd element that is which List value a Cons variant is pointing to.
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    // This tail() method gives us access to the second item if we have a Cons variant.
    // It will return an Option<T>
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    // Create new Cons List where 2nd element is List::Nil wrapped like RefCell<Rc<List::Nil>>
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(List::Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a)); // Strong is 1 since Rc<T> in 'a' is only owned by 'a'
    println!("a next item = {:?}", a.tail()); // This is Some(RefCell { value: Nil })

    // Creates new Cons List 'b' and with value 10 and RefCell::new(Rc::clone(&a)))
    // Thus String count of 'a' now becomes 2
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    // a.tail() gives us a reference to the RefCell<Rc<List<Nil>>> in 'a' we store that is 'link'
    // We then do `*link.borrow_mut()` which gives us the inner value that is Rc<List<Nil>>
    // *link.borrow_mut() -> *RefCell<Rc<List<Nil>>>.borrow_mut() -> *RefMut<Rc<List<Nil>>> -> Rc<List<Nil>>
    // Now we change the value inside from an Rc<List> that holds a Nil value to the Rc<List> in b
    // *link.borrow_mut() = Rc::clone(&b); This means we now make Cons List 'a' point to 'b'
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b)); // Strong count of 'b' is 2 since it now has 2 owners 'b' and 'a'
    println!("a rc count after changing a = {}", Rc::strong_count(&a));// Strong count of 'a' is 2 since it now has 2 owners 'a' and 'b'

    // Now we have a cycle a -> 5 -> b -> 10 -> a


    // This is possible memory leak
    // Tt will overflow the stack beaucse of the cylce a -> 5 -> b -> 10 -> a
    // Rust will try to print the Lists following the cycle until stack overflows
    // RUNTIME PANIC ! thread 'main' has overflowed its stack
    println!("a next item = {:?}", a.tail());

    // CYCLE LIKE: Some(RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { ...
}
```

There is memory leak in the above example because...

At the end of main, Rust will try to drop b first, which will decrease the count of the Rc<List> instance in b by 1.

That is Rc<List> in 'b' cannot be dropped since its referenced by 'a', that 'a' also owns it and thus it has strong count != 0
Similiary Rc<List> in 'a' cannot be dropped since its referenced by 'b', so the memory the Rc<List> has on the heap won’t be dropped.


== Preventing Reference Cycles: Turning an Rc<T> into a Weak<T> ==

Notes on Weak<T> :-

* Weak is a version of `Rc` that holds a *non-owning reference* to the managed allocation

* We can create a weak reference to the value within an Rc<T> instance by calling `Rc::downgrade` and passing a reference to the Rc<T>
  like `Rc::downgrade(&branch)`

* Rc::downgrade, gives a smart pointer of type Weak<T>. Instead of increasing the strong_count in the Rc<T> instance by 1, calling Rc::downgrade increases the    weak_count by 1

* *Unlike strong _count the weak_count doesn’t need to be 0 for the Rc<T> instance to be cleaned up.*

* Unlike strong references whihc are used for shated ownership, weak references are not for expressing an ownership relationship.
  *They won’t cause a reference cycle because any cycle involving some weak references will be broken once the strong reference count of values involved is 0.*

* In order to access the underlying value from a Weak<T> reference we use the `upgrade` method on a Weak<T> instance.
  However, since the value(Rc<T>) that Weak<Rc<T>> references might have been dropped so upgrade returns an Option<Rc<T>>.
  (You’ll get a result of Some if the Rc<T> value has not been dropped yet and a result of None if the Rc<T> value has been dropped.)


  Example Creating a Tree data structure using Weak<T>:
  -----------------------------------------------------

  We will create a Tree like Data structure where each Node can have multiple children that we will store in a Vector.
  Also each node will have a reference to its parent, since a node does not own its parent this relationship is best
  expressed by using a Weak<T>. Using Weak<T> also prevents cycles in relationship cauing memory leak because nodes with
  weak_count != 0 will be dropped if their strong_count = 0.

```
use std::cell::RefCell;
use std::rc::{Rc, Weak};

// The parent of a Node is expressed by a Weak<Node> type, thus even if the weak_count != 0 due to this child -> parent relation
// a child node can still be dropped when the parent goes out of scope(strong_count=0)
// In children we have the children of the node as a vector of `Rc<Node>` values
// We wrap both parent and children in RefCell to have shared mutable access to Nodes
// Due to this a parent can mutate its children like: branch.children.borrow_mut()[0] = new_node;
// Or child can mutate who its parent is(the Weak<Node>, like: *leaf.parent.borrow_mut() = Rc::downgrade(&branch);) so we do RefCell<Weak<Node>>
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {

    // Here we create a new node with value 3
    // We assign it a empty parent like RefCell::new(Weak::new())
    // We also give it a empty vector of children
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // Here, Rc<Node> in leaf node has a strong count of 1 and a weak count of 0

    // Here, we try to get a reference to the parent of leaf by using the upgrade method,
    // we get a None value as their is no parent.
    // leaf.parent.borrow().upgrade() -> RefCell::new(Weak::new()).borrow().upgrade()
    // -> Weak::new().upgrade() -> Option::None
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // leaf parent = None

    // Here we create another new node and
    // assign it a vetor having our leaf node as a children
    // Using Rc::clone(&leaf) gives us the Rc<Node> of leaf that we store in the children vector of branch
    // and creates a new owner of leaf
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()), // Empty Weak<Node> reference in the parent field, because branch doesn’t have a parent node.
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // Here, Rc<Node> in leaf node has a strong count of 2 and a weak count of 0
    // since the Rc<Node> is owned by leaf and by branch because of Rc::clone(&leaf)

    // Here Rc<Node> in branch node has a strong count of 1 and a weak count of 0
    // Since the Rc<Node> is owned by branch

    // Here we assign a weak parent refence of leaf to branch
    // *leaf.parent.borrow_mut() -> *RefCell::new(Weak::new()).borrow_mut() -> *Weak::new()
    // Rc::downgrade(&branch) creates a new weak reference of 'branch'
    // Now we replace leaf's parent that is the empty Weak::new() with the Weak reference to branch
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // Here, Rc<Node> in leaf node remains with a strong count of 2 and a weak count of 0
    // while Rc<Node> in branch node now has a strong count of 1 and a weak count of 1
    // weak_count is now 1 since branch is now weakly referenced from leaf


    // Now leaf has a parent so ...
    // leaf.parent.borrow().upgrade() -> RefCell::new(Weak::new(branch)).borrow().upgrade()
    // -> Weak::new(branch).upgrade() -> Option::Some<branch>
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // Output of above:
    // Some(Node { value: 5, parent: RefCell { value: (Weak) }, children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] } })
}
```

Summary of Box<T>, Rc<T> and RefCell<T>:

* The Box<T> type has a known size and points to data allocated on the heap.
* The Rc<T> type keeps track of the number of references to data on the heap so that data can have multiple owners.
* The RefCell<T> type with its interior mutability gives us a type that we can use when we need an immutable type
  but need to change an inner value of that type; it also enforces the borrowing rules at runtime instead of at compile time.

# INTERIOR MUTABILITY IN A NUTSHELL!

mut is exclusive access and non-mut is shared access.
Shared access means you have readonly immutable access but you can get mutable access through the interior mutability pattern using
RefCell<T> if there does not exist any other mutable access to that data at that point,
otherwise code will panic like  thread 'main' panicked at 'already mutably borrowed: BorrowError'.
*Interior mutability allowes us to get exclusive access(mut access) to something for which we only have shared access(non mut access).*

-----------------------------------------------------------------------------------------------------------------------------------------------


-----------------------------------------------------  FEARLESS CONCURRENCY ------------------------------------------------------------

# Some Interesting theory:

Programming languages implement threads in a few different ways. Many operating systems provide an API for creating new threads. This model where a language calls the operating system APIs to create threads is sometimes called 1:1, meaning *one operating system thread per one language thread.*

Many programming languages provide their own special implementation of threads. Programming language-provided threads are known as *green threads*, and languages that use these green threads will execute them in the context of a different number of operating system threads. For this reason, the green-threaded model is called the M:N model: there are M green threads per N operating system threads, where M and N are not necessarily the same number.

=========
Runtime ?
=========
by runtime we mean code that is included by the language in every binary. This code can be large or small depending on the language, but every non-assembly language will have some amount of runtime code.

*The green-threading M:N model requires a larger language runtime to manage threads.*
*As such, the Rust standard library only provides an implementation of 1:1 threading.*
Because Rust is such a low-level language, there are crates that implement M:N threading if you would rather trade overhead for aspects such as more control over which threads run when and lower costs of context switching, for example.

Examples:

```
// Create a new thread by passing a colusre to spawn

thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);

            // The calls to thread::sleep force a thread to stop its execution for a short duration, allowing a different thread to run.
            thread::sleep(Duration::from_millis(1)); // Sleep for 1 ms
        }
    });
```

*A new thread will be stopped when the thread from which its started ends, whether or not it has finished running.*
This means if we start a thread from the main thread that thread will stop when the main thread stops.


`thread.spawn` returns a join handle over which we can wait using the join() method.
Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates.

```
let handle = thread::spawn(|| {
        // some code to execute in the thread
    });


    handle.join().unwrap(); // wait for thread
```

OWNERSHIP ?

If the colsure in a thread captures a varaible from the outerscope then this might be a problem

Example:

```
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    drop(v); // oh no! the borrowed v in the thread is invalid now
    // Even if we remove the drop(v) it still wont compile

    handle.join().unwrap();
}
```

Above code gives error like:

`closure may outlive the current function, but it borrows `v`, which is owned by the current function`
because Rust can’t tell how long the spawned thread will run, so it doesn’t know if the reference to v will always be valid.

* Rust infers how to capture v, and because println! only needs a reference to v, the closure tries to borrow v. *

# Transfer ownership to thread using `move`

```
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || { // ownership of the captured varaibles(that is v here) is passed to closure
        println!("Here's a vector: {:?}", v);
    });

    // drop(v); This cant be done now as owner ship of v is lost here: Error, use of moved value: `v`

    handle.join().unwrap();
}
```

Thus, the move keyword overrides Rust’s conservative default of borrowing; it doesn’t let us violate the ownership rules.

== Message Passing using Channels ==

A channel in rust has two halves: a transmitter and a receiver.
The transmitter half is the upstream location where we put the data in teh channel, and the receiver half will recieve the data from the channel.

We create a new channel using the `mpsc::channel` function; mpsc stands for `multiple producer, single consumer`.

*MPSC - Multi-producer, single-consumer FIFO queue communication primitives.*

mpsc is the way Rust’s standard library implements channels, a channel can have multiple sending ends that produce values
but only one receiving end that consumes those values.

We call `mpsc::channel()` to create a channel a get a sender and receiver returned as a tuple like `(Sender<T>, Receiver<T>)`

Example:

```
use std::sync::mpsc;
use std::thread;

fn main() {

    // mpsc::channel function returns a tuple, the first element of which is the sending end and the second element is the receiving end.
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // Here the transmitting end `tx` is "moved" in the closure
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

*A spawned thread needs to own the transmitting end of a channel to be able to send messages through the channel.*

So, we use a closure that will capture the transmitting end, and use `move` to move ownership of the trsnamitting end to the thread.

### Transmitting using send():-

`tx.send(val)` returns a Result<T, E> type, so if the receiving end has already been dropped
and there’s nowhere to send a value, the send operation will return an error.

### Receiving using recv and try_recv

`recv()` blocks the thread and waits for the channel untill a message is recieved.
`try_recv()` method doesn’t block, but will instead return a Result<T, E> immediately.

Both `recv()` and `try_recv()` return a Result<T, E>.

`recv()` which returns Err if the sending end of the channel closes, recv will return an error to signal that no more values will be coming.
`try_recv()` returns Err if Err value if there aren’t any messages this time.

We can call `iter()` on the `Receiver<T>` which will return a iterator that will block when `next()` called, untill it receives
a new value from the channel.
So doing `for i in receiver.iter()` will wait forever since, it will always call `next()` and keep blocking untill a new request is obtained.


## Owenership transferenece and Multiple producers

Ownership of a value iks lost when we send it down a channel from the transmitting end.

If can have multiple senders/transmitters for a channel (but only one receiver), this can be achieved by using the
`mpsc::Sender::clone(&tx);` method.


Example:

```
let (tx, rx) = mpsc::channel();

// Creating a second sender for the channel using clone()
let tx1 = mpsc::Sender::clone(&tx);

thread::spawn(move || {

    // Vector of integers
    let vals = vec![1, 2, 3, 4, 5];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    // ERROR on next line! As ownership of val was lost in `for val in vals` as `vals` moved due to this implicit call to `.into_iter()`
    // println!("val is {:?}", vals);
});

thread::spawn(move || {
    let vals = vec![6, 7, 8, 9, 10];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

// Here, we’re not calling the recv() or try_recv() function explicitly anymore: instead,
// we’re treating rx as an iterator, since we receive a vector of values.
for received in rx {
    println!("Got: {}", received);
}

```

Note, here `for received in rx {...}` works and we can iterate over the `Receiver<T>` because it has a
blanked implementation of the `std::iter::IntoIterator` trait(having the next() method.


## Shared State concurrency using Mutex<T> and Arc<T>

Channels in Rust allow concurrency using message passing, we can also implement concurrency via shared state.

In Rust a Mutex( mutual exclusion) primitive is useful for protecting shared data, it allows only one thread to access some data at any given time.
To access some data a thread must first acquire a lock over that data and then mutate the data, also the thread must unlock the data afterwards.

## IMP NOTES about Mutex<T>:


* To access the data inside the mutex, we use the lock method to acquire the lock.
This call will block the current thread so it can’t do any work until it’s our turn to have the lock.

* The call to lock would fail if another thread holding the lock panicked.
In that case, no one would ever be able to get the lock (this is called "poisoning" where a mutex is considered poisoned whenever a thread panics while holding the mutex.)

* After we’ve acquired the lock,we get a mutable reference to the data inside. We must aquire a lock on the Mutex<T> to access the wrapped value `T` inside.

 * Mutex<T> is a smart pointer.
The call to lock returns a smart pointer called `MutexGuard`, wrapped in a `LockResult` that we handled with the call to unwrap.
The `MutexGuard` is also a smart pointer that implements `Deref` to point at our inner data (`pub fn lock(&self) -> LockResult<MutexGuard<'_, T>>`)
the smart pointer also has a `Drop` implementation that releases the lock automatically when a `MutexGuard` goes out of scope.

Example:

```
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5); // Here Rust infers type of `m` as Mutex<i32>

    {

        // This is a blocking operation that waits untill the current thread can acquire a lock over the data
        //  LockResult is actaully like, `type LockResult<Guard> = Result<Guard, PoisonError<Guard>>;`
        // so we can call unwrap on it to get the underlying MutexGuard<i32> here
        // `MutexGuard` is also a smart pointer that implements `Deref`, so we can do `*num = 6;`
        let mut num = m.lock().unwrap(); // We have a mutable MutexGuard<i32>
        *num = 6;

    } // At this point the lock is released automatically since `MutexGuard` has a `Drop` implementation
      // that releases the lock automatically when a MutexGuard goes out of scope

    println!("m = {:?}", m);
}
```

## Sharing Mutex accross mutiple threads using Arc<T>
https://doc.rust-lang.org/std/sync/struct.Arc.html

When we move a Mutex to a thread's closure it takes ownership of that Mutex, but then how can we pass it to other threads.

We can use Rc<T> smart pointer here to have mutiple owners of the Mutex however this will give an error like...

`Rc<Mutex<i32>> cannot be sent between threads safely.`
The compiler is also telling us the reason why: `the trait Send is not implemented for Rc<Mutex<i32>>`

Thus, `Rc<T>` is not safe to share across threads. When `Rc<T>` manages the reference count, it adds to the count for each call to clone and subtracts from the count when each clone is dropped. But it doesn’t use any concurrency primitives to make sure that changes to the count can’t be interrupted by another thread.

Unlike `Rc<T>`, `Arc<T>` uses atomic operations for its reference counting, so `Arc<T>` (atomic reference type) is safe to use in concurrent situations.
So, `Arc<T>` allows us to have multiple owners even in cocurrent situtaions, but the disadvantage is that atomic operations are more expensive than ordinary memory accesses. If you are not sharing reference-counted allocations between threads, consider using `Rc<T>` for lower overhead.

Arc<T> will implement `Send` and `Sync` as long as the T implements Send and Sync.
If you need to mutate through an Arc, use `Mutex`, `RwLock`, or one of the Atomic types wrapped in a Arc.

### Something about Atomic Types(Optional):
https://doc.rust-lang.org/std/sync/atomic/index.html

All atomic types are guaranteed to be lock-free if they're available. This means they don't internally acquire a global mutex.
Atomic types provide primitive shared-memory communication between threads, and are the building blocks of other concurrent types.
The `std::sync::atomic` defines atomic versions of a select number of primitive types, including AtomicBool, AtomicIsize, AtomicUsize, AtomicI8, AtomicU16, etc.
Atomic types present operations that, when used correctly, synchronize updates between threads.
Usage like: `let lock = Arc::new(AtomicUsize::new(1));`


Example of having multiple owners of data using Arc<T>

```
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0)); // Wrap a `Mutex` having data `0` inside a `Arc`
    let mut handles = vec![];

    // Create 10 threads to concurrently haver ownership and increment the `counter` value
    for _ in 0..10 {
        let counter = Arc::clone(&counter);  // Create a new owner of the Mutex by calling Arc::clone (this is similar to Rc<T> clone)

        let handle = thread::spawn(move || { // The new owner of the mutex that is `counter` is moved inside the thread's closure

            // Aquire lock over the Mutex
            // Arc<Mutex<i32>>.lock().unwrap() -> (Rust is automatically dereferencing) ->
            // Mutex<i32>.lock().unwrap() -> LockResult<MutexGuard<i32>>.unwrap() -> MutexGuard<i32>
            let mut num = counter.lock().unwrap();

            *num += 1;  // *MutexGuard<i32> ->(mutex guard implmene4ts deref trait) -> i32
        });
        handles.push(handle); // Push the threads handle in the handles vector
    }

    // Interate over each of the thread handles
    for handle in handles {

        // handle.join() will wait for each thread to complete, and the we call unwrap to get the underlying value
        handle.join().unwrap();
    }

    // By this point since we have waited on the handles of all the threads in the above loop,
    // all threads have finished and we get the final value of counter that is 10
    println!("Result: {}", *counter.lock().unwrap()); // counter is still a Arc<Mutex<i32>> so we have to lock it and then dereference to get the inner i32 value
}
```


## RefCell<T> vs Mutex<T> & Rc<T> vs Arc<T>

Here, in the above example counter is immutable but we could get a mutable reference to the value inside it to update it like `*num += 1;`;
this means Mutex<T> provides interior mutability, as the Cell family does.

Infact, Mutex is just a generalization of the RefCell, which does not panic on concurrent access unlike RefCell,
but rather allows one to wait (blocking the current thread, until another thread finishes its part of work).

Similarly, Arc<T> is similar to Rc<T> and provides safe multiple ownership in a concurrent environment.

Just like Rc<T>, there was arisk of creating reference cycles, where two Rc<T> values refer to each other, causing memory leaks (we solved this using Weak links).
Similarly, Mutex<T> comes with the risk of creating deadlocks. These occur when an operation needs to lock two resources and two threads
have each acquired one of the locks, causing them to wait for each other forever. We must take care to avoid this(mitigation strategies for mutexes).


Very little of how Rust handles concurrency is part of the language, many concurrency solutions are implemented in the standard library or as crates.
Only, the `std::marker` traits Sync and Send are two concurrency concepts are actually embedded in the language.

## Send trait

*The Send marker trait indicates that ownership of the type implementing Send can be transferred between threads*

Almost every Rust type is Send, but there are some exceptions, including Rc<T>: this cannot be Send because if you cloned an Rc<T> value and tried to transfer ownership of the clone to another thread, both threads might update the reference count at the same time. For this reason, Rc<T> is implemented for use in single-threaded situations where you don’t want to pay the thread-safe performance penalty. For Arc<T>, which is Send, so its safe in cocurrent environments.

Any type composed entirely of Send types is automatically marked as Send as well. Almost all primitive types are Send, aside from raw pointers.

## Sync trait

*The Sync marker trait indicates that it is safe for the type implementing Sync to be referenced from multiple threads.*

In other words, any type T is Sync if &T (a reference to T) is Send, meaning the reference can be sent safely to another thread.
Similar to Send, primitive types are Sync, and types composed entirely of types that are Sync are also Sync.

The smart pointer Rc<T> is also not Sync for the same reasons that it’s not Send.
The RefCell<T> type (which we talked about in Chapter 15) and the family of related Cell<T> types are not Sync.
The smart pointer Mutex<T> is Sync and can be used to share access with multiple threads(Send).

## Implementing Send and Sync

As marker traits, they don’t even have any methods to implement. They’re just useful for enforcing invariants related to concurrency.
Because types that are made up of Send and Sync traits are automatically also Send and Sync, we don’t have to implement those traits manually.
Manually implementing these traits involves implementing unsafe Rust code.



--------------------------------------------------------------------- TRAITS ---------------------------------------------------------------

With traits we can implement concepts like inheritance, polymorphism and dynamic method dispatch in RUST.

We can define a trait with method signatures and any type can now implement that trait and override that method, according to there needs.
This allows us to create things like collection of items of different types in a single vectors/hash, etc.

A generic type parameter can only be substituted with one concrete type at a time (done at *compile time*),
whereas trait objects allow for multiple concrete types to fill in for the trait object at *runtime*.

* *We create a trait object by specifying some sort of pointer, such as a '&' reference or a Box<T> smart pointer,*
* *then the dyn keyword, and then specifying the relevant trait.*

*THUS, A TRAIT OBJECT ALWAYS LOOKS LIKE `Box<dyn (Type that implements a Trait)>`*

When defining a trait we can also define default implementations of methods, the types which implement the trait can then choose to override the
default implementation if required.

```
// In src/lib.rs

// Defining a trait
pub trait Draw {
    fn draw(&self);
}


pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

// Implementing a trait
impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

// Implementing a trait
impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

// Now we can have a vector of items implementing Draw trait
// These items do not have to be necessarily the same
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // This is how we difine trait objects
}

// This run() method on the Sreen struct iterates over all the
// `components: Vec<Box<dyn Draw>>` and calls their draw() method
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}


// So now we can do this...
// in src/main.rs
use gui::{Button, Screen};

fn main() {
    let screen = Screen {
        // Here we have a vector of different items !
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```

## Dynamic Method dispatch, `dyn` keyword and its runtime implications:

When we use trait bounds on generics, the compiler generates nongeneric implementations of functions
and methods for each concrete type that we use in place of a generic type parameter, this is called monomorphization.
The code that results from monomorphization is doing static dispatch, which is when the compiler knows what method you’re calling at compile time.

When we use trait objects, Rust must use dynamic dispatch.
The compiler doesn’t know all the types that might be used with the code that is using trait objects,
so it doesn’t know which method implemented on which type to call.

`dyn` is a prefix of a trait object's type. (https://doc.rust-lang.org/std/keyword.dyn.html)
The dyn keyword is used to highlight that calls to methods on the associated Trait are dynamically dispatched.
To use the trait this way, it must be 'object safe'.

A dyn Trait reference contains two pointers. One pointer goes to the data (e.g., an instance of a struct).
Another pointer goes to a map of method call names to function pointers (known as a virtual method table or vtable).

At run-time, when a method needs to be called on the dyn Trait, the vtable is consulted to get the function pointer and then that function pointer is called.
The above indirection is the additional runtime cost of calling a function on a dyn Trait.
Methods called by dynamic dispatch generally cannot be inlined by the compiler.

## Object Safety

You can only make object-safe traits into trait objects. This means if a trait is NOT object safe you cannot make objects implementing that trait.
*A trait is object safe if all the methods defined in the trait have the following properties:*

* The return type isn’t Self.
* There are no generic type parameters.

The *Self* (notice capital 'S') keyword is an alias for the type we’re implementing the traits or methods on.
Trait objects must be object safe because once you’ve used a trait object, Rust no longer knows the concrete type that’s implementing that trait.

So, if a trait method returns `Self`, rust does not know what is `Self` here because the trait object itself orgets the exact type that `Self` is.
Similarly, if a trait method has generic type parameters it does not itself know, the concrete types that will become part of the type that implements the trait.
When the type is forgotten through the use of a trait object, there is no way to know what types to fill in the generic type parameters with.

Example:

The standard library’s clone method in the Clone trait looks like this...
Here Clone trait is not object-safe.

```
pub trait Clone {
    fn clone(&self) -> Self;
}

pub struct Screen {
    pub components: Vec<Box<dyn Clone>> // ERROR ! the trait `std::clone::Clone` cannot be made into an object
}
```

When we try to create trait objects of `Clone`, rust wont allow since the `clone` method of `Clone` is not object safe, since it returns `Self`.
The signature of clone needs to know what type will stand in for `Self`, because that’s the return type.

--------------------------------------------------------------------- PATTERN MATCHING -----------------------------------------------------------------------

Examples:

```
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
-----------------------------------
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() { // enumerate requires a table with indexes
    println!("{} is at index {}", value, index);
}
-----------------------------------
let (x, y, z) = (1, 2, 3);

let (x, y) = (1, 2, 3); // Error !

-----------------------------------
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

let point = (3, 5);
print_coordinates(&point);
```

##  Refutable vs Irrefutable pattern

Patterns come in two forms: refutable and irrefutable. Patterns that will match for any possible value passed are irrefutable.
An example would be x in the statement let x = 5;

Patterns that can fail to match for some possible value are refutable. An example would be Some(x) in the expression if let Some(x) = a_value because if the value in the a_value variable is None rather than Some, the Some(x) pattern will not match.

let statements, and for loops can only accept irrefutable patterns, because the program cannot do anything meaningful when values don’t match.

Eg: `let Some(x) = some_option_value; // Error wont compile,  refutable pattern in local binding: `None` not covered`

`let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant

-----

The if let and while let expressions accept refutable and irrefutable patterns.

Eg:

```
    if let Some(x) = some_option_value { // Works ! if let accept refutable and irrefutable patterns.
        println!("{}", x);
    }

```

For match statements, *Rust allows us to use an irrefutable pattern in a match with only one arm because*, match arms must use refutable patterns, except for the last arm, which should match any remaining values with an irrefutable pattern.

## Examples of different types of pattern matching in RUST:

```
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    _ => println!("anything"),
}

// --------------------------------

// ..= syntax allows us to match to an inclusive range of values

let x = 5;

match x {
    1..=5 => println!("one through five"), // If x is 1, 2, 3, 4, or 5
    _ => println!("something else"),
}

// Ranges are only allowed with numeric values or char values, because the compiler checks that the range isn’t empty at compile time.

// ------------NAMED VS UNAMED DESTRUCTURING-----------

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p; // named
    let Point { x, y } = p;       // unamed

    // In match...

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

// -------------Destructuring Nested Structs and Enums-------------

match msg {

        // matches a Message::ChangeColor enum variant that contains a Color::Rgb variant;
        // then the pattern binds to the three inner i32 values.
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        _ => (),
    }

// -------------Destructuring Structs and Tuples---------------

let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

// ----------------- IGNORING VALUES -------------------

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

(first, _, third, _, fifth) = (2, 4, 8, 16, 32);

let _x = 5;

// ------------------

// An unused variable starting with an underscore still binds the value, which might take ownership of the value
// Using _ does not bind the value(ownership is not lost) but doing _x will bind the value,
// the value is not usable because of the prefix(_x) but its ownership is lost

let s = Some(String::from("Hello!"));

if let Some(_s) = s {          // Using _ instead of _s here would solve the error!
    println!("found a string");
}

println!("{:?}", s); // ERROR ! borrow of moved value: `s`

// --------------- Ignoring Remaining Parts of a Value with .. -------------------

 let origin = Point { x: 0, y: 0, z: 0 };

  match origin {
    Point { x, .. } => println!("x is {}", x),
  }

  (x, .. , y) = (1,2,3,4); // Matching first and last value, this works !

  // using .. must be unambiguous!
  (.., second, ..) = (1,2,3,4); // ERROR! '..' can only be used once per tuple pattern
```

## Match Guards

A match guard is an additional if condition specified after the pattern in a match arm that must also match,
along with the pattern matching, for that arm to be chosen.

Example:

```
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // ----------

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"), // Here y = false
        _ => println!("no"),
    }
```

Using, match arms its also possible to avoid vraible shadowing when using varaibles with same names inside match arm
Example:
`Some(n) if n == y` Here we accept n only if it matches the variable y from outer scope, here we ensured that the pattern matches only if the value wrapped by Some is same as `y` and also we don't introduce a new variable y that would shadow the outer y isnide the match arm

## @Bindings

The at operator (@) lets us create a variable that holds a value at the same time we’re testing that value to see whether it matches a pattern.
Using @ lets us test a value and save it in a variable within one pattern.

Example:

```
    match msg {

        Message::Hello {
            id: id_variable @ 3..=7, // This will test that id lies in range 3..=7 as well as store id inside id_varaible for use inside the match arm
        } => println!("Found an id in range: {}", id_variable),
    ....

```


# UNSAFE RUST

Static analysis is conservative, sometimes code that might be correct might not compile.
Also, low-level systems programming is one of the goals of the language, this often requires unsafe coe.
Other use cases are when interfacing with C code, etc.

Thu Rust allows us to write unsafe code, this means problems due to memory unsafety,
such as null pointer dereferencing, can occur.

To switch to unsafe Rust, use the unsafe keyword and then start a new block that holds the unsafe code.

You can take five actions in unsafe Rust, called unsafe superpowers, that you can’t in safe Rust.

* Dereference a raw pointer
* Call an unsafe function or method
* Access or modify a mutable static variable
* Implement an unsafe trait
* Access fields of unions

unsafe doesn’t turn off the borrow checker or disable any other of Rust’s safety checks: if you use a reference in unsafe code, it will still be checked.

The unsafe keyword only gives you access to these five features that are then not checked by the compiler for memory safety.

## Dereferencing a Raw Pointer is Unsafe

Raw pointers can be immutable or mutable and are written as *const T and *mut T, respectively.
The asterisk isn’t the dereference operator; it’s part of the type name.

Difference of Raw pointer from (references and smart pointers)..

* Can ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location.
* Aren’t guaranteed to point to valid memory.
* Can be null.
* No automatic cleanup


Defining a raw pointer itself is not unsafe however dereferencing a raw pointer is Unsafe(since it might not point to a valid value)
and has to done inside an unsafe block.

Example:
```
    let mut num = 5;

    let r1 = &num as *const i32;    // Immutable raw pointer reference
    let r2 = &mut num as *mut i32;  // Mutable raw pointer reference

    // Here since we are using raw pointers we have a mutable as well as immutable
    // pointer to the same data, this is possible since borrowing rules don't apply to raw pointers.

    // Dreferencing raw pointers in unsafe and must be done inside a unsafe block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

```

### Safe Abstraction over Unsafe Code

Trying to code the `split_at_mut()` function which splits a string and returns
a tuple containg the 2 splited parts...

Example
```
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len); // Panic if  split index exceeds string length

    (&mut slice[..mid], &mut slice[mid..])
}

```

This wont work since although we’re borrowing different parts of the string slice, Rust’s borrow checker can’t understand that
and will not allow us to `cannot borrow `*slice` as mutable more than once at a time`, although
Borrowing different parts of a slice is fundamentally okay because the two slices aren’t overlapping, so we don't have two mutable
references to the safe data.

Writing unsafe code to solve this...

```
use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr(); // returns a raw pointer with the type *mut i32

    assert!(mid <= len);

    // `slice::from_raw_parts_mut` function takes a raw pointer and a length, and it creates a slice
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid), // Take (len-mid) chars from index `ptr.add(mid)`
        )
    }
}
```

Here, the function `slice::from_raw_parts_mut` is unsafe because it takes a raw pointer and must trust that this pointer is valid.
The `add` method on raw pointers is also unsafe.

We use a safe API to wrap the unsafe code which we know will never fail, but rust does not know this so we use the unsafe block.

## Using extern Functions to Call External Code is Unsafe

Rust has a keyword, extern, that facilitates the creation and use of a Foreign Function Interface (FFI). An FFI is a way for a programming language to define functions and enable a different (foreign) programming language to call those functions.

Example:

```
// "C" part defines which application binary interface (ABI) the external function uses.
// the ABI defines how to call the function at the assembly level. Here we use the "C" ABI.
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

## Accessing or Modifying a Mutable Static Variable is unsafe

In Rust, global variables are called static variables.

The names of static variables are in SCREAMING_SNAKE_CASE by convention, and we must annotate the variable’s type.

**Static variables can only store references with the 'static lifetime**

Accessing an immutable static variable is safe.
**Static variables can be mutable BUT Accessing and modifying mutable static variables is unsafe.**

This is because, with mutable data that is globally accessible, it’s difficult to ensure
there are no data races when accessing data from multiple threads is difficult,
which is why Rust considers mutable static variables to be unsafe.

Example:

```
static mut COUNTER: u32 = 0; // static mutable variable

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc; // Modifying static mutable variable is UNSAFE !
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER); // Even accessing static mutable variable is UNSAFE !
    }
}
```

## Implementing an Unsafe Trait is Unsafe

A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify.

Example:
```
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
```

## Accessing Fields of a Union is Unsafe

A union is similar to a struct, but only one declared field is used in a particular instance at one time. Unions are primarily used to interface with unions in C code. Accessing union fields is unsafe because Rust can’t guarantee the type of the data currently being stored in the union instance.

# Advanced Traits


## Associated Types

Associated types connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures.

So, we can define a trait that uses some types without needing to know exactly what those types are until the trait is implemented.

The implementor of a trait will specify the concrete type to be used in this type’s place for the particular implementation.

Example of the Iterator trait with assocaited type:

```
pub trait Iterator {
    type Item; // Associated Type

    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32; // We specify the concrete type to be used in this assocaited type’s place when implementing the trait

    fn next(&mut self) -> Option<Self::Item> {
        ....
    }
}

```

Difference between associated types and Generics?

If we use generics in a trait defination instead of associated types, it will work. However, whhen ever we implement that trait we MUST annotate the types. like `Iterator<String> for Counter`, since by using generics we say that there can be mutilple implementations of `Iterator` for Counter. Like Count<String>, Counter<u32>, etc. This is okay if we need different implementatitonso of counter for different types.

With associated types, we don’t need to annotate types because we can’t implement a trait on a type multiple times.

## Operator Overloading and Default generic Types

We can specify a default concrete type for the generic type.

Example:

Rust doesn’t allow you to create your own operators or overload arbitrary operators. But you can overload the operations and corresponding traits listed in std::ops by implementing the traits associated with the operator.

```
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

```

The `Add` trait looks like...

```
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

Here the `add()` method receives a argument `rhs` of generic type `Rhs`, but we don't need to specify
the type of `Rhs` since the default is `self` given by `trait Add<Rhs=Self>`.

This means by default the `add()` method assumes that the 2nd argument it receives will be of the same type as `Self`.
In the example we add two point instance so they are both of the same type, thus this assumption works fine, and due
to this default generic type we don't have to specify the generic type of the 2nd argument when implmenting `Add`.

However, we can override the default generic if the 2nd parameter passed to the `add()` method is of a different type than
`self`, for exaple if we want to add `Point` and `Offset` then we can implement add as: `impl Add<Offset> for Point {...}`
now Point `{x: 1, y: 0} + Offset {del: 9}` works !


## Disambiguation: Calling Methods with the Same Name

Self explanatory example:

```
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

// Human struct implements fly method for Pilot trait
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

// Human struct implements fly method for Wizard trait
impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

// Human struct implments fly method on its own type
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    let person = Human;
    person.fly();           // compiler defaults to calling the method that is directly implemented on the type
    Pilot::fly(&person);    // Fly for Pilot Trait
    Wizard::fly(&person);   // Fly for Wizrd Trait
}
```

For traits where the method takes in `&self` this works:
```
person.fly(); // This is same as below if there is only one implmentation of fly, otherwise error: disambiguate the associated function
Pilot::fly(&person); // This is required if there are many implmenetations of fly for different traits
```

But what if the trait trait method does not need `&self`, how to disambiguate incase of same method names for many traits?

We need to use fully qualified syntax like: `<Type as Trait>::function(receiver_if_method, next_arg, ...);`

Example:
`<Dog as Animal>::baby_name()`
Calls the baby_name() method from the Animal trait as implemented on Dog by saying that we want to treat the Dog type as an Animal for this function call.

* SUMMARY: So, when calling a method of a trait and passing it self we can do:`

```
 person.fly();                      // If only one fly() implementaions exists
 Wizard::fly(&person);              // If multiple fly() implementaions exists
 <Human as Wizard>::fly(&person);   // If multiple fly() implementaions exists and they dont take &self, use fully qualified syntax
                                    // This is not required here as fly takes &self here
```

## Supertraits: Require One Trait’s Functionality Within Another Trait

Sometimes, you might need one trait to use another trait’s functionality.
In this case, you need to rely on the dependent trait also being implemented, you can enforce this using super traits.

SYNTAX: trait <name>: <Super trait name>

Example:

Let’s say we want to make an OutlinePrint trait with an outline_print method that will print a value framed in asterisks. That is, given a Point struct that implements Display to result in (x, y), when we call outline_print on a Point instance

```
use std::fmt;

// To use OutlinePrint trait the type must also implement thhe Display trait
// Thus, Display trait is a super trait for the Outline print trait
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // ....
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// This works since Point implements Display trait otherwise we get "error[E0277]: `Point` doesn't implement `std::fmt::Display`"
impl OutlinePrint for Point {}

```

## Newtype Pattern to Implement External Traits on External Types

When implementing traits *we cannot implment a external trait for a external type*.
we’re allowed to implement a trait on a type as long as either the trait or the type are local to our crate.

How ever this is possible by using the *new type pattern* which involves wrapping up the external type in a tuple struct
thus making a new type which is now local to our crate.

There is no runtime performance penalty for using this pattern, and the wrapper type is omitted at compile time.

Example:

Both `Display` trait and `Vec<T>` type are external to us, so it is impossible to implement the DIaply trait on vectors.
However we can wrap up thr Vec in a tuple struct making it a new type over which we implement `Display`

```
use std::fmt;

// Tuple struct to wrap up Vec<String>
struct Wrapper(Vec<String>);

// Implement Displlay for the Wrapper type
impl fmt::Display for Wrapper {

    // self.0 to access the inner Vec<T>, because Wrapper is a tuple struct and Vec<T> is the item at index 0 in the tuple
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

However, we can't directly call the methods of the Vector type on the Wrapper, since wrapper is a new type.
But, we can get around this limitation by implementing the `Deref` trait on the Wrapper to return the inner type.
Then all vector methods is directly available with `Wrapper`.

ANother, use of the newtype pattern is in abstracting away some implementation details of a type: the new type can expose a public API that is different from the API of the private inner type if we used the new type directly to restrict the available functionality.

## Type Aliases

Type aliases give an existing type another name.
The main use case for type synonyms is to reduce repetition, for lengthy types.

With aliases we can call all methods that can be called on the original type and use any special operators like `?`, etc.

Examples:

```
type Int = i32; // Give an alias "Int" for type "i32"
let x: i32 = 5;
let y: Int = 5;

// Alias a length type like: type Thunk = Box<dyn Fn() + Send + 'static>;

// The std::io module in the standard library, uses an alias like: type Result<T> = std::result::Result<T, std::io::Error>;
// So, we can write `Result<usize>` instead of `Result<usize, Error>`
```

## Never type

Rust has a special type named ! (empty type).
Its also called the never type because it *stands in the place of the return type when a function will never return.*
Functions that return never are called diverging functions.

An expressions of type ! can be coerced into any other type.

Examples:

```
// Here, the function bar returns never
// Since it panics!
fn bar() -> ! { panic!(); }


// Continue has a ! value. That is, when Rust computes the type of guess, it looks at both match arms, the former with a value of u32
// and the latter with a ! value. Because ! can never have a value, Rust decides that the type of guess is u32.
loop {
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
}
```

## Dynamically Sized Types and the Sized Trait

Dynamically sized types or DSTs or unsized types, let us write code using values whose size we can know only at runtime.
`str` is a DST, since its size is not known at compile time that is we can’t know how long the string is until runtime,
meaning we can’t create a variable of type str.

Example:

```
let s1: str = "Hello there!"; // ERROR ! the size for values of type `str` cannot be known at compilation time
                              // = help: the trait `std::marker::Sized` is not implemented for `str`
                              // = note: all local variables must have a statically known size
```

This is because, the size of str depends upon the size of the string stored in it,
So, str = "cat" would need 3 bytes while str = "Elephant" woudl need 8 bytes, so we cant assign str a fixed size like other types like u32,f64, etc.

For, such types we instead use a pointer like `&str` in case of `str`, the pointer's size is known at compile time.
*The pointer &str has two values: the base address of the str and its length.*

In general, this is the way in which dynamically sized types are used in Rust: they have an extra bit of metadata that stores the size of the dynamic information. The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind.

Similary instead of using `&str`, we can use `str` with other types of pointers like `Box<Str>`, `Rc<str>`, etc.

This is also same with *trait objects*.
Thus, to use traits as trait objects, we must put them behind a pointer, such as &dyn Trait or `Box<dyn Trait>`, `Rc<dyn Trait>`, etc.

### Sized Trait

To work with DSTs, Rust has a particular trait called the *Sized trait to determine whether or not a type’s size is known at compile time. This trait is automatically implemented for everything whose size is known at compile time.*

When we define a function to use a generic type like: `fn generic<T>(t: T) {}`, it actually looks like: `fn generic<T: Sized>(t: T) {}`

**This means, by default generic functions will work only on types that have a known size at compile time.**

We can avoid the above sized restriction on generic types on functions be using `?Sized`.
```
fn generic<T: ?Sized>(t: &T) { // Use &T for generic type since its unsized!
    // --snip--
}
```
We would read this as “T may or may not be Sized.” This syntax is only available for Sized, not any other traits.
However, since now the generic type may be unsized(DST, size not known at compilel time) so we must keep it behind a pointer
**thus, we switched the type of the t parameter from T to &T**,
In this case, we’ve chosen a reference, we can use other pointers also like Box, Rc, etc.

## Function Pointers

We can padd closures as well ad regular funtions to other functions using function pointers.
When passing closures we use Fn traits are provided by the standard library that is Fn, FnMut and FnOnce.

However, **for function pointers we use the `fn` type.**
Unlike closures, fn is a type rather than a trait, so we specify fn as the parameter type directly rather than declaring a generic type parameter with one of the Fn traits as a trait bound.

Also, Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce), so you can always pass a function pointer as an argument for a function that expects a closure.

Examples:

```
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 { // Here `do_twice` accepts a function pointer like: f: fn(i32) -> i32
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

// ---------------------------------------------
// Another example:

let list_of_numbers = vec![1, 2, 3];

let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

// Here, we must use the fully qualified syntax since there are multiple functions available named to_string.

// ---------------------------------------------
// Another example:

// We can pass the enum value initalizers as a function pointer since
// the initializers are actually implemented as functions returning an instance that’s constructed from their arguments.

enum Status {
        Value(u32),
        Stop,
    }

let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect(); // WORKS ! Creates a vector of Status Enums
```

### Returning closures:

Inorder to pass closures to function we use traits like Fn, FnMut, FnOnce. Thus we can’t return closures directly from a function.

In other cases, when we need to return a trait from a function,
we usually return the concerete type that implements that trait instead, this works fine.
However, for closures there is no such create type.

So...

We are not allowed to use the function pointer fn as a return type.
We get error about the Sized trait. Rust doesn’t know how much space it will need to store the closure.

```
// ERROR ! `(dyn std::ops::Fn(i32) -> i32 + 'static)` doesn't have a size known at compile-time
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}

// When ever we want to return a DST(size not known at compile time), we can instead return a pointer
fn returns_closure() -> Box<dyn Fn(i32) -> i32> { // Works !
    Box::new(|x| x + 1)
}

// This works as rust now knowns the size of the pointer `Box<dyn Fn(i32) -> i32>`
```
