fn main() {

    // # of iterations
    static I:i32 = 500000;

    let mut input_door_list: Vec<i32> = vec![];
    let mut correct_door_list: Vec<i32> = vec![];
    let mut prize_sum_s: i32 = 0; //stickers
    let mut prize_sum_c: i32 = 0; //changers

    //fun rounding function
    fn d_round(decimal: f64, specificity: i32) -> f64 {
        // if specificity is too high, it will return NaN
        return (decimal * 10_f64.powi(specificity)).round()/(10_f64.powi(specificity))
    }

    // make initial list of input choices
    for _i in 0..I {
        let door = rand::random_range(1..=3);
        input_door_list.push(door);
        let door = rand::random_range(1..=3);
        correct_door_list.push(door);
    }

    // tally stickers' scores
    for (input_door, correct_door) in input_door_list.iter().zip(correct_door_list.iter()) {
        if input_door - correct_door == 0 {
            prize_sum_s += 1;
        }
    }

    // tally changers' scores
    for (input_door, correct_door) in input_door_list.iter().zip(correct_door_list.iter()) {
        let final_choice = match input_door + correct_door {
            2 => 3, // (1, 1), always wrong
            3 => match input_door { 1 => 2, 2 => 1, _ => return}, // (1, 2)
            4 => match input_door { 1 => 3, 2 => 1, 3 => 1, _ => return }, // (1, 3), (2, 2)
            5 => match input_door { 2 => 3, 3 => 2, _ => return}, // (2, 3)
            6 => 1,// (3, 3), always wrong
            _ => return
        };
        if final_choice == *correct_door {
            prize_sum_c += 1
        }
    }


    // print results

    let percent_win_s: f64 = f64::from(prize_sum_s) / f64::from(I);
    let percent_win_c: f64 = f64::from(prize_sum_c) / f64::from(I);
    let percent_error_s: f64 = (((0.3333333333333333-percent_win_s).abs())/percent_win_s)*100.;
    let percent_error_c: f64 = (((0.6666666666666666-percent_win_c).abs())/percent_win_c)*100.;
    println!("
    Monty Hall Problem\n
    \n iterations : {}
    \n\n stayed:
    \n prizes won : {}
    \n winrate : %{:?}
    \n percent error : %{}
    \n\n switched:
    \n prizes won : {}
    \n winrate : %{:?}
    \n percent error : %{}
    ",

             I, //iterations

             prize_sum_s, // prizes won
             d_round(percent_win_s*100., 4), // winrate
             d_round(percent_error_s,3), // % error

             prize_sum_c, // prizes won
             d_round(percent_win_c*100., 4), // winrate
             d_round(percent_error_c,3) ); // % error


    /*
    \n  in this dilemma, contestants are told to choose from 3 doors.
    \n  after they choose, a door that was unpicked and does not have
    \n  a prize behind it is opened, leaving two doors. they are then
    \n  given the option to switch to the other remaining door,
    \n  if they so choose. Most do not switch because they believe
    \n  the odds are equal; in reality, switching gives a 2/3 chance
    \n  of winning, over staying, which gives a 1/3 chance of winning.
    \n  this aims to prove that through the law of large numbers.
    */
}
