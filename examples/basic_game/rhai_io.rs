fn io() {
    // Channel: Script -> Master
    let (tx_script, rx_master) = std::sync::mpsc::channel();
    // Channel: Master -> Script
    let (tx_master, rx_script) = std::sync::mpsc::channel();

    // Spawn thread with Engine
    let thread = std::thread::spawn(move || {
        // Create Engine
        let mut engine = RhaiEngine::new();

        // Register API
        // Notice that the API functions are blocking
        engine
            .register_fn("get", move || match rx_script.recv() {
                Ok(o) => o,
                Err(_) => -1,
            })
            .register_fn("put", move |v: i64| match tx_script.send(v) {
                Ok(_) => (),
                Err(_) => (),
            });

        // Run script
        match engine.run_file("/home/alphasteam/Escritorio/Dev/Motor de videojuegos/alpha_engine/examples/basic_game/loop.rhai".into()) {
             Ok(_) => (),
             Err(err) => println!("Script error: {}",err),
         };
    });

    println!("Starting main loop...");

    let mut value = 0_i64;

    while value < 10 {
        println!("Value: {}", value);
        // Send value to script
        tx_master.send(value).unwrap();
        // Receive value from script
        value = rx_master.recv().unwrap();
    }
}
