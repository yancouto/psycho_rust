-- This function wraps all event functions of the given table in coroutine yields
-- We use this since this is not doable from the rlua API

local function is_snake_case(name)
    return name:match("^[a-z_]+$") ~= nil
end

return function(internal_functions)
    local lm = {}

    for name, func in pairs(internal_functions) do
        -- Our convention is that snake case functions are events
        -- and the rest are helpers
        if is_snake_case(name) then
            lm[name] = function(...)
                return coroutine.yield(func(...))
            end
        else
            lm[name] = func
        end
    end

    return lm
end