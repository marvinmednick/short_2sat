use log::{ info, debug, trace };
use std::collections::{HashMap};
use crate::dirgraph::DirectedGraph;
use std::io::{stdout,Write};

use crate::log_files::LogFile;

#[derive(Debug,Clone)]
pub struct Kosaraju<'a> {
    graph:  &'a DirectedGraph,
    explored:  HashMap<isize,bool>,
    finished:  HashMap<isize,bool>,
	finished_order:  Vec::<isize>,
	start_search:  HashMap::<isize,Vec::<isize>>,
    vertex_scc_map: HashMap::<isize,isize>,
	top_search_cnts:  HashMap::<isize, usize>,
    use_iter: bool,
}


impl<'a> Kosaraju<'a> {

	pub fn new(graph: &DirectedGraph,use_iter : bool) -> Kosaraju {
        debug!("Use iter {}",use_iter);
		Kosaraju {
            graph,
            explored:  HashMap::<isize,bool>::new(),
            finished:  HashMap::<isize,bool>::new(),
            finished_order:  Vec::<isize>::new(),
            start_search : HashMap::<isize,Vec::<isize>>::new(),
            vertex_scc_map: HashMap::<isize,isize>::new(),
            top_search_cnts : HashMap::<isize,usize>::new(),
            use_iter,
		}
	}

    pub fn is_explored(&self, vertex_id: isize) -> bool {
        self.explored.contains_key(&vertex_id) 
    }

    pub fn mark_explored(&mut self, vertex_id: &isize) {
        self.explored.insert(*vertex_id,true); 
    }

    pub fn is_finished(&self, vertex_id: isize) -> bool {
        self.finished.contains_key(&vertex_id) 
    }

    pub fn mark_finished(&mut self, vertex_id: &isize)  {
        self.finished.insert(*vertex_id,true); 
        trace!("Adding {} - finsihed now {:?}",vertex_id,self.finished);
    }

    pub fn set_group(&mut self, vertex_id: isize, group: isize)  {

        self.vertex_scc_map.insert(vertex_id,group); 
    }

    pub fn get_group(&self, vertex_id: isize) -> Option<&isize> {

        self.vertex_scc_map.get(&vertex_id) 
    }



	pub fn add_search_entry(&mut self, vertex: isize, count: usize) {
            // keep track of the current counts
			self.top_search_cnts.insert(vertex,count);
	}

    /// Perform the next level of the Depth First Search on the outgoing edges
    /// from a specfic vertex
	pub fn dfs_outgoing(&mut self, vertex_id:  isize, start_vertex: isize, logfile: &LogFile) {
			
        debug!("Outgoing Exploring {} - start vertex is {}",vertex_id, start_vertex);
        // Set current node to explored
        self.explored.insert(vertex_id,true);
        log_writeln!(logfile,"{}",vertex_id);
    
        // Add this vertex to the group associcated with the 
        // starting vertex, and update the list of top counts (TODO: is this needed)?
        let start_group = self.start_search.entry(start_vertex)
                            .or_insert(Vec::<isize>::new());
        start_group.push(vertex_id);
        let new_len = start_group.len().clone();
        self.add_search_entry(start_vertex,new_len);
        self.set_group(vertex_id,start_vertex);
        trace!("Added vertex {} to list {} len now {}",vertex_id,start_vertex,new_len);


        for edge in self.graph.get_outgoing_edges(vertex_id) {
            let dest_vertex = edge.dest();
            trace!("Checking outgoing to {}",dest_vertex);
           // trace!("Processing {} edge {:?} with dest {}",vertex_id,edge,dest_vertex);
            if !self.is_explored(dest_vertex) {
           //     trace!("Vertex {} Not yet explored",dest_vertex);
                self.dfs_outgoing(dest_vertex,start_vertex,logfile);
            }
            else {
            //    trace!("Vertex {} Already explored: edge",dest_vertex);
            }

        }
        // done with vertex so add it the finished list
        trace!("Outgoing Vertex {} finished",vertex_id);
//        self.finished_order.push(vertex_id);
			
	}

    pub fn dfs_incoming(&mut self, vertex_id:  isize, logfile: &LogFile) {

        if self.use_iter {
            self.dfs_in_iter(vertex_id,logfile);
        }
        else {
            self.dfs_in(vertex_id,logfile);
        }
        trace!("Finishing order now {:?}",self.finished_order);
    }
    /// Perform the next level of the Depth First Search using the incoming edges
    /// from a specfic vertex, keeping track of where the search started
	pub fn dfs_in(&mut self, vertex_id:  isize, logfile: &LogFile) {
			
        debug!("Incoming Exploring {}",vertex_id);
        // Set current node to explored
        self.explored.insert(vertex_id,true);
        log_writeln!(logfile,"{}",vertex_id);

        /*
        // Add this vertex to the group associcated with the 
        // starting vertex, and update the list of top counts (TODO: is this needed)?
        let start_group = self.start_search.entry(start_vertex)
                            .or_insert(Vec::<isize>::new());
        start_group.push(vertex_id);
        let new_len = start_group.len().clone();
        self.add_search_entry(start_vertex,new_len);
        */

        for edge in self.graph.get_incoming_edges(vertex_id) {
            let source_vertex = edge.source();
            trace!("Checking incoming from {}",source_vertex);
            if !self.is_explored(source_vertex) {
                self.dfs_incoming(source_vertex,logfile);
            }
        }
        // done with vertex so add it the finished list
        trace!("Incoming Vertex {} finished",vertex_id);
        self.finished_order.push(vertex_id);

	}

    /// Perform the next level of the Depth First Search using the incoming edges
    /// from a specfic vertex, keeping track of where the search started
	pub fn dfs_in_iter(&mut self, vertex_id:  isize, logfile: &LogFile) {

        let mut dfs_stack = Vec::<isize>::new();
        dfs_stack.push(vertex_id);
			
        while !dfs_stack.is_empty() {
            
            // get a copy the curent vertex from the top of the stack
            let last_index = dfs_stack.len() - 1;

            let cur_vertex = dfs_stack[last_index];
            // trace!("Cur vertex is now {}",cur_vertex);
            // trace!("Explored:  {:?}",self.explored);
            // trace!("Processed:  {:?}",self.processed);
            
            if self.is_explored(cur_vertex) {
                let _ = dfs_stack.pop().unwrap();
                if !self.is_finished(cur_vertex) {
                    trace!("Iter Incoming Vertex {} finished",cur_vertex);
                    self.finished_order.push(cur_vertex);
                    self.mark_finished(&cur_vertex);
                }
                else {
                    trace!("Vertex {} Already finished -- skipping",cur_vertex);
                }
            }
            // if not yet explored then process it...
            else {
                // mark it as explored
                self.explored.insert(cur_vertex,true);
                log_writeln!(logfile,"{}",cur_vertex);

                let in_vertex : Vec<isize> = self.graph.get_incoming_edges(cur_vertex).iter().map(|e| e.source()).collect() ;
                trace!("Incoming edges for loop {:?}",in_vertex);

                // add all the adajacent vertexs with incoming edges that haven't been explored yet
                for edge in self.graph.get_incoming_edges(cur_vertex).iter().rev() {
                    let source_vertex = edge.source();

                    trace!("Check source vertex {}",source_vertex);
                    // if the source vertex has not yet been seen, mark it as seen
                    // and add it to the stack
                    if !self.is_explored(source_vertex) {

                        // and put it on the stack for processing
                        trace!("Iter Adding incoming from {}",source_vertex);
                        dfs_stack.push(source_vertex);
                        trace!("Stack now {:?}",dfs_stack); }

                }
                trace!("Incoming Edge Loop done for {}", cur_vertex);
            }


        }

	}


	pub fn dfs_loop_incoming(&mut self, list: &Vec<isize>,show_progress: bool) {

		info!("Starting on incoming DFS");
		self.finished_order = Vec::<isize>::new();
		self.start_search = HashMap::<isize,Vec::<isize>>::new();
		self.explored = HashMap::<isize,bool>::new();
		self.top_search_cnts = HashMap::<isize,usize>::new();
        let explored_in_log = LogFile::new("kosara_explored_in").unwrap();

		let mut _count : isize = 0;
		for v in list {
			if show_progress && _count % 1000000 == 0 {
				print!("*");
				stdout().flush().unwrap();
			} 
			let vertex = v.clone();
            
			if !self.is_explored(vertex) {
//				self.dfs_incoming(vertex,vertex,0);
				self.dfs_incoming(vertex,&explored_in_log);
//                assert_eq!(self.finished_order,self.iter_finished_order);
              trace!("Finishing Order {:?}",self.finished_order);
			}
			_count += 1;
		}
	}

	pub fn dfs_loop_outgoing(&mut self, list: &Vec<isize>,show_progress: bool) {
		info!("Looping on outgoing DFS {:?}",list);
		self.start_search = HashMap::<isize,Vec::<isize>>::new();
		self.explored = HashMap::<isize,bool>::new();
		self.top_search_cnts = HashMap::<isize,usize>::new();
        let explored_out_log = LogFile::new("kosara_explored_out").unwrap();

		let mut _count : isize = 0;
		for v in list {
			if show_progress && _count % 1000000 == 0 {
				print!("#");
				stdout().flush().unwrap();
			} 
			let vertex = v.clone();

			trace!("OutLoop from {}",vertex);
			if !self.is_explored(vertex) {
				self.dfs_outgoing(vertex,vertex,&explored_out_log);
			}
		}
	}


    pub fn find_scc(&mut self) {


        // Performae a DFS on all vertex to define a finshing order for use in the 2nd DFS
        let list : Vec<isize> = self.graph.get_vertex_ids();
        self.dfs_loop_incoming(&list,false);

        // use the finishing order from the incoming edge dfs as the
        // order of vertexs for teh outogoing search
        let finish_order : Vec<isize> = self.finished_order.iter().rev().cloned().collect();
        self.dfs_loop_outgoing(&finish_order,false);

        info!("Start search has {} entries",self.start_search.len());
        // println!("\n Start search {:?} entries",g.start_search);
        info!("Top Counts {:?} entries",self.top_search_cnts);
        let mut top_search_count_vec : Vec::<(isize, usize)> = self.top_search_cnts.iter().map(|(k,v)| (*k, *v)).collect();
        top_search_count_vec.sort_by(|a, b| b.1.cmp(&a.1));
        info!("Top Search counts Kosaraju.rs {:?}",top_search_count_vec)
            

    }

    pub fn get_scc_sizes(&self) -> Vec<usize> {
        let sizes = self.top_search_cnts.iter().map(|(k,v)| *v).collect();
        debug!("getscc top search cnts {:?} sizes {:?}",self.top_search_cnts, sizes);
        sizes
    }

    pub fn get_finish_order(&self) -> Vec<isize> {
        self.finished_order.iter().cloned().collect()

    }

    pub fn get_scc_group(&self,group : isize) -> Vec<isize> {
        let start_group = self.start_search.get(&group).unwrap();
        start_group.iter().cloned().collect()
    }

    pub fn get_top_scc_groups(&self) -> HashMap<isize,Vec<isize>> {
        let mut result = HashMap::<isize,Vec<isize>>::new();
        for (group,count) in &self.top_search_cnts {
            result.insert(*group,self.get_scc_group(*group));
        }
        result
    }

    pub fn log_scc_to_files(&self, prefix: &str) {

        let summary_file = LogFile::new(&format!("{}_scc_summary.txt",prefix)[..]).unwrap();
        let mut hash_vec: Vec<(&isize, &usize)> = self.top_search_cnts.iter().collect();
        hash_vec.sort_by(|a, b| b.1.cmp(a.1));

        for (group, count) in hash_vec {
            if count > &1 {
                log_writeln!(summary_file,"{}  group {}",count,group);
            }
        }

        let single_scc_file = LogFile::new(&format!("{}/1_single_vertex.scc",prefix)[..]).unwrap();
        let mut sort_by_count: Vec<(&isize, &Vec<isize>)> = self.start_search.iter().collect();
        sort_by_count.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
        for (group_id, vertex_list) in &self.start_search {
            let length = vertex_list.len();
            if length > 1 {
                let scc_file = LogFile::new(&format!("{}/{}_{}.scc",prefix,length,group_id)[..]).unwrap();
                let mut sorted_list : Vec<&isize> = vertex_list.iter().collect();
                sorted_list.sort();
                for v in sorted_list {
                    log_writeln!(scc_file,"{}",v);
                }
            }
            else {
                log_writeln!(single_scc_file,"{}",group_id); 
            }
        }
        
    }
}

