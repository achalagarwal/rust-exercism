use indexmap::IndexMap;
use std::collections::HashSet;
use uuid::Uuid;
extern crate nanoid;
use nanoid::nanoid;
use rand::prelude::*;


/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct InputCellID{
    uuid:u64,
}
/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct ComputeCellID{
    uuid:u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct CallbackID{
    uuid:u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<T: 'static> {
    // Just so that the compiler doesn't complain about an unused type parameter.
    // You probably want to delete this field.
    // dummy: ::std::marker::PhantomData<T>,
    input_cells: IndexMap<InputCellID, T>,
    // these two are not needed in hindsight
    input_to_cell: IndexMap<InputCellID, CellID>,
    compute_to_cell: IndexMap<ComputeCellID, CellID>,
    // compute_cells: IndexMap<ComputeCellID, T>,
    compute_cells: IndexMap<ComputeCellID, Box<dyn Fn(&[T]) -> T>>,
    compute_cache: IndexMap<ComputeCellID, T>,
    compute_callbacks: IndexMap<ComputeCellID, Vec<CallbackID>>,
    callbacks: IndexMap<CallbackID, Box<dyn FnMut(T) -> ()>>,
    dependencies: IndexMap<CellID, Vec<ComputeCellID>>,
    compute_dependencies: IndexMap<ComputeCellID, Vec<CellID>>,
    deleted_callbacks: HashSet<CallbackID>
    // reactor needs two types of cell hashmaps

    // the ce
    // let mut input_cells = IndexMap::new();
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<T: Copy + PartialEq> Reactor<T> {
    pub fn new() -> Self {
        Reactor {
            input_cells: IndexMap::new(),
            compute_cells: IndexMap::new(),
            compute_cache: IndexMap::new(),
            callbacks: IndexMap::new(),
            input_to_cell: IndexMap::new(),
            dependencies: IndexMap::new(),
            compute_dependencies: IndexMap::new(),
            deleted_callbacks: HashSet::new(),
            compute_callbacks: IndexMap::new(),
            compute_to_cell: IndexMap::new()
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, _initial: T) -> InputCellID {
        let input_cell_id = InputCellID{uuid:rand::random::<u64>()};
        self.input_cells.insert(input_cell_id, _initial);
        self.input_to_cell.insert(input_cell_id, CellID::Input(input_cell_id));
        input_cell_id

        // unimplemented!()
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<'a, F: Fn(&[T]) -> T + 'static>(
        &mut self,
        _dependencies: &[CellID],
        _compute_func: F,
    ) -> Result<ComputeCellID, CellID> {


        let compute_cell_id = ComputeCellID{uuid:rand::random::<u64>()};
        self.compute_to_cell.insert(compute_cell_id, CellID::Compute(compute_cell_id));
                // compute cell value
        // TODO add dirty bit
        let dep_vec_ = _dependencies.iter().fold(Ok(Vec::<T>::new()), |mut acc,x| match self.value(*x) {
            Some(val) => {
                match acc
                    {
                        Ok(mut acc_) => {acc_.push(val); return Ok(acc_)},
                        Err(e) => Err(e) 
                    }
        },
        None => Err(*x) 
        }
    )?;
        // let dep_vec = dep_vec_.collect::<Vec<T>>();
        let arr = dep_vec_.as_slice();
        // print!("{}",arr);
        let v = _compute_func(arr);
        self.compute_cache.insert(compute_cell_id, v);
        
        self.compute_cells
            .insert(compute_cell_id, Box::new(_compute_func));

        _dependencies.iter().for_each(|x| {
            if self.dependencies.contains_key(x) {
                let v = self.dependencies.get_mut(x);
                v.unwrap().push(compute_cell_id)
            } else {
                let mut v = Vec::new();
                v.push(compute_cell_id);
                self.dependencies.insert(*x, v);
            }
        });

        self.compute_dependencies
            .insert(compute_cell_id, _dependencies.to_vec());


        
        Ok(compute_cell_id)
    }

    fn reset(&mut self, id: &ComputeCellID) {



        let dep_vec = self.compute_dependencies.get(id).unwrap().iter().map(|x| self.value(*x).unwrap()).collect::<Vec<T>>();
        let arr = dep_vec.as_slice();

        let cfn = &*self.compute_cells.get(id).unwrap();
        let v = cfn(arr);
        self.compute_cache.insert(*id, v);

    }
    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(v) => {
                if self.input_cells.contains_key(&v) {
                    return Some(*(self.input_cells.get(&v).unwrap()));
                } else {
                    return None;
                }
            }

            CellID::Compute(v) => {
                if self.compute_cells.contains_key(&v) {
                    // recompute a compute cell
                    // call the function
                    // that computes the value of the compute cell from the
                    // dependencies
                    return Some(*self.compute_cache.get(&v).unwrap());
                } else {
                    return None;
                }
            }
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, _id: InputCellID, _new_value: T) -> bool {
        
        let mut vec_v = Vec::<T>::new();
        let mut vec_cc = Vec::<ComputeCellID>::new();
        if self.input_cells.contains_key(&_id) {
            
            self.input_cells.remove(&_id);
            self.input_cells.insert(_id, _new_value);
        
        
            if self.dependencies.contains_key(self.input_to_cell.get(&_id).unwrap()){
            self.dependencies.get(self.input_to_cell.get(&_id).unwrap()).unwrap().iter().for_each(|id| {

            let dep_vec = self.compute_dependencies.get(id).unwrap().iter().map(|x| self.value(*x).unwrap()).collect::<Vec<T>>();
            let arr = dep_vec.as_slice();

            let cfn = &*self.compute_cells.get(id).unwrap();
            let v = cfn(arr);
            vec_cc.push(*id);
            vec_v.push(v);
            // self.compute_cache.insert(*id, v);

            }
        );
    }
        
        for (i, cc) in vec_cc.iter().enumerate(){

            let cvv = *vec_v.get(i).unwrap();
            self.compute_cache.insert(*cc, cvv);
            self.set_compute_value(*cc, cvv );
        }
        return true;

    }

            
            
    
         else {
            return false;
        }
    // }
}

    // pub fn helper(&self, mut imap: IndexMap<InputCellID, T>, _id:InputCellID, _new_value:T, mut ccache: IndexMap<ComputeCellID,T>){
    //     if imap.contains_key(&_id) 
    //         {
    //             imap.remove(&_id);
    //             imap.insert(_id, _new_value);
    //             self.dependencies.get(&CellID::Input(_id)).unwrap().iter().for_each(|id| {

    //                 let dep_vec = self.compute_dependencies.get(id).unwrap().iter().map(|x| self.value(*x).unwrap()).collect::<Vec<T>>();
    //                 let arr = dep_vec.as_slice();
        
    //                 let cfn = &*self.compute_cells.get(id).unwrap();
    //                 let v = cfn(arr);
    //                 self.compute_cache.insert(*id, v);
    //         });
    //     }
    // }
    

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.

    pub fn set_compute_value(&mut self, _id:ComputeCellID, _new_value:T){

        let mut vec_v = Vec::<T>::new();
        let mut vec_cc = Vec::<ComputeCellID>::new();

        // self.input_cells.remove(&_id);
            // self.input_cells.insert(_id, _new_value);
        
        
            if self.dependencies.contains_key(self.compute_to_cell.get(&_id).unwrap()){
            self.dependencies.get(self.compute_to_cell.get(&_id).unwrap()).unwrap().iter().for_each(|id| {

            let dep_vec = self.compute_dependencies.get(id).unwrap().iter().map(|x| self.value(*x).unwrap()).collect::<Vec<T>>();
            let arr = dep_vec.as_slice();

            let cfn = &*self.compute_cells.get(id).unwrap();
            let v = cfn(arr);
            vec_cc.push(*id);
            vec_v.push(v);
            // self.compute_cache.insert(*id, v);

            }
        );
    }
        
        for (i, cc) in vec_cc.iter().enumerate(){

            let cvv = *vec_v.get(i).unwrap();
            self.compute_cache.insert(*cc, cvv);
            self.set_compute_value(*cc, cvv );
        }
        // return true;

        return;
    }
    pub fn add_callback<F: FnMut(T) -> () + 'static>(
        &mut self,
        _id: ComputeCellID,
        _callback: F,
    ) -> Option<CallbackID> {

        let cbd = CallbackID{uuid:rand::random::<u64>()};
        self.callbacks.insert(cbd, Box::new(_callback));

        if self.compute_callbacks.contains_key(&_id){
            self.compute_callbacks.get_mut(&_id).unwrap().push(cbd);
        }

        Some(cbd)

        // self.callbacks.
        // self.callbacks.÷

        // unimplemented!()
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {

        if self.compute_callbacks.contains_key(&cell){
            let vec = self.compute_callbacks.get_mut(&cell).unwrap();
            let old_len = vec.len();
            vec.retain(|&x| x != callback);
            if vec.len() == old_len{
                return Err(RemoveCallbackError::NonexistentCallback);
            }
            else{
                return Ok(());
            }
        }
        else{
            return Err(RemoveCallbackError::NonexistentCell)
        }

        // self.compute_callbacks.

        // unimplemented!(
        //     "Remove the callback identified by the CallbackID {:?} from the cell {:?}",
        //     callback,
        //     cell,
        // )
    }
}
