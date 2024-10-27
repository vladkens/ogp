#### How to use?

Add the following meta tag to `<head>` section of your HTML page and replace highlighted tokens:

```sh
<meta property="og:image" content="http://localhost:8080/v0/png?title={title}&author={author}&photo={photo}&url={url}&theme={theme}" />
```

#### What are Open Graph?

Open Graph (OG) tags are HTML elements that control how web content appears in social media previews, helping to define elements like title, description, and image. These tags, embedded in the HTML `<head>`, enable consistent presentation across platforms and improve link engagement by creating appealing previews.

```html
<!-- Defines the title -->
<meta property="og:title" content="Page title here" />
<!-- Adds a brief summary -->
<meta property="og:description" content="Brief description here" />
<!-- Sets preview image -->
<meta property="og:image" content="Image URL here" />
<!-- Canonical link for the page -->
<meta property="og:url" content="Page URL here" />
```

This setup optimizes social media link displays, enhancing user interaction and engagement.
