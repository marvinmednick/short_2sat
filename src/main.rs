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
    fs::remove_dir_all("log").ok();

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
    let num_variables = read_constraints(&mut file, &mut graph);

    if cmd_line.show_graph {
        println!("DirGraph");
        graph.print_graph();
        process::exit(0);
        
    }
    graph.log_graph("kosara");

	let child = thread::Builder::new().name("Working Thread".to_string()).stack_size(512 * 1024 * 1024).spawn(move || { 

	   // code to be executed in thread

        let _handle = thread::current();

        let mut k = Kosaraju::new(&mut graph,true);
        k.find_scc();
        k.log_scc_to_files("kosara");
        // sort in reverse order
        let mut scc_sizes = k.get_scc_sizes();
        scc_sizes.sort_by(|a, b| b.cmp(a));
        info!("K sizes {:?}",scc_sizes);

        let mut constraint_met = true;
        for i in 0..num_variables {
            let vertex_id = (i+1) as isize;
            let not_vertex_id = 0-vertex_id;
            if k.get_group(vertex_id) == k.get_group(not_vertex_id) {
                constraint_met = false;

                break;
            }
        }
        if constraint_met {
            println!("1");
        }
        else {
            println!("0");
        }


	}).unwrap(); 
	child.join().unwrap();

}

