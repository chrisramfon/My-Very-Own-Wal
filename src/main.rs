mod services;

fn main() {

    let palette = services::palette_maker::PaletteMaker::get_palette( "/home/junior/Projects/My-Very-Own-Wal/wallhaven-e873x8.png" );

    println!( "{:?}", palette );

    // Como convertir un código rgb a hex
    println!( "#{:02X}{:02X}{:02X}", palette[ 0 ].r, palette[ 0 ].g, palette[ 0 ].b );



}


