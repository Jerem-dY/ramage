use std::collections::VecDeque;
use std::iter::zip;

use pyo3::prelude::*;
use pyo3::exceptions::*;

#[pyclass]
enum Property {
    Children = 0,
    Transitions = 1,
    Parents = 2,
    Values = 3
}

#[pyclass]
enum Search {
    Depth = 0,
    Breadth = 1
}

#[pyclass]
struct Tree {

    #[pyo3(get, name = "children")]
    _children: Vec<Vec<usize>>, 

    #[pyo3(get, name = "transitions")]
    _transitions: Vec<Vec<Option<Py<PyAny>>>>,

    #[pyo3(get, name = "parents")]
    _parents: Vec<Option<usize>>,

    #[pyo3(get, name = "values")]
    _values: Vec<Option<Py<PyAny>>>,

    _size: usize
}

#[pymethods]
impl Tree {

    #[new]
    fn new() -> Self {
        Tree{
            _children: Vec::<Vec<usize>>::new(),
            _transitions: Vec::<Vec<Option<Py<PyAny>>>>::new(),
            _parents: Vec::<Option<usize>>::new(),
            _values: Vec::<Option<Py<PyAny>>>::new(),
            _size: 0
        }
    }

    fn __len__(&self) -> PyResult<usize> {
        Ok(self._size)
    }

    #[pyo3(signature=(parent, conns, trans, value, parent_transition))]
    ///  Function to add a node to the tree.
    fn _add_node(&mut self, parent: Option<usize>, conns: Vec<usize>, trans: Vec<Option<Py<PyAny>>>, value: &Bound<'_, PyAny>, parent_transition: Option<&Bound<'_, PyAny>>) -> PyResult<usize> {

        let index: usize = self._children.len();

        // If a parent was given, add the new node to the children list of the parent
        if let Some(p) = parent {

            if let (Some(ch), Some(tr)) = (self._children.get_mut(p), self._transitions.get_mut(p)) {
                ch.push(index);

                if let Some(p_tr) = parent_transition {
                    tr.push(Some(p_tr.to_owned().unbind()));
                }
                
            }
            else {
                return Err(PyIndexError::new_err("Parent should point to a valid node!"));
            }
        }

        self._children.push(conns);
        self._transitions.push(trans);
        self._parents.push(parent);
        self._values.push(Some(value.clone().unbind()));

        self._size += 1;

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

            self._size -= 1;
            Ok(())
        }
        else {
            Err(PyIndexError::new_err("Specified node doesn't exist!"))
        }

    }

    /// Search value using a depth first algorithm.
    fn search<'py>(&self, py: Python<'py>, item: &Bound<'_, PyAny>, all: bool, property: &Property, method: &Search) -> PyResult<Option<PyObject>> {

        let mut stack = VecDeque::from([0usize]);
        let mut indices = Vec::<usize>::new();

        while let Some(i) = match method {
            Search::Depth => stack.pop_back(),
            Search::Breadth => stack.pop_front()
            
        } {

            //println!("{i}");

            let prop = match property{
                Property::Children => self._children[i].clone().iter().map(|x| x.to_object(py)).collect(),
                Property::Transitions => self._transitions[i].to_vec().iter().map(|x| {
                    if let Some(val) = x {
                        val.to_object(py)
                    }
                    else {
                        None::<Py<PyAny>>.to_object(py)
                    }
                }).collect(),
                Property::Parents => {
                    if let Some(val) = self._parents[i].to_owned() {
                        vec![val.to_object(py)]
                    } 
                    else {
                        vec![None::<Py<PyAny>>.to_object(py)]
                    }
                },
                Property::Values => {
                    
                    if let Some(val) = &self._values[i] {
                        vec![val.to_object(py)]
                    } 
                    else {
                        vec![None::<Py<PyAny>>.to_object(py)]
                    }
                },                            
            };
            
            let prop = prop.iter().map(|x| x.bind(py)).collect::<Vec<&Bound<PyAny>>>();

            //println!("{:?}", prop);

                // Si l'objet à trouver est une liste, on compare avec tout
            if {

                let mut result = false;

                if let Ok(list) = item.extract::<Vec<Py<PyAny>>>() {
    
                    for (a, b) in zip(list, prop.iter()) {
                        if let Ok(b) = a.bind(py).eq(b) {
                            //println!("Comparing {:?} to {:?}: {b}", a.bind(py), b);
                            result = b;
                        }
                        else {
                            todo!()
                        }
                    }
                }
                // Si l'on compare un seul élément 
                else if let Ok(obj) = item.extract::<Py<PyAny>>() {
    
                    let obj = obj.bind(py);
    
                    for el in prop.iter() {
                        if let Ok(b) = el.eq(obj) {
                            //println!("Comparing {:?} to {:?}: {b}", el, obj);
                            result = b;
                        }
                        else {
                            todo!()
                        }
                    }
                }
                else {
                    todo!()
                }

                result

            } == true
            {
                if all {
                    indices.push(i);
                }
                else {
                    return Ok(Some(i.to_object(py)));
                }
            }

            
            stack.extend(self._children[i].to_vec());
            

        }

        if all && indices.len() > 0 {
            Ok(Some(indices.to_object(py)))
        }
        else {
            Ok(None)
        }
        
    }

    
}


/// A Python module implemented in Rust.
#[pymodule]
fn ramage(m: &Bound<'_, PyModule>) -> PyResult<()> {
    //m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<Tree>()?;
    m.add_class::<Property>()?;
    m.add_class::<Search>()?;
    Ok(())
}
