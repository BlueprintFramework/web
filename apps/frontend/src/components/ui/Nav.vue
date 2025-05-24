<template>
  <nav class="navbar navbar-expand-md bg-body border-bottom">
    <div class="container">
      <NuxtLink to="/" class="navbar-brand">
        <BrandEmblem :size="24" class="d-inline-block align-text-top me-1" />
        Blueprint
      </NuxtLink>
      <button
        class="btn btn-secondary btn-square"
        type="button"
        data-bs-toggle="collapse"
        data-bs-target="#navbar-collapse-1"
        aria-controls="navbar-collapse-1"
        aria-expanded="false"
        aria-label="Toggle navigation"
      >
        <Icon name="lucide:menu" :size="14" mode="svg" />
      </button>
      <div class="collapse navbar-collapse" id="navbar-collapse-1">
        <ul class="navbar-nav me-auto mb-2 mb-lg-0">
          <li class="nav-item">
            <a class="nav-link" href="#">Home</a>
          </li>
          <li class="nav-item">
            <a class="nav-link active" aria-current="page" href="#">Docs</a>
          </li>
          <li class="nav-item dropdown">
            <a
              class="nav-link dropdown-toggle"
              href="#"
              role="button"
              data-bs-toggle="dropdown"
              aria-expanded="false"
            >
              Product
            </a>
            <ul class="dropdown-menu mt-lg-2 rounded-top-0">
              <li><a class="dropdown-item" href="#">Page builder</a></li>
              <li><a class="dropdown-item" href="#">Form builder</a></li>
              <li><hr class="dropdown-divider" /></li>
              <li><a class="dropdown-item" href="#">Plan and pricing</a></li>
            </ul>
          </li>
          <li class="nav-item">
            <a class="nav-link disabled">Blog</a>
          </li>
        </ul>
        <form class="d-flex" role="search">
          <input
            class="form-control me-2"
            type="search"
            placeholder="Search docs"
            aria-label="Search"
          />
          <button class="btn btn-primary" type="submit">Search</button>
        </form>
      </div>
    </div>
  </nav>
</template>
