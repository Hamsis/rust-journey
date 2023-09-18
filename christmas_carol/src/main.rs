fn main() {
    let days = [
        ("A", "first"),
        ("Two", "second"),
        ("Three", "third"),
        ("Four", "fourth"),
        ("Five", "fifth"),
        ("Six", "sixth"),
        ("Seven", "seventh"),
        ("Eight", "eighth"),
        ("Nine", "ninth"),
        ("Ten", "tenth"),
        ("Eleven", "eleventh"),
        ("Twelve", "twelfth"),
    ];

    let verses = [
        "partridge in a pear tree",
        "turtles doves, and",
        "french hens",
        "calling birds",
        "golden rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];

    for x in 0..12 {
        println!("[Verse {}]", x + 1);
        println!(
            "On the {} day of Christmas, my true love sent to me",
            days[x].1
        );
        for y in (0..x + 1).rev() {
            println!("{} {}", days[y].0, verses[y]);
        }
        println!();
    }
}
