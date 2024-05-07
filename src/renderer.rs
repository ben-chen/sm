use sdl2::{
    gfx::primitives::DrawRenderer,
    pixels::Color,
    rect::Rect,
    render::{Texture, TextureCreator, WindowCanvas},
    ttf::Font,
    video::WindowContext,
};
use specs::{Join, ReadStorage, World, WorldExt};

use crate::{Framerate, PhysicsData, PointFi32, Sprite};

pub type SystemData<'a> = (ReadStorage<'a, PhysicsData>, ReadStorage<'a, Sprite>);

pub fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture_creator: &TextureCreator<WindowContext>,
    textures: &[Texture],
    font: &Font,
    world: &World,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;
    let data = world.system_data::<SystemData>();

    for (physics_data, sprite) in (&data.0, &data.1).join() {
        let screen_position = physics_data.position + PointFi32::new(width / 2, height / 2);
        let screen_rect = Rect::from_center(
            screen_position,
            sprite.current.width(),
            sprite.current.height(),
        );
        if sprite.glow {
            canvas.aa_circle(
                screen_position.x.to_num(),
                screen_position.y.to_num(),
                36,
                Color::RED,
            )?;
        }
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
    // Draw the Framerate
    let fps: Framerate = *world.read_resource();
    let fps = fps.get().to_string();
    let surface = font
        .render(&fps)
        .blended(Color::WHITE)
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    let (surface_width, surface_height) = surface.size();
    let target = Rect::new(
        (width - surface_width) as i32,
        0,
        surface_width,
        surface_height,
    );
    canvas.copy(&texture, None, target)?;

    canvas.present();

    Ok(())
}
