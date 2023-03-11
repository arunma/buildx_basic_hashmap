use buildx_basic_hashmap::HashMap;

fn main() {
    let mut book_reviews = HashMap::new();

    book_reviews.insert("Adventures of Huckleberry Finn", "My favorite book.");
    book_reviews.insert("Grimms' Fairy Tales", "Masterpiece.");
    book_reviews.insert("Pride and Prejudice", "Very enjoyable.");
    book_reviews.insert("The Adventures of Sherlock Holmes", "Eye lyked it alot.");

    if !book_reviews.contains_key("Les Miserables") {
        println!(
            "We've got {} reviews but Les Miserables isn't one",
            book_reviews.len()
        );
    }

    book_reviews.remove("The Adventures of Sherlock Holmes");

    let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];

    for &book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{}:{}", book, review),
            None => println!("{} is unreviewed", book),
        }
    }

    for (book, review) in &book_reviews {
        println!("{book}:{review}")
    }
}
