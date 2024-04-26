use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::{Texture, WindowCanvas},
};
use specs::{Join, ReadStorage};

use sm::{PhysicsData, Sprite};

pub type SystemData<'a> = (ReadStorage<'a, PhysicsData>, ReadStorage<'a, Sprite>);

pub fn render(canvas: &mut WindowCanvas, color: Color, textures: &[Texture], data: SystemData) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    for (physics_data, sprite) in (&data.0, &data.1).join() {
        let screen_position = physics_data.position + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, sprite.current.width(), sprite.current.height());
        canvas.copy_ex(
            &textures[sprite.spritesheet],
            sprite.current,
            screen_rect,
            0.0,
            None,
            sprite.flip,
            false,
        )?;
    }

    canvas.present();

    Ok(())
}
