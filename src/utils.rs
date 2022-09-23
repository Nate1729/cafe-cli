use std::io;
use io::Write;

pub struct SelectOption<T> {
    pub description: String,
    pub option: T
}

pub fn read_select_options<T>(select_options: &Vec<SelectOption<T>>, header: Option<String>) -> T {
    let mut buffer = String::new();

    loop {
        match &header {
            Some(s) => print!("{}", s),
            None => ()
        }
        
        for (i, val) in select_options.iter().enumerate() {
            print!("({}), {}\n", i, val.description);
        }
        io::stdout().flush();
        io::stdin().read_line(&mut buffer);

        // Converting to number
        let num: usize = buffer.trim().parse().expect("Enter a valid option!");
        
        if num < select_options.len() {
            return select_options[num].option;
        }
    } 
}
