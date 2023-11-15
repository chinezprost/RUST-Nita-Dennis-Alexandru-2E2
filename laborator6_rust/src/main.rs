use std::fs::File;
use std::collections::HashMap;
use std::io::{self, BufRead};

trait Command
{
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: Vec<&str>);
}
struct PingCommand;
struct CountCommand;
struct TimesCommand
{
    count: usize,
}
struct CostumCommand;

impl Command for PingCommand
{
    fn get_name(&self) -> &str
    {
        return "ping";
    }

    fn exec(&mut self, args: Vec<&str>)
    {
        println!("pong!");
    }
}


impl Command for CountCommand
{
    fn get_name(&self) -> &str
    {
        return "count";
    }

    fn exec(&mut self, args: Vec<&str>)
    {
        println!("Counted {} arguments", args.len());
    }
}

impl Command for TimesCommand
{
    fn get_name(&self) -> &str
    {
        return "times";
    }

    fn exec(&mut self, args: Vec<&str>)
    {
        self.count += 1;
        println!("Command has been called for {} times!", self.count);
    }
}

impl Command for CostumCommand
{
    fn get_name(&self) -> &str
    {
        return "costum";
    }

    fn exec(&mut self, args: Vec<&str>)
    {
        println!("Costum");
    }
}

struct Terminal
{
    commands: HashMap<String, Box<dyn Command>>
}

impl Terminal
{
    fn new() -> Self
    {
        Terminal
        {
            commands: HashMap::new(),
        }
    }

    fn register(&mut self, command: Box<dyn Command>)
    {
        self.commands.insert(command.get_name().to_string(), command);
    }

    fn run(&mut self)
    {
        if let Ok(opened_file) = File::open("commands.txt")
        {
            let buffer = io::BufReader::new(opened_file);

            for line in buffer.lines()
            {
                if let Ok(line) = line
                {
                    let mut splitted_line = line.trim().split_whitespace();
                    if let Some(command_name) = splitted_line.next()
                    {
                        let args: Vec<&str> = splitted_line.collect();
                        if let Some(command) = self.commands.get_mut(command_name)
                        {
                            command.exec(args);
                        } else 
                        {
                            println!("Unknown command!");    
                        }
                    }
                }
            }
        } else 
        {
            println!("Couldn't open commands file!")    
        }
        
    }
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));

    terminal.run();
}
