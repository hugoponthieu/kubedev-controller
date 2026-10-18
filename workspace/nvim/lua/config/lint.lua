local lint = require("lint")

-- Seuls les linters déjà installés sur cette machine sont activés.
-- Le nom à gauche est le filetype Neovim (`:set filetype?`).
lint.linters_by_ft = {
  c = { "clangtidy" },
  cpp = { "clangtidy" },
  dockerfile = { "hadolint" },
  rust = { "clippy" },

  -- Exemples à décommenter après installation de l'exécutable :
  -- python = { "ruff" },
  -- javascript = { "eslint_d" },
  -- javascriptreact = { "eslint_d" },
  -- typescript = { "eslint_d" },
  -- typescriptreact = { "eslint_d" },
  -- sh = { "shellcheck" },
  -- markdown = { "markdownlint-cli2" },
}

local function run_lint()
  lint.try_lint(nil, {
    ignore_errors = true,
  })
end

vim.api.nvim_create_autocmd("BufWritePost", {
  group = vim.api.nvim_create_augroup("user_lint", { clear = true }),
  callback = run_lint,
})

vim.keymap.set("n", "<leader>cl", run_lint, { desc = "Relancer le lint" })
