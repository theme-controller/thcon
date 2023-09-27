---
title: Apps
layout: layouts/main.html
eleventyNavigation:
  key: apps
  parent: root
---

<table class="compat-matrix">
  <caption>
    Compatibility matrix showing which apps are supported on which platforms,
    and which apps need manual configuration.
  </caption>

  <thead>
    <tr>
      <th>App</td>
      <th>Needs<br/>Config</td>
      <th>Linux</td>
      <th>macOS</td>
      <th>FreeBSD</td>
      <th>Windows</td>
    </tr>
  </thead>
  <tbody>
{%- for app in collections.app -%}
    <tr>
      <td>
        <a href="{{ app.url }}">{{ app.data.title }}</a>
      </td>
      <td>{{ "Yes" if app.data.needs_config else "No" }}</td>
      <td>{{ "Yes" if app.data.platforms.includes("linux") else "No"}}</td>
      <td>{{ "Yes" if app.data.platforms.includes("macos") else "No"}}</td>
      <td>{{ "Yes" if app.data.platforms.includes("freebsd") else "No"}}</td>
      <td>{{ "Yes" if app.data.platforms.includes("windows") else "No"}}</td>
    </tr>
{%- endfor -%}
  </tbody>
</table>
