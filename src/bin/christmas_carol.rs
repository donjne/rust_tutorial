fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = ["a Partridge in a Pear Tree", "Two Turtle Doves", "Three French Hens", "Four Calling Birds", "Five Golden Rings", "Six Geese a Laying", "Seven Swans a Swimming", "Eight Maids a Milking", "Nine Ladies Dancing", "Ten Lords a Leaping", "Eleven Pipers Piping", "Twelve Drummers Drumming"];

    for day in 0..12 {
        println!("On the {} day of Christmas,", days[day]);
        println!("My true love gave to me:");

        for gift_day in (0..=day).rev() {
            if day == 0 && gift_day == 0 {
                println!("{}.", gifts[gift_day]);
            } else if gift_day == 0 {
                println!("{}", gift_day);
                // println!("And {}", gifts[gift_day]);
            } else {
                println!("{},", gifts[gift_day]);
            }
        }

        println!();
    }
}
