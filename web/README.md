# Build for the web

In order to build the `three-d` examples for the web, you can follow these steps. All commands should run in this `web/` directory.

1. Make sure you have both `Rust` and `npm` (which should include `npx`) installed.

2. Run

```console
$ npm install
```

3. Run the following command in the root folder

```console
$ npx wasm-pack build "." --target web --out-name web --out-dir web/pkg
```

4. Run

```console
$ npm run serve
```

5. Open `http://localhost:8080` in a browser
