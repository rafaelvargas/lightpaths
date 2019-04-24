use std::fs::File;
use std::io::prelude::*;
use std::ops::{Index, IndexMut};

pub struct Image {
    height: usize,
    width: usize,
    pixelmap: Vec<Vec<Vec<u8>>>,
}

impl Image {
    pub fn new(height: usize, width: usize) -> Image {
        Image {
            height: height,
            width: width,
            pixelmap: vec![vec![vec![0; 3]; width]; height],
        }
    }

    pub fn write(&self, filepath: &str) -> std::io::Result<()> {
        let format = "P3";
        let max_value: u8 = 255;
        let mut file = File::create(filepath)?;
        file.write_fmt(format_args!(
            "{}\n{} {}\n{}\n",
            format, self.width, self.height, max_value
        ))?;
        for i in 0..self.height as usize {
            for j in 0..self.width as usize {
                file.write_fmt(format_args!(
                    "{} {} {}\n",
                    self.pixelmap[i][j][0], self.pixelmap[i][j][1], self.pixelmap[i][j][2]
                ))?;
            }
        }
        Ok(())
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
}

impl Index<usize> for Image {
    type Output = Vec<Vec<u8>>;

    fn index(&self, i: usize) -> &Vec<Vec<u8>> {
        &self.pixelmap[i]
    }
}

impl IndexMut<usize> for Image {
    fn index_mut(&mut self, i: usize) -> &mut Vec<Vec<u8>> {
        &mut self.pixelmap[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixel_color_assignment() {
        let mut image = Image::new(2, 2);
        image[0][0][0] = 255;
        image[0][0][1] = 127;
        image[0][0][2] = 10;
        assert_eq!(image[0][0], [255, 127, 10])
    }
}
