trait Visitor {
    type Value;

    fn visit_device(&self, v: Device) -> Self::Value;
    fn visit_ip(&self, v: Ip) -> Self::Value;
}

struct Device {
    id: String,
}

enum Ip {
    V4(String),
    V6(String),
}

struct Parser;
impl Visitor for Parser {
    type Value = String;

    fn visit_device(&self, v: Device) -> Self::Value {
        v.id
    }

    fn visit_ip(&self, v: Ip) -> Self::Value {
        match v {
            Ip::V4(v) => format!("V4 {v}"),
            Ip::V6(v) => format!("V6 {v}"),
        }
    }
}

/// Router - enables communication between multiple networks by forwarding data packets.
/// It examines incoming packets and determines destination and using routing table,
/// determines destinations.
///
/// Switch - connects multiple devices within a network, allowing them to communicate. These
/// are mostly used in LANs.
///
///
enum Node {
    Router(Vec<Node>),
    Switch(Vec<Node>),
    Firewall,
    PC,
    Server,
}

fn visualise_network_topology(node: &Node, indent: usize, buffer: &mut String) {
    let prefix = " ".repeat(indent);

    match node {
        Node::Router(children) => {
            buffer.push_str(&format!("{prefix}Router\n"));

            for child in children {
                visualise_network_topology(child, indent + 2, buffer);
            }
        }
        Node::Switch(childen) => {
            buffer.push_str(&format!("{prefix}Switch\n"));

            for child in childen {
                visualise_network_topology(child, indent + 2, buffer);
            }
        }
        Node::PC => {
            buffer.push_str(&format!("{prefix}PC\n"));
        }
        Node::Firewall => {
            buffer.push_str(&format!("{prefix}Firewall\n"));
        }
        Node::Server => {
            buffer.push_str(&format!("{prefix}Server\n"));
        }
        _ => (),
    }
}

fn count_endpoints(node: &Node) -> usize {
    match node {
        Node::PC | Node::Server => 1,
        Node::Router(children) | Node::Switch(children) => {
            children.iter().map(count_endpoints).sum()
        }
        Node::Firewall => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn network_walk_works() {
        let topology = Node::Router(vec![
            Node::Switch(vec![Node::PC, Node::Server]),
            Node::Firewall,
        ]);

        let total_endpoints = count_endpoints(&topology);
        assert!(total_endpoints == 2);
    }

    #[test]
    fn visualise_topology_works() {
        let topology = Node::Router(vec![
            Node::Switch(vec![Node::PC, Node::Server, Node::PC, Node::PC]),
            Node::Firewall,
            Node::Router(vec![
                Node::Server,
                Node::Server,
                Node::PC,
                Node::Router(vec![Node::Server, Node::Server]),
            ]),
        ]);

        let mut buffer = String::new();
        visualise_network_topology(&topology, 2, &mut buffer);

        let expected = "  Router\n    Switch\n      PC\n      Server\n      PC\n      PC\n    Firewall\n    Router\n      Server\n      Server\n      PC\n      Router\n        Server\n        Server\n";
        assert_eq!(buffer, expected);
    }
}
