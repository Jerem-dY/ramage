use pyo3::prelude::*;
use pyo3::exceptions::*;
use pyo3::types::PyList;
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
}

/// A Python module implemented in Rust.
#[pymodule]
fn ramage(m: &Bound<'_, PyModule>) -> PyResult<()> {
    //m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<Tree>()?;
    Ok(())
}
