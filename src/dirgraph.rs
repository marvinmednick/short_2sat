//use std::process; use std::io::{self, Write}; // use std::error::Error;
//use std::cmp;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use log::{  info, error, debug, /*warn ,*/ trace };

use std::fmt::Display; 
use std::fmt;

use crate::graphbuilder::GraphBuilder;
use crate::LogFile;


#[derive(Debug,Clone)]
pub struct Edge {
    edge_id: usize,
    source:  isize,
    dest:    isize,
    weight:  i32,
}

impl Display for Edge {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: ({} -> {} w{})", self.edge_id, self.source, self.dest, self.weight)
    }

}

impl Edge {

    pub fn new(new_edge_id: usize, source_vertex_id: isize, dest_vertex_id: isize, weight: i32 ) -> Edge {
        trace!("New Edge {} from {} to {} with weight {}",new_edge_id,source_vertex_id,dest_vertex_id,weight);
        Edge {
            edge_id:    new_edge_id,
            source:     source_vertex_id,
            dest:       dest_vertex_id,
            weight:     weight,
        }
    }


    /// Returns the starting vertex of the egde
    pub fn source(&self) -> isize {
        self.source
    }

    /// Returns the terminating vertex of the egde
    pub fn dest(&self) -> isize {
        self.dest
    }

    /// Returns the weight of the egde
    pub fn weight(&self) -> i32 {
        self.weight
    }
}



#[derive(Debug, Clone)]
pub struct Vertex {
	vertex_id: isize,
    // set of incomin and outgoing edge ids
	incoming: BTreeSet<usize>,
	outgoing: BTreeSet<usize>,
    adjustment: i32,
}

impl Vertex {

	pub fn new(id : isize) -> Vertex {
		let incoming = BTreeSet::<usize>::new();
		let outgoing = BTreeSet::<usize>::new();
		Vertex {vertex_id: id, 
				incoming: incoming, 
				outgoing: outgoing,
                adjustment: 0,
				}
	}

    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Vertex {}", self.vertex_id)
    }
	
	pub fn add_outgoing_edge_id(&mut self, edge_id: usize) {
        trace!("Adding outgoing edge {} to vertex {}",edge_id, self.vertex_id);
        if !self.outgoing.insert(edge_id) {
           error!("add_outgoing_edge_id: Vertex {} - outgoing edge {} already exists",edge_id, self.vertex_id)
        }
	}

	pub fn delete_outgoing_edge_id (&mut self, edge_id: usize) {
        if !self.outgoing.remove(&edge_id) {
           error!("delete_outgoing_edge_id:  Vertex {} - outgoing edge {} doesn't exist",self.vertex_id,edge_id)
        }
	}

	pub fn add_incoming_edge_id(&mut self, edge_id: usize) {
        trace!("Adding incoming edge {} to vertex {}",edge_id,self.vertex_id);
        if !self.incoming.insert(edge_id) {
           error!("add_incoming: Vertex {} - outgoing edge {} already exists",self.vertex_id,edge_id)
        }
	}

	pub fn delete_incoming_edge_id (&mut self, edge_id: usize) {
        if !self.incoming.remove(&edge_id) {
           error!("delete_incoming_edge_id:  Vertex {} - outgoing edge {} doesn't exist",self.vertex_id,edge_id)
        }
	
	}

    /// Gets a vector of the incoming edge Ids
    pub fn get_outgoing_edge_ids(&self)  -> Vec<usize>{
        // get the list of outgoing edges and map them to the dest vertex
		self.outgoing.iter().cloned().collect()
    }

    /// Gets a vector of the outgoing edge Ids
    pub fn get_incoming_edge_ids(&self)  -> Vec<usize>{
        // get the list of outgoing edges and map them to the dest vertex
		self.incoming.iter().cloned().collect()
    }

    pub fn id(&self) -> isize {
        self.vertex_id
    }

    pub fn set_adjustment(&mut self, amount: i32) {
        self.adjustment = amount;
    }

    pub fn adjustment(&self) -> i32 {
        self.adjustment
    }



}


#[derive(Debug,Clone)]
pub struct DirectedGraph {
    ///Vertex Map maps a vertex Id to the Vertex Data structure for it
	vertex_map:  BTreeMap::<isize, Vertex>,
    ///Edge Map maps a edge Id to the Edge Data structure for it
    edge_map:   BTreeMap::<usize, Edge>,
    /// Edge Ids are automatically assiged by define edge and this is the ID of the next edge to be defined
    next_edge_id:  usize
}


impl GraphBuilder for &mut DirectedGraph {

	fn add_edge(&mut self, v1: isize, v2: isize, weight: i32) -> Option<usize> {

		//create the vertexes, if the don't exist
		self.define_vertex(v1.clone());
		self.define_vertex(v2.clone());
        if let Some (edge_id) = self.define_edge(v1.clone(),v2.clone(),weight) {
            let v_map = &mut self.vertex_map;

            // add the edge to the first vertex's adjacency outgoing list
            let vert1 = v_map.get_mut(&v1).unwrap();
            vert1.add_outgoing_edge_id(edge_id);

            // add the edge to the second vertex adjacency incoming list
            let vert2 = v_map.get_mut(&v2).unwrap();
            vert2.add_incoming_edge_id(edge_id);
            Some(edge_id)
        }
        else {
            error!("Error adding Edge  v1 {} v2 {} w {}",v1,v2,weight);
            None
        }

	}


    fn add_vertex(&mut self, id:  isize) { 
        self.define_vertex(id);
    }
}


impl DirectedGraph {
	pub fn new() -> DirectedGraph {
		let v_map = BTreeMap::<isize, Vertex>::new();
		let e_map = BTreeMap::<usize, Edge>::new();
		DirectedGraph {
				vertex_map:     v_map,
				edge_map:       e_map,
                next_edge_id:   1,
		}
	}

    /// Defines a new Vertex
	pub fn define_vertex(&mut self, id: isize) -> Option<usize> {

		if self.vertex_map.contains_key(&id) {
			None
		} 
		else { 
            trace!("Adding Vertex {}",id);
			let v = Vertex::new(id.clone());
			self.vertex_map.insert(id,v);
			Some(self.vertex_map.len())  
		}
    }

	pub fn define_edge(&mut self, source: isize, dest: isize, weight: i32 ) -> Option<usize> {
    //    if source != 0 && dest != 0 {
            let edge_id = self.next_edge_id.clone();
            self.next_edge_id += 1;
			let e = Edge::new(edge_id, source, dest, weight);
			self.edge_map.insert(edge_id,e);
            Some(edge_id)
     //   }
      //  else {
       //     warn!("Invalid edge input 0  source {} dest {} weight {}", source, dest, weight);
        //    None
        //}
	}



    
	pub fn get_outgoing_edges(&self, vertex: isize) -> Vec<Edge>{
		let v = self.vertex_map.get(&vertex).unwrap();
        // get the list of outgoing edges
        // by mapping each id to its dest element
        // NOTE: since edge list is coming from the vertex, this isn't handling the case where edge_map.get
        // returns 'None' ; this shouldn't occur, and will crash here if it did
		v.get_outgoing_edge_ids().iter().map(|x| self.edge_map.get(&x).unwrap().clone()).collect()
		
	}

    /// retreives a vector of outogoing vertex_id from a given vertex
	pub fn get_outgoing_vertex_ids(&self, vertex: isize) -> Vec<isize>{
		let v = self.vertex_map.get(&vertex).unwrap();
        // get the list of vertexe that this vertex has outgoing edges to (i.e vertexes that )accessible from this vertex)
        // by mapping each id to its dest element
        // NOTE: since edge list is coming from the vertex, this isn't handling the case where edge_map.get
        // returns 'None' ; this shouldn't occur, and will crash here if it did
		v.get_outgoing_edge_ids()
            .iter()
            .map(|x| {let e = self.edge_map.get(&x).unwrap(); e.dest }
            .clone())
            .collect()
	}


	pub fn get_outgoing_edge_ids(&self, vertex: isize) -> Vec<usize>{
		let v = self.vertex_map.get(&vertex).unwrap();
		v.get_outgoing_edge_ids()
    }

    /// retreives a vector of incoming edges to a given vertex
	pub fn get_incoming_edges(&self, vertex: isize) -> Vec<Edge>{
		let v = self.vertex_map.get(&vertex).unwrap();
        // get the list of outgoing edges
        // by mapping each id to its dest element
        // NOTE: since edge list is coming from the vertex, this isn't handling the case where edge_map.get
        // returns 'None' ; this shouldn't occur, and will crash here if it did
		v.get_incoming_edge_ids().iter().map(|x| self.edge_map.get(&x).unwrap().clone()).collect()
		
	}

    /// retreives a vector of incoming vertex_id from a given vertex
	pub fn get_incoming_vertex_ids(&self, vertex: isize) -> Vec<isize>{
		let v = self.vertex_map.get(&vertex).unwrap();
        // get the list of vertexes that have edges incoming to this vertex 
        // by mapping each id to its dest element
        // NOTE: since edge list is coming from the vertex, this isn't handling the case where edge_map.get
        // returns 'None' ; this shouldn't occur, and will crash here if it did
		v.get_incoming_edge_ids()
            .iter()
            .map(|x| {let e = self.edge_map.get(&x).unwrap(); e.source }
            .clone())
            .collect()
	}


	pub fn get_incoming_edge_ids(&self, vertex: isize) -> Vec<usize>{
		let v = self.vertex_map.get(&vertex).unwrap();
		v.get_incoming_edge_ids()
    }

    /// return the weight of the incoming connection from a given ource vertex (if it existss) or
    /// None
    ///
    pub fn get_incoming_connection_weight(&self, source: isize, vertex: isize) -> Option<i32> {
		let v = self.vertex_map.get(&vertex).unwrap();

		let find_result = v.get_incoming_edge_ids()
            .iter()
            .map(|x| { let edge = self.edge_map.get(&x).unwrap(); (edge.source.clone(), edge.weight.clone()) })
            .find(|e| { e.0 == source } );
        
        debug!("Incoming Result info looking for source {} as incoming to {} {:?}",source, vertex , find_result);
        match find_result {
            None => None,
            Some((_vertex,weight)) => Some(weight),
        }
    }


    /// return the weight of the incoming connection from a given ource vertex (if it existss) or
    /// None
    ///
    pub fn get_outgoing_connection_weight(&self, vertex: isize, dest: isize) -> Option<i32> {
		let v = self.vertex_map.get(&vertex).unwrap();

		let find_result = v.get_outgoing_edge_ids()
            .iter()
            .map(|x| { let edge = self.edge_map.get(&x).unwrap(); (edge.dest.clone(), edge.weight.clone()) })
            .find(|e| { e.0 == dest } );
        
        debug!("get_outoing_conn_weight: dest {} outgoing from {} {:?}",dest, vertex , find_result);
        match find_result {
            None => None,
            Some((_vertex,weight)) => Some(weight),
        }
    }


    /// get an iterator to all of the vertexes in the graph
    pub fn vertex_iter(&self) -> std::collections::btree_map::Iter<'_, isize, Vertex> {
        self.vertex_map.iter()
    }

    /// get an iterator to all of the vertexes in the graph
    pub fn vertex_iter_mut(&mut self) -> std::collections::btree_map::IterMut<'_, isize, Vertex> {
        self.vertex_map.iter_mut()
    }
    /// get an iterator to all of the edges in the graph
    pub fn edge_iter(&self) -> std::collections::btree_map::Iter<'_, usize, Edge> {
        self.edge_map.iter()
    }

    /// get a complete list of vertex ids in the graph
	pub fn get_vertex_from_id(&self, id: isize) -> Option<&Vertex> {
		self.vertex_map.get(&id)
	}
    ///
    /// get a complete list of vertex ids in the graph
	pub fn get_edge_from_id(&self, id: usize) -> Option<&Edge> {
		self.edge_map.get(&id)
	}


    /// get a complete list of vertex ids in the graph
	pub fn get_vertex_ids(&self) -> Vec<isize> {
		self.vertex_map.keys().cloned().collect()
	}

    /// get a complete list of edge ids in the graph
	pub fn get_edge_ids(&self) -> Vec<usize> {
		self.edge_map.keys().cloned().collect()
	}

	pub fn print_graph(&self) {
        println!("Vertexes:");
		for (key, value) in &self.vertex_map {
//			let out_list : String = value.outgoing.iter().map(|x| {let e = self.edge_map.get(x).unwrap(); format!("e{} v{}(w{}) ; ",x,e.dest,e.weight) }).collect();
			let out_list : String = value.outgoing.iter().map(|x| 
                  {
                    let e = self.edge_map.get(x).unwrap_or(&Edge { edge_id: 0,source: 0, dest: 0, weight: 0 }); 
                    format!("{} ; ",e)
                  }
                  ).collect();
			println!("Vertex {} ({}) :  outgoing list: {}",key,value.vertex_id,out_list);

			let in_list : String = value.incoming.iter().map(|x| {let e = 
                    self.edge_map.get(x)
                        .unwrap_or(&Edge { edge_id: 0,source: 0, dest: 0, weight: 0 }); format!("{} ; ",e) })
                        .collect();
			println!("Vertex {} ({}) :  incoming list: {}",key,value.vertex_id,in_list);
		}
        println!("Edges");
        for (key, value) in &self.edge_map {
            println!("Edge id {}   {:?}", key, value);
        }


					
	}

    pub fn log_graph(&self, prefix : &str) {

        let outgoing_file = LogFile::new(&format!("{}_outgoing",prefix)[..]).unwrap();
        let incoming_file = LogFile::new(&format!("{}_incoming",prefix)[..]).unwrap();
        trace!("Logging Graph");

		for (key, value) in &self.vertex_map {
			let mut list_vec : Vec<isize> = value.outgoing.iter()
                .map(|x|
                    { let e = self.edge_map.get(x).unwrap_or(&Edge { edge_id: 0,source: 0, dest: 0, weight: 0 }); 
                     e.dest()
                    }).collect();
                list_vec.sort();
                let list : String = list_vec.iter().map(|x| format!("{} ; ",x)).collect();
			log_writeln!(outgoing_file, "Vertex {} ({}) : {}",key,value.vertex_id,list);

			let list_vec : Vec<isize> = value.incoming.iter()
                .map(|x|
                     { let e = self.edge_map.get(x).unwrap_or(&Edge { edge_id: 0,source: 0, dest: 0, weight: 0 });
                        trace!("v {} src {} e {}",key,e.source(),e);
                        e.source()
                     }).collect();
            let list : String = list_vec.iter().map(|x| format!("{} ; ",x)).collect();
			log_writeln!(incoming_file,"Vertex {} ({}) : {}",key,value.vertex_id,list);
		}

    }


	pub fn delete_edge(&mut self,edge_id: usize) -> Result<(),String>  {
	
        if let Some(edge) = self.edge_map.get(&edge_id) {
            self.vertex_map.get_mut(&edge.source).unwrap().delete_outgoing_edge_id(edge_id)	;
            self.vertex_map.get_mut(&edge.dest).unwrap().delete_incoming_edge_id(edge_id);
            self.edge_map.remove(&edge_id);
            Ok(())
        }
        else {
            error!("delete edge:  No such edge {}",edge_id);
            Err("Delete Edge: No such edge".to_string())
        }

	}

    pub fn vertex_count(&self) -> usize {
        self.vertex_map.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edge_map.len()
    }

    pub fn verify_path(&self, path: Vec<isize> ) -> Option<i32> {
        let mut total_weight = 0;
        
        info!("Checking to see if path {:?} is valid",path);
        for path_index in 0..path.len()-1 {
            let source = path[path_index];
            let dest = path[path_index+1];
            if let Some(weight) = self.get_outgoing_connection_weight(source,dest) {
                total_weight += weight;
                info!("Vertex {} has an outgoing connection to Vertex {} with a weight of {}",source,dest,weight);
            }
            else {
                error!("No outgoing connection from {} to {}",source,dest);
                return None
            }

        }
        Some(total_weight)
        
    }

    /// get an iterator to all of the edges in the graph
    pub fn adjust_edges(&mut self) {

        for edge_id in self.get_edge_ids() {
            let edge_info = self.get_edge_from_id(edge_id).unwrap();
            let source_adjustment = self.get_vertex_from_id(edge_info.source).unwrap().adjustment(); 
            let dest_adjustment = self.get_vertex_from_id(edge_info.dest).unwrap().adjustment(); 
            let adjusted_weight = edge_info.weight + source_adjustment - dest_adjustment;

            let mut edge = self.edge_map.get_mut(&edge_id).unwrap();
            edge.weight = adjusted_weight;

        }
    }

}




// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use crate::dirgraph::DirectedGraph;
    use crate::graphbuilder::GraphBuilder;
    use log::{  info };


    #[test]
    fn most_basic() {
		let mut g = DirectedGraph::new();
		assert_eq!(g.define_vertex(1),Some(1));
		assert_eq!((&mut g).add_edge(2,3,1),Some(1));
		assert_eq!((&mut g).add_edge(1,2,1),Some(2));
    }

    fn test_init() -> DirectedGraph {
          println!("starting");
          let _ = env_logger::builder().is_test(true).try_init();
          info!("Init {}",module_path!());
          DirectedGraph::new()
    }

	fn setup_basic1() -> DirectedGraph {
		let mut graph = test_init();
        let mut g = &mut graph;
        // 1->2->3-4
        // 1->3
        // 2->
		assert_eq!(g.add_edge(1,2,1),Some(1));
		assert_eq!(g.add_edge(1,3,1),Some(2));
		assert_eq!(g.add_edge(2,3,1),Some(3));
		assert_eq!(g.add_edge(2,4,22),Some(4));
		assert_eq!(g.add_edge(3,4,33),Some(5));
		assert_eq!(g.get_outgoing_vertex_ids(1),&[2,3]);
		assert_eq!(g.get_outgoing_vertex_ids(2),&[3,4]);
		assert_eq!(g.get_outgoing_vertex_ids(3),&[4]);
		assert_eq!(g.get_outgoing_vertex_ids(4),&[]);
		graph
	} 

    #[test]
    fn basic() {
		let mut graph = test_init();
        let mut g = &mut graph;
		assert_eq!(g.define_vertex(1),Some(1));
		assert_eq!(g.define_vertex(2),Some(2));
		assert_eq!(g.add_edge(1,2,1),Some(1));
		assert_eq!(g.get_vertex_ids(),vec!(1,2));
		assert_eq!(g.define_vertex(3),Some(3));
		assert_eq!(g.add_edge(1,3,1),Some(2));
		assert_eq!(g.add_edge(2,3,1),Some(3));
		assert_eq!(g.get_vertex_ids(),vec!(1,2,3));
		assert_eq!(g.add_edge(1,4,1),Some(4));
		assert_eq!(g.get_vertex_ids(),vec!(1,2,3,4));
//		println!("{:?}",g);

    }

    
	#[test]
	fn test_add() {
		let mut graph = test_init();
        let mut g = &mut graph;
		assert_eq!(g.add_edge(1,2,1),Some(1));
//		println!("{:#?}",g);
		assert_eq!(g.get_outgoing_vertex_ids(1),&[2]);
		assert_eq!(g.get_incoming_vertex_ids(2),&[1]);
		assert_eq!(g.add_edge(1,3,1),Some(2));
		assert_eq!(g.get_outgoing_vertex_ids(1),&[2,3]);
		assert_eq!(g.get_incoming_vertex_ids(2),&[1]);
	}

	#[test]
	fn test_add_del() {
		let mut graph = setup_basic1();
        let mut g = &mut graph;
		assert_eq!(g.get_outgoing_vertex_ids(1),&[2,3]);
		assert_eq!(g.add_edge(1,2,1),Some(6));
//		println!("{:#?}",g);
		assert_eq!(g.get_outgoing_vertex_ids(1),&[2,3,2]);
		assert_eq!(g.get_outgoing_vertex_ids(2),&[3,4]);
		assert_eq!(g.get_outgoing_vertex_ids(3),&[4]);
		assert_eq!(g.delete_edge(6),Ok(()));
		assert_eq!(g.get_outgoing_vertex_ids(1),&[2,3]);
		assert_eq!(g.delete_edge(1),Ok(()));
		assert_eq!(g.get_outgoing_vertex_ids(1),&[3]);
		
	}


	#[test]
	fn test_incoming_connection_info() {
		let mut graph = setup_basic1();
        let mut g = &mut graph;
//		println!("{:#?}",g);
		assert_eq!(g.get_incoming_connection_weight(2,4),Some(22));
		assert_eq!(g.get_incoming_connection_weight(1,4),None);
		assert_eq!(g.get_outgoing_connection_weight(1,2),Some(1));
		assert_eq!(g.get_outgoing_connection_weight(1,4),None);
		assert_eq!(g.add_edge(1,4,44),Some(6));
		assert_eq!(g.get_outgoing_connection_weight(1,4),Some(44));

	}

	#[test]
	fn test_verify_path() {
		let mut graph = setup_basic1();
        let g = &mut graph;
		assert_eq!(g.verify_path(vec!(1,2,3,4)),Some(35));
		assert_eq!(g.verify_path(vec!(4,1)),None);
    }
}
