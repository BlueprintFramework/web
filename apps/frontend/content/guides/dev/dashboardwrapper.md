---
title: Extending the user dashboard wrapper
description: Extend the Pterodactyl user dashboard using the Laravel blade wrapper
author: Emma
category: dev
thumbnail: dashboard.jpeg
---

::card
The dashboard wrapper should not be your first choice for dashboard extensibility. It's quite limited and can cause weird issues with the React DOM. Use [Components.yml](/docs/configs/componentsyml) instead if available for your extension's use case.
::

## Introduction

Blueprint's `dashboard.wrapper` [conf.yml](/docs/configs/confyml#dashboardwrapper) bind extends the user-side dashboard a `.blade.php` view. Dashboard wrappers can be used for quite a lot of purposes. For this guide, we'll be adding a (useless) "barrel roll" button to the Pterodactyl user-side dashboard.

## Create a basic dashboard wrapper

Create a file named `wrapper.blade.php` and bind it to `dashboard.wrapper` in your extension's [conf.yml](/docs/configs/confyml).

```yaml [conf.yml]
dashboard:
  # bind dashboard.wrapper to the wrapper.blade.php
  # file you created:
  wrapper: 'wrapper.blade.php'
```

In your `dashboard.wrapper`, add a simple button and run `blueprint -build` to apply your changes.

<!-- prettier-ignore -->
```html [wrapper.blade.php]
<button style="background: black;">
  Example button
</button>
```

Visit your Pterodactyl instance in your browser, it should look similar to this:

![Image showcasing the Pterodactyl panel, with an example button reading "Example button" in the bottom left.](/img/guides/examplebutton.png)

## Doing a barrel roll

Alright, now that we've got ourselves a button, let's replace "Example button" with something else and give it an identifier.

::card
We're prefixing the buttons `id=` with the `{identifier}` [placeholder](/docs/concepts/placeholders#identifier). This prevents conflicts with other extensions when two extensions assign the same identifier for different elements.

Say, for example, both extensions "foo" and "bar" have an element with the identifier `fizz`. Extension "foo" wants to modify it's own `fizz` element, and does so via JavaScript, there's a non-zero chance that extension "bar"s element gets modified.

_tldr; don't give your element IDs generic names_
::

<!-- prettier-ignore -->
```html [wrapper.blade.php]
<button id="{identifier}-barrel" style="background: black;">
  Do a barrel roll!
</button>
```

### Running a function on button press

We can now create ourselves a JavaScript function, then make `{identifier}-barrel` run it whenever it's pressed.

::card
The same conflict potential as mentioned for the `id=` can happen here too. Adding placeholders for function names, however, may break syntax highlighting. Either bite the bullet on syntax highlighting, or name your function something unique.
::

<!-- prettier-ignore -->
```html [wrapper.blade.php]
<script>
function doBarrelRoll() {
  // something happened?!
  console.log("something happened!!")
}
</script>
```

<!-- prettier-ignore -->
```diff [wrapper.blade.php]
- <button id="{identifier}-barrel" style="background: black;">
+ <button id="{identifier}-barrel" onclick="doBarrelRoll()" style="background: black;">
```

### Adding an animation

We'll use a CSS animation to animate rotating the page. Just like elements and scripts, you can also add stylesheets to your `dashboard.wrapper`.

::card
Stylesheets added inside of `<style />` tags in `dashboard.wrapper` are slightly different to the `dashboard.css` stylesheet. The `dashboard.css` stylesheet is bundled with the React application, the `dashboard.wrapper` is not.

For styling elements made in `dashboard.wrapper`, you should use `dashboard.wrapper`. For styling elements from the React application, you should use `dashboard.css`.
::

<!-- prettier-ignore -->
```html [wrapper.blade.php]
<style>
@keyframes barrelRoll {
  0% { transform: rotate(0deg); }
  50% { transform: rotate(360deg); }
  100% { transform: rotate(0deg); }
}

.barrel-roll {
  animation: barrelRoll 1s ease-in-out;
}
</style>
```

This CSS animation will rotate any element with the `barrel-roll` class. We'll use JavaScript to apply this class to the `body` element when `{identifier}-barrel` is pressed.

### Applying the animation class

Update the JavaScript function to apply the `barrel-roll` class to the `body` element when the `doBarrelRoll()` function is called.

<!-- prettier-ignore -->
```html [wrapper.blade.php]
<script>
function doBarrelRoll() {
  const body = document.body;
  body.classList.add('barrel-roll');

  setTimeout(() => {
    body.classList.remove('barrel-roll');
  }, 1000);
}
</script>
```

## Final results

Apply your changes using `blueprint -build` and visit your panel in your browser. The final result should look somewhat like this:

:prose-video-player{src='/img/guides/barrelroll.mp4'}
