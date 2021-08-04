module.exports = {
  lang: 'en-US',
  title: 'thcon',
  description: 'A configurable theme-controller',
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
              '/app/sublime-text.md',
              '/app/vscode.md',
              '/app/vim.md'
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
  }
};
