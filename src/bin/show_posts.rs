use self::models::*;
use diesel::prelude::*;
use actix_basic_crud::*;

fn main() {
    use self::schema::posts::dsl::*;
    let connection = &mut establish_connection();
    let result = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} post", result.len());
    for post in result {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}