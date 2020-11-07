
#[derive(Debug)]
struct Node {
    input_sum: f64,
    output_sum: f64,
    layer: u64,
}

fn sigmoid(value: f64) -> f64 {
    return 1.0 / (1.0 + std::f64::consts::E.powf(-1.0 * value));
}

impl Node {
    fn engage(&mut self) -> () {
        // iterate over the inbound items and do the sigmoid
        println!("Engage of node: {}", self.layer)
    }
}

struct Edge {
    from_node: u64,
    to_node: u64,
    inno_id: u64,
    weight: f64
}

pub struct Network {

    // first I nodes are input nodes
    // after which the output nodes are next.
    nodes: Vec<Node>,
    edges: Vec<Edge>,

    input_node_count: u32,
    output_node_count: u32,
    layer_count: u32,
}

impl Network {
    pub fn new(input_node_count: u32, output_node_count: u32) -> Network {
        return Network{nodes: Vec::new(),
                       edges: Vec::new(),
                       input_node_count: input_node_count,
                       output_node_count: output_node_count,
                       layer_count: 2};
    }

    pub fn feed_input(&mut self, inputs: Vec<f64> ) -> Vec<f64> {
        let mut output = Vec::new();
        println!("Setting input nodes to values");
        for i in 0..self.input_node_count {
            println!("Setting {} to {}", i, inputs[i as usize]);
            self.nodes[i as usize].output_sum = inputs[i as usize];
        }

        for node_index in 0..self.nodes.len() {
            for layer in 0..self.layer_count {
                if self.nodes[node_index].layer == layer.into() {
                    if layer != 0 {
                        self.nodes[node_index].output_sum = sigmoid(self.nodes[node_index].input_sum);
                        println!("Setting output of {} to {}", node_index, self.nodes[node_index].output_sum );
                    }

                    for edge in self.edges.iter() {
                        if edge.to_node as usize == node_index {
                            self.nodes[edge.to_node as usize].input_sum += edge.weight * self.nodes[node_index].output_sum;
                            println!("Setting next layer {} -> {} {} {}", self.nodes[edge.to_node as usize].layer,
                                     edge.from_node, edge.to_node, self.nodes[edge.to_node as usize].input_sum );
                        }
                    }
                }
            }
        }

        println!("Reading output values");
        for node in self.nodes.iter() {
            if node.layer == (self.layer_count-1).into() {
                output.push(node.output_sum);
            }
        }

        return output;
    }

    pub fn new_node(&mut self, layer: u64) -> () {
        let m = Node{ input_sum: 0.0, output_sum: 0.0, layer: layer};
        self.nodes.push(m);
    }

    /// Takes an edge and inserts a node inline. 
    pub fn add_node(&mut self, _conn: &Edge) -> () {
        
    }

    pub fn add_connection(&mut self, _node_one: usize, _node_two: usize) -> () {
        self.edges.push(Edge{from_node: _node_one as u64,
                             to_node: _node_two as u64,
                             inno_id: 1,
                             weight: 0.5}
                        );
    }

    fn connect_nodes(&mut self) -> () {
        
    }

    pub fn construct_network(& mut self) -> () {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
        let mut network = Network::new(1, 1);
        network.new_node(0);
        network.new_node(1);

        network.add_connection(0, 1);

        let input_value = vec![1.0, 2.0];
        let output_values = network.feed_input(input_value);

        // simplest network
        // 1.0 -> node 1 -(0.5)> node 2 -> output (0.5)
        assert_eq!(output_values.len(), 1);
        assert_eq!(output_values[0], 0.5);
    }

    #[test]
    fn test_sigmoid() {

        let r = sigmoid(0.0);
        println!("Sigmoid of 0 is: {}", r);
        if r > 0.52 {
            assert!(false);
        }
        if r < 0.48 {
            println!("Result of r: {}", r);
            assert!(false);
        }
        
    }
    
}
