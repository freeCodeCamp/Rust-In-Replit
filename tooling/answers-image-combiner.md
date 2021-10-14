# freeCodeCamp - Rust in Replit - Image Combiner

## 1

### --description--

Start by creating a new crate called `combiner`.

Run `fcc test 1` to see if you completed the task correctly.

### --seed--

```rust
// Lesson #1
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- You should see a new directory `combiner` created in the root.
- `null`

## 2

### --description--

In this project, you will be creating a CLI (combiner) which expects three arguments:

```bash
  $ combiner image1.png image2.png output.png
```

The first two arguments are the paths to the images you want to combine. The third argument is the path to the output image.

Task: Run `cargo run --bin combiner` to see if your application is set up correctly.

You should see `Hello, world!` printed to the console.

### --seed--

```rust
// Lesson #2
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- Your code should output `Hello, world!`
- `getCommandOutput(Hello, world!)`

## 3

### --description--

Text

### --seed--

```rust
// Lesson #3
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- Run `cargo test --bin combiner` to see if you completed the task correctly.
- `null`

## Final

### --description--

### --seed--

```rust
// Lesson #Final
use image::{io::Reader, DynamicImage, GenericImageView, ImageFormat};
use std::convert::TryInto;
mod args;
pub use args::{get_nth_arg, Args};

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

#[derive(Debug)]
enum ImageDataErrors {
  BufferToSmall,
  DifferentImageFormats,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> FloatingImage {
    // Initialise image with buffer based on RGBA data
    let buffer_capacity = (width * height * 4).try_into().unwrap();
    let buffer = Vec::with_capacity(buffer_capacity);

    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
  // TODO: Turn into Result?
  fn set_data(&mut self, data: Vec<u8>) -> Result<&str, ImageDataErrors> {
    if data.len() > self.data.capacity() {
      return Err(ImageDataErrors::BufferToSmall);
    }
    // TODO: Un-hard-code expected_len
    let expected_len = self.width * self.height * 4;
    let mut temp = data.clone();
    temp.resize(expected_len.try_into().unwrap(), 0);
    self.data = temp;
    Ok("Data has been set")
  }
}

fn main() -> Result<(), ImageDataErrors> {
  // Get command line arguments - paths to images
  let args: Args = Args::new();

  // Create Image objects
  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(ImageDataErrors::DifferentImageFormats);
  }

  let (width, height) = get_largest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {} , height: {}\n", width, height);
  let mut output = FloatingImage::new(width, height, args.output);

  let comb = combine_images(&image_1, image_2);

  output.set_data(comb)?;

  image::save_buffer_with_format(
    output.name,
    &output.data,
    output.width,
    output.height,
    image::ColorType::Rgba8,
    image_1_format,
  )
  .expect("Unable to save combined image");
  Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).expect("Image not found");
  let image_format = image_reader.format().expect("Image format undeterminable");
  let image = image_reader.decode().expect("Image data invalid");
  (image, image_format)
}

fn combine_images(image_1: &DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  // Turn image into vector of RGBA u8 -> one pixel == [u8, u8, u8, u8]
  let data_1 = image_1.to_rgba8();
  let data_2 = image_2.to_rgba8();
  let vec_1: Vec<u8> = data_1.into_vec();
  let vec_2: Vec<u8> = data_2.into_vec();

  alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  // Set buffer for combined pixel data
  let largest_len = vec_1.len().max(vec_2.len());
  let mut combined_data = vec![0u8; largest_len];

  println!("{:?}, {:?}", &vec_1.len(), &vec_2.len());

  let mut i = 0;
  while i < vec_1.len() {
    if i % 8 == 0 {
      combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
    } else {
      combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
    }
    i += 4;
  }
  combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val = match vec.get(i) {
      Some(d) => *d,
      None => 0,
    };
    rgba.push(val);
  }
  rgba
}

fn get_largest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  return dim_1.max(dim_2);
}

#[cfg(test)]
mod tests {
  use image::GenericImageView;
  use image::ImageFormat;

  use crate::alternate_pixels;
  use crate::find_image_from_path;
  use crate::get_largest_dimensions;
  use crate::set_rgba;

  #[test]
  fn main_contains_arg_declaration() {
    let contents = return_file_in_src("main.rs");
    assert!(reg_with_con(r"pub use arg", contents));
  }
  #[test]
  fn finding_fcc_glyph_format_is_png() {
    let (_, image_format) = find_image_from_path("./images/fcc_glyph.png".to_string());
    assert_eq!(image_format, ImageFormat::Png);
  }
  #[test]
  fn finding_fcc_glyph_image_dimensions() {
    let (image, _) = find_image_from_path("./images/fcc_glyph.png".to_string());
    let dimensions = image.dimensions();
    println!("{:?}", dimensions);
    assert_eq!(dimensions, (712, 484));
  }
  #[test]
  #[should_panic]
  fn finding_image_from_non_existant_path_panics() {
    find_image_from_path("path/does/not/exist".to_string());
  }

  #[test]
  fn rgba_set_returned() {
    let vec = vec![0, 1, 2, 3, 4, 5, 6, 7];
    assert_eq!(set_rgba(&vec, 4, 4 + 3), vec![4, 5, 6, 7]);
  }

  #[test]
  fn vector_members_are_alternated() {
    let vec_1 = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23];
    let vec_2 = vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22];
    assert_eq!(
      alternate_pixels(vec_1, vec_2),
      vec![1, 3, 5, 7, 8, 10, 12, 14, 17, 19, 21, 23]
    );
  }
  #[test]
  fn largest_tuple() {
    let smaller_tup = (10, 10);
    let larger_tup = (10, 11);
    assert_eq!(get_largest_dimensions(smaller_tup, larger_tup), (10, 11));
  }

  fn reg_with_con(regex: &str, file_contents: String) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(&file_contents)
  }

  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from(""),
    }
  }
}
```

### --tests--

## 100
