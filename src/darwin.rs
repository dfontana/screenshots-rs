use crate::{Image, Screen};
use core_graphics::display::{CGDisplay, CGPoint, CGRect, CGSize};

pub fn capture_screen(screen: &Screen) -> Option<Image> {
  let cg_display = CGDisplay::new(screen.id);
  let cg_image_raw = cg_display.image()?;
  let bounds = CGRect::new(
    &CGPoint::new(screen.x as f64, screen.y as f64),
    &CGSize::new(screen.width as f64, screen.height as f64),
  );
  let cg_image = cg_image_raw.cropped(bounds)?;

  match Image::from_bgr(
    cg_image.width() as u32,
    cg_image.height() as u32,
    Vec::from(cg_image.data().bytes()),
  ) {
    Ok(image) => Some(image),
    Err(_) => None,
  }
}
