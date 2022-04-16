fn main() {
    //phase 1
    let a = 2;
    let result = stack_only(a);
    println!("result {}", result);
    dbg!()
}

fn stack_only(b: i32) -> i32 {
    /*
    * Stack region:
    * is a special region of the process memory that stores variables created by each function
    * \forall function call, a new stack frame is allocated on top of the current one
    * The stack frame is only accessible by only the function whose call is associated with the frame
    * The size of \forall variable on the stack has to be known at compile time
    * When a function exits its stack frame is released
    */
    let c = 3;
    //return 3;
    return b + c + stack_and_heap();

}

fn stack_and_heap() -> i32 {
    /*
    * Heap region:
    * is a region of the process memory that is NOT automatically managed
    * has no size restrictions
    * is accessible by any function,  anywhere in the program.
    * heap allocations are expensive and we should avoid them when possible
    */
    let d = 5;
    /*
    * allocation can be difficult if the memory left on the heap is fragmented
    * stack_and_heap for e will only store the reference of e on the stack
    * allocation should always be done with proper deallocation: error-prone
    */ 
    let e = Box::new(7);
    /*
    * Box -> is a smart pointer
    * used in rust and modern cpp
    * wrapper around the raw pointer with additional capabilities
    * there are many types: example - those that make sure it's properly deallocated
    * on stack we store smart_pointer(reference of the pointer)
    */
    return d + *e;
}