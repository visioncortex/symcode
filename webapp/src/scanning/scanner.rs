use wasm_bindgen::prelude::*;

use crate::{canvas::Canvas, util::console_log_util};

use super::{AlphabetReader, AlphabetReaderParams, FinderCandidate, GlyphLibrary, Recognizer, RecognizerInput, SymcodeConfig, binarize_image_util, implementation::transformer::{Transformer, TransformerInput}, is_black_hsv, pipeline::ScanningProcessor, render_binary_image_to_canvas};

#[wasm_bindgen]
pub struct SymcodeScanner {
    glyph_library: GlyphLibrary,
    /// Used only in building the library, but not in scanning
    stat_tolerance: f64,
}

impl Default for SymcodeScanner {
    fn default() -> Self {
        Self {
            glyph_library: GlyphLibrary::default(),
            stat_tolerance: 0.2,
        }
    }
}

#[wasm_bindgen]
impl SymcodeScanner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_stat_tolerance(stat_tolerance: f64) -> Self {
        Self {
            glyph_library: GlyphLibrary::default(),
            stat_tolerance,
        }
    }

    /// Takes the id of the canvas element storing the template image, and the usize representation of the glyph label
    pub fn load_template_from_canvas_id(&mut self, canvas_id: &str) {
        let canvas = &match Canvas::new_from_id(canvas_id) {
            Some(c) => c,
            None => panic!("Canvas with id ".to_owned() + canvas_id + " is not found!"),
        };
        let image = canvas
            .get_image_data_as_color_image(0, 0, canvas.width() as u32, canvas.height() as u32)
            .to_binary_image(|c| is_black_hsv(&c.to_hsv()));
        self.glyph_library.add_template(image, self.stat_tolerance);
    }

    /// Takes the id of the canvas element storing the alphabet.
    pub fn load_alphabet_from_canvas_id(&mut self, canvas_id: &str, params: AlphabetReaderParams) {
        let canvas = &match Canvas::new_from_id(canvas_id) {
            Some(c) => c,
            None => panic!("Canvas with id ".to_owned() + canvas_id + " is not found!"),
        };
        let image = canvas
            .get_image_data_as_color_image(0, 0, canvas.width() as u32, canvas.height() as u32)
            .to_binary_image(|c| is_black_hsv(&c.to_hsv()));
        AlphabetReader::read_alphabet_to_library(&mut self.glyph_library, image, params, self.stat_tolerance);
    }

    pub fn scan_with_config(&self, symcode_config: SymcodeConfig) -> JsValue {
        if self.glyph_library.is_empty() {
            return "No templates loaded into RawScanner object yet!".into();
        }

        // Stage 0: Prepare the raw input
        let raw_frame = if let Some(canvas) = &symcode_config.canvas {
            canvas.get_image_data_as_color_image(0, 0, canvas.width() as u32, canvas.height() as u32)
        } else {
            panic!("Cannot read input image from canvas.");
        };
        let binary_raw_frame = binarize_image_util(&raw_frame);
        // Take a look
        if let Some(canvas) = &symcode_config.canvas {
            render_binary_image_to_canvas(&binary_raw_frame, canvas);
        }

        // Wrap the config in a option so that it can be flexibly reused by other components later on
        let symcode_config = &Some(symcode_config);
        
        // Stage 1: Locate finder candidates
        let finder_positions = match FinderCandidate::process(
            binary_raw_frame,
            symcode_config
        ) {
            Ok(finder_positions) => finder_positions,
            Err(e) => {
                return e.into();
            }
        };
        
        // Stage 2: Rectify the raw image using the correct perspective transform
        let rectified_image = match Transformer::process(
            TransformerInput {
                raw_image: raw_frame,
                finder_positions_image: finder_positions,
            },
            symcode_config
        ) {
            Ok(rectified_image) => rectified_image,
            Err(e) => {
                return e.into();
            }
        };

        // Stage 3: Recognize the glyphs
        match Recognizer::process(
            RecognizerInput {
                rectified_image,
                glyph_library: &self.glyph_library,
            },
            symcode_config
        ) {
            Ok(glyph_code) => {
                console_log_util(&format!("{:?}", glyph_code));
                
                "Success".into()
            },
            Err(e) => {
                e.into()
            }
        }
    }
}