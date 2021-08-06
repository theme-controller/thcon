# thcon docs-site
Documentation web page for `thcon`, built with [VuePress](https://v2.vuepress.vuejs.org/).  The markdown files that turn into served HTML pages are located in [./docs](./docs).

## Prerequisites
* `node` 12+
* `yarn` v1

## Development
VuePress takes care of the heavy lifting here.  Simply install VuePress and its dependencies, then launch the development server.

```
yarn # install dependencies
yarn docs:dev
```

## Deployment
The generated site will automatically be deployed to [Vercel](https://vercel.com?utm_source=theme-controller&utm_campaign=oss) when a commit is pushed to `main`.
