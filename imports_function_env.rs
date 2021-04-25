//! A Wasm module can import entities, like functions, memories,
//! globals and tables.
//!
//! In this example, we'll create a system for getting and adjusting a counter value. However, host
//! functions are not limited to storing data outside of Wasm, they're normal host functions and
//! can do anything that the host can do.
//!
//!   1. There will be a `get_counter` function that will return an i32 of
//!      the current global counter,
//!   2. There will be an `add_to_counter` function will add the passed
//!      i32 value to the counter, and return an i32 of the current
//!      global counter.
//!
//! You can run the example directly by executing in Wasmer root:
//!
//! ```shell
//! cargo run --example imported-function-env --release --features "cranelift"
//! ```
//!
//! Ready?

use std::sync::{Arc, Mutex};
use wasmer::{imports, wat2wasm, Function, Instance, Module, Store, WasmerEnv};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_jit::JIT;

use wasmer::Memory;
use wasmer::MemoryType;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Let's declare the Wasm module with the text representation.
    let wasm_bytes = wat2wasm(
        br#"
(module
  (memory $memory_name (import "env" "memory_name") 1)

  (type $mem_size_t (func (result i32)))

  (func $mem_size (type $mem_size_t) (result i32)
    (memory.size))

  (export "mem_size" (func $mem_size))
  (export "memory" (memory $memory_name)))
"#,
    )?;

    // Create a Store.
    // Note that we don't need to specify the engine/compiler if we want to use
    // the default provided by Wasmer.
    // You can use `Store::default()` for that.
    let store = Store::new(&JIT::new(Cranelift::default()).engine());

    println!("Compiling module...");
    // Let's compile the Wasm module.
    let module = Module::new(&store, wasm_bytes)?;

    
    let host_mem = Memory::new(&Store::default(),MemoryType::new(1, Some(1), false)).unwrap();
    let host_ptr = host_mem.data_ptr();
    
    unsafe{
        println!("write 12 in the first two entries\n");
        std::ptr::write_bytes(host_ptr, 12, 2);
	    //*host_ptr.add(1) = 7;
        println!("\nhost_ptr = {:?}", *host_ptr);
        println!("host_ptr = {:?}", *host_ptr.add(1));
        println!("host_ptr = {:?}", *host_ptr.add(2));
	    println!("host_ptr = {:?}", *host_ptr.add(3));
        //std::ptr::write(host_ptr, );
    }
    //unsafe{
        //let f = host_mem.data_unchecked();
	//let mut buffer = [0; 10];
        //f.read(&mut buffer)?;
	//println!("host memory data_unchecked: {:?}", buffer);


    // Create an import object.
    let import_object = imports! {
        "env" => {
            "memory_name" => host_mem,
        }
    };

    println!("Instantiating module...");
    // Let's instantiate the Wasm module.
    let instance = Instance::new(&module, &import_object)?;
    let instance2 = Instance::new(&module, &import_object)?;


    
    let mem_size = instance.exports.get_function("mem_size")?.native::<(), i32>()?;
    

    let result = mem_size.call()?;
    println!("Memory size: {:?}", result);

    let memory = instance.exports.get_memory("memory")?;
    let ptr = memory.data_ptr();
    unsafe{
	    println!("\nptr = {:?}", *ptr);
        println!("ptr = {:?}", *ptr.add(1));
        println!("ptr = {:?}", *ptr.add(2));
	    println!("ptr = {:?}", *ptr.add(3));
        //println!("Memory data_unchecked: {:?}", memory.data_unchecked());
    }
    
    
    unsafe{
        println!("write 11 in the first entry of ptr\n");
        std::ptr::write(ptr, 11);
    }
    
    unsafe{
	    println!("\nptr = {:?}", ptr);
	    println!("ptr = {:?}", *ptr);
        println!("ptr = {:?}", *ptr.add(1));
        println!("ptr = {:?}", *ptr.add(2));
	    println!("ptr = {:?}", *ptr.add(3));
        //println!("Memory data_unchecked: {:?}", memory.data_unchecked());
        println!("\nhost_ptr = {:?}", *host_ptr);
        println!("host_ptr = {:?}", *host_ptr.add(1));
        println!("host_ptr = {:?}", *host_ptr.add(2));
	    println!("host_ptr = {:?}", *host_ptr.add(3));
    }

    let memory2 = instance2.exports.get_memory("memory")?;
    let ptr2 = memory2.data_ptr();
    
    unsafe{
        println!("write 31 in the fourth entry of ptr\n");
        std::ptr::write(ptr2.add(3), 31);
    }
    
    unsafe{
	    println!("\nptr = {:?}", ptr);
	    println!("ptr = {:?}", *ptr);
        println!("ptr = {:?}", *ptr.add(1));
        println!("ptr = {:?}", *ptr.add(2));
	    println!("ptr = {:?}", *ptr.add(3));
        //println!("Memory data_unchecked: {:?}", memory.data_unchecked());
        println!("\nhost_ptr = {:?}", *host_ptr);
        println!("host_ptr = {:?}", *host_ptr.add(1));
        println!("host_ptr = {:?}", *host_ptr.add(2));
	    println!("host_ptr = {:?}", *host_ptr.add(3));
    }



    Ok(())
}

#[test]
fn test_imported_memory_env() -> Result<(), Box<dyn std::error::Error>> {
    main()
}
