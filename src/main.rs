mod wages;

fn main() {
        let term: 
            console::Term = console::Term::stdout();
        let title: 
            &str = "Wages";
        let description: 
            &str = "A simple wage calculator written in rust";
        let author: 
            &str = "tavichh";
    
        term.set_title(title);
    
        println!("{}",description);
        println!("Created by {}",author);
        println!("", );
        loop{
            println!("Please enter your hourly wage.", );
            let wage = &term.read_line()
                .unwrap()
                .parse::<f32>();
        
            println!("Please enter how many hours you worked.");
            let hours = &term.read_line()
                .unwrap()
                .parse::<f32>();
            println!("You have earned ${}",
                wages::evaluate(
                &wage
                    .as_ref()
                    .unwrap(), 
                &hours
                    .as_ref()
                    .unwrap()));
            wages::show_table(
                &wage
                    .as_ref()
                    .unwrap());
            println!("Press a key to preform another calculation.");
            term.read_key().ok();
            term.clear_screen().ok();
        }
}
