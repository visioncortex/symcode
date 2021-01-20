use visioncortex::{ColorImage};

use crate::{math::PerspectiveTransform, scanning::{GlyphReader, SymcodeConfig, pipeline::ScanningProcessor}};

use super::{GlyphLabel, GlyphLibrary};

/// An implementation of GlyphReader
pub struct Recognizer;

impl GlyphReader for Recognizer {
    type Label = GlyphLabel;

    type Library = GlyphLibrary;

    fn find_most_similar_glyph(image: visioncortex::BinaryImage, glyph_library: &Self::Library, symcode_config: &crate::scanning::SymcodeConfig) -> Self::Label {
        glyph_library.find_most_similar_glyph(
            image,
            symcode_config
        )
    }
}

pub struct RecognizerInput {
    pub raw_frame: ColorImage,
    pub image_to_object: PerspectiveTransform,
    pub glyph_library: *const GlyphLibrary,
}

impl ScanningProcessor for Recognizer {
    type Input = RecognizerInput;

    type Output = Vec<Option<GlyphLabel>>;

    type Params = SymcodeConfig;

    fn process(input: Self::Input, params: &Option<Self::Params>) -> Result<Self::Output, &str> {
        Self::valid_input_and_params(&input, params)?;

        let params = if let Some(params) = params {params} else {
            return Err("Recognizer Processor expects params.");
        };

        // Processing starts
        let glyph_library = unsafe {&*input.glyph_library};
        Ok(Self::read_glyphs_from_raw_frame(input.raw_frame, input.image_to_object, glyph_library, params))
    }
}