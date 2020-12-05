use crate::puzzle_input;

pub fn run() {
    let input = puzzle_input::read_string("./input/2019-d08-input1.txt");
    let mut img = Image::new(25, 6);
    img.read_string(&input);

    let mut min = std::i32::MAX;
    let mut result = 0;
    for layer in &img.layers {
        if layer.zeros < min {
            min = layer.zeros;
            result = layer.ones * layer.twos;
        }
    }

    println!("** Part 1 Final: {:?}", result);
    img.draw_image();
    println!("** Part 2 Final: what does image say?")
}

#[derive(Debug)]
struct Layer {
    data: Vec<i32>,
    zeros: i32,
    ones: i32,
    twos: i32,
}
impl Layer {
    pub fn new(data: Vec<i32>) -> Layer {
        let mut zeros = 0;
        let mut ones = 0;
        let mut twos = 0;

        for i in &data {
            match i {
                0 => zeros += 1,
                1 => ones += 1,
                2 => twos += 1,
                _ => {}
            }
        }

        Layer {
            data: data,
            zeros: zeros,
            ones: ones,
            twos: twos,
        }
    }
}

#[derive(Debug)]
struct Image {
    w: usize,
    h: usize,
    layers: Vec<Layer>,
}
impl Image {
    pub fn new(w: usize, h: usize) -> Image {
        Image {
            w: w,
            h: h,
            layers: Vec::new(),
        }
    }

    pub fn read_string(&mut self, input_ref: &str) {
        let size = (self.w * self.h) as usize;
        let layers = input_ref.len() / size;
        println!("{:?} {:?} -> {:?}", size, input_ref.len(), layers);
        for n in 0..layers {
            let start = (n * size) as usize;
            let end = start + size;
            let section = &input_ref[start..end];
            let l = Layer::new(
                section
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect(),
            );
            //println!("made layer {} {:?}", n, l);
            self.layers.push(l);
        }
    }

    fn print(&self, canvas: &Vec<Vec<char>>) {
        for y in canvas.iter() {
            for x in y.iter() {
                print!("{}", x);
            }
            println!("");
        }
        println!("");
    }

    pub fn draw_image(&self) {
        let mut canvas: Vec<Vec<char>> = vec![vec![' '; self.w]; self.h];

        for layer in self.layers.iter().rev() {
            let mut i = 0;
            for y in 0..self.h {
                for x in 0..self.w {
                    match layer.data[i] {
                        0 => canvas[y][x] = ' ', // black
                        1 => canvas[y][x] = '*', // white
                        2 => {}
                        _ => canvas[y][x] = '%', // badness
                    }
                    i += 1;
                }
            }
            self.print(&canvas);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unpack_image() {
        let input = "123456789012";
        let mut img = Image::new(3, 2);
        img.read_string(&input);

        assert_eq!(img.layers[0].data, [1, 2, 3, 4, 5, 6]);
        assert_eq!(img.layers[0].zeros, 0);
        assert_eq!(img.layers[1].data, [7, 8, 9, 0, 1, 2]);
        assert_eq!(img.layers[1].zeros, 1);
    }
}
