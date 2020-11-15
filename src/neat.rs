use crate::nn::Edge;

pub struct Species {
    excess_coeff: f64,
    weight_diff_coeff: f64,
    compat_threashold: f64,
    champion: Vec<Edge>,
}

impl Species {
    pub fn new(excess_coeff: f64, weight_diff_coeff: f64, compat_threashold: f64) -> Species {
        return Species {
            excess_coeff: excess_coeff,
            weight_diff_coeff: weight_diff_coeff,
            compat_threashold: compat_threashold,
            champion: vec![],
        };
    }

    pub fn set_champion(&mut self, new_champ: &Vec<Edge>) -> () {
        self.champion.clear();
        for edge in new_champ.iter() {
            self.champion.push(edge.clone());
        }
    }

    pub fn same_species(&self, other: &Vec<Edge>) -> bool {
        let excess_disjoin = Species::get_excess_disjoint(&self.champion, other);
        let average_weight_diff = Species::get_average_weight_diff(&self.champion, other);
        let compat = (self.excess_coeff * excess_disjoin as f64)
            + (self.weight_diff_coeff * average_weight_diff);
        return self.compat_threashold > compat;
    }

    /// returns the number of excess and disjoint edges.
    /// i.e the number of extra edges and the number of non matching edges.
    pub fn get_excess_disjoint(one: &Vec<Edge>, two: &Vec<Edge>) -> usize {
        let mut matching = 0;
        println!("Edge 1 length: {}", one.len());
        println!("Edge 2 length: {}", two.len());
        for edge_one in one.iter() {
            for edge_two in two.iter() {
                if edge_one.inno_id == edge_two.inno_id {
                    matching += 1;
                    break;
                }
            }
        }
        return one.len() + two.len() - 2 * matching;
    }

    pub fn get_average_weight_diff(one: &Vec<Edge>, two: &Vec<Edge>) -> f64 {
        let mut matching = 0;
        let mut total_diff = 0.0;

        for edge_one in one.iter() {
            for edge_two in two.iter() {
                if edge_one.inno_id == edge_two.inno_id {
                    matching += 1;
                    total_diff += (edge_one.weight - edge_two.weight).abs();
                    break;
                }
            }
        }
        if matching == 0 {
            // divide by zero.
            return 100.0; // todo make this an option?
        }
        return total_diff / matching as f64;
    }
}

#[derive(Debug)]
pub struct InnovationHistory {
    pub global_inno_id: usize,
    pub conn_history: Vec<ConnHistory>,
}

impl InnovationHistory {



    
    pub fn get_inno_number(&mut self,
                           network_inno_ids: &Vec<u64>,
                           from_node: usize,
                           to_node: usize) -> usize {

        let mut is_new = true;
        // todo: change zero to next conn number.
        let mut connect_inno_num = self.global_inno_id;
        self.global_inno_id += 1;
        for conn_history in self.conn_history.iter() {
            if conn_history.matches(network_inno_ids, from_node, to_node) {
            
                is_new = false;
                connect_inno_num = conn_history.inno_number;
                break;
            }
        }

        if is_new {
            let mut new_inno_nums = Vec::<u64>::new();
            for edge in network_inno_ids.iter() {
                new_inno_nums.push(edge.clone());
            }

            self.conn_history.push(ConnHistory::new(
                from_node,
                to_node,
                connect_inno_num,
                new_inno_nums,
            ));
        }

        return connect_inno_num;
    }
}

#[derive(Debug)]
pub struct ConnHistory {
    from_node: usize,
    to_node: usize,
    inno_number: usize,
    inno_numbers: Vec<u64>,
}

impl ConnHistory {
    pub fn new(
        from_node: usize,
        to_node: usize,
        inno_number: usize,
        inno_numbers: Vec<u64>,
    ) -> ConnHistory {
        return ConnHistory {
            from_node: from_node,
            to_node: to_node,
            inno_number: inno_number,
            inno_numbers: inno_numbers,
        };
    }

    /// inserts and returns a new inno_id or if already existing returns current one.
    pub fn matches(&self, network_inno_ids: &Vec<u64>, from_node: usize, to_node: usize) -> bool {
        // ConnHistory must have the same number of numbers as edges
        if network_inno_ids.len() == self.inno_numbers.len() {
            if from_node == self.from_node && to_node == self.to_node {
                for inno_index in 0..network_inno_ids.len() {
                    let net_inno_id = &network_inno_ids[inno_index];
                    if !self
                        .inno_numbers
                        .iter()
                        .any(|inno_id| net_inno_id == inno_id)
                    {
                        return false;
                    }
                }
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_matches() {
        let mut network: Vec<u64> = Vec::new();
        for edge_count in 0..12 {
            network.push(edge_count);
        }
        let global_inno_id = 2;

        let mut innovation_history = InnovationHistory {
            global_inno_id: global_inno_id,
            conn_history: vec![],
        };

        let new_inno_num = innovation_history.get_inno_number(&network, 0, 2);
        assert_eq!(new_inno_num, 2);
        let conn_history = &innovation_history.conn_history[0];

        println!("{:#?}", conn_history);
        println!("{:#?}", innovation_history);

        assert!(conn_history.matches(&network, 0, 2));

        // new connection that hasn't been seen before.
        let new_inno_num = innovation_history.get_inno_number(&network, 0, 20);
        assert_eq!(new_inno_num, global_inno_id + 1);
    }
}
