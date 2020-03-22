use super::helper::Float;
use super::sweep_event::{EdgeType, ResultTransition, SweepEvent};
use super::Operation;
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

        // Connect to previous in result: Only use the given `prev` if it is
        // part of the result and not a vertical segment. Otherwise connect
        // to its previous in result if any.
        if prev.is_in_result() && !prev.is_vertical() {
            event.set_prev_in_result(prev);
        } else if let Some(prev_of_prev) = prev.get_prev_in_result() {
            event.set_prev_in_result(&prev_of_prev);
        } else {
            // Clearing prev_in_result is necessary for re-computations, if the first
            // computation has already set prev_in_result, but it is no longer valid now.
            event.unset_prev_in_result();
        }
    } else {
        event.set_in_out(false, true);
        // Clearing prev_in_result is necessary for re-computations, if the first
        // computation has already set prev_in_result, but it is no longer valid now.
        event.unset_prev_in_result();
    }

    // Determine whether segment is in result, and if so, whether it is an
    // in-out or out-in transition.
    let in_result = in_result(event, operation);
    let result_transition = if !in_result {
        ResultTransition::None
    } else {
        determine_result_transition(&event, operation)
    };
    event.set_result_transition(result_transition);

    #[cfg(feature = "debug-booleanop")]
    {
        println!(
            "{{\"computeFields\": {{\"inOut\": {}, \"otherOut\": {}, \"resultTransition\": \"{:?}\", \"edgeType\": \"{:?}\"}}}}",
            event.is_in_out(),
            event.is_other_in_out(),
            event.get_result_transition(),
            event.get_edge_type(),
        );
    }
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

fn determine_result_transition<F>(event: &SweepEvent<F>, operation: Operation) -> ResultTransition
where
    F: Float,
{
    let this_in = !event.is_in_out();
    let that_in = !event.is_other_in_out();
    let is_in = match operation {
        Operation::Intersection => this_in && that_in,
        Operation::Union => this_in || that_in,
        Operation::Xor => this_in ^ that_in,
        Operation::Difference =>
        // Difference is assymmetric, so subject vs clipping matters.
        {
            if event.is_subject {
                this_in && !that_in
            } else {
                that_in && !this_in
            }
        }
    };
    if is_in {
        ResultTransition::OutIn
    } else {
        ResultTransition::InOut
    }
}
