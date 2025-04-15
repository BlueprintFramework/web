<div class="position-relative p-4 text-body bg-body border rounded-4 d-flex align-items-center">
  <div class="me-3">
    <i class="bi bi-book h2"></i>
  </div>
  <p class="me-3 my-0">
    Written by those who've walked the path. Want to improve our guides? Contribute and help build something awesome!
  </p>
  <a href="https://github.com/BlueprintFramework/web/tree/main/docs/pages/developing-extensions">
    <button class="btn btn-primary px-4 rounded-pill placeholder-wave" type="button">
      Contribute
    </button>
  </a>
</div><br>

# Admin configuration
<h4 class="fw-light">Add configuration options to your extension's admin page.</h4><br/>

This example demonstrates how to create an admin controller that loads and saves extension-specific configuration values using the `$blueprint` database utility. It also includes input validation via a custom form request.

<br/>

## Controller

<br/>

### Controller setup

After creating your [custom controller](?page=developing-extensions/Custom-controllers), you can now extend it to load and store configuration values.

Start by importing the necessary classes:

```php
namespace Pterodactyl\Http\Controllers\Admin\Extensions\{identifier};

use Illuminate\View\View;
use Illuminate\View\Factory as ViewFactory;
use Pterodactyl\Http\Controllers\Controller;
use Pterodactyl\Contracts\Repository\SettingsRepositoryInterface;
use Pterodactyl\Http\Requests\Admin\AdminFormRequest;
use Illuminate\Http\RedirectResponse;

use Pterodactyl\BlueprintFramework\Libraries\ExtensionLibrary\Admin\BlueprintAdminLibrary as BlueprintExtensionLibrary;
```

Most of these classes should be familiar from the [custom controller](?page=developing-extensions/Custom-controllers) setup.

Here’s what’s new:

- `SettingsRepositoryInterface`: Used to persist settings to the database.
- `AdminFormRequest`: Handles form input validation.
- `RedirectResponse`: Used to return the user back to the admin page after saving changes.

Inject the `SettingsRepositoryInterface` into your controller constructor:

```php
public function __construct(
    private ViewFactory $view,
    private BlueprintExtensionLibrary $blueprint,
    private SettingsRepositoryInterface $settings,
) {}
```

<br/>

### Loading configuration and setting defaults

To handle configuration, extend your `index()` function to retrieve the current values (or fall back to defaults if none exist). 

For this example, we will add the configuration `theme` and `customName`:

```php
public function index(): View
{
    $theme = $this->blueprint->dbGet('{identifier}', 'theme');
    $customName = $this->blueprint->dbGet('{identifier}', 'customName');

    $defaulttheme = 0;
    $defaultCustomName = "superuser";

    if ($theme == "") {
        $this->blueprint->dbSet('{identifier}', 'theme', $defaulttheme);
        $theme = $defaulttheme;
    }
    if ($customName == "") {
        $this->blueprint->dbSet('{identifier}', 'customName', $defaultCustomName;
        $customName = $defaultCustomName;
    }

    return $this->view->make(
      'admin.extensions.{identifier}.index', [
        'theme' => $theme,
        'customName' => $customName,
        'root' => "/admin/extensions/{identifier}",
        'blueprint' => $this->blueprint,
    ]
);

```

The `$blueprint->dbGet()` function is used to retrieve values from the database. If no value is found, the default is applied and saved using dbSet().
At the end of the function, the Blade view is returned along with the configuration values.

<div class="p-2 border-start border-4 mb-5">
    <i class="bi bi-journal-text text-primary me-1"></i>
    For more information about Blueprints database helper functions, take a look at the <a href="?page=documentation/$blueprint">$blueprint documentation</a>
</div>

### Saving configuration

To save the configuration, add an `update()` function to your controller. This will be called when the user submits the admin form located in your view via `PATCH` request.

```php
public function update({identifier}SettingsFormRequest $request): RedirectResponse
{
    foreach ($request->normalize() as $key => $value) {
        $this->settings->set('{identifier}::' . $key, $value);
    }

    return redirect()->route('admin.extensions.{identifier}.index');
}

```

The `update()` function accepts a validated form request and saves each setting using the `SettingsRepositoryInterface`. At the end you will be redirect to the `index()` function to refresh all the data on your extension's admin page to check if everything got saved correctly.

<br/>

### Input validation

Finally, create a form request class at the bottom of the controller file (outside of your controller class) to validate incoming form data:

```php
class {identifier}SettingsFormRequest extends AdminFormRequest
{
    public function rules(): array
    {
        return [
            'theme' => 'string',
            'customName' => 'string',
        ];
    }

    public function attributes(): array
    {
        return [
            'theme' => 'Theme',
            'customName' => 'Custom Name,
        ];
    }
}

```

- `rules()` defines the validation rules for each input field.

- `attributes()` provides user-friendly names for use in validation error messages.

<div class="p-2 border-start border-4 mb-5">
    <i class="bi bi-globe text-primary me-1"></i>
    For more information about validation rules, see <a href="https://laravel.com/docs/10.x/validation#available-validation-rules">Laravel’s validation documentation</a>.
</div>

## View

<br/>

### Form structure

```php
<form id="config-form" action="" method="POST">
  ...
  {{ csrf_field() }}
  <button type="submit" name="_method" value="PATCH" class="btn btn-primary">Save Changes</button>
</form>
```

This is the basic form structure you can use in your view. Don't be surprised that the `POST` method is used in the form and not `PATCH`.<br/>
This is because [HTML Forms do not support this request method](https://laravel.com/docs/10.x/routing#form-method-spoofing). So that we can still send the message as `PATCH`, meta information is added to the submit button. Here you can see that the request type is set to `PATCH` via `name="_method"` and `value="PATCH"`.

`{{ csrf_field() }}` inserts a hidden CSRF token input into the form, which Laravel uses to protect against Cross-Site Request Forgery (CSRF) attacks. It ensures that the form submission is coming from an authenticated and trusted source.

<br/>

### Form fields

To go along with the current example, we add a text `input` for `customName` and a `dropdown` for `theme` to the `form`:
```html
<form id="config-form" action="" method="POST">
  <input 
    type="text"
    name="customName"
    id="customName"
    value="{{ $customName }}"
    placeholder="type here"
    class="form-control"
  />
  <select class="form-control" name="theme">
    <option value="0" @if($theme == "0") selected @endif>Theme 1</option>
    <option value="1" @if($theme == "1") selected @endif>Theme 2</option>
    <option value="2" @if($theme == "2") selected @endif>Theme 3</option>
  </select>

  {{ csrf_field() }}
  <button type="submit" name="_method" value="PATCH" class="btn btn-primary">Save Changes</button>
</form>
```

And now you're done. You should be able to load and save your admin configuration.


<div class="btn-group docs-navigator" role="group" aria-label="Navigation" style="float: right">
  <a href="?page=developing-extensions/Custom-controllers" class="btn btn-dark bg-light-subtle border-0 rounded-start-pill">Previous</a>
  <a href="?page=developing-extensions/Dashboard-wrappers" class="btn btn-dark bg-light-subtle border-0 rounded-end-pill">Next</a>
</div>