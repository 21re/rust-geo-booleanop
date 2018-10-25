use super::sweep_event::{EdgeType, SweepEvent};
use super::Operation;
use num_traits::Float;
use std::rc::Rc;

pub fn compute_fields<F>(event: &Rc<SweepEvent<F>>, maybe_prev: Option<&Rc<SweepEvent<F>>>, operation: Operation)
where
    F: Float,
{
    if let Some(prev) = maybe_prev {
        if event.is_subject == prev.is_subject {
            event.set_in_out(!prev.is_in_out(), prev.is_other_in_out());
        } else if prev.is_vertical() {
            event.set_in_out(!prev.is_other_in_out(), !prev.is_in_out());
        } else {
            event.set_in_out(!prev.is_other_in_out(), prev.is_in_out());
        }
    } else {
        event.set_in_out(false, true);
    }

    event.set_in_result(in_result(event, operation));
}

fn in_result<F>(event: &SweepEvent<F>, operation: Operation) -> bool
where
    F: Float,
{
    match event.get_edge_type() {
        EdgeType::Normal => match operation {
            Operation::Intersection => !event.is_other_in_out(),
            Operation::Union => event.is_other_in_out(),
            Operation::Difference => {
                (event.is_subject && event.is_other_in_out()) || (!event.is_subject && !event.is_other_in_out())
            }
            Operation::Xor => true,
        },
        EdgeType::SameTransition => operation == Operation::Intersection || operation == Operation::Union,
        EdgeType::DifferentTransition => operation == Operation::Difference,
        EdgeType::NonContributing => false,
    }
}
