-- vim.pack fait partie de Neovim 0.12. Il clone les dépôts dans stdpath("data")
-- et crée nvim-pack-lock.json à la racine de cette configuration.
vim.api.nvim_create_autocmd("PackChanged", {
  group = vim.api.nvim_create_augroup("user_pack_build", { clear = true }),
  callback = function(event)
    if event.data.spec.name == "markdown-preview.nvim"
        and (event.data.kind == "install" or event.data.kind == "update") then
      if not event.data.active then
        vim.cmd.packadd("markdown-preview.nvim")
      end
      vim.fn["mkdp#util#install"]()
    end
  end,
})

vim.pack.add({
  { src = "https://github.com/nvim-lua/plenary.nvim" },
  { src = "https://github.com/folke/tokyonight.nvim",                    version = vim.version.range("*") },
  { src = "https://github.com/nvim-treesitter/nvim-treesitter" },
  { src = "https://github.com/saghen/blink.cmp",                         version = vim.version.range("*") },
  { src = "https://github.com/rafamadriz/friendly-snippets" },
  { src = "https://github.com/nvim-tree/nvim-web-devicons" },
  { src = "https://github.com/lewis6991/gitsigns.nvim" },
  { src = "https://github.com/akinsho/bufferline.nvim",                  version = vim.version.range("*") },
  { src = "https://github.com/MunifTanjim/nui.nvim" },
  { src = "https://github.com/folke/noice.nvim",                         version = vim.version.range("*") },
  { src = "https://github.com/nvim-telescope/telescope.nvim" },
  { src = "https://github.com/folke/trouble.nvim",                       version = vim.version.range("*") },
  { src = "https://github.com/folke/snacks.nvim" },
  { src = "https://github.com/MagicDuck/grug-far.nvim" },
  { src = "https://github.com/stevearc/oil.nvim" },
  { src = "https://github.com/mason-org/mason.nvim" },
  { src = "https://github.com/WhoIsSethDaniel/mason-tool-installer.nvim" },
  { src = "https://github.com/neovim/nvim-lspconfig" },
  { src = "https://github.com/mfussenegger/nvim-lint" },
  { src = "https://github.com/iamcco/markdown-preview.nvim",             version = vim.version.range("*") },
  { src = "https://github.com/sphamba/smear-cursor.nvim" },
}, { confirm = false })

-- Mason installe les outils manquants et ajoute ses exécutables au PATH de
-- Neovim. Cette liste est commune à Linux et macOS.
require("mason").setup()
require("mason-tool-installer").setup({
  ensure_installed = {
    "bash-language-server",
    "clangd",
    "dockerfile-language-server",
    "hadolint",
    "rust-analyzer",
    "typescript-language-server",
    "lua-language-server",
    "json-lsp",
    "yaml-language-server",
    "taplo",
    "vue-language-server",
    "ruby-lsp",
  },
  run_on_start = true,
})

vim.g.mkdp_auto_start = 0
vim.g.mkdp_auto_close = 1
vim.g.mkdp_refresh_slow = 0
vim.g.mkdp_theme = "dark"
vim.g.mkdp_filetypes = { "markdown" }
vim.cmd.packadd("markdown-preview.nvim")

require("smear_cursor").setup({
  smear_between_buffers = true,
  smear_between_neighbor_lines = true,
  smear_insert_mode = true,
  stiffness = 0.8,
  trailing_stiffness = 0.6,
  stiffness_insert_mode = 0.7,
  trailing_stiffness_insert_mode = 0.7,
  damping = 0.95,
  damping_insert_mode = 0.95,
  distance_stop_animating = 0.5,
})

-- Thème par défaut de LazyVim. Le chargement protégé garde Neovim utilisable
-- avec habamax si le thème n'a pas pu être installé.
require("tokyonight").setup({
  style = "moon",
  styles = {
    comments = { italic = true },
    keywords = { italic = true },
    sidebars = "dark",
    floats = "dark",
  },
  plugins = { all = true },
})

local theme_ok = pcall(vim.cmd.colorscheme, "tokyonight-moon")
if not theme_ok then
  vim.cmd.colorscheme("habamax")
end

local treesitter = require("nvim-treesitter")
treesitter.setup({})

local parsers = {
  "bash",
  "c",
  "cpp",
  "css",
  "dockerfile",
  "html",
  "java",
  "javascript",
  "json",
  "kotlin",
  "lua",
  "markdown",
  "markdown_inline",
  "python",
  "rust",
  "sql",
  "toml",
  "tsx",
  "typescript",
  "vue",
  "vim",
  "vimdoc",
  "yaml",
  "ruby",
}

-- install() est asynchrone et ne retélécharge pas les parseurs déjà présents.
treesitter.install(parsers)

vim.api.nvim_create_autocmd("FileType", {
  group = vim.api.nvim_create_augroup("user_treesitter", { clear = true }),
  pattern = {
    "sh",
    "c",
    "cpp",
    "css",
    "dockerfile",
    "html",
    "java",
    "javascript",
    "javascriptreact",
    "json",
    "kotlin",
    "lua",
    "markdown",
    "python",
    "rust",
    "sql",
    "toml",
    "typescript",
    "typescriptreact",
    "vue",
    "vim",
    "help",
    "yaml",
    "ruby",
  },
  callback = function()
    pcall(vim.treesitter.start)
  end,
})

-- Même moteur de complétion que LazyVim : LSP, mots des buffers, chemins et
-- snippets. Le moteur fuzzy Lua fonctionne sans compilation ni téléchargement.
local cmp = require("blink.cmp")
cmp.setup({
  keymap = {
    preset = "default",
    ["<CR>"] = { "select_and_accept", "fallback" },
  },
  fuzzy = { implementation = "lua" },
  completion = {
    documentation = { auto_show = true, auto_show_delay_ms = 300 },
    ghost_text = { enabled = true },
  },
  sources = {
    default = { "lsp", "path", "snippets", "buffer" },
    providers = {
      lsp = { fallbacks = {} },
    },
  },
  cmdline = {
    enabled = true,
    keymap = { preset = "cmdline" },
    sources = { "cmdline", "buffer" },
    completion = {
      menu = { auto_show = true },
    },
  },
  signature = { enabled = true },
})

require("noice").setup({
  -- Noice centralise la progression LSP et les notifications.
  lsp = {
    progress = { enabled = true },
  },
  cmdline = {
    enabled = true,
    view = "cmdline_popup",
  },
  presets = {
    bottom_search = true,
    command_palette = true,
    long_message_to_split = true,
    lsp_doc_border = true,
  },
})

require("gitsigns").setup({
  current_line_blame = false,
  on_attach = function(bufnr)
    local gs = require("gitsigns")
    local function map(mode, lhs, rhs, desc)
      vim.keymap.set(mode, lhs, rhs, { buffer = bufnr, desc = desc })
    end

    map("n", "]h", function()
      gs.nav_hunk("next")
    end, "Changement Git suivant")
    map("n", "[h", function()
      gs.nav_hunk("prev")
    end, "Changement Git précédent")
    map("n", "<leader>ghp", gs.preview_hunk, "Aperçu du changement Git")
    map("n", "<leader>gd", gs.preview_hunk_inline, "Aperçu du changement Git dans le fichier")
    map("n", "<leader>ghs", gs.stage_hunk, "Stage du changement Git")
    map("n", "<leader>ghr", gs.reset_hunk, "Annuler le changement Git")
    map("n", "<leader>ghb", function()
      gs.blame_line({ full = true })
    end, "Auteur de la ligne Git")
    map("n", "<leader>gb", function()
      gs.blame_line({ full = true })
    end, "Blame de la ligne Git")

    map("v", "<leader>ghs", function()
      gs.stage_hunk({ vim.fn.line("."), vim.fn.line("v") })
    end, "Stage de la sélection Git")
    map("v", "<leader>ghr", function()
      gs.reset_hunk({ vim.fn.line("."), vim.fn.line("v") })
    end, "Annuler la sélection Git")
  end,
})

require("bufferline").setup({
  options = {
    mode = "buffers",
    diagnostics = "nvim_lsp",
    always_show_bufferline = true,
    show_buffer_close_icons = true,
    show_close_icon = false,
    separator_style = "slant",
  },
})

require("telescope").setup({
  defaults = {
    layout_strategy = "horizontal",
    layout_config = { prompt_position = "top" },
    sorting_strategy = "ascending",
  },
})

-- Panneau des diagnostics, comme dans LazyVim.
require("trouble").setup({})

-- Panneau de recherche/remplacement multi-fichiers utilisé par LazyVim.
require("grug-far").setup({ headerMaxWidth = 80 })

require("snacks").setup({
  explorer = { enabled = true },
  scroll = {
    enabled = true,
    animate = {
      duration = { step = 10, total = 180 },
      easing = "linear",
    },
    animate_repeat = {
      delay = 100,
      duration = { step = 5, total = 60 },
      easing = "linear",
    },
  },
  picker = {
    enabled = true,
    sources = {
      explorer = {
        layout = {
          preset = "sidebar",
          preview = false,
          layout = {
            position = "left",
            width = 35,
          },
        },
      },
    },
  },
})

require("oil").setup({
  view_options = { show_hidden = true },
  keymaps = {
    ["q"] = { "actions.close", mode = "n" },
  },
})
