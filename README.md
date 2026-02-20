# reussir-lang.github.io

Technical website for Reussir.

## Local development

```bash
npm install
npm run dev
```

## Production build

```bash
npm run build
npm run preview   # preview the built site locally
```

## GitHub Pages deployment

This repo uses `.github/workflows/pages.yml` to:

1. install dependencies with `npm ci`
2. build the site with `npm run build` (Astro)
3. upload `dist/` as Pages artifact
4. deploy via `actions/deploy-pages`

For organization root site hosting, the repository name must be:

`reussir-lang.github.io`
