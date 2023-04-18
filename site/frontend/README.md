# `rustc-perf` site frontend
This directory contains code for the `rustc-perf` website frontend.

## Build instructions
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

## Build system
The frontend is built using the `Parcel` bundler. It generates several JavaScript files (one for each
page in the perf.RLO website) into the `dist` directory.

It uses a custom backend for transpiling `TypeScript`, which transpiles for browsers defined in
the `browserslist` attribute in `package.json`. It does not perform type-checking, that is why
there is a dedicated `npm run check` action that uses `tsc`. It is performed on CI to find typ errors.
The `tsconfig.json` file is only used by the type-checking action.
