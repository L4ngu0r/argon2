# argon2-wasm

## Generate WASM binary capable of generating argon2 hash

### Prepare

This project use a [build script](./build.rs) to inject an environment variable.

You need to create a `.env` file to specify your **SALT** ! The build script will inject it at compile time to place it in WASM binary.

#### Ex

```
ARGON_SALT=yoursupersaltsentence
```

### Build

```bash
wasm-pack build --target web
```

Using your prefered http server, server `pkg` folder!

#### index.html example

```html
<script type="module">
    import init, { hash_pass } from './argon2_wasm.js'

    async function run() {
        await init()

        const hash = hash_pass('testpassword')
        console.log(hash)
    }

    run()
</script>
```
