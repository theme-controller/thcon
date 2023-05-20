const syntaxHighlight = require("@11ty/eleventy-plugin-syntaxhighlight");
const { EleventyRenderPlugin, EleventyHtmlBasePlugin } = require("@11ty/eleventy");
const eleventyNavigationPlugin = require("@11ty/eleventy-navigation");

const filters = {
  byOS(osName) {
    return (item) => "os" in item.data && item.data.os.includes(osName);
  },
  byCategory(catName) {
    return (item) => item.data.category === catName;
  },
};

module.exports = function(eleventyConfig) {
  eleventyConfig.addPassthroughCopy({
    "./public/": "/",
  });

  eleventyConfig.addPlugin(syntaxHighlight);
  eleventyConfig.addPlugin(EleventyRenderPlugin);
  eleventyConfig.addPlugin(EleventyHtmlBasePlugin);
  eleventyConfig.addPlugin(eleventyNavigationPlugin);

  eleventyConfig.addCollection("categories", function(collectionApi) {
    const knownCategories = {
      desktop: {
        name: "Desktop Environments",
        items: [],
      },
      editor: {
        name: "Editors",
        items: [],
      },
      terminal: {
        name: "Terminals",
        items: [],
      },
      other: {
        name: "Other",
        items: [],
      },
    };

    for (const item of collectionApi.getAll()) {
      const category = item.data.category;
      if (!category || !knownCategories[category]) {
        continue;
      }
      knownCategories[category].items.push(item);
    }

    for (const category of Object.values(knownCategories)) {
      category.items.sort(
        (a, b) => a.data.title.localeCompare(b.data.title)
      );
    }
    return knownCategories;
  });

  eleventyConfig.addCollection("platforms", function(collectionApi) {
    const knownPlatforms = {
      windows: {
        name: "Windows",
        url: "/platforms/windows",
        items: [],
      },
      macos: {
        name: "macOS",
        url: "/platforms/macos",
        items: [],
      },
      linux: {
        name: "Linux",
        url: "/platforms/linux",
        items: [],
      },
      freebsd: {
        name: "FreeBSD",
        url: "/platforms/freebsd",
        items: [],
      },
      openbsd: {
        name: "OpenBSD",
        url: "/platforms/openbsd",
        items: [],
      },
    };

    for (const item of collectionApi.getAll()) {
      if (!Array.isArray(item.data.platforms)) {
        continue;
      }

      for (const platform of item.data.platforms) {
        if (knownPlatforms[platform]) {
          knownPlatforms[platform].items.push(item);
        }
      }
    }

    for (const platform of Object.values(knownPlatforms)) {
      platform.items.sort(
        (a, b) => a.data.title.localeCompare(b.data.title)
      );
    }
    return knownPlatforms;
  });

  return {
    markdownTemplateEngine: "njk",
    htmlTemplateEngine: "njk",

    dir: {
      input: "content",
      // includes is relative to input.
      includes: "../_includes",
    },
  };
}
