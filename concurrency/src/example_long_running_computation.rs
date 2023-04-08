pub fn run() {
    let result = async_std::task::block_on(verify_password("pwd", "fdfd", "key"));

    match result {
        Ok(response) => println!("{}", response),
        Err(err) => eprintln!("Error: {}", err),
    }
}

async fn verify_password(
    password: &str,
    hash: &str,
    key: &str,
) -> Result<bool, argonautica::Error> {
    //Make copies of the arguments, so the closure can be 'static
    let password = password.to_string();
    let hash = hash.to_string();
    let key = key.to_string();

    //spawn_blocking takes a closure, starts it running on its own thread and returns a future of its return value.
    //since we are awaiting it, we yield its thread to other tasks until the computation is ready
    async_std::task::spawn_blocking(move || {
        argonautica::Verifier::default()
            .with_hash(&hash)
            .with_password(&password)
            .with_secret_key(&key)
            .verify()
    })
    .await
}
