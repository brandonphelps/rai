// can't use this here ?
//#[macro_use]
// extern crate more_asserts;
use rand::prelude::*;

#[derive(Debug)]
pub struct Node {
    input_sum: f64,
    output_sum: f64,
    layer: u64,
}

impl Node {
    pub fn clone(&self) -> Node {
        Node{input_sum:0.0,
             output_sum:0.0,
             layer:self.layer}
    }
}

fn sigmoid(value: f64) -> f64 {
    return 1.0 / (1.0 + std::f64::consts::E.powf(-1.0 * value));
}

#[derive(Debug)]
pub struct Edge {
    pub from_node: u64,
    to_node: u64,
    inno_id: u64,
    pub weight: f64,
    pub enabled: bool
}

impl Edge {
    pub fn clone(&self) -> Edge {
        return Edge{from_node: self.from_node,
                    to_node: self.to_node,
                    weight: self.weight,
                    inno_id: self.inno_id,
                    enabled: self.enabled};
    }
}

#[derive(Debug)]
pub struct Network {

    // first I nodes are input nodes
    // after which the output nodes are next.
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,

    pub input_node_count: u32,
    pub output_node_count: u32,
    pub layer_count: u32,
    pub bias_node_id: u64
}

impl Network {
    fn fully_connect(&mut self) -> () {

    }


    pub fn new(input_node_count: u32, output_node_count: u32, fully_connect: bool ) -> Network {
        let mut network = Network{nodes: Vec::new(),
                                  edges: Vec::new(),
                                  input_node_count: input_node_count,
                                  output_node_count: output_node_count,
                                  layer_count: 2,
                                  bias_node_id: 0};
        for _input_n in 0..input_node_count {
            network.new_node(0);
        }

        for _output_n in 0..output_node_count {
            network.new_node(1);
        }

        // bias node.
        network.new_node(0);

        network.bias_node_id = (network.nodes.len()-1) as u64;

        if fully_connect {
            let mut inno_id = 0;
            for _input_n in 0..input_node_count {
                for _output_n in 0..output_node_count {
                    println!("Constructing egde: {} -> {}", _input_n, _output_n + input_node_count);
                    network.edges.push(Edge {
                        from_node: _input_n as u64,
                        to_node: (_output_n + input_node_count) as u64,
                        inno_id: inno_id,
                        weight: 0.5,
                        enabled: true });
                    inno_id += 1;
                }
            }

            println!("Connectin bias node");
            // connect bias nodes up 
            for _output_n in 0..output_node_count {
                println!("Constructing egde: {} -> {}", network.bias_node_id, _output_n + input_node_count);
                network.edges.push(Edge {
                    from_node: network.bias_node_id,
                    to_node: (_output_n + input_node_count)as u64,
                    inno_id: inno_id,
                    weight: 400.0,
                    enabled: true });
                inno_id += 1;
            }
        }

        return network;
    }

    pub fn pretty_print(&self) -> () {
        for layer in 0..self.layer_count {
            for node in self.nodes.iter() {
                if node.layer == layer.into() {
                    println!("{:#?}", node);
                }
            }
        }
        for layer in 0..self.layer_count {
            println!("Layer: {}", layer);
            for edge in self.edges.iter() {
                if edge.enabled && self.nodes[edge.from_node as usize].layer == layer.into() { 
                    println!("{:#?}", edge);
                }
            }
        }
    }


    pub fn feed_input(&mut self, inputs: Vec<f64> ) -> Vec<f64> {

        let mut output = Vec::new();
        for i in 0..self.input_node_count {
            self.nodes[i as usize].output_sum = inputs[i as usize];
        }

        // set bias to true
        self.nodes[self.bias_node_id as usize].output_sum = 1.0;

        // self.pretty_print();

        for layer in 0..self.layer_count {
            // println!("Feeding layer: {}", layer);
            for node_index in 0..self.nodes.len() {
                if self.nodes[node_index].layer == layer.into() {
                    // set the output sum of the node so it can be used as input for next layer
                    if layer != 0 {
                        // println!("output: {} of {} {}", sigmoid(self.nodes[node_index].input_sum), node_index, self.nodes[node_index].input_sum);
                        self.nodes[node_index].output_sum = sigmoid(self.nodes[node_index].input_sum);
                    }

                    // 
                    for edge in self.edges.iter() {
                        if edge.from_node as usize == node_index {
                            if edge.enabled {
                                let tmp_p = edge.weight * self.nodes[node_index].output_sum;

                                self.nodes[edge.to_node as usize].input_sum += tmp_p;
                                // println!("{} -> {}, ({}) = {}", edge.from_node, edge.to_node, tmp_p, self.nodes[edge.to_node as usize].input_sum);
                            }
                        }
                    }
                }
            }
            // self.pretty_print()
        }

        for output_i in 0..self.output_node_count { 
            let o_node = &self.nodes[(output_i + self.input_node_count) as usize];
            output.push(o_node.output_sum);
        }

        // reset all the input sums for next feed
        for node in self.nodes.iter_mut() {
            node.input_sum = 0.0;
        }

        return output;
    }

    pub fn new_node(&mut self, layer: u64) -> u64 {
        let m = Node{ input_sum: 0.0, output_sum: 0.0, layer: layer};
        self.nodes.push(m);
        return (self.nodes.len() - 1) as u64;
    }
    
    pub fn random_edge(&self) -> u64 {
        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen();
        return (y * (self.edges.len()-1) as f64) as u64;
    }

    pub fn random_non_bias_edge(&self) -> u64 {
        let mut edge_index: u64 = self.random_edge();

        while self.edges[edge_index as usize].from_node == self.bias_node_id  {
            edge_index = self.random_edge();
        }
        return edge_index;
    }


    /// Takes an edge and inserts a node inline. 
    pub fn add_node(&mut self, _edge_index: usize, edge1_w: f64, edge2_w: f64) -> u64 {
        let edge = &mut self.edges[_edge_index];
        edge.enabled = false;
        let outgoing_node_id = edge.to_node;

        // get teh node the edge we are breaking up was pointing to. 
        let node = &self.nodes[edge.from_node as usize];
        let current_node_layer = node.layer + 1;
        
        // new node. 
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


        // bias edge 
        let edge3 = Edge{from_node: self.bias_node_id,
                         to_node: (self.nodes.len() - 1) as u64,
                         inno_id: 2,
                         weight: (edge1_w + edge2_w) / 2.0,
                         enabled: true};

        self.edges.push(edge1);
        self.edges.push(edge2);
        // self.edges.push(edge3);
        
        if self.nodes[outgoing_node_id as usize].layer == current_node_layer.into() {
            for node_i in 0..self.nodes.len()-1 {
                let node_t = &mut self.nodes[node_i];
                if node_t.layer >= current_node_layer {
                    node_t.layer += 1;
                }
            }
            self.layer_count += 1;
        }

        // assert we didn't increase number of outputs
        let mut output_count = 0;
        for node_i in self.nodes.iter() {
            if node_i.layer == (self.layer_count-1).into() {
                output_count += 1;
            }
        }
        assert_eq!(output_count, self.output_node_count);
        return (self.nodes.len() - 1) as u64;
    }

    pub fn add_connection(&mut self, _node_one: usize, _node_two: usize, weight: f64) -> usize {
        // don't add in edges if the edge already exists. 
        let edge = Edge{from_node: _node_one as u64,
                        to_node: _node_two as u64,
                        inno_id: 1,
                        weight: weight,
                        enabled: true};
        self.edges.push(edge);
        return self.edges.len() - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn construct_and_network() -> Network {
        let mut network = Network::new(2, 1, false);
        let edge1 = network.add_connection(0, 2, 20.0);
        // let node1 = network.add_node(edge1, 20.0, 20.0) as usize;
        let edge2 = network.add_connection(1, 2, 20.0);
        let edge3 = network.add_connection(3, 2, -30.0);
        return network;
    }

    fn construct_xor_network() -> Network {
        let mut network = Network::new(2, 1, false);
        
        // node 0 - > end node
        let edge1 = network.add_connection(0, 2, 1.0);
        // node 1 - > end node
        let edge2 = network.add_connection(1, 2, -1.0);
        
        // node 0 -> node 2 - > end node
        let node1_index = network.add_node(edge1, 20.0, 20.0);

        // node 1 -> node 3 - > end node
        let node2_index = network.add_node(edge2, -20.0, 20.0);

        network.add_connection(3, node1_index as usize, -10.0);
        network.add_connection(3, node2_index as usize, 30.0);
        network.add_connection(3, 2 as usize, -30.0);
        
        // node 0 -> node 3 -> end node
        let edge3 = network.add_connection(0, node2_index as usize, -20.0);

        // node 1 -> node 2 -> end node
        let edge4 = network.add_connection(1, node1_index as usize, 20.0);
        return network;
    }

    #[test]
    fn test_add_node_layer_checking() {
        let mut network = Network::new(1, 3, true);
        let mut output = network.feed_input(vec![0.3]);
        assert_eq!(network.layer_count, 2);
        assert_eq!(output.len(), 3);
        network.add_node(2, 1.0, 2.0);
        output = network.feed_input(vec![0.4]);
        assert_eq!(network.layer_count, 3);
        assert_eq!(output.len(), 3);
        println!("Output of graph");
        network.pretty_print();
    }


    #[test]
    fn test_simple_add() {
        let mut network = Network::new(1, 1, false);

        network.add_connection(0, 1, 0.5);

        let input_value = vec![1.0, 2.0];
        println!("First evaulation");
        let mut output_values = network.feed_input(input_value);

        // simplest network
        // 1.0 -> node 1 -(0.5)> node 2 -> output (0.5)

        assert_eq!(output_values.len(), 1);
        assert_eq!(output_values[0], 0.6224593312018546);

        println!("Second evaulation");
        output_values = network.feed_input(vec![1.0]);
        assert_eq!(output_values.len(), 1);
        assert_eq!(output_values[0], 0.6224593312018546);
    }


    #[test]
    fn test_xor_network_one_one() {
        let mut network = construct_xor_network();
        let mut output_values = network.feed_input(vec![1.0, 1.0]);
        println!("{:?}", output_values);
        assert_eq!(output_values.len(), 1);
        assert!(output_values[0] < 0.1);
        assert!(output_values[0] > -0.1);

        output_values = network.feed_input(vec![1.0, 1.0]);
        assert_eq!(output_values.len(), 1);
        assert!(output_values[0] < 0.1);
        assert!(output_values[0] > -0.1);

    }

    // looks to test that all connections go forward rather than backwards. 
    #[test]
    fn test_connections_forward_construction() {
        let mut network = Network::new(4, 400, true);

        for edge in network.edges.iter() {
            let from_node = &network.nodes[edge.from_node as usize];
            let to_node = &network.nodes[edge.to_node as usize];
            println!("Checking: {} -> {}", edge.from_node, edge.to_node);
            assert!(from_node.layer < to_node.layer);
        }
    }


    #[test]
    // check to see as we add in nodes that edges preserve
    // forward feed direction. 
    fn test_connections_add_node_forward() {
        let mut network = Network::new(4, 400, true);
        let mut node_count = 4 + 400 + 1;
        assert_eq!(network.nodes.len(), node_count);

        for i in 0..100 {
            let random_edge = network.random_non_bias_edge();
            network.add_node(random_edge as usize, 1.0, 3.0);
            node_count += 1;
            assert_eq!(network.nodes.len(), node_count);
            for edge in network.edges.iter() {
                let from_node = &network.nodes[edge.from_node as usize];
                let to_node = &network.nodes[edge.to_node as usize];
                println!("Checking: {} -> {}", edge.from_node, edge.to_node);
                assert!(from_node.layer < to_node.layer);
            }
        }
    }

    #[test]
    // check to see as we add in nodes that edges preserve
    // forward feed direction. 
    fn test_connections_add_connection_forward() {
        let mut network = Network::new(4, 400, true);
        let mut node_count = 4 + 400 + 1;
        assert_eq!(network.nodes.len(), node_count);

        for i in 0..100 {
            let random_edge = network.random_non_bias_edge();
            network.add_node(random_edge as usize, 1.0, 3.0);
            node_count += 1;
            assert_eq!(network.nodes.len(), node_count);
            for edge in network.edges.iter() {
                let from_node = &network.nodes[edge.from_node as usize];
                let to_node = &network.nodes[edge.to_node as usize];
                println!("Checking: {} -> {}", edge.from_node, edge.to_node);
                assert!(from_node.layer < to_node.layer);
            }
        }
    }

    macro_rules! network_test {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (mut network, input, expected) = $value;
                    let output = network.feed_input(input);
                    network.pretty_print();
                    assert_eq!(output.len(), 1);
                    assert!(output[0] > expected[0]);
                    assert!(output[0] < expected[1]);
                }
            )*
        }
    }

    network_test! {
        xor_one_one: (construct_xor_network(), vec![1.0, 1.0], vec![-0.1, 0.1]),
        xor_zero_zero: (construct_xor_network(), vec![0.0, 0.0], vec![-0.1, 0.1]),
        xor_one_zero: (construct_xor_network(), vec![1.0, 0.0], vec![0.9, 1.1]),
        xor_zero_one: (construct_xor_network(), vec![0.0, 1.0], vec![0.9, 1.1]),

        and_one_one: (construct_and_network(), vec![1.0, 1.0], vec![0.9, 1.1]),
        and_one_zero: (construct_and_network(), vec![1.0, 0.0], vec![-0.1, 0.1]),
        and_zero_one: (construct_and_network(), vec![0.0, 1.0], vec![-0.1, 0.1]),
        and_zero_zero: (construct_and_network(), vec![0.0, 0.0], vec![-0.1, 0.1]),
            
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
