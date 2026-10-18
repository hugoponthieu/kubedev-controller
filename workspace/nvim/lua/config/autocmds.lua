vim.api.nvim_create_autocmd("TextYankPost", {
  group = vim.api.nvim_create_augroup("user_yank_highlight", { clear = true }),
  callback = function()
    vim.hl.on_yank()
  end,
})

vim.filetype.add({
  pattern = {
    [".*/Vagrantfile%..*"] = "ruby",
  },
})
