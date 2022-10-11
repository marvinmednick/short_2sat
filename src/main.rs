#[macro_use]
mod log_files;
mod cmd_line;
mod dirgraph;
mod graphbuilder;
mod kosaraju;
mod parse;


use crate::cmd_line::CommandArgs;
use crate::kosaraju::Kosaraju;
use crate::dirgraph::DirectedGraph;
//use crate::graphbuilder::GraphBuilder;
use crate::parse::read_constraints;


use clap::Parser;
use log::{ info, debug };

use std::process;
use std::path::Path;
use std::fs::File;
use std::thread;

use std::fs;
use crate::log_files::{LogFile,set_log_dir};



fn main() {

    env_logger::init();

    set_log_dir(&"log");
    fs::remove_dir_all("log");

    let cmd_line = CommandArgs::parse();
    debug!("The Command Line, {:?}!",cmd_line);

    // Create a path to the desired file
    let path = Path::new(&cmd_line.filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };


	let mut graph = DirectedGraph::new();
    read_constraints(&mut file, &mut graph);

    if cmd_line.show_graph {
        println!("DirGraph");
        graph.print_graph();
        process::exit(0);
        
    }
    graph.log_graph("kosara");

	let child = thread::Builder::new().name("Working Thread".to_string()).stack_size(512 * 1024 * 1024).spawn(move || { 

	   // code to be executed in thread

        let handle = thread::current();

        let mut k = Kosaraju::new(&mut graph,true);
        k.find_scc();
        k.log_scc_to_files("kosara");
        // sort in reverse order
        let mut scc_sizes = k.get_scc_sizes();
        scc_sizes.sort_by(|a, b| b.cmp(a));
        info!("K sizes {:?}",scc_sizes);
        let k_finish_order = k.get_finish_order();

        let mut output_vec = &scc_sizes;
           
        if cmd_line.counts {
            let mut count = 0;
            for search_count in output_vec {
                if count > 0 {
                    print!(",");
                }
                count += 1;
                print!("{}",search_count);
                if count >= 5 {
                    break;
                }
            }
            while count < 5 {
                if count > 0 {
                    print!(",");
                }
                count += 1;
                print!("0");
            }
            println!();
        }
        else {
            let disp_count = std::cmp::min(10,output_vec.len());
            println!("\n {} Top Counts {:?} entries",handle.name().unwrap(),&output_vec[0..disp_count]);
        }
       

	}).unwrap(); 
	child.join().unwrap();

}

