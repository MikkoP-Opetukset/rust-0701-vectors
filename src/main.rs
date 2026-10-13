#![allow(dead_code, unused_variables, unused_mut)]

/// Entry point for Chapter 8.1: Storing Lists of Values with Vectors
///
/// A vector, `Vec<T>`, stores a growable list of values of the same type next
/// to each other in memory. This file continues the small game-world theme
/// from the earlier demos.
fn main() {
    creating_vectors();
    updating_vectors();
    reading_elements();
    iterating_over_vectors();
    storing_multiple_types_with_enums();
    dropping_a_vector();
}

/// A thing a character can carry. Used to store different kinds of values in
/// one vector.
enum Item {
    Gold(u32),
    Potion { name: String, healing: u32 },
    Key(String),
}

/// # Creating Vectors
/// - `Vec::new()` creates an empty vector. Because nothing is stored yet, the
///   element type must be annotated or inferred from later use.
/// - The `vec!` macro creates a vector with initial values and infers the type.
/// - All elements of a vector must have the same type.
fn creating_vectors() {
    println!("\n{:=>80}", "");
    println!("creating_vectors()\n");

    let empty: Vec<i32> = Vec::new();
    // println! formats text using placeholders in curly braces:
    // - `{}` inserts a value using its `Display` format, as in `{}` with
    //   `empty.len()` as the extra argument.
    // - `{name}` inserts the variable with that name directly from scope.
    // - `{name:?}` uses the `Debug` format instead. Vectors do not implement
    //   `Display`, but `Debug` prints them as a list such as `[1, 2, 3]`.
    println!("empty = {empty:?}, length = {}", empty.len());

    let party_levels = vec![12, 9, 15];
    println!("party_levels = {party_levels:?}");

    // A repeated value can be created with vec![value; count].
    let health_bars = vec![100; 4];
    println!("health_bars = {health_bars:?}");

    // All elements must have the same type.
    // let mixed = vec![1, "two", 3.0]; // Uncomment to see the error.
}

/// # Updating Vectors
/// - A vector must be declared `mut` before elements can be added or removed.
/// - `push` adds an element to the end.
/// - `pop` removes the last element and returns it as an `Option<T>`.
/// - `insert` and `remove` work at a given index and shift later elements.
fn updating_vectors() {
    println!("\n{:=>80}", "");
    println!("updating_vectors()\n");

    let mut inventory = Vec::new();
    inventory.push("sword");
    inventory.push("shield");
    inventory.push("torch");
    println!("after push: {inventory:?}");

    inventory.insert(1, "bow");
    println!("after insert at 1: {inventory:?}");

    let removed = inventory.remove(0);
    println!("removed {removed}: {inventory:?}");

    let last = inventory.pop();
    println!("popped {last:?}: {inventory:?}");

    // Without `mut`, the vector cannot change.
    // let frozen = vec![1, 2, 3];
    // frozen.push(4); // Uncomment to see the error.
}

/// # Reading Elements
/// - Indexing with `&v[i]` panics if the index is out of range.
/// - `v.get(i)` returns an `Option<&T>`, so the program decides what to do
///   when the element is missing.
/// - Holding a reference to an element borrows the whole vector. Adding an
///   element may reallocate the memory, so the borrow checker forbids
///   changing the vector while such a reference is alive.
fn reading_elements() {
    println!("\n{:=>80}", "");
    println!("reading_elements()\n");

    let mut scores = vec![10, 20, 30, 40, 50];

    let third = &scores[2];
    println!("The third score is {third}.");

    match scores.get(2) {
        Some(score) => println!("get(2) found {score}."),
        None => println!("get(2) found nothing."),
    }

    match scores.get(100) {
        Some(score) => println!("get(100) found {score}."),
        None => println!("get(100) found nothing."),
    }

    // Indexing outside the vector panics at runtime.
    // let missing = &scores[100]; // Uncomment to see the panic.

    // The borrow of `first` is still alive when `push` needs a mutable borrow.
    // let first = &scores[0];
    // scores.push(60);
    // println!("The first score is {first}."); // Uncomment to see the error.
    scores.push(60);
}

/// # Iterating over Vectors
/// - A `for` loop over `&v` gives immutable references to each element.
/// - A `for` loop over `&mut v` gives mutable references. Use `*` to
///   dereference and change the value.
/// - A `for` loop over `v` itself consumes the vector.
fn iterating_over_vectors() {
    println!("\n{:=>80}", "");
    println!("iterating_over_vectors()\n");

    let mut damage = vec![5, 8, 13];

    for hit in &damage {
        println!("hit for {hit}");
    }

    for hit in &mut damage {
        *hit += 2;
    }
    println!("after a damage buff: {damage:?}");

    for hit in damage {
        println!("consumed {hit}");
    }

    // `damage` was moved into the loop above.
    // println!("{damage:?}"); // Uncomment to see the error.
}

/// # Storing Multiple Types with Enums
/// - A vector can hold only one type, but enum variants all belong to the
///   same enum type.
/// - An enum therefore lets a vector hold values of different shapes.
/// - The set of variants must be known at compile time.
fn storing_multiple_types_with_enums() {
    println!("\n{:=>80}", "");
    println!("storing_multiple_types_with_enums()\n");

    let backpack = vec![
        Item::Gold(250),
        Item::Potion {
            name: String::from("Elixir"),
            healing: 40,
        },
        Item::Key(String::from("Dungeon")),
    ];

    for item in &backpack {
        match item {
            Item::Gold(amount) => println!("{amount} gold coins"),
            Item::Potion { name, healing } => println!("{name} potion heals {healing}"),
            Item::Key(door) => println!("Key to the {door}"),
        }
    }
}

/// # Dropping a Vector
/// - A vector is freed when it goes out of scope, along with its elements.
/// - References to elements cannot outlive the vector.
fn dropping_a_vector() {
    println!("\n{:=>80}", "");
    println!("dropping_a_vector()\n");

    {
        let temporary = vec![String::from("short-lived"), String::from("values")];
        println!("inside the scope: {temporary:?}");
    } // `temporary` and its Strings are dropped here.

    // println!("{temporary:?}"); // Uncomment to see the error.
    println!("The vector has been dropped.");
}
