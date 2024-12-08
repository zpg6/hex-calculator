# hex-calculator

A simple calculator for converting hexadecimal to decimal numbers, and vice versa. This project is hosted on Cloudflare Workers, and is written in Rust and client-side JavaScript.

There are probably thousands of calculators like this on the internet, this is just a toy implementation because I've always wanted to make my own.
<br><br>

Access the calculator here:
https://hex-calculator.zpg6.workers.dev/

![Page Screenshot](./docs/page_screenshot.png)

## Tools Used

- 🔸 [Cloudflare Workers](https://workers.cloudflare.com/) for serverless hosting.
- ⛅️ [Wrangler CLI](https://developers.cloudflare.com/workers/wrangler/install-and-update/) for deploying to Cloudflare Workers.
- 🦀 [Workers Rust Bindings](https://github.com/cloudflare/workers-rs) for receiving requests.
- 📄 [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark/) to power our pre-processing of Markdown to HTML.
- ⚙️ [Wasm target for Rust](https://developers.cloudflare.com/workers/languages/rust/) to compile the Rust code to WebAssembly.
- 🎨 [Simple.css](https://simplecss.org/) for easy styling of HTML coming from Markdown.

## Commands

Prepare the project:

```
npm i
```

Run a local development server:

```
npx wrangler dev
```

Publish the website to Cloudflare Workers:

```
npx wrangler deploy
```

## Changelog

| Date       | Version | Description                                   |
| ---------- | ------- | --------------------------------------------- |
| 2024-12-08 | 1.0.0   | Initial release of MVP to Cloudflare Workers. |
