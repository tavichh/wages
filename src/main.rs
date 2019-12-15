mod wages;

fn main() {
    loop{
        let term = console::Term::stdout();
        let title = "Wages";
        let description = "A simple wage calculator written in rust";
        let author = "tavichh";
    
        term.set_title(title);
    
        println!("{}",description);
        println!("Created by {}",author);
        println!("", );
    
        println!("Please enter your hourly wage.", );
        let wage = term.read_line()
            .unwrap()
            .parse::<f32>();
    
        println!("Please enter how many hours you worked.");
        let hours = term.read_line()
            .unwrap()
            .parse::<f32>();
    
        println!("You have made: ${}",
            wages::evaluate(wage.unwrap(), 
            hours.unwrap()));
    
        println!("Press a key to preform another calculation.");
        term.read_key().ok();
    }
}
