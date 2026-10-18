local map = vim.keymap.set
local builtin = require("telescope.builtin")

-- Formate avec le LSP, sauvegarde, et laisse toujours l'éditeur en mode normal.
local function format_and_save()
  if vim.bo.modifiable then
    pcall(vim.lsp.buf.format, { async = false, timeout_ms = 2000 })
    vim.cmd.write()
  end
end

map("n", "<C-s>", format_and_save, { desc = "Formater et sauvegarder" })
map({ "i", "x", "s" }, "<C-s>", function()
  vim.cmd.stopinsert()
  format_and_save()
end, { desc = "Mode normal, formater et sauvegarder" })

-- Recherche et navigation avec Telescope.
map("n", "<leader><leader>", builtin.find_files, { desc = "Fichiers" })
map("n", "<leader>/", builtin.live_grep, { desc = "Grep dans le projet" })
map("n", "<leader>fw", builtin.grep_string, { desc = "Mot sous le curseur" })
map("n", "<leader>fb", builtin.buffers, { desc = "Buffers ouverts" })
map("n", "<leader>fr", builtin.oldfiles, { desc = "Fichiers récents" })
map("n", "<leader>fh", builtin.help_tags, { desc = "Aide Neovim" })

map("n", "<leader>ss", function()
  if #vim.lsp.get_clients({ bufnr = 0 }) > 0 then
    builtin.lsp_document_symbols()
  else
    builtin.treesitter()
  end
end, { desc = "Symboles du fichier" })

-- Panneau LazyVim avec des champs séparés pour la recherche, le remplacement,
-- le filtre de fichiers et les options ripgrep.
map({ "n", "x" }, "<leader>sr", function()
  local extension = vim.bo.buftype == "" and vim.fn.expand("%:e")
  require("grug-far").open({
    transient = true,
    prefills = {
      filesFilter = extension and extension ~= "" and "*." .. extension or nil,
    },
  })
end, { desc = "Rechercher et remplacer" })

-- Buffers affichés dans la barre supérieure.
map("n", "H", "<cmd>bprevious<cr>", { desc = "Buffer précédent" })
map("n", "L", "<cmd>bnext<cr>", { desc = "Buffer suivant" })
map("n", "<leader>bd", function()
  Snacks.bufdelete()
end, { desc = "Fermer le buffer" })
map("n", "<leader>bo", function()
  Snacks.bufdelete.other()
end, { desc = "Fermer les autres buffers" })

-- Déplacement entre les fenêtres, y compris l'explorateur latéral.
map("n", "<C-h>", "<C-w>h", { desc = "Fenêtre gauche" })
map("n", "<C-j>", "<C-w>j", { desc = "Fenêtre basse" })
map("n", "<C-k>", "<C-w>k", { desc = "Fenêtre haute" })
map("n", "<C-l>", "<C-w>l", { desc = "Fenêtre droite" })

-- Explorateur en panneau latéral, comme dans LazyVim.
map("n", "<leader>e", function()
  Snacks.explorer()
end, { desc = "Explorateur de fichiers" })

-- Oil reste pratique pour manipuler le dossier courant comme un buffer.
map("n", "-", "<cmd>Oil<cr>", { desc = "Dossier parent" })

map("n", "<Esc>", "<cmd>nohlsearch<cr>")
map("t", "<C-n>", "<C-\\><C-n>", { desc = "Quitter le mode terminal" })

map("n", "<leader>cd", function()
  vim.diagnostic.open_float({ scope = "cursor", border = "rounded", source = true })
end, { desc = "Diagnostic sous le curseur" })

map("n", "<leader>cp", "<Plug>MarkdownPreviewToggle", { desc = "Aperçu Markdown dans le navigateur" })

-- Diagnostics de tous les fichiers connus ou du fichier courant.
map("n", "[e", function()
  vim.diagnostic.jump({ count = vim.v.count1, severity = vim.diagnostic.severity.ERROR, float = true })
end, { desc = "Erreur suivante" })
map("n", "]e", function()
  vim.diagnostic.jump({ count = -vim.v.count1, severity = vim.diagnostic.severity.ERROR, float = true })
end, { desc = "Erreur précédente" })

map("n", "<leader>xx", builtin.diagnostics, { desc = "Diagnostics (Telescope)" })
map("n", "<leader>xX", "<cmd>Trouble diagnostics toggle filter.buf=0<cr>", { desc = "Diagnostics du fichier (Trouble)" })
