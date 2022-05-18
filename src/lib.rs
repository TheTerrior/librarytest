use cpython::{Python, PyResult, PyList, py_module_initializer, py_fn, py_class};
use std::cell::Cell;
use std::cell::RefCell;

py_module_initializer!{librarytest, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "get_result", py_fn!(py, get_result(val: &str)))?;
    m.add(py, "hellopy", py_fn!(py, hellopy()))?;
    m.add_class::<Greeter>(py)?;
    m.add_class::<Storage>(py)?;
    //m.add_class::<NeuralNet>(py)?;
    return Ok(());
}}

fn get_result(_py: Python, val: &str) -> PyResult<String> {
    return Ok("Rust says: ".to_owned() + val);
}

fn hellopy(_py: Python) -> PyResult<i32> {
    return Ok(88);
}

py_class!{pub class Greeter |_py| {
    def __new__(_cls) -> PyResult<Greeter> {
        return Greeter::create_instance(_py);
    }

    def greet(&self, name: String) -> PyResult<String> {
        return Ok(format!("Hello, {}!", name));
    }
}}

py_class!{pub class Storage |py| {
    data _num: Cell<i32>;
    data _num1: Cell<i32>;
    data _obj: Cell<Option<Storage>>;
    data _obj1: RefCell<Option<Storage>>;

    //constructor
    def __new__(_cls, arg: i32) -> PyResult<Storage> {
        return Storage::create_instance(py, Cell::new(100), Cell::new(arg), Cell::new(None), RefCell::new(None));
    }

    //_num
    @property def num(&self) -> PyResult<i32> {
        return Ok(self._num(py).get());
    }
    @num.setter def set_num(&self, value: Option<i32>) -> PyResult<()> {
        self._num(py).set(value.unwrap_or(0));
        return Ok(());
    }

    //_num1
    @property def num1(&self) -> PyResult<i32> {
        return Ok(self._num1(py).get());
    }
    @num1.setter def set_num1(&self, value: Option<i32>) -> PyResult<()> {
        self._num1(py).set(value.unwrap_or(0));
        return Ok(());
    }

    def half(&self) -> PyResult<i32> {
        //println!("half() was called with self={:?}", self.num(py));
        return Ok(self._num(py).get() / 2);
    }

    def sum_nums(&self, second: Storage) -> PyResult<i32> {
        return Ok(self._num(py).get() + second._num(py).get());
    }

    //def store(&self, second: Option<Storage>) -> PyResult<()> {
    //    self._obj1(py).replace(second);
    //    return Ok(());
    //}
}}

/*
py_class!{pub class NeuralNet |py| {
    data shape: [u32; 4]; //input size, 2 hidden layer sizes, output layer size
    data nn: NeuralNetwork;

    def __new__(_cls, shape: Option<PyList>) -> PyResult<NeuralNet> {
        match shape {
            Some(ushape) => { //extract shape from python list
                let ashape: [u32; 4] = [
                    ushape.get_item(py, 0).extract(py).unwrap(), 
                    ushape.get_item(py, 1).extract(py).unwrap(), 
                    ushape.get_item(py, 2).extract(py).unwrap(), 
                    ushape.get_item(py, 3).extract(py).unwrap()
                ];
                return NeuralNet::create_instance(py, ashape, 
                        NeuralNetwork {shape: Vec::with_capacity(0), weights: Vec::with_capacity(0), biases: Vec::with_capacity(0)});
                
            },
            None => {
                return NeuralNetwork::new_neuralnetwork(shape);
            }
        }
    }

    def greet(&self, name: String) -> PyResult<String> {
        return Ok(format!("Hello, {}!", name));
    }
}}
*/

py_class!(pub class AI |py| {
    data nn: NeuralNetwork;

    def __new__(_cls, shape: Option<PyList>) -> PyResult<AI> {
        match shape {
            Some(ushape) => {
                let mut ashape: Vec<u32> = Vec::new();
                for i in 0..ushape.len(py) {
                    ashape[i] = ushape.get_item(py, i).extract(py).unwrap();
                }
                return AI::create_instance(py, NeuralNetwork::new(Some(ashape)));
            },
            None => {
                return AI::create_instance(py, NeuralNetwork::new(None));
            }
        }
    }
});

#[derive(Debug)]
pub struct Matrix {
    matrix: Vec<Vec<f64>>
}

impl Matrix {
    fn new(dimensions: (u32, u32)) -> Matrix {
        let mut ret: Matrix = Matrix {
            matrix: Vec::with_capacity(dimensions.0 as usize)
        };
        for i in 0..dimensions.0 {
            ret.matrix[i as usize] = Vec::with_capacity(dimensions.1 as usize);
        }
        return ret;
    }
}

#[derive(Debug)]
pub struct NeuralNetwork {
    shape: Vec<u32>, //input size, x hidden layer sizes, output layer size, must be at least 2 in size
    weights: Vec<Matrix>, //weights for all edges
    biases: Vec<Vec<f64>> //biases for all nodes
}

impl NeuralNetwork {
    fn new(shape: Option<Vec<u32>>) -> NeuralNetwork {
        match shape {
            None => {
                return NeuralNetwork {
                    shape: Vec::with_capacity(0), 
                    weights: Vec::with_capacity(0), 
                    biases: Vec::with_capacity(0)
                };
            },
            Some(ushape) => {
                let mut weights: Vec<Matrix> = Vec::with_capacity(ushape.len() - 1);
                let mut biases: Vec<Vec<f64>> = Vec::with_capacity(ushape.len() - 1);
                let mut layer: usize = 0;
                loop {
                    weights.push(Matrix::new((ushape[layer], ushape[layer + 1])));
                    biases.push(Vec::with_capacity(ushape[layer] as usize));
                    layer += 1;
                    if layer >= ushape.len() - 1 {
                        break;
                    }
                }
                let mut ret: NeuralNetwork = NeuralNetwork {
                    shape: ushape, 
                    weights,
                    biases
                };
                return ret;
            }
        }
    }

    // run a given input through the neural network and calculate the output
    fn calculate(&self, input: Vec<f64>) -> Option<Vec<f64>> {
        // filter out any invalid inputs
        if input.len() != self.shape[0] as usize {
            return None;
        }

        // the vectors that hold the previous layer's output and the newly calculated values
        let mut prev: Vec<f64> = input;
        //let mut hold: Vec<f64> = Vec::with_capacity(self.shape[1] as usize).fill_with(0);
        //let mut hold: Vec<f64> = vec![0.0; self.shape[1] as usize];
        let mut hold: Vec<f64> = Vec::new();

        // for every layer
        for i in 0..self.shape.len() - 1 {
            // for every output
            for k in 0..self.shape[i + 1] {
                hold = self.biases[i].clone(); //clone the biases for this layer
                // for every input
                for j in 0..self.shape[i] {
                    hold[k]
                }
            }
            
        }
        return Some(hold);
    }
}