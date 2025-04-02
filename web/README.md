trails-web

Web app to add an api and site to call `trails`.

## Usage

`cargo run` for backend. Locally this will read the compiled frontend code from `frontend/dist` so will not hot reload or pick up changes.

The frontend uses Vite, so to run the hot-reloading dev server:
```
cd frontend
npm i
npm run dev
```

or `npm run build` to create the prod output.