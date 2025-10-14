use image::ImageReader;
mod services;

fn main() {

    println!("Hello, world!");

    let message = "Testing a var!";

    println!("{}", message);

    // let img = match image::open( "/home/junior/Projects/My-Very-Own-Wal/wallhaven-e873x8.png" ) {
    //     Ok(data) => data,
    //     Err( e ) => {
    //         eprint!( "Error when opening an image." );
    //         return;
    //     }
    // };
    //
    // // let palette: Palette<f64> = Palette::extract( &image_data ).unwrap();
    //
    // let pixels = img.as_bytes();
    //
    // let palette = get_palette_with_options( &pixels, PixelEncoding::Rgba, Quality::new( 1 ), MaxColors::new( 9 ), PixelFilter::None );
    

    let palette = services::palette_maker::PaletteMaker::get_palette( "/home/junior/Projects/My-Very-Own-Wal/wallhaven-e873x8.png" );


    println!( "{:?}", palette[ 0 ] );

    println!( "#{:02X}{:02X}{:02X}", palette[ 0 ].r, palette[ 0 ].g, palette[ 0 ].b );



}


