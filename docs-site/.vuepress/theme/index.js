// see https://github.com/vuepress/vuepress-next/blob/4e0fd99189ffca4566dd4ef4f9bd9eb0c5aee64b/docs/advanced/cookbook/extending-a-theme.md#extend-default-theme

const { path } = require('@vuepress/utils');

module.exports = {
  name: 'vuepress-theme-local',
  extends: '@vuepress/theme-default',
  layouts: {
    Layout: path.resolve(__dirname, 'layouts/Layout.vue'),
  },
};
