-- nvim-lspconfig fournit les descriptions des serveurs. Mason installe leurs
-- exécutables au premier démarrage et les ajoute au PATH de Neovim.
local servers = {
  "bashls",        -- Bash / shell
  "clangd",        -- C / C++
  "dockerls",      -- Dockerfile
  "rust_analyzer", -- Rust
  "ts_ls",         -- typescript-language-server
  "lua_ls",        -- lua-language-server
  "jsonls",        -- vscode-json-language-server
  "yamlls",        -- yaml-language-server
  "taplo",         -- TOML
  "vue_ls",        -- Vue 3 / Nuxt 3
  "protols",
  "ruby_lsp",
  -- Décommenter une ligne après avoir installé le serveur correspondant :
  -- "pyright",                -- pyright-langserver
  -- "jdtls",                  -- Java
  -- "kotlin_language_server", -- Kotlin
  -- "tailwindcss",            -- Tailwind CSS
  -- "marksman",               -- Markdown
  -- "sqls",                   -- SQL
}

-- Vue 3 / Nuxt 3 utilisent le mode hybride : vue_ls pour template/CSS et
-- ts_ls pour JavaScript/TypeScript, y compris dans les fichiers .vue.
local vue_language_server_path = vim.fn.stdpath("data")
    .. "/mason/packages/vue-language-server/node_modules/@vue/language-server"

vim.lsp.config("ts_ls", {
  filetypes = {
    "javascript",
    "javascriptreact",
    "typescript",
    "typescriptreact",
    "vue",
  },
  init_options = {
    plugins = {
      {
        name = "@vue/typescript-plugin",
        location = vue_language_server_path,
        languages = { "vue" },
        configNamespace = "typescript",
      },
    },
  },
})

-- Blink annonce à tous les serveurs LSP les fonctions de complétion prises en
-- charge (snippets, documentation, imports automatiques, etc.).
vim.lsp.config("*", {
  capabilities = require("blink.cmp").get_lsp_capabilities(),
})

vim.lsp.config("rust_analyzer", {
  settings = {
    ["rust-analyzer"] = {
      cargo = {
        allFeatures = true,
      },
    },
  },
})

vim.lsp.enable(servers)

vim.api.nvim_create_autocmd("LspAttach", {
  group = vim.api.nvim_create_augroup("user_lsp", { clear = true }),
  callback = function(event)
    local bufnr = event.buf
    local map = function(lhs, rhs, desc)
      vim.keymap.set("n", lhs, rhs, { buffer = bufnr, desc = desc })
    end

    map("gd", vim.lsp.buf.definition, "Aller à la définition")
    map("gr", vim.lsp.buf.references, "Références")
    map("K", vim.lsp.buf.hover, "Documentation")
    map("<leader>rn", vim.lsp.buf.rename, "Renommer")
    map("<leader>ca", vim.lsp.buf.code_action, "Action de code")
    map("<leader>cf", function()
      vim.lsp.buf.format({ async = true })
    end, "Formater")
  end,
})

vim.diagnostic.config({
  virtual_text = true,
  severity_sort = true,
  float = { border = "rounded", source = true },
  signs = true,
})
