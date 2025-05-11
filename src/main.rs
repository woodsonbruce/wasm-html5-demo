use macroquad::prelude::*;

#[macroquad::main("Strife")]
async fn main() {
    let mut my_colors = Vec::<Color>::new();

    my_colors.push(BLACK);
    my_colors.push(BEIGE);
    my_colors.push(BLUE);
    my_colors.push(RED);
    my_colors.push(ORANGE);
    my_colors.push(BROWN);
    my_colors.push(DARKBLUE);
    my_colors.push(DARKBROWN);
    my_colors.push(DARKGRAY);
    my_colors.push(DARKGREEN);
    my_colors.push(DARKPURPLE);
    my_colors.push(YELLOW);
    my_colors.push(GRAY);
    my_colors.push(GREEN);
    my_colors.push(LIGHTGRAY);
    my_colors.push(LIME);
    my_colors.push(MAGENTA);
    my_colors.push(MAROON);
    my_colors.push(GOLD);
    my_colors.push(PINK);
    my_colors.push(PURPLE);
    my_colors.push(SKYBLUE);
    my_colors.push(VIOLET);
    my_colors.push(WHITE);

    const W: i32 = 600;
    const H: i32 = 600;
    const MAX_ITER: u32 = 90;

    let mut image = Image::gen_image_color(W as u16, H as u16, WHITE);
    let texture = Texture2D::from_image(&image);

    loop {
        clear_background(WHITE);
        if is_key_down(KeyCode::Space) {
            my_colors.rotate_right(1);
        }
        let seed = 0.5;
        let c = num::complex::Complex::new(seed, seed);
        for py in 0..H as u32 {
            let y_scaled = -1.0 + 2.0 * py as f32 / H as f32;
            for px in 0..W as u32 {
                let x_scaled = -1.0 + 2.0 * px as f32 / W as f32;
                let mut z = num::complex::Complex::new(x_scaled, y_scaled);
                let mut count = 0;
                while (z * z * z.sin()).norm() <= c.norm() && count < MAX_ITER {
                    z = z * z * z.sin() + c;
                    count += 1;
                }
                image.set_pixel(px, py, my_colors[count as usize % my_colors.len()]);
            }
        }
        texture.update(&image);
        draw_texture(&texture, 0., 0., WHITE);
        next_frame().await
    }
}
