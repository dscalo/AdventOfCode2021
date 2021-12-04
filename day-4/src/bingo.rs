use std::collections::HashMap;

struct Pos(usize,usize);

pub struct BingoBoard  {
    board: Vec<Vec<bool>>,
    lookup: HashMap<u32,Pos>
}

impl BingoBoard {
   pub fn new(numbs: Vec<Vec<u32>>) -> BingoBoard {
        let mut board: Vec<Vec<bool>> = Vec::new();
        let mut map: HashMap<u32,Pos> = HashMap::new(); 
        let row = vec![false,false,false,false,false];
        for _ in 0..5 {
            board.push(row.clone());
        }        

        for y in 0..numbs.len() {
            for x in 0..numbs[y].len() {
                map.insert(numbs[y][x], Pos(y,x));
            }
        }

        BingoBoard {board:board, lookup: map}
    }

    pub fn mark(&mut self, n: u32) {
        match self.lookup.get(&n) {
            Some(pos) => self.board[pos.0][pos.1] = true,
            None => {}
        }
    }

   pub fn bingo(&self) -> bool {

        // check hoz
        for y in 0..self.board.len() {
            let mut is_bingo = true;
            for x in 0..self.board[y].len() {
                if self.board[y][x] == false {
                    is_bingo = false;
                    break;
                }
            }
            if is_bingo {
                return true
            }
        }
        // check verticle
        for x in 0..self.board.len() {
            let mut is_bingo = true;
            for y in 0..self.board[x].len() {
                if self.board[y][x] == false {                   
                    is_bingo = false;
                    break;
                }
            }
            if is_bingo {
                return true
            }
        }

        false
    }

    pub fn unmarked_sum(&self) -> u32 {
        let mut sum = 0;

        for (k,pos) in &self.lookup {
            if self.board[pos.0][pos.1] == false {
                sum += k
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn hoz_bingo() {
        let b = vec![
            vec![1,2,3,4,5],
            vec![6,7,8,9,10],
            vec![11,12,13,14,15],
            vec![16,17,18,19,20],
            vec![21,22,23,24,25]
        ];

        let mut board = BingoBoard::new(b);

        board.mark(6);
        assert_eq!(board.bingo(), false);
        board.mark(7);
        assert_eq!(board.bingo(), false);
        board.mark(8);
        assert_eq!(board.bingo(), false);
        board.mark(9);
        assert_eq!(board.bingo(), false);
        board.mark(11);
        assert_eq!(board.bingo(), false);
        board.mark(56);
        assert_eq!(board.bingo(), false);
        board.mark(11);
        assert_eq!(board.bingo(), false);

        board.mark(10);
        assert_eq!(board.bingo(), true);
    }

    #[test]
    fn vert_bingo() {
        let b = vec![
            vec![1,2,3,4,5],
            vec![6,7,8,9,10],
            vec![11,12,13,14,15],
            vec![16,17,18,19,20],
            vec![21,22,23,24,25]
        ];

        let mut board = BingoBoard::new(b);

        board.mark(3);
        assert_eq!(board.bingo(), false);
        board.mark(8);
        assert_eq!(board.bingo(), false);
        board.mark(13);
        assert_eq!(board.bingo(), false);
        board.mark(18);
        assert_eq!(board.bingo(), false);
        board.mark(11);
        assert_eq!(board.bingo(), false);
        board.mark(56);
        assert_eq!(board.bingo(), false);
        board.mark(11);
        assert_eq!(board.bingo(), false);

        board.mark(23);
        assert_eq!(board.bingo(), true);
    }

    #[test]
    fn returns_current_sum() {
        let b = vec![
            vec![14,21,17,24,4],
            vec![10,16,15,9,19],
            vec![18,8,23,26,20],
            vec![22,11,13,6,5],
            vec![2,0,12,3,7]
        ];

        let mut board = BingoBoard::new(b);

        board.mark(7);
        board.mark(4);
        board.mark(9);
        board.mark(5);
        board.mark(11);
        board.mark(17);
        board.mark(23);
        board.mark(2);
        board.mark(0);
        board.mark(14);
        board.mark(21);
        board.mark(24);

        assert_eq!(board.bingo(), true);
        assert_eq!(board.unmarked_sum(), 188);
    }
}
