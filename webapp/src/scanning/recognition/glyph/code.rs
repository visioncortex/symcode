use visioncortex::{BinaryImage, BoundingRect, PointF64, color_clusters::{Cluster, ClustersView}};
use web_sys::console;

use crate::{canvas::Canvas, math::euclid_dist_f64, scanning::{render_vec_image_rect_to_canvas}};

use super::{GlyphLabel, GlyphLibrary};

#[derive(Debug, Default)]
pub struct GlyphCode {
    glyphs: [GlyphLabel; Self::LENGTH],
}

/// Define the glyph anchors
impl GlyphCode {
    pub const CODE_WIDTH: usize = 400;
    pub const CODE_HEIGHT: usize = 390;
    
    /// Top-left corners of the glyphs, in U-shaped order
    const ANCHORS: [PointF64; Self::LENGTH] = [
        PointF64 {
            x: 40.0,
            y: 40.0,
        },
        PointF64 {
            x: 40.0,
            y: 160.0,
        },
        PointF64 {
            x: 160.0,
            y: 280.0,
        },
        PointF64 {
            x: 280.0,
            y: 160.0,
        },
        PointF64 {
            x: 280.0,
            y: 40.0,
        },
    ];
}

impl GlyphCode {
    /// Square bounding box
    pub const GLYPH_SIZE: usize = 80; // 80x80

    /// As GLYPH_SIZE is in object space, we can define the error tolerance based on GLYPH_SIZE on an absolute scale
    ///
    /// Allows fluctuations of up to this number of units in object space
    const GLYPH_SIZE_TOLERANCE: usize = 10;

    pub fn rect_not_too_large(rect: &BoundingRect) -> bool {
        let rect_longer_side = std::cmp::max(rect.width(), rect.height()) as usize;
        (rect_longer_side <= Self::GLYPH_SIZE) || (rect_longer_side - Self::GLYPH_SIZE <= Self::GLYPH_SIZE_TOLERANCE) 
    }

    pub fn rect_not_too_small(rect: &BoundingRect) -> bool {
        let rect_shorter_side = std::cmp::min(rect.width(), rect.height()) as usize;
        (rect_shorter_side >= Self::GLYPH_SIZE) || (Self::GLYPH_SIZE - rect_shorter_side <= Self::GLYPH_SIZE_TOLERANCE) 
    }

    /// True if the bounding rect is approximately the same size as a valid glyph.
    pub fn rect_size_is_reasonable(rect: &BoundingRect) -> bool {
        /*
        let width = rect.width() as usize;
        let height = rect.height() as usize;

        let reasonable_error = 
            |a: usize| {(std::cmp::max(a, Self::GLYPH_SIZE) - std::cmp::min(a, Self::GLYPH_SIZE)) <= Self::GLYPH_SIZE_TOLERANCE};
        
        reasonable_error(width) && reasonable_error(height)
        */
        Self::rect_not_too_large(rect) && Self::rect_not_too_small(rect)
    }
}

impl GlyphCode {
    const LENGTH: usize = 5;

    /// Given cropped images and their bounding rects (effectively `clusters`), for each anchor, check which cluster is the closest (and is close enough) and flag the glyph at that anchor
    pub fn add_clusters_near_anchors(&mut self, clusters: Vec<(BinaryImage, BoundingRect)>, error_threshold: f64, glyph_library: &GlyphLibrary, debug_canvas: &Canvas) {
        
        let clusters: Vec<(BinaryImage, BoundingRect)> =
            clusters.into_iter()
                .filter(|(_, rect)| Self::rect_size_is_reasonable(rect))
                .collect();
        render_vec_image_rect_to_canvas(&clusters, debug_canvas);

        for (i, anchor) in Self::ANCHORS.iter().enumerate() {
            let closest_cluster = Self::find_closest_cluster(anchor, &clusters, error_threshold);
            self.set_glyph_with_cluster(i, closest_cluster, &glyph_library);
        }
    }

    /// Find the cluster in clusters that is the closest to point, with error smaller than the error_threshold.
    fn find_closest_cluster(point: &PointF64, clusters: &[(BinaryImage, BoundingRect)], error_threshold: f64) -> Option<BinaryImage> {
        let eval_error = |p: &PointF64, rect: &BoundingRect| {euclid_dist_f64(&p, &PointF64::new(rect.left as f64, rect.top as f64))};
        
        let (closest_cluster_index, _,  min_error) =
            clusters.iter().enumerate().skip(1)
            // Find the cluster with minimum error
            .fold((0, &clusters[0].1, eval_error(point, &clusters[0].1)), |min_tuple, (index, (_, rect))| {
                let error = eval_error(point, rect);
                if error < min_tuple.2 {
                    (index, rect, error)
                } else {
                    min_tuple
                }
            });

        if min_error > error_threshold {
            None
        } else {
            Some(clusters[closest_cluster_index].0.clone())
        }
    }

    fn set_glyph_with_cluster(&mut self, i: usize, cluster: Option<BinaryImage>, glyph_library: &GlyphLibrary) {
        if let Some(cluster) = cluster {
            self.glyphs[i] = glyph_library.find_most_similar_glyph(cluster);
        } else {
            self.glyphs[i] = GlyphLabel::Empty;
        }
    }
}