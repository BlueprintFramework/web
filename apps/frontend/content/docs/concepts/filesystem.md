---
title: Storing files
description: Handle and manage (user-uploaded) files
category: concepts
---

::card
This document is a simplified, Blueprint-oriented version of [Laravel's File Storage documentation](https://laravel.com/docs/10.x/filesystem). For a complete list of methods, check out their documentation.
::

## Filesystems

Each extension automatically gets assigned 2 filesystems, `{fs}` and `{fs/private}`. These serve two different purposes, so it's important that you use the right one.

| Disk           | Description                                                                                                            |
| -------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `{fs}`         | Files are stored in `storage/extensions/identifier/` and **can be visited by anyone** from `domain.ext/fs/identifier/` |
| `{fs/private}` | Files are stored in your extension's private directory and cannot be visited publicly                                  |

Keep in mind that `{fs}`'s files can be visited by **anyone that can visit your panel**, whether they are logged in or not.

## Storing files

Making files is simple, import the `Illuminate\Support\Facades\Storage` library and use the `put` method. In this example, we are creating a file called `foo.txt` with `bar` as it's content on the filesystem.

```php
use Illuminate\Support\Facades\Storage;

// Create public file 'foo.txt' with content 'bar'
Storage::disk('{fs}')->put('foo.txt', 'bar');

// Create private file 'bar.txt' with content 'foo'
Storage::disk('{fs/private}')->put('bar.txt', 'foo');
```

Writing a file to `{fs}` will make it public. This file should be accessible on `domain.ext/fs/identifier/foo.txt`.

#### Failed writes

If any write operation is unable to succeed (e.g. insufficient permissions), false is returned.

```php
// Create public file 'hello.txt' with content 'hi!!'
if (Storage::disk('{fs}')->put('hello.txt', 'hi!!')) {
  // Successfully created file..
} else {
  // Could not create file..
}
```

### Copying and moving files

Copying files may be done through the `copy` method. To move files, use the `move` method.

```php
// Copy 'foo.txt' to 'foo2.txt'
Storage::disk('{fs/private}')->copy('foo.txt', 'foo2.txt');

// Move 'old.txt' to 'new.txt'
Storage::disk('{fs/private}')->move('old.txt', 'new.txt');
```

### Prepending and appending content

The `prepend` and `append` methods allow you to write to the start or end of a file:

```php
// Write 'prepended text' to the start of 'example.log'
Storage::disk('{fs/private}')->prepend('example.log', 'prepended text');

// Write 'appended text' to the end of 'example.log'
Storage::disk('{fs/private}')->append('example.log', 'appended text');
```

## Retrieving files

Use the `get` method to retreive a file's contents. The raw string contents of the file will be returned by the method.

```php
use Illuminate\Support\Facades\Storage;

// Get the contents of 'foo.txt'
$contents = Storage::disk('{fs}')->('foo.txt');
```

If your file contains JSON, you can use the `json` method to retreive the file and decode its contents instead:

```php
// Get and decode the contents of 'config.json'
$config = Storage::disk('{fs}')->json('config.json');
```

### Check if file exists

To check if a file exists or not, use the `exists` method:

```php
if (Storage::disk('{fs}')->exists('image.jpg')) {
  // File 'image.jpg' exists on filesystem
} else {
  // File 'image.jpg' does not exist on filesystem
}
```

### File URLs

Blueprint comes with placeholders that can help determine the public `{fs}` file URL:

```php
Storage::disk('{fs}')->put('dog.txt', 'woof');

$url = "{webroot/fs}/dog.txt";
```

## Deleting files

Using the `delete` method, an extension can remove a single file or array of multiple files:

```php
use Illuminate\Support\Facades\Storage;

// Delete 'foo/bar.txt'
Storage::disk('{fs/private}')->delete('foo/bar.txt');
```

## Directories

Throwing all your files in the same folder might work for your local `Downloads/` directory, but when writing extensions it's nice to keep things tidy.

### Get directory contents

The `files` and `directories` methods can be used to return arrays of all files/directories in a given directory.

```php
use Illuminate\Support\Facades\Storage;

// Array of all files in 'parent_directory'
$files = Storage::disk('{fs}')->files('parent_directory');

// Array of all folders in 'parent_directory'
$directories = Storage::disk('{fs}')->directories('parent_directory');
```

### Create a directory

The `makeDirectory` method will create a directory, including any needed subdirectories:

```php
// Creates directory 'foo'
Storage::disk('{fs}')->makeDirectory("foo");

// Creates directory 'foo' and it's parent 'bar' if it does not exist
Storage::disk('{fs}')->makeDirectory("bar/foo");
```

### Delete a directory

The `deleteDirectory` method will remove a directory and all of its files:

```php
// Deletes directory 'bar' and all of its contents
Storage::disk('{fs}')->deleteDirectory('bar');
```
