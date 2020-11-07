
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
    weight: f64,
    enabled: bool
}

pub struct Network {

    // first I nodes are input nodes
    // after which the output nodes are next.
    nodes: Vec<Node>,
    edges: Vec<Edge>,

    input_node_count: u32,
    output_node_count: u32,
    layer_count: u32,
    bias_node_id: u64
}

impl Network {
    pub fn new(input_node_count: u32, output_node_count: u32) -> Network {
        let mut network = Network{nodes: Vec::new(),
                                  edges: Vec::new(),
                                  input_node_count: input_node_count,
                                  output_node_count: output_node_count,
                                  layer_count: 2,
                                  bias_node_id: 0};
        for input_n in 0..input_node_count {
            network.new_node(0);
        }

        for output_n in 0..output_node_count {
            network.new_node(1);
        }

        // bias node.
        network.new_node(0);

        network.bias_node_id = (network.nodes.len()-1) as u64;

        return network;
    }

    pub fn feed_input(&mut self, inputs: Vec<f64> ) -> Vec<f64> {
        let mut output = Vec::new();
        println!("Setting input nodes to values");
        for i in 0..self.input_node_count {
            println!("Setting output of node {} to {}", i, inputs[i as usize]);
            self.nodes[i as usize].output_sum = inputs[i as usize];
        }

        for layer in 0..self.layer_count {
            println!("Evaluating layer: {}", layer);
            for node_index in 0..self.nodes.len() {

                if self.nodes[node_index].layer == layer.into() {
                    println!("evaluting node: {}", node_index);
                    // set the output sum of the node so it can be used as input for next layer
                    if layer != 0 {
                        self.nodes[node_index].output_sum = sigmoid(self.nodes[node_index].input_sum);
                        println!("Setting output of node {} to value of {}", node_index, self.nodes[node_index].output_sum );
                    }

                    // 
                    for edge in self.edges.iter() {
                        if edge.to_node as usize == node_index {
                            if edge.enabled {
                                println!("Updating nodes {} -> {}", edge.from_node, node_index);
                                self.nodes[edge.to_node as usize].input_sum += edge.weight * self.nodes[edge.from_node as usize].output_sum;
                                println!("Setting input of node {} to value of {}", edge.to_node, self.nodes[edge.to_node as usize].input_sum);
                            }
                        }
                    }
                }
                self.nodes[node_index].input_sum = 0.0;
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


    // nod1, nod2
    // edge <nod1 -- node2>
    //   edge 0, 1
    // nod1 --> node3 --> node2
    


    /// Takes an edge and inserts a node inline. 
    pub fn add_node(&mut self, _edge_index: usize, edge1_w: f64, edge2_w: f64) -> () {
        let edge = &mut self.edges[_edge_index];
        edge.enabled = false;

        
        // let edge = &self.edges[_edge_index];

        let node = &self.nodes[edge.from_node as usize];

        let current_node_layer = node.layer + 1;
        let m = Node{ input_sum: 0.0, output_sum: 0.0, layer: current_node_layer};

        self.nodes.push(m);

        let edge1 = Edge{from_node: edge.from_node,
                         to_node: (self.nodes.len() - 1) as u64,
                         inno_id: 2,
                         weight: edge1_w,
                         enabled: true};

        let edge2 = Edge{from_node: (self.nodes.len() - 1) as u64,
                         to_node: edge.to_node,
                         inno_id: 2,
                         weight: edge2_w,
                         enabled: true};
        let outgoing_node_id = edge.to_node;

        self.edges.push(edge1);
        self.edges.push(edge2);

        for node_i in 0..self.nodes.len()-1 {
            let node_t = &mut self.nodes[node_i];
            if node_t.layer >= current_node_layer {
                node_t.layer += 1;
            }
        }
        self.layer_count += 1;
    }

    pub fn add_connection(&mut self, _node_one: usize, _node_two: usize, weight: f64) -> usize {
        let edge = Edge{from_node: _node_one as u64,
                        to_node: _node_two as u64,
                        inno_id: 1,
                        weight: weight,
                        enabled: true};
        self.edges.push(edge);
        return self.edges.len() - 1;
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

        network.add_connection(0, 1, 0.5);

        let input_value = vec![1.0, 2.0];
        println!("First evaulation");
        let mut output_values = network.feed_input(input_value);

        // simplest network
        // 1.0 -> node 1 -(0.5)> node 2 -> output (0.5)

        assert_eq!(output_values.len(), 1);
        assert_eq!(output_values[0], 0.5);

        println!("Second evaulation");
        output_values = network.feed_input(vec![1.0]);
        assert_eq!(output_values.len(), 1);
        assert_eq!(output_values[0], 0.5);
    }


    fn test_xor_network() {
        let mut network = Network::new(2, 1);

        
    }


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
