const FILE: &str = "data/day4data.txt";

pub fn part_one() -> u32 {
    let file = crate::util::read_file(FILE);
    let lines = file.lines().collect::<Vec<&str>>();
    let drawings = lines[0].split(",").map(|num| num.parse::<u8>().expect("Couldnt parse drawing number")).collect::<Vec<u8>>();
    let mut bingo = Bingo::new(file);
    for drawing in drawings {
        bingo.call_number(drawing);
    }
    bingo.winning_scores[0]
}

pub fn part_two() -> u32 {
    let file = crate::util::read_file(FILE);
    let lines = file.lines().collect::<Vec<&str>>();
    let drawings = lines[0].split(",").map(|num| num.parse::<u8>().expect("Couldnt parse drawing number")).collect::<Vec<u8>>();
    let mut bingo = Bingo::new(file);
    for drawing in drawings {
        bingo.call_number(drawing);
    }
    bingo.winning_scores.last().expect("No Items in winning scores array").to_owned()
}

struct Bingo {
    drawings: Vec<u8>,
    boards: Vec<Board>,
    winning_scores: Vec<u32>,
    round: usize,
}

impl Bingo {
    pub fn new(file: String) -> Self {
        let mut boards = vec![];

        let mut collector: Vec<u8> = Vec::new();
        for (idx, line) in file.lines().enumerate() {
            if idx == 0 { continue; }
            for space in line.split(" ") {
                if space.is_empty() { continue; }
                if collector.len() < 25 {
                    collector.push(space.parse::<u8>().expect("Couldnt parse string for board"));

                    if collector.len() == 25 {
                        boards.push(Board::new(&collector));
                        collector.clear();
                    }
                }
            }
        }

        Self {
            drawings: vec![],
            boards,
            winning_scores: vec![],
            round: 1,
        }
    }

    pub fn call_number(&mut self, called_number: u8) {
        //number called
        self.drawings.push(called_number);
        //call mark space on all boards
        let boards = self.boards.clone();
        let marked_boards = boards.into_iter().map(|board| board.mark_board_and_return(&called_number)).collect::<Vec<Board>>();
        self.boards = marked_boards;
        //check for winning boards
        for board in &self.boards {
            if board.check() {
                let score = board.calculate_score(called_number);
                self.winning_scores.push(score);
            }
        }
        //remove winning boards
        self.boards.retain(|board| !board.check());


        // if self.round >= 5 {
        //     let mut boards_to_remove: Vec<usize> = vec![];
        //     for idx in 0..self.boards.len() {
        //         let board = &self.boards[idx];
        //         // self.boards[idx].check(self.drawings.clone());
        //         if board.winning {
        //             let score = board.calculate_score(self.drawings.clone());
        //             println!("Round:{} Board:{} Boards Left: {}", self.round, idx, self.boards.len());
        //             println!("Winning score: {}", score);
        //             self.winning_scores.push(score);
        //             boards_to_remove.push(idx);
        //         }
        //     }
        //     // for idx in boards_to_remove {
        //     //     self.boards.remove(idx);
        //     // }
        //     self.boards.retain(|board| !board.winning)
        // }

        self.round += 1;
    }
}

#[derive(Debug, Clone)]
struct Board {
    spaces: [u8; 25],
    marks: Vec<u8>,
    width: usize,
    height: usize,
    winning: bool,
}

impl Board {
    pub fn new(input: &Vec<u8>) -> Self {
        if input.len() != 25 { panic!("Wrong size for board") }
        let mut spaces = [0; 25];
        for idx in 0..25 {
            spaces[idx] = input[idx];
        }
        Board { spaces, marks: vec![], width: 5, height: 5, winning: false }
    }

    fn mark_board(&mut self, called_number: u8) {
        if self.spaces.contains(&called_number) {
            self.marks.push(called_number);
            self.check();
        }
    }

    fn mark_board_and_return(self, called_number: &u8) -> Self {
        let mut marks = self.marks.clone();
        if self.spaces.contains(called_number) {
            marks.push(*called_number)
        }
        Self {
            spaces: self.spaces,
            marks,
            width: self.width,
            height: self.height,
            winning: self.check(),
        }
    }

    fn check_mut(&mut self) {
        //loop all rows and cols  and check if theres 5 matches
        if self.winning { return; }
        for idx in 0..self.width {
            if self.check_row(idx, &self.marks) || self.check_col(idx, &self.marks) {
                self.winning = true;
            }
        }
    }

    fn check(&self) -> bool {
        //loop all rows and cols  and check if theres 5 matches
        if self.winning { return true; }
        for idx in 0..self.width {
            if self.check_row(idx, &self.marks) || self.check_col(idx, &self.marks) {
                return true;
            }
        }
        return false;
    }

    fn calculate_score(&self, called_number: u8) -> u32 {
        //sum winning numbers
        let mut marked_sum: u32 = 0;
        for space in self.spaces {
            if self.marks.contains(&space) {
                marked_sum += space as u32;
            }
        }
        //sum unmarked numbers
        let unmarked_sum = self.sum_board() - marked_sum;
        //multiply unmarked sum by called number
        unmarked_sum * called_number as u32
    }

    fn sum_board(&self) -> u32 {
        self.spaces.iter().map(|space| space.to_owned() as u32).sum()
    }

    // fn sum_row(&self, row: usize) -> u32 {
    //     let mut sum: u32 = 0;
    //     for col in 0..self.width {
    //         sum += self.get_space_at(row, col) as u32
    //     }
    //     sum
    // }
    //
    // fn sum_col(&self, col: usize) -> u32 {
    //     let mut sum: u32 = 0;
    //     for row in 0..self.height {
    //         sum += self.get_space_at(row, col) as u32
    //     }
    //     sum
    // }

    fn get_space_at(&self, row: usize, col: usize) -> u8 {
        let index = row * self.width + col;
        self.spaces[index]
    }

    fn check_row(&self, row: usize, drawings: &Vec<u8>) -> bool {
        let mut matches = 0;
        for col in 0..self.width {
            if drawings.contains(&self.get_space_at(row, col)) {
                matches += 1;
                // println!("Row:{} Space:{} Matches:{}",row,&self.get_space_at(row, col),matches )
            }
        }
        matches == self.width
    }

    fn check_col(&self, col: usize, drawings: &Vec<u8>) -> bool {
        let mut matches = 0;
        for row in 0..self.height {
            if drawings.contains(&self.get_space_at(row, col)) {
                matches += 1;
                // println!("Col:{} Space:{} Matches:{}",col,&self.get_space_at(row, col),matches )
            }
        }
        matches == self.height
    }
}