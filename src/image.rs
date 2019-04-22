use std::fs::File;
use std::io::prelude::*;

pub struct Image {
    height: u8,
    width: u8,
    pixelmap: Vec<Vec<Vec<u8>>>,
}

impl Image {
    pub fn new(height: usize, width: usize) -> Image {
        Image {
            height: height as u8,
            width: width as u8,
            pixelmap: vec![vec![vec![0; 3]; width]; height],
        }
    }

    pub fn print_pixel(&self, x: usize, y: usize) {
        println!(
            "{} {} {}",
            self.pixelmap[y][x][0], self.pixelmap[y][x][1], self.pixelmap[y][x][2]
        );
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
}
