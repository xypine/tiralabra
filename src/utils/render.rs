use palette::{FromColor, IntoColor, Oklab, Srgb, Srgba};

use crate::{
    grid::{GridInterface, dynamic_2d::DynamicSizeGrid2D},
    tile::{TileState, interface::TileInterface},
    utils::space::s2d::Location2D,
};

use super::space::s2d::{Direction2D, NEIGHBOUR_COUNT_2D};

pub trait CanvasRenderable<T: TileInterface<TileState> + Clone>:
    GridInterface<NEIGHBOUR_COUNT_2D, TileState, Location2D, Direction2D, T>
{
    fn render(&self, total_w: usize, total_h: usize, time: Option<usize>) -> String {
        let Location2D {
            x: width,
            y: height,
        } = self.get_dimensions();
        let tiles_at_t = time.map(|t| self.get_tiles_at_time(t));

        let tiles_x = width as f64;
        let tiles_y = height as f64;
        let cell_w = total_w as f64 / tiles_x;
        let cell_h = total_h as f64 / tiles_y;

        let mut out = format!(r#"<svg width="{total_w}" height="{total_h}">"#);

        for y in 0..height {
            for x in 0..width {
                let css_x = x as f64 * cell_w;
                let css_y = y as f64 * cell_h;
                let tile_opt = if let Some(ref tiles_at_t) = tiles_at_t {
                    tiles_at_t.get(&Location2D { x, y })
                } else {
                    self.get_tile(Location2D { x, y })
                };
                if let Some(tile) = tile_opt {
                    let states: Vec<_> = tile.possible_states_ref().collect();
                    if !states.is_empty() {
                        let mut lab_sum = Oklab::new(0.0, 0.0, 0.0);
                        let mut alpha_sum = 0.0;
                        let mut count = 0.0;

                        for state in states {
                            if let Some(color) = self.get_rules().represent_tile(*state) {
                                let a = ((color >> 24) & 0xFF) as f32 / 255.0;
                                let r = ((color >> 16) & 0xFF) as f32 / 255.0;
                                let g = ((color >> 8) & 0xFF) as f32 / 255.0;
                                let b = (color & 0xFF) as f32 / 255.0;

                                let srgba = Srgba::new(r, g, b, a);
                                let lab: Oklab = srgba.into_color();

                                lab_sum.l += lab.l;
                                lab_sum.a += lab.a;
                                lab_sum.b += lab.b;
                                alpha_sum += a;
                                count += 1.0;
                            }
                        }

                        if count > 0.0 {
                            let avg_lab =
                                Oklab::new(lab_sum.l / count, lab_sum.a / count, lab_sum.b / count);
                            let avg_alpha = alpha_sum / count;

                            let rgb: Srgb<f32> = Srgb::from_color(avg_lab).into_format();

                            let r = (rgb.red * 255.0).round() as u8;
                            let g = (rgb.green * 255.0).round() as u8;
                            let b = (rgb.blue * 255.0).round() as u8;
                            let a = avg_alpha;

                            let fill = format!("rgba({r},{g},{b},{a:.2})");

                            out.push_str(&format!(
                        r#"<rect x="{css_x}" y="{css_y}" width="{cell_w}" height="{cell_h}" fill="{fill}" />"#,
                    ));
                        }
                    }
                }
            }
        }

        out.push_str("</svg>");
        out
    }
}

impl<T: TileInterface<TileState> + Clone> CanvasRenderable<T> for DynamicSizeGrid2D where
    DynamicSizeGrid2D: GridInterface<4, TileState, Location2D, Direction2D, T>
{
}
