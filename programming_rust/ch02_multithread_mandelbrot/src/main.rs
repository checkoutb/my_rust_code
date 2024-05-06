
use num::Complex;

use std::str::FromStr;

fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0};
    loop {
        z = z * z + c;
    }
}

/// 尝试测定`c`是否位于曼德博集中，使用最多`limit`次迭代来判定
///
/// 如果`c`不是集合成员之一，则返回`Some(i)`，其中的`i`是`c`离开以原点
/// 为中心的半径为2的圆时所需的迭代次数。如果`c`似乎是集合成员之一（确
/// 切而言是达到了迭代次数限制但仍然无法证明`c`不是成员），则返回`None`
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

// 泛型函数
/// 把字符串`s`（形如`"400x600"`或`"1.0,0.5"`）解析成一个坐标对
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}


fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex {re, im}),
        None => None
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex{re: 1.25, im: -0.0625}));
    assert_eq!(parse_complex(",-0.0962"), None);
}


fn pixel_to_point(bounds: (usize, usize), pixel: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) -> Complex<f64> {
    let (w, h) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 * w / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * h / bounds.1 as f64
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 200), (25, 175), Complex {re: -1.0, im: 1.0}, Complex {re: 1.0, im: -1.0}), Complex{re: -0.5, im: -0.75});
}

fn render(pixels: &mut [u8], bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>)
{
    assert!(pixels.len() == bounds.0 * bounds.1);

    for r in 0..bounds.1 {
        for c in 0..bounds.0 {
            let point = pixel_to_point(bounds, (c, r), upper_left, lower_right);
            pixels[r * bounds.0 + c] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8
            };
        }
    }
}


use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    // 等价于
    // let output = match File::create(filename) {
    //     Ok(f) => f,
    //     Err(e) => {
    //     return Err(e);
    //     }
    // };

    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
    Ok(())
}


use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("usage: {} file pixels upperleft, lowerright", args[0]);
        eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }
    let bounds = parse_pair(&args[2], 'x').expect("error bounds");
    let upper_left = parse_complex(&args[3]).expect("error upper_left");
    let lower_right = parse_complex(&args[4]).expect("error lower_right");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    // concurrency
    // render(&mut pixels, bounds, upper_left, lower_right);
    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;
    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        }).unwrap();
    }

    
    write_image(&args[1], &pixels, bounds).expect("error write pgn file");
}