use palette_extract::{get_palette_with_options, PixelEncoding, Quality, MaxColors, PixelFilter};

pub struct PaletteMaker;

impl PaletteMaker {


    pub fn get_palette( image_path: &str ) -> Vec<palette_extract::Color> {

        let opened_image = image::open( &image_path );

        let image = match opened_image {
            Ok( data ) => data,
            Err( error ) => panic!( "There was a problem when oppening the image {:?}", error )
        };

        let image_pixels = image.as_bytes();

        let palette = get_palette_with_options( &image_pixels, PixelEncoding::Rgba, Quality::new( 1 ), MaxColors::new( 9 ), PixelFilter::None );

        return palette;

    }

}
