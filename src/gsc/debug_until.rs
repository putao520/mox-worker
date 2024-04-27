use anyhow::Result;
use base64::{Engine as _, engine::general_purpose};
use image::{load_from_memory, Pixel};
use image::GenericImageView;

// 图片绘制
pub fn draw_image(base64_string: String) -> Result<()> {
    let image_data = general_purpose::STANDARD.decode(base64_string)?;

    // 从内存中加载图像
    let img = load_from_memory(&image_data)?;

    // 获取图像的尺寸
    let (width, height) = img.dimensions();

    // 遍历每个像素并将其转换为字符
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let ch = pixel.channels();
            let grayscale = if ch.len() == 3 {
                ((ch[0] as u32 + ch[1] as u32 + ch[2] as u32) / 3) as u8
            } else {
                ch[0]
            };
            // 根据灰度值选择字符
            let character = match grayscale {
                0..=30 => '@',
                31..=60 => '#',
                61..=120 => '*',
                121..=180 => '+',
                181..=220 => '-',
                _ => ' ',
            };
            print!("{}", character);
        }
        println!(); // 换行
    }

    Ok(())
}

// 验证码输入界面
pub fn captcha_input() -> Result<String> {
    use std::io::{self, Write};

    print!("请输入验证码: ");
    io::stdout().flush()?;

    let mut captcha = String::new();
    io::stdin().read_line(&mut captcha)?;

    Ok(captcha.trim().to_string())
}