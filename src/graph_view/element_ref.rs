use uuid::Uuid;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum ElementRef {
    Node(Uuid),
    Edge(Uuid),
}