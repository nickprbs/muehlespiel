#[derive(PartialEq)]
pub struct Node<'lifetime, Value: PartialEq> {
    num_visits: usize,
    num_losses: usize,
    children: Vec<Node<'lifetime, Value>>,
    parent: Option<&'lifetime Node<'lifetime, Value>>,
    value: Value
}

impl<'lifetime, Value: PartialEq> Node<'lifetime, Value> {
    /*fn add_child(&'lifetime mut self, child_value: Value) {
        let child = Node {
            num_visits: 0,
            num_losses: 0,
            children: vec![],
            parent: None,
            value: child_value,
        };
        self.children.push(child);
        (self.children.iter_mut()
            .find(|child| child == child)
            .unwrap()).parent = Some(self);
    }*/

    fn get_highest_scoring_child(&self) -> Option<&Node<'lifetime, Value>> {
        /*self.children.iter()
            .max_by(|node|
                (node.num_losses / node.num_visits) + todo!() * sqrt(ln(self.num_visits)/node.num_visits)
            )*/
        todo!()
    }

    fn select_leaf(&self) -> Option<&Node<'lifetime, Value>> {
        let mut highest_scoring_node = None;

        let mut next = self.get_highest_scoring_child();

        while let Some(next_highest_scoring_node) = next {
            highest_scoring_node = Some(next_highest_scoring_node);

            next = next_highest_scoring_node.get_highest_scoring_child();
        }

        highest_scoring_node
    }

    fn expand(&self) {

    }
}