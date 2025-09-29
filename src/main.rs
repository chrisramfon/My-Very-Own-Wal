use auto_palette::{ImageData, Palette};

fn main() {

    println!("Hello, world!");

    let message = "Testing a var!";

    println!("{}", message);

    let image_data = match ImageData::load( "/home/chrisramfon/Projects/new_wal/wallhaven-e873x8.png" ) {
        Ok(data) => data,
        Err(e) => {
            eprintln!( "Error when opening the image: {}", e );
            return;
        }
    };


    let palette: Palette<f64> = Palette::extract( &image_data ).unwrap();

    println!( "{:?}", palette );



}


