extern crate cpython;

use cpython::{Python, PyResult, PyType, PythonObject, ToPyObject, PythonObjectWithCheckedDowncast,
    PythonObjectWithTypeObject, py_class::PythonObjectFromPyClassMacro, py_module_initializer, py_fn, py_class};

py_module_initializer!{librarytest, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "get_result", py_fn!(py, get_result(val: &str)))?;
    m.add(py, "hellopy", py_fn!(py, hellopy()))?;
    m.add_class::<Greeter>(py)?;
    m.add_class::<Storage>(py)?;
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
    data num: i32;

    def __new__(_cls, arg: i32) -> PyResult<Storage> {
        return Storage::create_instance(py, arg);
    }

    def half(&self) -> PyResult<i32> {
        //println!("half() was called with self={:?}", self.num(py));
        return Ok(self.num(py) / 2);
    }
}}

struct MyType {
    pub number: i32
}

impl MyType {
    fn create_instance(py: Python, number: i32) -> PyResult<MyType> {
        //return Ok(MyType.as_objec);
        //return Ok(PythonObject::into_object(self));
        //return Ok(MyType {number});

        /*
        let obj = unsafe {
            <MyType as py_class::BaseObject>::alloc(
                py, &py.get_type::<MyType>(), ( number )
            )
        }?;
        return Ok(MyType { _unsafe_inner: obj });
        */
        return Ok(MyType{number});

    }

    // data accessors
    fn number<'a>(&'a self, py: Python<'a>) -> &'a i32 {
        return &self.number;
    }

    // functions callable from python
    pub fn __new__(_cls: &PyType, py: Python, arg: i32) -> PyResult<MyType> {
        MyType::create_instance(py, arg)
    }

    pub fn half(&self, py: Python) -> PyResult<i32> {
        println!("half() was called with self={:?}", self.number(py));
        Ok(self.number(py) / 2)
    }
}