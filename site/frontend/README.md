# `rustc-perf` site frontend
This directory contains code for the `rustc-perf` website frontend.

## Build
- Install dependencies
   ```console
   $ npm install
   ```
- Development build - dynamically reloads after each change.
    ```console
    $ npm run watch
    ```
- Production build - build optimized and minimized files.
    ```console
    $ npm run build
    ```

The files are packaged into the `dist` directory, from which they are embedded into the `rustc-perf`
binary.
