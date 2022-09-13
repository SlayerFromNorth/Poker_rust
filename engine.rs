use cute::c;
extern crate cute;

pub fn find_gant(cards: [usize;7]) -> i64{
    /* straight flush | four of a kind | house | flush | straight | three alike | two pair | pair | high card*/
    
    let card_deck = [   [20, 2], [20, 3], [20, 4], [20, 5], [20, 6], [20, 7], [20, 8], [20, 9], [20, 10], [20, 11], [20, 12], [20, 13], [20, 14],
                    [30, 2], [30, 3], [30, 4], [30, 5], [30, 6], [30, 7], [30, 8], [30, 9], [30, 10], [30, 11], [30, 12], [30, 13], [30, 14],
                    [40, 2], [40, 3], [40, 4], [40, 5], [40, 6], [40, 7], [40, 8], [40, 9], [40, 10], [40, 11], [40, 12], [40, 13], [40, 14],
                    [50, 2], [50, 3], [50, 4], [50, 5], [50, 6], [50, 7], [50, 8], [50, 9], [50, 10], [50, 11], [50, 12], [50, 13], [50, 14]];
    
    let colors = [20, 30, 40, 50];

    #[macro_use(c)]
    let color = c![card_deck[x][0], for x in cards];
    let mut number_value = c![card_deck[x][1], for x in cards];

    number_value.sort_by(|a, b| b.cmp(a));
    
    // high card
    let mut high = format!("{:02}{:02}{:02}{:02}{:02}", number_value[0], number_value[1], number_value[2], number_value[3], number_value[4]);
    let high_int = high.parse::<i32>().unwrap();
    let mut points = [1, high_int];

    
    // Flush 
    for ss in colors{
        let gg = c![1, for a in &color, if a == &ss];
        let sum: i32 = gg.iter().sum();
        if sum > 4{
            let mut right = c![cards[a.0] +2, for a in gg.iter().enumerate(), if *a.1 == 1];
            
            right.sort_by(|a, b| b.cmp(a));
            let mut high = format!("{:02}{:02}{:02}{:02}{:02}", right[0], right[1], right[2], right[3], right[4]);
            let high_int = high.parse::<i32>().unwrap();
            let mut points = [6, high_int];
        
            if right[0] == 14{
                right.push(1)
            }
            for cc in 0..right.len()-4{
                if right[cc] - right[cc+4] == 4{
                    let points = [9,right[cc]];
                    return (points[0] * 10_000_000_000_000) as i64 + (points[1]) as i64;
                }
            }
        }
    }
    // Straight
    let mut counter = 0;
    let mut high = 0;

    if number_value[0] == 14{
        number_value.push(1)
    }
    for a in 0..number_value.len()-1{
        if high == 0{
            high = number_value[a]
        }

        if number_value[a] -1 == number_value[a+1]{
            counter += 1;
            if counter > 3 {
                points = [5, high];
                return (points[0]) as i64 * 10_000_000_000_000 + (points[1]) as i64;
            }
        } else if number_value[a] == number_value[a+1]{
            continue;
        } else {
            counter = 0;
            high = 0
        }
    }
    // Four of a kind, 3 alike, 2pair, 1 pair and house
    let mut pair = 0;
    let mut pairs: Vec<i32> = Vec::new();
    let mut pair_tree_alike = 0;
    let mut tree_alike = 0;

    for ss in 2..15{
        let output: Vec<i32> = c![1, for x in &number_value, if *x == ss];
        
        let gg: i32 = output.iter().sum();

        if gg > 3 {
            let right = c![x, for x in &number_value, if *x != ss];
            let high = format!("{:02}{:02}{:02}{:02}{:02}", ss, ss, ss, ss, right[0]);
            let high_int = high.parse::<i32>().unwrap();
            let points = [8, high_int];
        }
        else if gg > 2 {
            tree_alike += 1;
            pair_tree_alike = ss;
            let right = c![x, for x in &number_value, if *x != ss];
            let high = format!("{:02}{:02}{:02}{:02}{:02}", ss, ss, ss, right[0], right[1]);
            let high_int = high.parse::<i32>().unwrap();
            let points = [4, high_int];
        }
        else if gg > 1 {
            pairs.push(ss);
            pair += 1;
            let right = c![x, for x in &number_value, if *x != ss];
            let high = format!("{:02}{:02}{:02}{:02}{:02}", ss, ss, right[0], right[1], right[2]);
            let high_int = high.parse::<i32>().unwrap();
            let points = [2, high_int];
        }
    }
    if pair == 2 {
        let right = c![x, for x in &number_value, if *x != pairs[0] || *x != pairs[1]];
        let high = format!("{:02}{:02}{:02}{:02}{:02}", pairs[1], pairs[1], pairs[0], pairs[0], right[0]);
        let high_int = high.parse::<i32>().unwrap();
        let points = [3, high_int];
    }
    if pair > 2 {
        let right = c![x, for x in &number_value, if *x != pairs[2] || *x != pairs[1]];
        let high = format!("{:02}{:02}{:02}{:02}{:02}", pairs[2], pairs[2], pairs[1], pairs[1], right[0]);
        let high_int = high.parse::<i32>().unwrap();
        let points = [3, high_int];
    }
    if tree_alike > 0 && pair > 0 {
        let high = format!("{:02}{:02}", pair_tree_alike, pairs[0]);
        let high_int = high.parse::<i32>().unwrap();
        let points = [7, high_int];
    }
    return (points[0]) as i64 * 10_000_000_000_000 + (points[1]) as i64;
}
fn main() {
    let a = find_gant([0, 1, 2, 3, 4, 12, 23]);
    println!("{a}");
}
