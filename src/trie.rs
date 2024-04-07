use crate::tree::*;

use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::exceptions::*;

#[pyclass(extends=Tree)]
pub struct Trie {}


impl Trie {
    fn __fetch<'py>(self_: PyRef<'_, Self>, py: Python<'py>, key: &Bound<'_, PyList>) -> (PyResult<Vec<Option<Py<PyAny>>>>, bool) {
        
        let mut i = 0usize;
        let super_ = self_.as_ref();

        let mut values = Vec::<Option<Py<PyAny>>>::new();

        for el in key {

            if let Ok(b) = &super_._transitions[i].to_object(py).bind(py).contains(&el) {

                if *b {
                    i = super_._children[i][super_._transitions[i].iter().position(|r| if let Some(f) = r {if let Ok(b) = f.bind(py).eq(&el) {b} else {false}} else {false}).unwrap()];
                    values.push(super_._values[i].to_owned());
                }
                else {
                    return (Err(PyKeyError::new_err("Key is not present in the tree!")), false);
                }

            }
        }

        (Ok(values), true)
    }
}

#[pymethods]
impl Trie {

    #[new]
    fn new<'py>(py: Python<'py>) -> (Self, Tree) {
        (Trie {  }, Tree::new(py))
    }


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


    

    fn __getitem__<'py>(self_: PyRef<'_, Self>, py: Python<'py>, key: &Bound<'_, PyList>) -> PyResult<Option<Py<PyAny>>> {


        let res = Trie::__fetch(self_, py, key).0;

        if let Ok(vec) = res {
            Ok(vec.last().unwrap().to_owned())
        }
        else {
            Err(res.err().unwrap())
        }
        
    }

    fn get_values<'py>(self_: PyRef<'_, Self>, py: Python<'py>, key: &Bound<'_, PyList>) -> PyResult<Vec<Option<Py<PyAny>>>> {


        let res = Trie::__fetch(self_, py, key).0;

        if let Ok(vec) = res {
            Ok(vec.to_owned())
        }
        else {
            Err(res.err().unwrap())
        }
        
    }

    fn __contains__<'py>(self_: PyRef<'_, Self>, py: Python<'py>, key: &Bound<'_, PyList>) -> PyResult<bool> {
        Ok(Trie::__fetch(self_, py, key).1)
    }
}