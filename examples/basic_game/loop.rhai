print("Starting script loop...");

loop {
    // The following call blocks until there is data
    // in the channel
    let x = get();
    if(x == -1){
        break
    }
    print(`Script Read: ${x}`);

    x += 1;

    print(`Script Write: ${x}`);

    // The following call blocks until the data
    // is successfully sent to the channel
    put(x);
}