struct WrapVal {
    val: usize, // (val - 1) actually
    max: usize,
}

impl WrapVal {
    fn new(val: usize, max: usize) -> WrapVal {
        WrapVal { val: val - 1, max }
    }
    fn get(&self) -> usize {
        self.val + 1
    }
    fn inc(&mut self, val: usize) {
        self.val = (self.val + val) % self.max;
    }
}

fn play_game(player1: usize, player2: usize) -> usize {
    let mut players = [
        (WrapVal::new(player1, 10), 0),
        (WrapVal::new(player2, 10), 0),
    ];
    let mut die = WrapVal::new(1, 1000);
    let mut cur_player_n = WrapVal::new(1, 2);
    let mut roll_count = 0;

    loop {
        let cur_player = &mut players[cur_player_n.get() - 1];
        let roll_sum = (0..3)
            .map(|_| {
                let roll = die.get();
                die.inc(1);
                roll_count += 1;
                roll
            })
            .fold(0, |sum, roll| sum + roll);
        cur_player.0.inc(roll_sum);
        cur_player.1 += cur_player.0.get();

        cur_player_n.inc(1);

        if cur_player.1 >= 1000 {
            break;
        }
    }

    let cur_player = &mut players[cur_player_n.get() - 1];

    cur_player.1 * roll_count
}

pub fn main() {
    assert_eq!(play_game(4, 8), 739785);
    println!("tests ok");

    println!("day21 pt1 {}", play_game(7, 8));
}
