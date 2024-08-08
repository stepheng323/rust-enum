#[derive(Debug)]
enum Media {
    Book {title: String, author: String },
    Movie {title: String, director: String },
    AudioBook {title: String }
}

impl Media {

    fn description (&self) -> String {

        match self {
            Media::Book {title, author} => {
                    format!("{} {}", title, author)
            }
            Media::Movie {title, director} => {
                    format!("{} {}", title, director)
            }            Media::AudioBook {title} => {
                    format!("{}", title)
            }
        }
        // if let Media::AudioBook {title} = self {
        //     format!("{}", title)
        // }
        // else if let Media::Book {title, author} =self {
        //     format!("{} {}", title, author)
        // }
        // else if let Media::Movie { title, director }  = self {
        //     format!("{} {}", title, director)
        // }
        // else { String::from("A random description") }
    }
}


struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    fn new (&self) -> self {
        Catalog {items: vec![]}
    }
}

fn print_media(media: Media) {
    println!("{:?}", media)
}

fn main() {

    let audio_book = Media::AudioBook {
        title: String::from("My audio book")
    };

    let good_movie = Media::Movie {
        title: String::from("The last air bender"),
        director: String::from("Avatar")
    };

    let bad_book = Media::Book {
        title: String::from("A bad book"),
        author: String::from("Jon Doe")
    };

    println!("{}", audio_book.description());
    println!("{}", bad_book.description());
    println!("{}", good_movie.description())
    // print_media(audio_book.);
    // print_media(bad_book);
    // print_media(good_movie);

}
