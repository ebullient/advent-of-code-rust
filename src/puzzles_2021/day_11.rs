use crate::puzzle_input;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::RangeInclusive;


pub fn run() {
    let mut grid = Grid::new(&puzzle_input::read_all_lines("./input/2021-d11-input.txt"));

    let mut flashes_at_100 = 0;
    let mut flashes = 0;
    let mut i = 0;
    while flashes != 100 {
        if i == 100 {
            flashes_at_100 = grid.flashes;
        }
        flashes = grid.step();
        i += 1;
    }

    println!("** Part 1 Final: {:?}", flashes_at_100);
    assert_eq!(1625, flashes_at_100);
    println!("** Part 2 Final: {:?}", i);
    assert_eq!(244, i);
}

fn box_range(i: usize, max: usize) -> RangeInclusive<usize> {
    std::ops::RangeInclusive::new(
        if i == 0 { 0 } else { i - 1 },
        if i + 1 == max { i } else { i + 1 },
    )
}

fn neighbors(point: (usize, usize), height: usize, width: usize) -> Vec<(usize, usize)> {
    let mut pts = Vec::with_capacity(8);
    for y1 in box_range(point.0, height) {
        for x1 in box_range(point.1, width) {
            if y1 == point.0 && x1 == point.1 {
                continue;
            }
            pts.push((y1, x1));
        }
    }
    pts
}

#[derive(Clone, Debug, PartialEq)]
struct Grid {
    data: HashMap<(usize, usize), i32>,
    width: usize,
    height: usize,
    flashes: usize
}

impl Grid {
    fn new(input: &Vec<String>) -> Grid {
        let mut data: HashMap<(usize, usize), i32> = HashMap::new();
        for (y, row) in input.iter().enumerate() {
            for (x, col) in row.trim().chars().enumerate() {
                data.insert((y, x), col.to_digit(10).unwrap() as i32);
            }
        }

        Grid {
            data: data,
            height: input.len(),
            width: input[0].len(),
            flashes: 0
        }
    }

    fn try_flash(&mut self, k: (usize, usize), flash: &mut HashSet<(usize, usize)>) {
        if let Some(v) = self.data.get_mut(&k) {
            if *v > 9 && flash.insert(k) {
                for n in neighbors(k, self.height, self.width) {
                    let vn = self.data.get_mut(&n).unwrap();
                    *vn += 1;
                    self.try_flash(n, flash);
                }
            }
        }
    }

    fn step(&mut self) -> usize {
        let mut flash: HashSet<(usize, usize)> = HashSet::new();
        for (_, v) in self.data.iter_mut() {
            *v += 1;
        }

        for y in 0..self.height {
            for x in 0..self.width {
                self.try_flash((y, x), &mut flash)
            }
        }

        for (_, v) in self.data.iter_mut() {
            if *v > 9 {
                *v = 0;
            }
        }
        self.flashes += flash.len();
        flash.len()
    }

    #[allow(dead_code)]
    fn dump(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(pt) = self.data.get(&(y, x)) {
                    print!("{}", pt);
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // Before any steps:
        let mut ex = Grid::new(&puzzle_input::split_string("11111
            19991
            19191
            19991
            11111"));

        // After step 1:
        let ex_step1 = Grid::new(&puzzle_input::split_string("34543
            40004
            50005
            40004
            34543"));
        ex.step();
        assert_eq!(ex_step1.data, ex.data);

        // After step 2:
        let ex_step2 = Grid::new(&puzzle_input::split_string("45654
            51115
            61116
            51115
            45654"));
        ex.step();
        assert_eq!(ex_step2.data, ex.data);

        let mut grid = Grid::new(&puzzle_input::split_string("5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526"));

        let grid_1 = Grid::new(&puzzle_input::split_string("6594254334
            3856965822
            6375667284
            7252447257
            7468496589
            5278635756
            3287952832
            7993992245
            5957959665
            6394862637"));
        grid.step();
        assert_eq!(grid_1.data, grid.data);

        let grid_2 = Grid::new(&puzzle_input::split_string("8807476555
            5089087054
            8597889608
            8485769600
            8700908800
            6600088989
            6800005943
            0000007456
            9000000876
            8700006848"));
        grid.step();
        assert_eq!(grid_2.data, grid.data);

        let grid_3 = Grid::new(&puzzle_input::split_string("0050900866
            8500800575
            9900000039
            9700000041
            9935080063
            7712300000
            7911250009
            2211130000
            0421125000
            0021119000"));
        grid.step();
        assert_eq!(grid_3.data, grid.data);

        let grid_4 = Grid::new(&puzzle_input::split_string("2263031977
            0923031697
            0032221150
            0041111163
            0076191174
            0053411122
            0042361120
            5532241122
            1532247211
            1132230211"));
        grid.step();
        assert_eq!(grid_4.data, grid.data);

        for _ in 4..10 {
            grid.step();
        }

        let grid_10 = Grid::new(&puzzle_input::split_string("0481112976
            0031112009
            0041112504
            0081111406
            0099111306
            0093511233
            0442361130
            5532252350
            0532250600
            0032240000"));
        assert_eq!(grid_10.data, grid.data);
        assert_eq!(204, grid.flashes);

        for _ in 0..10 {
            grid.step();
        }

        let grid_20 = Grid::new(&puzzle_input::split_string("3936556452
            5686556806
            4496555690
            4448655580
            4456865570
            5680086577
            7000009896
            0000000344
            6000000364
            4600009543"));
        assert_eq!(grid_20.data, grid.data);

        for _ in 0..10 {
            grid.step();
        }

        let grid_30 = Grid::new(&puzzle_input::split_string("0643334118
            4253334611
            3374333458
            2225333337
            2229333338
            2276733333
            2754574565
            5544458511
            9444447111
            7944446119"));
        assert_eq!(grid_30.data, grid.data);

        for _ in 0..10 {
            grid.step();
        }

        let grid_40 = Grid::new(&puzzle_input::split_string("6211111981
            0421111119
            0042111115
            0003111115
            0003111116
            0065611111
            0532351111
            3322234597
            2222222976
            2222222762"));
        assert_eq!(grid_40.data, grid.data);

        for _ in 0..10 {
            grid.step();
        }

        let grid_50 = Grid::new(&puzzle_input::split_string("9655556447
            4865556805
            4486555690
            4458655580
            4574865570
            5700086566
            6000009887
            8000000533
            6800000633
            5680000538"));
        assert_eq!(grid_50.data, grid.data);

        for _ in 0..10 {
            grid.step();
        }

        let grid_60 = Grid::new(&puzzle_input::split_string("2533334200
            2743334640
            2264333458
            2225333337
            2225333338
            2287833333
            3854573455
            1854458611
            1175447111
            1115446111"));
        assert_eq!(grid_60.data, grid.data);

        for _ in 0..10 {
            grid.step();
        }

        let grid_70 = Grid::new(&puzzle_input::split_string("8211111164
            0421111166
            0042111114
            0004211115
            0000211116
            0065611111
            0532351111
            7322235117
            5722223475
            4572222754"));
        assert_eq!(grid_70.data, grid.data);

        for _ in 0..10 {
            grid.step();
        }

        let grid_80 = Grid::new(&puzzle_input::split_string("1755555697
            5965555609
            4486555680
            4458655580
            4570865570
            5700086566
            7000008666
            0000000990
            0000000800
            0000000000"));
        assert_eq!(grid_80.data, grid.data);

        for _ in 0..10 {
            grid.step();
        }

        let grid_90 = Grid::new(&puzzle_input::split_string("7433333522
            2643333522
            2264333458
            2226433337
            2222433338
            2287833333
            2854573333
            4854458333
            3387779333
            3333333333"));
        assert_eq!(grid_90.data, grid.data);

        for _ in 0..10 {
            grid.step();
        }

        println!("Grid 100");
        let grid_100 = Grid::new(&puzzle_input::split_string("0397666866
            0749766918
            0053976933
            0004297822
            0004229892
            0053222877
            0532222966
            9322228966
            7922286866
            6789998766"));
        assert_eq!(grid_100.data, grid.data);
        assert_eq!(1656, grid.flashes);



        let mut flashes = 0;
        let mut i = 100;
        while flashes != 100 {
            flashes = grid.step();
            println!("{:?} => {:?}", i, flashes);
            i += 1;
        }
        assert_eq!(195, i);
    }

    #[test]
    fn test_part_2() {
        let mut grid = Grid::new(&puzzle_input::split_string("5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526"));

        let mut flashes = 0;
        let mut i = 0;
        while flashes != 100 {
            flashes = grid.step();
            i += 1;
        }
        assert_eq!(195, i);
    }
}
