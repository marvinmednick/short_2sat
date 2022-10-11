pub trait GraphBuilder {
    fn add_edge(&mut self, source: isize,dest: isize,weight: i32) -> Option<usize>;
    fn add_vertex(&mut self, id:  isize); 
}

