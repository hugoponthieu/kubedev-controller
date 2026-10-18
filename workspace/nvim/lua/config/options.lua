local opt = vim.opt

opt.number = true
opt.relativenumber = true
opt.mouse = "a"
opt.clipboard = "unnamedplus"
opt.breakindent = true
opt.undofile = true
opt.ignorecase = true
opt.smartcase = true
opt.signcolumn = "yes"
opt.updatetime = 250
-- Laisser le temps de saisir les raccourcis à plusieurs touches (Espace b d).
opt.timeoutlen = 1000
opt.splitright = true
opt.splitbelow = true
opt.termguicolors = true
opt.cursorline = true
opt.scrolloff = 8
opt.confirm = true
opt.completeopt = { "menu", "menuone", "noselect", "popup" }

opt.expandtab = true
opt.shiftwidth = 2
opt.tabstop = 2
opt.smartindent = true
