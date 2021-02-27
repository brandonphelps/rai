/// This module contains data structures and functions for defining a nerual network
/// i've taken a very "real object" approach and modeled it like with realish option,
/// likely they'll be reduced to remove the unneeded objects 


// can't use this here ?
use crate::neat::InnovationHistory;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

fn matching_edge(parent2: &Network, inno_id: u64) -> Option<&Edge> {
    for edge in parent2.edges.iter() {
        if edge.inno_id == inno_id {
            return Some(&edge);
        }
    }
    return None;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    layer: u64,
}

impl Node {
    pub fn clone(&self) -> Node {
        Node {
            layer: self.layer,
        }
    }
}

#[allow(dead_code)]
pub fn sigmoid(value: f64) -> f64 {
    return 1.0 / (1.0 + std::f64::consts::E.powf(-1.0 * value));
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub from_node: u64,
    pub to_node: u64,
    pub weight: f64,
    pub enabled: bool,
    pub inno_id: u64,
}

impl Edge {
    pub fn clone(&self) -> Edge {
        return Edge {
            from_node: self.from_node,
            to_node: self.to_node,
            weight: self.weight,
            enabled: self.enabled,
            inno_id: self.inno_id,
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    // first I nodes are input nodes
    // after which the output nodes are next.
    // todo: determine which of these we can remove from public
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,

    pub input_node_count: u32,
    pub output_node_count: u32,
    pub layer_count: u32,
    pub bias_node_id: u64,
    // can we remove this? networks don't need a fitness, EA items do however
    pub fitness: f64, 
}

pub fn node_per_layer(network: &Network, num_layer: u64) -> Option<u64> {
    let mut node_count = 0;
    if num_layer > network.layer_count.into() {
        return None;
    } else {
        for n in network.nodes.iter() {
            if n.layer == num_layer {
                node_count += 1;
            }
        }
        return Some(node_count);
    }
}

impl Network {
    pub fn fitness(&self) -> f64 {
        return self.fitness;
    }

    pub fn new(input_node_count: u32, output_node_count: u32, fully_connect: bool) -> Network {
        let mut network = Network {
            nodes: Vec::new(),
            edges: Vec::new(),
            input_node_count,
            output_node_count,
            layer_count: 2,
            bias_node_id: 0,
            fitness: 0.0,
        };
        for _input_n in 0..input_node_count {
            network.new_node(0);
        }

        for _output_n in 0..output_node_count {
            network.new_node(1);
        }

        // bias node.
        network.new_node(0);

        network.bias_node_id = (network.nodes.len() - 1) as u64;

        if fully_connect {
            let mut local_inno_id = 0;
            for _input_n in 0..input_node_count {
                for _output_n in 0..output_node_count {
                    network.edges.push(Edge {
                        from_node: _input_n as u64,
                        to_node: (_output_n + input_node_count) as u64,
                        // todo: could be random or zero.
                        weight: 0.5,
                        enabled: true,
                        inno_id: local_inno_id,
                    });
                    local_inno_id += 1;
                }
            }
            // connect bias nodes up
            for _output_n in 0..output_node_count {
                network.edges.push(Edge {
                    from_node: network.bias_node_id,
                    to_node: (_output_n + input_node_count) as u64,
                    weight: 1.0,
                    enabled: true,
                    inno_id: local_inno_id,
                });
                local_inno_id += 1;
            }
        }

        return network;
    }

    pub fn is_fully_connected(&self) -> bool {
        let mut max_connections = 0;
        let mut num_nodes_in_layer: Vec<u64> = Vec::new();

        for _n in 0..self.layer_count {
            num_nodes_in_layer.push(0);
        }

        for node in self.nodes.iter() {
            num_nodes_in_layer[node.layer as usize] += 1;
        }

        for i in 0..self.layer_count - 1 {
            let mut nodes_in_front = 0;
            for j in i + 1..self.layer_count {
                nodes_in_front += num_nodes_in_layer[j as usize];
            }
            max_connections += num_nodes_in_layer[i as usize] * nodes_in_front;
        }

        return (max_connections as usize) == self.edges.len();
    }

    #[allow(dead_code)]
    pub fn pretty_print(&self) -> () {
        for layer in 0..self.layer_count {
            for node in self.nodes.iter() {
                if node.layer == layer as u64 {
                    println!("{:#?}", node);
                }
            }
        }
        for layer in 0..self.layer_count {
            println!("Layer: {}", layer);
            for edge in self.edges.iter() {
                if edge.enabled && self.nodes[edge.from_node as usize].layer == layer as u64 {
                    println!("{:#?}", edge);
                }
            }
        }
    }

    pub fn get_layer(&self, layer_num: u64) -> Vec<&Node> {
        let mut res: Vec<&Node> = Vec::new();
        for n in self.nodes.iter() {
            if layer_num == n.layer {
                res.push(n);
            }
        }
        return res;
    }

    pub fn feed_input(&self, inputs: Vec<f64>) -> Vec<f64> {
	let mut output = Vec::new();


	// todo: should use list instead?
	// maybe map? 
	let mut node_input_sums = Vec::<f64>::new();
	let mut node_output_sums = Vec::<f64>::new();
	for _ in 0..self.nodes.len() {
	    node_input_sums.push(0.0);
	    node_output_sums.push(0.0);
	}

	// set the inputs. 
	for i in inputs.iter() {
	    node_input_sums.push(*i);
	}

	for i in 0..inputs.len() {
	    node_output_sums[i] = inputs[i];
	}

	// set bias to true.
	node_output_sums[self.bias_node_id as usize] = 1.0;


	for layer in 0..self.layer_count {
	    for node_index in 0..self.nodes.len() {
		if self.nodes[node_index].layer == layer as u64 {
		    if layer != 0 {
			node_output_sums[node_index] = sigmoid(node_input_sums[node_index]);
		    }

		    for edge in self.edges.iter() {
			if edge.from_node as usize == node_index {
			    if edge.enabled {
				let tmp_p = edge.weight * node_output_sums[node_index];
				node_input_sums[edge.to_node as usize] += tmp_p;
			    }
			}
		    }
		}
	    }
	}

	for output_i in 0..self.output_node_count {
	    let o_node = node_output_sums[(output_i + self.input_node_count) as usize];
	    output.push(o_node);
	}

	return output;
    }


    pub fn new_node(&mut self, layer: u64) -> u64 {
        let m = Node {
            layer: layer,
        };
        self.nodes.push(m);
        return (self.nodes.len() - 1) as u64;
    }

    pub fn random_node(&self) -> usize {
        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen();
        return (y * (self.nodes.len() - 1) as f64) as usize;
    }

    pub fn random_edge(&self) -> u64 {
        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen();
        return (y * (self.edges.len() - 1) as f64) as u64;
    }

    pub fn random_non_bias_edge(&self) -> u64 {
        let mut edge_index: u64 = self.random_edge();

        while self.edges[edge_index as usize].from_node == self.bias_node_id {
            edge_index = self.random_edge();
        }

        return edge_index;
    }

    fn get_inno_ids(&self) -> Vec<u64> {
        let mut inno_ids: Vec<u64> = Vec::new();

        for edge in self.edges.iter() {
            inno_ids.push(edge.inno_id);
        }
        return inno_ids;
    }

    fn construct_edge(
        &self,
        from_id: usize,
        to_id: usize,
        edge_weight: f64,
        inno_handler: &mut Option<&mut InnovationHistory>,
    ) -> Edge {
        let mut inno_id = 2;
        if let Some(ref mut inno_history) = inno_handler {
            let network_inno_ids = self.get_inno_ids();
            inno_id = inno_history.get_inno_number(&network_inno_ids, from_id, to_id);
        }
        return Edge {
            from_node: from_id as u64,
            to_node: to_id as u64,
            weight: edge_weight,
            enabled: true,
            inno_id: inno_id as u64,
        };
    }

    /// Takes an edge and inserts a node inline.
    pub fn add_node(
        &mut self,
        edge_index: usize,
        edge1_w: f64,
        edge2_w: f64,
        mut inno_handler: Option<&mut InnovationHistory>,
    ) -> u64 {
        self.edges[edge_index].enabled = false;

        let edge = &self.edges[edge_index];
        let incoming_node_id = edge.from_node;
        let outgoing_node_id = edge.to_node;

        // get teh node the edge we are breaking up was pointing to.
        let node = &self.nodes[edge.from_node as usize];
        let current_node_layer = node.layer + 1;

        // new node.
        let m = Node {
            layer: current_node_layer,
        };
        self.nodes.push(m);

        let edge1 = self.construct_edge(
            incoming_node_id as usize,
            self.nodes.len() - 1,
            edge1_w,
            &mut inno_handler,
        );
        self.edges.push(edge1);
        let edge2 = self.construct_edge(
            self.nodes.len() - 1,
            outgoing_node_id as usize,
            edge2_w,
            &mut inno_handler,
        );
        self.edges.push(edge2);

        if self.nodes[outgoing_node_id as usize].layer == current_node_layer as u64 {
            for node_i in 0..self.nodes.len() - 1 {
                let node_t = &mut self.nodes[node_i];
                if node_t.layer >= current_node_layer {
                    node_t.layer += 1;
                }
            }
            self.layer_count += 1;
        }

        return (self.nodes.len() - 1) as u64;
    }

    pub fn are_connected(&self, _node_one: usize, _node_two: usize) -> bool {
        self.edges
            .iter()
            .position(|edge| edge.to_node == _node_one as u64 && edge.from_node == _node_two as u64)
            .is_some()
    }

    pub fn add_connection(
        &mut self,
        _node_one: usize,
        _node_two: usize,
        weight: f64,
        mut inno_hist: Option<&mut InnovationHistory>,
    ) -> usize {
        // todo: don't add in edges if the edge already exists.
        let node_one = &self.nodes[_node_one];
        let node_two = &self.nodes[_node_two];
        if node_one.layer == node_two.layer {
            return 0;
        }
        let edge;
        // allow for nodes to be in reverse order, so if node 1 layer is greater than node 2, swap.
        if node_one.layer > node_two.layer {
            let pos_check = self.edges.iter().position(|edge| {
                edge.to_node == _node_one as u64 && edge.from_node == _node_two as u64
            });
            match pos_check {
                Some(t) => return t,
                None => (),
            }

            edge = self.construct_edge(_node_two, _node_one, weight, &mut inno_hist);
        } else {
            let pos_check = self.edges.iter().position(|edge| {
                edge.to_node == _node_two as u64 && edge.from_node == _node_one as u64
            });
            match pos_check {
                Some(t) => return t,
                None => (),
            }
            edge = self.construct_edge(_node_one, _node_two, weight, &mut inno_hist);
        }
        self.edges.push(edge);
        return self.edges.len() - 1;
    }

    #[allow(dead_code)]
    pub fn mutate(&mut self, inno_history: &mut InnovationHistory) -> () {
        let mut rng = rand::thread_rng();
        // 80% chance to mutate edges node.
        if rng.gen::<f64>() < 0.8 {
            for edge_index in 0..self.edges.len() {
                self.mutate_edge(edge_index);
            }
        }

        // 5% add new connection
        if rng.gen::<f64>() < 0.05 && !self.is_fully_connected() {
            let mut node_one = self.random_node();
            let mut node_two = self.random_node();

            while self.are_connected(node_one, node_two)
                || self.nodes[node_one].layer == self.nodes[node_two].layer
            {
                node_one = self.random_node();
                node_two = self.random_node();
            }
            self.add_connection(node_one, node_two, rng.gen::<f64>(), Some(inno_history));
        }

        // 3% add new node.
        if rng.gen::<f64>() < 0.03 {
            let edge = self.random_non_bias_edge();
            self.add_node(
                edge as usize,
                rng.gen::<f64>(),
                rng.gen::<f64>(),
                Some(inno_history),
            );
        }
    }

    fn mutate_edge(&mut self, edge: usize) -> () {
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < 0.1 {
            self.edges[edge].weight = rng.gen::<f64>();
        } else {
            self.edges[edge].weight += rng.gen_range(-1.0, 1.0);
            if self.edges[edge].weight > 1.0 {
                self.edges[edge].weight = 1.0;
            } else if self.edges[edge].weight < -1.0 {
                self.edges[edge].weight = -1.0;
            }
        }
    }

    pub fn crossover(&self, rhs: &Network) -> Network {
        let mut child_network = Network::new(rhs.input_node_count, rhs.output_node_count, false);
        child_network.nodes.clear();
        child_network.edges.clear();
        child_network.layer_count = self.layer_count;
        child_network.bias_node_id = self.bias_node_id;

        let mut rng = rand::thread_rng();
        for edge in self.edges.iter() {
            let parent2_edge_maybe = matching_edge(rhs, edge.inno_id);
            let mut child_edge_enabled = true;
            // if parent 2 also contains the same edge then determine which to use.
            if let Some(parent2_edge) = parent2_edge_maybe {
                if !edge.enabled || !parent2_edge.enabled && rng.gen::<f64>() < 0.75 {
                    child_edge_enabled = false;
                }
                // determine if edge froms from parent1 or parent2
                let mut new_edge;
                if rng.gen::<f64>() < 0.5 {
                    new_edge = edge.clone();
                    new_edge.enabled = child_edge_enabled;
                } else {
                    new_edge = parent2_edge.clone();
                    new_edge.enabled = child_edge_enabled;
                }
                child_network.edges.push(new_edge);
            } else {
                // disjoint edge from parent 1 and parent 2.
                child_network.edges.push(edge.clone());
            }
        }

        for node in self.nodes.iter() {
            child_network.nodes.push(node.clone());
        }
        return child_network;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn construct_and_network() -> Network {
        let mut network = Network::new(2, 1, false);
        let _edge1 = network.add_connection(0, 2, 20.0, None);
        // let node1 = network.add_node(edge1, 20.0, 20.0) as usize;
        let _edge2 = network.add_connection(1, 2, 20.0, None);
        let _edge3 = network.add_connection(3, 2, -30.0, None);
        return network;
    }

    fn construct_xor_network() -> Network {
        let mut network = Network::new(2, 1, false);

        // node 0 - > end node
        let edge1 = network.add_connection(0, 2, 1.0, None);
        // node 1 - > end node
        let edge2 = network.add_connection(1, 2, -1.0, None);

        // node 0 -> node 2 - > end node
        let node1_index = network.add_node(edge1, 20.0, 20.0, None);

        // node 1 -> node 3 - > end node
        let node2_index = network.add_node(edge2, -20.0, 20.0, None);

        network.add_connection(3, node1_index as usize, -10.0, None);
        network.add_connection(3, node2_index as usize, 30.0, None);
        network.add_connection(3, 2 as usize, -30.0, None);

        // node 0 -> node 3 -> end node
        let _edge3 = network.add_connection(0, node2_index as usize, -20.0, None);

        // node 1 -> node 2 -> end node
        let _edge4 = network.add_connection(1, node1_index as usize, 20.0, None);
        return network;
    }

    #[test]
    fn test_add_node_layer_checking() {
        let mut network = Network::new(1, 3, true);
        let mut output = network.feed_input(vec![0.3]);
        assert_eq!(network.layer_count, 2);
        assert_eq!(output.len(), 3);
        assert_eq!(node_per_layer(&network, 0).unwrap(), 2);

        network.add_node(2, 1.0, 2.0, None);
        output = network.feed_input(vec![0.4]);
        assert_eq!(network.layer_count, 3);
        assert_eq!(output.len(), 3);

        assert_eq!(node_per_layer(&network, 0).unwrap(), 2);

        println!("Output of graph");
        network.pretty_print();
    }

    #[test]
    fn test_simple_add() {
        let mut network = Network::new(1, 1, false);

        network.add_connection(0, 1, 0.5, None);

        assert_eq!(node_per_layer(&network, 0).unwrap(), 2);

        let input_value = vec![1.0, 2.0];
        let mut output_values = network.feed_input(input_value);

        // simplest network
        // 1.0 -> node 1 -(0.5)> node 2 -> output (0.5)

        assert_eq!(output_values.len(), 1);
        assert_eq!(output_values[0], 0.6224593312018546);

        println!("Second evaulation");
        output_values = network.feed_input(vec![1.0]);
        assert_eq!(output_values.len(), 1);
        assert_eq!(node_per_layer(&network, 0).unwrap(), 2);
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
        let network = Network::new(4, 400, true);

        for edge in network.edges.iter() {
            let from_node = &network.nodes[edge.from_node as usize];
            let to_node = &network.nodes[edge.to_node as usize];
            println!("Checking: {} -> {}", edge.from_node, edge.to_node);
            assert!(from_node.layer < to_node.layer);
        }
        assert_eq!(node_per_layer(&network, 0).unwrap(), 5)
    }

    #[test]
    // check to see as we add in nodes that edges preserve
    // forward feed direction.
    fn test_connections_add_node_forward() {
        let mut network = Network::new(4, 400, true);
        let mut node_count = 4 + 400 + 1;
        assert_eq!(network.nodes.len(), node_count);

        for _i in 0..100 {
            let random_edge = network.random_non_bias_edge();
            network.add_node(random_edge as usize, 1.0, 3.0, None);
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

        for _i in 0..100 {
            let random_edge = network.random_non_bias_edge();
            network.add_node(random_edge as usize, 1.0, 3.0, None);
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
    fn test_fully_connected() {
        let network = Network::new(2, 40, true);
        assert!(network.is_fully_connected());

        // todo: add more neteworks with connections.

        let mut net = Network::new(3, 5, true);
        net.add_node(2, 1.0, 2.0, None);
        println!("{:#?}", net);
        assert!(!net.is_fully_connected());
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

    #[test]
    fn test_nodes_per_layer() {
        let mut network = Network::new(2, 3, true);
        // +1 for bias node.
        assert_eq!(node_per_layer(&network, 0).unwrap(), 2 + 1);
        assert_eq!(node_per_layer(&network, 1).unwrap(), 3);

        network.add_node(2, 1.0, 3.0, None);
        assert_eq!(node_per_layer(&network, 0).unwrap(), 2 + 1);
        assert_eq!(node_per_layer(&network, 1).unwrap(), 1);
        assert_eq!(node_per_layer(&network, 2).unwrap(), 3);

        network.add_node(0, 2.0, 2.0, None);
        assert_eq!(node_per_layer(&network, 0).unwrap(), 2 + 1);
        assert_eq!(node_per_layer(&network, 2).unwrap(), 3);
    }

    #[test]
    fn test_max_nodes_of_layers() {}

    #[test]
    fn test_crossover() {
        let network_one = Network::new(2, 3, true);
        let network_two = Network::new(2, 3, true);

        assert_eq!(node_per_layer(&network_one, 0).unwrap(), 2 + 1);
        assert_eq!(node_per_layer(&network_one, 1).unwrap(), 3);

        assert_eq!(node_per_layer(&network_two, 0).unwrap(), 2 + 1);
        assert_eq!(node_per_layer(&network_two, 1).unwrap(), 3);

        let network_three = network_one.crossover(&network_two);

        assert_eq!(node_per_layer(&network_three, 0).unwrap(), 2 + 1);
        assert_eq!(node_per_layer(&network_three, 1).unwrap(), 3);

        network_one.pretty_print();

        network_three.pretty_print();
    }

    #[test]
    fn test_get_layer() {
        let network_one = Network::new(2, 3, true);

        let p = network_one.get_layer(0);
        assert_eq!(p.len(), 3);
        for i in p.iter() {
            assert_eq!(i.layer, 0);
        }
    }
}
