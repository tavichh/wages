pub fn evaluate(u_wage: &f32, 
    u_hour: &f32) -> f32 {
        let total = u_hour * u_wage;
        total
}
pub fn show_table(u_wage: &f32) {
    println!("", );
    println!(" HOURS |   | PAY ", );
    println!("-------|   |-----", );
    println!("1      |   |{}   ", u_wage*1.00 );
    println!("2      |   |{}   ", u_wage*2.00 );
    println!("3      |   |{}   ", u_wage*3.00 );
    println!("4      |   |{}   ", u_wage*4.00 );
    println!("5      |   |{}   ", u_wage*5.00 );
    println!("6      |   |{}   ", u_wage*6.00 );
    println!("7      |   |{}   ", u_wage*7.00 );
    println!("8      |   |{}   ", u_wage*8.00 );
    println!("9      |   |{}   ", u_wage*9.00 );
    println!("10     |   |{}   ", u_wage*10.00 );
    println!("11     |   |{}   ", u_wage*11.00 );
    println!("12     |   |{}   ", u_wage*12.00 );
}