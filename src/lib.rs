use pyo3::prelude::*;
use pyo3::exceptions::*;
//use pyo3::types::{PyAny, Bound};


#[pyclass]
struct Tree {

    #[pyo3(get, name = "children")]
    _children: Vec<Vec<usize>>, 

    #[pyo3(get, name = "transitions")]
    _transitions: Vec<Vec<Py<PyAny>>>,

    #[pyo3(get, name = "parents")]
    _parents: Vec<Option<usize>>,

    #[pyo3(get, name = "values")]
    _values: Vec<Option<Py<PyAny>>>
}

#[pymethods]
impl Tree {

    #[new]
    fn new() -> Self {
        Tree{
            _children: Vec::<Vec<usize>>::new(),
            _transitions: Vec::<Vec<Py<PyAny>>>::new(),
            _parents: Vec::<Option<usize>>::new(),
            _values: Vec::<Option<Py<PyAny>>>::new()
        }
    }

    #[pyo3(signature=(parent, conns, trans, value, parent_transition))]
    ///  Function to add a node to the tree.
    fn _add_node(&mut self, parent: Option<usize>, conns: Vec<usize>, trans: Vec<Py<PyAny>>, value: &Bound<'_, PyAny>, parent_transition: &Bound<'_, PyAny>) -> PyResult<usize> {

        let index: usize = self._children.len();

        // If a parent was given, add the new node to the children list of the parent
        if let Some(p) = parent {

            if let (Some(ch), Some(tr)) = (self._children.get_mut(p), self._transitions.get_mut(p)) {
                ch.push(index);
                tr.push(parent_transition.clone().unbind());
            }
            else {
                return Err(PyIndexError::new_err("Parent should point to a valid node!"));
            }
        }

        self._children.push(conns);
        self._transitions.push(trans);
        self._parents.push(parent);
        self._values.push(Some(value.clone().unbind()));

        Ok(index+1)
    }

    /// Function to remove a node from the graph.
    fn _del_node(&mut self, i: usize) -> PyResult<()> {

        if let (Some(p), (ch_p, ch_c)) = (self._parents.get_mut(i), self._children.split_at_mut(i)) {

            if let Some(parent) = *p {

                // Setting the parent of the detached node to None
                *p = None;

                //println!("Parent is {parent} for node {i}, with {:?} (before i) and {:?} (from i to the end)", ch_p, ch_c);

                if let (Some(par_ch), Some(ch)) = (ch_p.get_mut(parent), ch_c.get_mut(0)) {

                    //println!("Erasing every {i} from {parent}'s children");
                    par_ch.retain(|value| *value != i);

                    //println!("Adding {:?} to parent's children", ch);
                    par_ch.extend(ch.to_owned());

                    for child in ch.iter_mut() {
                        //println!("Setting {child}'s parent to {parent}");
                        self._parents[*child] = Some(parent);
                    }
                    
                    // Erasing children references of deleted node
                    ch.clear();
                }
                else {
                    todo!()
                }
                
            }
            else {
                todo!()
            }

            Ok(())
        }
        else {
            Err(PyIndexError::new_err("Specified node doesn't exist!"))
        }

    }


    
}


/// A Python module implemented in Rust.
#[pymodule]
fn ramage(m: &Bound<'_, PyModule>) -> PyResult<()> {
    //m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<Tree>()?;
    Ok(())
}
