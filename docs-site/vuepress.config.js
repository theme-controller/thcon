const path = require('path');

module.exports = {
  lang: 'en-US',
  title: 'thcon',
  description: 'A configurable theme-controller',
  theme: path.resolve(__dirname, '.vuepress/theme'),
  themeConfig: {
    navbar: [
      'guide.md',
      {
        text: 'Apps',
        children: [
          {
            text: 'Cross-Platform',
            children: [
              '/app/alacritty.md',
              '/app/atom.md',
              '/app/sublime-text-3.md',
              '/app/sublime-text-4.md',
              '/app/vscode.md',
              '/app/vim.md',
            ],
          },
          {
            text: 'macOS',
            children: [
              '/app/macos.md',
              '/app/iterm.md',
              '/app/terminal-app.md',
            ],
          },
          {
            text: 'Linux/BSD',
            children: [
              '/app/gtk.md',
              '/app/gnome-shell.md',
              '/app/gnome-terminal.md',
              '/app/konsole.md',
              '/app/plasma.md',
            ],
          },
        ],
      },
    ],
    repo: 'https://github.com/theme-controller/thcon',
    repoLabel: 'GitHub',
    docsDir: 'docs-site/docs',
    contributors: false,
    lastUpdated: false,
  },
footer: `<img src="/images/powered-by-vercel.svg" alt="Powered by Vercel"/>`,
footerHtml: true
};
