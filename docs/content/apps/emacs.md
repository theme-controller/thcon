---
title: Emacs
layout: layouts/app.html
platforms: [ macos, linux, freebsd, openbsd ]
category: editor

config_section: emacs
url: https://www.gnu.org/software/emacs/
setup: |
  A small snippet of Emacs Lisp is required, but no external dependencies are
  required otherwise:

  1. Replace your existing `load-theme` call with this snippet:
  ```emacs-lisp
  (let ((thcon-emacs-el "~/.local/share/thcon/emacs.el"))
    (require 'filenotify)

    (defun reload-thcon-config (event)
      (when (file-readable-p (nth 2 event))
        (load-file (nth 2 event))))

    (file-notify-add-watch
      thcon-emacs-el '(change) 'reload-thcon-config)

    (when (file-readable-p thcon-emacs-el)
      (load-file thcon-emacs-el)))
  ```
  2. Split your dark mode and light mode configs into separate `.el` files
  3. Use the paths to those files in `thcom.toml` (examples below)

  In other words:
  ```diff-emacs-lisp
   ; ~/.emacs.d/init.el
  -(load-theme 'tango)
  +(let ((thcon-emacs-el "~/.local/share/thcon/emacs.el"))
  +  (require 'filenotify)

  +(defun reload-thcon-config (event)
  +  (when (file-readable-p (nth 2 event))
  +    (load-file (nth 2 event))))

  +(file-notify-add-watch
  +  thcon-emacs-el '(change) 'reload-thcon-config)

  +(when (file-readable-p thcon-emacs-el)
  +  (load-file thcon-emacs-el)))
  ```

  ```emacs-lisp
  ; ~/.emacs.d/dark.thcon.el

  (load-theme 'tsdh-dark)
  ; ...
  ```

  ```emacs-lisp
  ; ~/.emacs.d/light.thcon.el

  (load-theme 'tsdh-light)
  ; ...
  ```

options:
  - key: dark
    default: (none)
    description: Path to the file defining dark mode settings
  - key: light
    default: (none)
    description: Path to the file defining light mode settings
example: |
  [emacs]
  dark = "~/.emacs.d/dark.thcon.el"
  light = "~/.emacs.d/light.thcon.el"
---
