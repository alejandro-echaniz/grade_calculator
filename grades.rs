/* grades.rs : main file for calculating class averages and grades*/
/* A rust project by Alejandro Echaniz*/

use std::io;
use std::io::Write;
// fn compute_numeric_score(score: &mut [i32], weight: &mut [i32], days_late: &mut [i32], size: i32, penalty: i32, dropped: i32) -> f32 {
// }


fn main(){
    
    print!("Enter total number of midterms: ");
        std::io::stdout().flush();
        let mut total_tests = String::new();
        std::io::stdin().read_line(&mut total_tests).expect("Err: not able to read line");
    
    print!("Enter total number of assignments to drop: ");
        std::io::stdout().flush();
        let mut dropped_assignments = String::new();
        std::io::stdin().read_line(&mut dropped_assignments).expect("Err: not able to read line");
        
    print!("Enter late penalty per assignment: ");
        std::io::stdout().flush();
        let mut late_deduction = String::new();
        std::io::stdin().read_line(&mut late_deduction).expect("Err: not able to read line"); 

        println!("{} total midterms", total_tests.trim());
        println!("{} total dropped assignments", dropped_assignments.trim());
        println!("{} late deduction", late_deduction.trim());
}
