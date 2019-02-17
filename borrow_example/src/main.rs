struct Book {
    title: String,
    pages: i32,
    finished: bool,
}

fn read_book(book: Book) {
    //.. reading
    println!("Reading {} - {} pages", book.title, book.pages);
}

/*
fn read_book( book: &Book) {

  //.. reading
  println!("Reading {} - {} pages", book.title, book.pages);
}
*/

/*
fn read_book( book: &mut Book) {
  //.. reading
  println!("Reading {} - {} pages", book.title, book.pages);
  book.finished = true;
}
*/

fn main() {
    let hhg = Book {
        title: "Hitchhikers Guide".to_string(),
        pages: 340,
        finished: false,
    };
    read_book(hhg);

    /*
    let hhg = Book { title: "Hitchhikers Guide".to_string(), pages: 340, finished: false };
    read_book( hhg );
    read_book( hhg );
    */

    /*
    let hhg = Book { title: "Hitchhikers Guide".to_string(), pages: 340, finished: false };
    read_book( &hhg );
    read_book( &hhg );
    */

    /*
    let mut hhg = Book { title: "Hitchhikers Guide".to_string(), pages: 340, finished: false };
    read_book( &mut hhg );
    */
}
