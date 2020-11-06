
struct Node {
    input_sum: f32,
    output_sum: f32,
    layer: u64,
}

impl Node {
    fn engage(&mut self) -> () {
        // iterate over the inbound items and do the sigmoid
    }
}

struct Edge {
    from_node: u64,
    to_node: u64,
    innoId: u64,
    weight: f64
}


pub struct Network<'a> {

    // first I nodes are input nodes
    // after which the output nodes are next.
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    network: Vec<&'a Node>,
    input_node_count: u32,
    output_node_count: u32,
    layer_count: u32,
}

impl<'a> Network<'a> {
    pub fn feed_input(&mut self, inputs: Vec<f32> ) -> Vec<f32> {
        let mut output = Vec::new();
        for i in 0..self.input_node_count {
            self.nodes[i as usize].output_sum = inputs[i as usize];
        }

        return output;
    }

    pub fn add_connection(&mut self, conn: Edge) -> () {
        
    }

    pub fn construct_network(&'a mut self) -> () {
        self.network.clear();

        for layer in 0..self.layer_count {
            for node in self.nodes.iter() {
                if node.layer == layer.into() {
                    self.network.push(&node);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
    }
}
