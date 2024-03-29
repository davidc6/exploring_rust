use num::Complex;
use std::str::FromStr;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::env;

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

// opens a file and attempts to write the image to it
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    // ? operator is a shorthand for the code below which returns error if it fails or successfully opend file
    // let output = File::create(path)?;
    // handle error happy and sad paths
    let output = match File::create(filename) {
        Ok(f) => f,
        Err(e) => {
            return Err(e);
        }
    };
    let encoder = PNGEncoder::new(output);
    // pixels data, width and height from bounds and bytes in pixels in 8 bit grayscale value
    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
    Ok(())
}

// &mut [u8] - mutable reference to vector of unsigned integers
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixels_to_point(bounds, (column, row), upper_left, lower_right);

            pixels[row * bounds.0 + column] = 
                match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8
                }
        }
    }
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) { // find the location of a separator
        None => None, // no such location is found
        Some(index) => { // location found
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) { // get string portion before the separator and after
                (Ok(l), Ok(r)) => Some((l, r)), // left and right values
                _ => None
            }
        }
    }
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') { // parse pair of numbers first
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}

fn pixels_to_point(
    bounds: (usize, usize), 
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>
) -> Complex<f64> {
    let (
        width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im
    );

    Complex { 
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64, // "as" Rust's syntax for a type conversion
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // check cli parameters
    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }

    // create bounds, left and right corner
    let bounds = parse_pair(&args[2], 'x')
        .expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3])
        .expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4])
        .expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    // work out rows of pixels each band should have given that we have 8 threads
    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;

    {
        // divides pixel buffer into bands
        // produce mutable, nonoverlaping slices of the buffer (rows of pixels)
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();

        // we use crossbeam here to enable multi-threading
        // a closure that expects a single "spawner" argument
        // closure fn argument type gets inferred
        // ::scope also ensures that all threads have completed before returning
        // if none of the threads panic, crossbeam returns Ok() enum variant or Err() if any of threads panic
        crossbeam::scope(|spawner| {
            // iterate over the pixel buffer's bands
            // into_iter() gives exclusive ownership of one band to each iteration
            // i.e. one thread at a time can write to it
            // enumerate() produces tuples (each vector element, index)
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixels_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right = pixels_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                // create a thread
                // "move" indicates that the closure takes ownership of the variables it uses
                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        }).unwrap();
    }

    write_image(&args[1], &pixels, bounds)
        .expect("error writing PNG file");
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625"), None);
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixels_to_point(
        (100, 200),
        (25, 175),
        Complex { re: -1.0, im: 1.0 },
        Complex { re: 1.0, im: -1.0 }), Complex { re: -0.5, im: -0.75 })
}

#[test]
fn test_escape_time() {
    assert_eq!(escape_time(Complex { re: 1.25, im: -0.0625 }, 6), Some(2));
}
