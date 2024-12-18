# OG Image Generator

OG Image Generator is a service designed to create [Open Graph](https://ogp.me/) images for your static pages. This project is inspired by [og-image-generator](https://github.com/sagarhani/og-image-generator). You can read more about the technical details in this [article](https://vnotes.pages.dev/og-image-generator/).

## Usage

To use this service, add the following meta tag to the `<head>` section of your HTML page template and replace the tokens with your desired values:

```html
<meta
  property="og:image"
  content="https://ogp.fly.dev/v0/png?title={title}&author={author}&photo={photo}&url={url}&theme={theme}"
/>
```

### Example

Here is an example of how to use the service in a Zola-based blog:

- [Link generation](https://github.com/vladkens/blog/blob/ae18520/templates/base.html#L21)
- [Meta tags](https://github.com/vladkens/blog/blob/ae18520/templates/base.html#L44)

## Self-hosted Version

To run a self-hosted version of the OG Image Generator, use the following Docker command:

```sh
docker run -d -p 8080:8080 -e PUBLIC_URL="https://example.com" --name ogp ghcr.io/vladkens/ogp:dev
```

## Contributing

If you need additional templates or have any suggestions, feel free to open an issue or submit a pull request. Contributions are welcome!
