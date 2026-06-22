#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shape(pub [usize; 4]);

impl Shape {
    fn is_scalar(&self) -> bool {
        self.0 == [1; 4]
    }

    fn rangify(&self) -> [Range; 4] {
        self.0.map(Range)
    }
}

#[derive(Clone)]
pub enum HirNode {
    Const(Vec<f32>, Shape),
    Input(usize, Shape),
    Add(Box<Self>, Box<Self>),
    Sub(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),
    Div(Box<Self>, Box<Self>),
    MatMul(Box<Self>, Box<Self>),
}

impl HirNode {
    fn require_valid_elementwise_op(a: &Self, b: &Self) {
        let is_same_shape = a.shape() == b.shape();
        let is_broadcast = a.shape().is_scalar() || b.shape().is_scalar(); // TODO: support more than scalars for broadcasting
        assert!(is_same_shape || is_broadcast);
    }

    fn add(&self, other: &Self) -> Self {
        Self::require_valid_elementwise_op(self, other);
        HirNode::Add(Box::new(self.clone()), Box::new(other.clone()))
    }

    fn mul(&self, other: &Self) -> Self {
        Self::require_valid_elementwise_op(self, other);
        HirNode::Mul(Box::new(self.clone()), Box::new(other.clone()))
    }

    fn shape(&self) -> Shape {
        match self {
            HirNode::Const(_, shape) | HirNode::Input(_, shape) => *shape,
            HirNode::Add(a, b) | HirNode::Sub(a, b) | HirNode::Mul(a, b) | HirNode::Div(a, b) => {
                if dbg!(a.shape().is_scalar()) {
                    b.shape()
                } else {
                    dbg!(a.shape())
                }
            }
            HirNode::MatMul(a, b) => {
                let [ba, ca, ha, _] = a.shape().0;
                let [_, _, _, wb] = b.shape().0;
                Shape([ba, ca, ha, wb])
            }
        }
    }

    /// direct ranges for now ..
    pub fn rangify(&self) -> [Range; 4] {
        self.shape().rangify()
    }
}
