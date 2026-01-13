fn main() {
    println!("The Twelve Days of Christmas");
    println!("============================\n");

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", 
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for day in 0..12 {
        println!(
            "On the {} day of Christmas, my true love gave to me:",
            days[day]
        );

        // Print gifts in reverse order (from current day back to first)
        for gift_index in (0..=day).rev() {
            if gift_index == 0 && day > 0 {
                // "And a partridge..." for days after the first
                println!("And a partridge in a pear tree.");
            } else if gift_index == 0 {
                // First day: just "A partridge..."
                println!("{}.", gifts[gift_index]);
            } else {
                // All other gifts
                println!("{},", gifts[gift_index]);
            }
        }

        println!(); // Blank line between verses
    }
}