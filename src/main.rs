use shut_nine_boxes::*;

fn main() {
    let mut shutbox = Shutbox::new();
    println!("Let's play shut the boxes!!");
    
    loop {
        println!("-----------------");
        println!("{}", shutbox.get_string());
        println!("-----------------");

        // roll dices
        let dice_sum;
        if shutbox.get_open_num() == 1 && shutbox.result() == 1 {
            dice_sum = dice();
            println!("roll one dice: {}", dice_sum);
        } else {
            let first = dice();
            let second = dice();
            dice_sum = first + second;
            println!("roll two dices: {}, {} => {}", first, second, dice_sum);
        }

        // print list of available answers
        let answers = shutbox.get_avail_answers(dice_sum);
        if let None = answers {
            break;
        }
        let answers = answers.unwrap();
        for (idx, v) in answers.iter().enumerate() {
            println!("{} => {:?}", idx, v);
        }
        let select = get_user_input().trim().parse::<u8>().unwrap();

        shutbox.shut(&answers[select as usize]);
    }

    println!("");
    println!("=============================");
    println!("          RESULT");
    println!("=============================");

    if shutbox.get_open_num() == 0 {
        println!("All 9 boxes shut! You Win!!");
    } else {
        println!("{}", shutbox.get_string());
        println!("Your score: {}", shutbox.result());
    }
}
