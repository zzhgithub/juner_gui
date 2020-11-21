use alloc::vec::Vec;
use core::iter;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::{drawable::Pixel, prelude::Point};
use hashbrown::HashMap;

// 一个 根据工件方法生成 图像的工具类
pub fn image_by_text(
    text: &[&str],
    position: (u32, u32),
    map: HashMap<char, Rgb888>,
) -> Vec<Pixel<Rgb888>> {
    let width = text.first().map_or(0, |row| row.len());
    let height = text.len();
    let (x, y) = position;
    let mut i = 0;

    let all = text
        .iter()
        .flat_map(|raw| {
            i += 1;
            let mut j = 0;
            raw.chars()
                .map(move |c| {
                    j += 1;
                    match c {
                        ' ' => None,
                        _ => Some((i, j, c)),
                    }
                })
                .chain(iter::repeat(None))
                .take(width)
        })
        .chain(iter::repeat(None))
        .take(width * height);

    let mut image: Vec<Pixel<Rgb888>> = Vec::new();
    // 然后循环遍历
    for w in all {
        match w {
            Some((i, j, k)) => match map.get(&k) {
                Some(color) => image.push(Pixel(
                    Point::new((x + j) as i32, (y + i) as i32),
                    color.clone(),
                )),
                None => {}
            },
            None => {}
        }
    }
    image
}
