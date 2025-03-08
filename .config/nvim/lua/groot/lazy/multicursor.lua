  return {
        "smoka7/multicursors.nvim",
        event = "VeryLazy",
        dependencies = {
            "nvimtools/hydra.nvim",
        },
        opts = {
            DEBUG_MODE = false,
            create_commands = true,
            updatetime = 50, -- Number, expected value
            nowait = true,   -- Boolean, expected value
            mode_keys = {
                append = 'a',  -- String, expected value
                change = 'c',  -- String, expected value
                extend = 'e',  -- String, expected value
                insert = 'i',  -- String, expected value
            },
            hint_config = {
                float_opts = {
                    border = 'none',  -- String, expected value
                },
                position = 'bottom',  -- String, expected value
            },
            generate_hints = {
                normal = true,
                insert = true,
                extend = true,
                config = {
                    column_count = nil,       -- Nil or Number, expected value
                    max_hint_length = 25,     -- Number, expected value
                },
            },
        },
        cmd = { 'MCstart', 'MCvisual', 'MCclear', 'MCpattern', 'MCvisualPattern', 'MCunderCursor' },
        keys = {
            {
                mode = { 'v', 'n' },
                '<Leader>m',
                '<cmd>MCstart<cr>',
                desc = 'Create a selection for selected text or word under the cursor',
            },
        },
    }
