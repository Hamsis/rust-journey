use std::io;

fn main() {
    let mut actual = 1;
    let mut previous = 0;
    let mut indice = 2;

    println!("Donner le nombre de fibonacci voulu");
    let number = loop {
        let mut choix = String::new();
        io::stdin()
            .read_line(&mut choix)
            .expect("Failed to read line");

        break match choix.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
    };

    let resultat = if number == 0 {
        0
    } else if number == 1 {
        1
    } else {
        loop {
            actual = actual + previous;
            previous = actual - previous;

            if indice == number {
                break actual;
            } else {
                indice += 1;
            }
        }
    };

    println!("f({number}): {resultat}");
}
