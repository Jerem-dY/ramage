use crate::tree::*;

use pyo3::prelude::*;
use pyo3::types::PyList;

#[pyclass(extends=Tree)]
pub struct Trie {}


#[pymethods]
impl Trie {

    #[new]
    fn new<'py>(py: Python<'py>) -> (Self, Tree) {

        let mut tr = Tree::new();
        tr._add_node(None, vec![], vec![], None::<Py<PyAny>>.to_object(py).bind(py), None).unwrap();
        (Trie {  }, tr)
    }

    /*def __setitem__(self, key: Iterable, value):
        i = 0

        for el in key:

            if el in self._transitions[i]:
                i = self._children[i][self._transitions[i].index(el)]
            else:
                self._add_node(i, [], [], None, el)
                i = len(self._children)-1
        
        self._values[i] = value */
    fn __setitem__<'py>(mut self_: PyRefMut<'_, Self>, py: Python<'py>, key: &Bound<'_, PyList>, value: &Bound<'_, PyAny>) -> PyResult<()> {

        let mut i = 0usize;
        let super_ = self_.as_mut();

        for el in key {

            if let Ok(b) = &super_._transitions[i].to_object(py).bind(py).contains(&el) {

                if *b {
                    i = super_._children[i][super_._transitions[i].iter().position(|r| if let Some(f) = r {if let Ok(b) = f.bind(py).eq(&el) {b} else {false}} else {false}).unwrap()]
                }
                else {
                    super_._add_node(Some(i), vec![], vec![], None::<Py<PyAny>>.to_object(py).bind(py), Some(&el)).unwrap();
                    i = super_._children.len()-1;
                }

            }
        }

        super_._values[i] = Some(value.as_unbound().to_owned());

        Ok(())
    }
}