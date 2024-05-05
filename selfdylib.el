;;; package --- Summary
;;; Commentary:
;;; Selfdylib:

;;; Code:
;; emacs dylib file
(setq dylib-file "~/.emacs.d/selfdylib.dylib")
(when (file-exists-p dylib-file)
  (module-load (expand-file-name dylib-file)))

(provide 'selfdylib)

;;
;;; selfdylib.el ends here
